//! Browser integration tests.
//!
//! Starts `testing/test-app` through `cargo leptos serve` and drives it with Chrome for Testing.
//! Every case is a test of its own (`cases`), run in a fresh or reset WebDriver session. A failing
//! test fails `cargo test`.
//!
//! Every test's timing is logged when it finishes, and a summary (slowest tests, slowest steps,
//! time spent on sessions) is printed at the end. The `PageActions` helpers (navigation, waits,
//! lookups) run as `browser_test::step`s: slow ones (> 2s) are logged as warnings, and
//! `BROWSER_TEST_LOG_STEPS=1` logs every step with its duration.
//!
//! Useful environment variables:
//! - `BROWSER_TEST_VISIBLE=1`: show the browser window.
//! - `BROWSER_TEST_PAUSE=1`: pause before each test for manual inspection.
//! - `BROWSER_TEST_DRIVER_OUTPUT=1`: forward chromedriver output.
//! - `BROWSER_TEST_FILTER=<text>`: only run the tests whose name contains `<text>`.
//! - `BROWSER_TEST_LOG_STEPS=1`: log every step of every test with its duration.
//! - `BROWSER_TEST_SESSION_REUSE=0`: give every test a fresh browser instead of resetting the one
//!   of the test before.
//! - `BROWSER_TEST_SESSION_RESET=new-context`: reset sessions by giving every test a new browser
//!   context (keeping no cache) instead of resetting the tab item by item (keeping the HTTP
//!   cache), e.g. to cross-check that no test depends on how it is reset.
//! - `BROWSER_TEST_STAYS_MS=<ms>`: stays checks also observe that long after the page settled
//!   (default 0), to find checks that pass only because they don't observe long enough.
//! - `BROWSER_TEST_PARALLELISM=<n>`: how many tests run at the same time (default 8, `1`:
//!   sequential, e.g. with `BROWSER_TEST_VISIBLE=1`).
#![cfg(not(target_arch = "wasm32"))]

mod cases;
mod common;
mod pages;
mod timing;
mod ui_tests;

use std::{
    path::Path,
    time::{Duration, Instant},
};

use browser_test::{
    BrowserTestRunner, CachedData, Cancellation, ChromeBinary, ChromeProfilesDir, DriverOutput,
    FailurePolicy, Parallelism, Pause, SessionReset, SessionReuse, StderrSummary, Timeouts,
    Visibility, thirtyfour::ChromiumLikeCapabilities,
};
use leptos_browser_test::{BuildProfile, LeptosTestAppConfig, Report};

#[tokio::test(flavor = "multi_thread")]
async fn browser_tests() -> Result<(), Report> {
    common::tracing::init_subscriber();
    // Chrome for Testing is downloaded through rustls with `ring` (browser-test's `rustls-no-provider`).
    // Installing fails only if a provider is installed already, which is as good.
    let _ = rustls::crypto::ring::default_provider().install_default();
    // Before the runner reads it for thirtyfour's lookups.
    timing::install();

    let app_start = Instant::now();
    // The test app uses no leptonic theme: it declares no `[package.metadata.leptonic]`, so
    // leptonic's build script generates none.
    let app_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../testing/test-app");
    let mut app = LeptosTestAppConfig::new(app_dir).with_app_name("leptonic test app");
    // An optimized wasm bundle (the app's `wasm-release` profile, see its Cargo.toml): pages load and
    // hydrate in less than half the time, and rebuild as fast as in dev.
    app = app.with_build_profile(BuildProfile::Release);
    // The app's own target dir (`TEST_APP_TARGET_DIR`), so that a `CARGO_TARGET_DIR` set for this
    // crate doesn't also receive the app's server and wasm builds.
    if let Some(target_dir) = std::env::var_os("TEST_APP_TARGET_DIR") {
        app = app.with_env("CARGO_TARGET_DIR", target_dir);
    }
    let app = app.start().await.map_err(Report::into_dynamic)?;
    tracing::info!(
        "Built and started the test app in {:.2}s.",
        app_start.elapsed().as_secs_f64()
    );

    // Tests run in parallel (`BROWSER_TEST_PARALLELISM=<n>`, default 8; 1: sequential), each in a
    // fresh or reset browser. They don't share app state: each loads its own page. The checks of the whole
    // run follow, sequentially (see `ui_tests::all`).
    runner()?
        .run(app.base_url(), ui_tests::all(parallelism()?))
        .await
        .map_err(Report::into_dynamic)?;

    Ok(())
}

/// `BROWSER_TEST_SESSION_RESET`: `manual` (the default) resets a session's tab item by item and
/// keeps the HTTP cache (the test-app's content-hashed wasm and scripts, with their compiled code),
/// `new-context` runs every test in a new browser context (keeping nothing), e.g. to cross-check.
fn session_reset() -> Result<SessionReset, Report> {
    match std::env::var("BROWSER_TEST_SESSION_RESET").as_deref() {
        Err(_) | Ok("" | "manual") => Ok(SessionReset::manual([CachedData::Http])),
        Ok("new-context") => Ok(SessionReset::NewContext),
        Ok(other) => Err(rootcause::report!(
            "BROWSER_TEST_SESSION_RESET is {other:?}, expected `manual` or `new-context`"
        )
        .into_dynamic()),
    }
}

/// `BROWSER_TEST_PARALLELISM`, default 8 (every parallel test is a browser of ~0.5 GB; more than 8
/// gains little on 32 threads, see `documentation/browser-tests.md`). An invalid value is an error.
fn parallelism() -> Result<Parallelism, Report> {
    Ok(Parallelism::from_env()?.unwrap_or(Parallelism::parallel(8)))
}

/// A runner with the settings of the `BROWSER_TEST_*` variables. An invalid value is an error.
fn runner() -> Result<BrowserTestRunner, Report> {
    // Ctrl-C (or SIGTERM) cancels the run: running tests stop, and chromedriver and its browsers
    // shut down instead of outliving the process.
    Ok(BrowserTestRunner::new(Cancellation::on_shutdown_signals())
        .with_report_consumer(StderrSummary)
        // The sessions' Chrome profiles (tens of MB each) go into the target dir's `tmp`, on disk,
        // rather than the system's temporary directory (often a RAM disk). Each is removed when its
        // session ends; a run removes those that killed runs left behind.
        .with_chrome_profiles_dir(ChromeProfilesDir::new(
            Path::new(env!("CARGO_TARGET_TMPDIR")).join("browser-test-profiles"),
        ))
        .with_chrome_capabilities(|caps| {
            // Chrome for Testing is extracted in user mode, so its setuid sandbox helper can't be
            // installed. Without these flags, Chrome may exit before chromedriver opens a session.
            caps.add_arg("--no-sandbox")?;
            caps.add_arg("--disable-dev-shm-usage")?;
            Ok(())
        })
        .with_failure_policy(FailurePolicy::RunAll)
        // Chrome Headless Shell: a third less memory than Chrome and faster session resets
        // (`documentation/browser-tests.md`, "Speed and memory"). Visible runs use Chrome.
        .with_headless_chrome_binary(ChromeBinary::ChromeHeadlessShell)
        // Every case is a test of its own (`cases`): a test's browser is reset and runs the next
        // test instead of starting a new one. `BROWSER_TEST_SESSION_REUSE=0`: a fresh browser per
        // test.
        .with_session_reuse(
            SessionReuse::from_env()?
                .unwrap_or(SessionReuse::enabled())
                .with_reset(session_reset()?),
        )
        .with_visibility(Visibility::from_env()?.unwrap_or_default())
        .with_pause(Pause::from_env()?.unwrap_or_default())
        .with_driver_output(DriverOutput::from_env()?.unwrap_or_default())
        // Lookups poll with thirtyfour's element queries, as long and as often as eventual
        // assertions (`timing`); an implicit wait would make every lookup of a missing element
        // block, and compound with the polling.
        .with_timeouts(
            Timeouts::builder()
                .implicit_wait_timeout(Duration::ZERO)
                .build(),
        )
        .with_element_query_wait(timing::element_query_wait()))
}
