//! Browser integration tests.
//!
//! Starts `testing/test-app` through `cargo leptos serve` and drives it with Chrome for Testing.
//! Every case is a test of its own (registered in `ui_tests`), run in a fresh or reset WebDriver session. A failing
//! test fails `cargo test`.
//!
//! The output is short: the run's milestones, warnings and errors, each failing test's failure report, and a
//! summary at the end (slowest tests, slowest steps, time spent on sessions). Each test's start and end (with its
//! timing) and every `browser_test::step` (the `Page` and fixture helpers: navigation, waits, lookups, with its
//! duration) are logged at `debug` level.
//!
//! Useful environment variables:
//! - `BROWSER_TEST_LOG=<filter>`: which logs to show (default `info`): a level (`debug`) or levels per target
//!   (`info,browser_test::step=debug`: every step). The `just` recipes set it, to edit there.
//! - `BROWSER_TEST_APP_OUTPUT=1`: forward the test-app's output (its build and its server's logs). A startup
//!   failure's report carries that output anyway.
//! - `BROWSER_TEST_VISIBLE=1`: show the browser window.
//! - `BROWSER_TEST_PAUSE=1`: pause before each test for manual inspection.
//! - `BROWSER_TEST_DRIVER_OUTPUT=1`: forward chromedriver output.
//! - `BROWSER_TEST_GROUP=<name>`: only run these exact logical groups (comma-separated names).
//!   Names and membership are explicit in `ui_tests/mod.rs`, independent of Rust module placement.
//! - `BROWSER_TEST_FILTER=<text>`: only run the tests whose name contains `<text>` (several
//!   texts separated by commas: any of them). Combined with group selection, both must match.
//!   browser-test's `TestFilter` implements selection. The checks of the whole run always run.
//! - `BROWSER_TEST_SESSION_REUSE=0`: give every test a fresh browser instead of resetting the one
//!   of the test before.
//! - `BROWSER_TEST_SESSION_RESET=new-context`: reset sessions by giving every test a new browser
//!   context (keeping no cache) instead of resetting the tab item by item (keeping the HTTP
//!   cache), e.g. to cross-check that no test depends on how it is reset.
//! - `BROWSER_TEST_PARALLELISM=<n>`: how many tests run at the same time (default 8, `1`:
//!   sequential, e.g. with `BROWSER_TEST_VISIBLE=1`).
//! - `TEST_APP_TARGET_DIR=<dir>`: build the test-app, and its site, there (default: the inherited
//!   `CARGO_TARGET_DIR`, else `testing/test-app/target`). Suites with different target dirs can
//!   run at the same time; two suites with one target dir can't.
#![cfg(not(target_arch = "wasm32"))]

mod common;
mod fixtures;
mod harness;
mod pages;
mod ui_tests;

use std::{
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use assertr::assertions::Patience;
use browser_test::{
    BrowserTestRunner, CachedData, Cancellation, ChromeBinary, DriverOutput, FailurePolicy,
    Parallelism, Pause, ProgressWarnings, SessionReset, SessionReuse, StderrSummary, Timeouts,
    Visibility, thirtyfour::ChromiumLikeCapabilities,
};
use leptos_browser_test::{BuildProfile, LeptosTestAppConfig, Report};

#[tokio::test(flavor = "multi_thread")]
async fn browser_tests() -> Result<(), Report> {
    common::tracing::init_subscriber()?;
    // Chrome for Testing is downloaded through rustls with `ring` (browser-test's `rustls-no-provider`).
    // Installing fails only if a provider is installed already, which is as good.
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Install assertr timing defaults.
    Patience::DEFAULT
        .with_timeout(Duration::from_secs(5))
        .with_interval(Duration::from_millis(20))
        .with_consistency_duration(Duration::from_millis(100))
        .set_global();

    let tests = ui_tests::all(parallelism()?)?;
    let app_start = Instant::now();
    // The test app uses no leptonic theme: it declares no `[package.metadata.leptonic]`, so
    // leptonic's build script generates none.
    let app_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../testing/test-app");
    let mut app = LeptosTestAppConfig::new(app_dir)
        .with_app_name("leptonic test app")
        .with_forward_logs(app_output()?);
    // An optimized wasm bundle (the app's `wasm-release` profile, see its Cargo.toml): pages load and
    // hydrate in less than half the time, and rebuild as fast as in dev.
    app = app.with_build_profile(BuildProfile::Release);
    // The suite's builds and its site (the wasm, JS and CSS the server serves) go into one target
    // dir, so that suites with different target dirs (other sessions, other checkouts) and
    // `just serve-test-app` can run at the same time: cargo-leptos would write the site next to
    // the app's `Cargo.toml` (`site-root`), whatever the target dir, and another build would
    // replace the files under a running suite.
    let target_dir = app_target_dir(Path::new(app_dir))?;
    app = app
        .with_env("CARGO_TARGET_DIR", &target_dir)
        .with_env("LEPTOS_SITE_ROOT", target_dir.join("browser-test-site"));
    let app = app.start().await.map_err(Report::into_dynamic)?;
    tracing::info!(
        "Built and started the test app in {:.2}s.",
        app_start.elapsed().as_secs_f64()
    );

    // Tests run in parallel (`BROWSER_TEST_PARALLELISM=<n>`, default 8; 1: sequential), each in a
    // fresh or reset browser. They don't share app state: each loads its own page. The checks of the whole
    // run follow, sequentially (see `ui_tests::all`).
    runner()?
        .run(app.base_url(), tests)
        .await
        .map_err(Report::into_dynamic)?;

    Ok(())
}

/// The test-app's target dir: `TEST_APP_TARGET_DIR` (so that a `CARGO_TARGET_DIR` set for this
/// crate doesn't also receive the app's server and wasm builds), else the inherited
/// `CARGO_TARGET_DIR`, else the app's own `target`. Relative paths are relative to the current
/// directory.
fn app_target_dir(app_dir: &Path) -> Result<PathBuf, Report> {
    let target_dir = std::env::var_os("TEST_APP_TARGET_DIR")
        .or_else(|| std::env::var_os("CARGO_TARGET_DIR"))
        .map_or_else(|| app_dir.join("target"), PathBuf::from);
    Ok(std::path::absolute(target_dir)?)
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

/// `BROWSER_TEST_APP_OUTPUT`: `1` forwards the test-app's output (its build and its server's logs), `0` (the
/// default) doesn't.
fn app_output() -> Result<bool, Report> {
    match std::env::var("BROWSER_TEST_APP_OUTPUT").as_deref() {
        Err(_) | Ok("" | "0") => Ok(false),
        Ok("1") => Ok(true),
        Ok(other) => Err(rootcause::report!(
            "BROWSER_TEST_APP_OUTPUT is {other:?}, expected `1` or `0`"
        )
        .into_dynamic()),
    }
}

/// `BROWSER_TEST_PARALLELISM`, default 8 (every parallel test is a browser of ~0.5 GB; more than 8
/// gains little on 32 threads, see `documentation/testing.md`). An invalid value is an error.
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
        .with_chrome_profiles_dir(
            Path::new(env!("CARGO_TARGET_TMPDIR")).join("browser-test-profiles"),
        )
        .with_chrome_capabilities(|caps| {
            // Chrome for Testing is extracted in user mode, so its setuid sandbox helper can't be
            // installed. Without these flags, Chrome may exit before chromedriver opens a session.
            caps.add_arg("--no-sandbox")?;
            caps.add_arg("--disable-dev-shm-usage")?;
            Ok(())
        })
        .with_failure_policy(FailurePolicy::RunAll)
        // No warning per slow step: steps are logged at `debug` level with their duration, and the
        // summary lists the slowest.
        .with_progress_warnings(ProgressWarnings::default().with_slow_step(None))
        // Chrome Headless Shell: a third less memory than Chrome and faster session resets
        // (`documentation/testing.md`, "Speed and memory"). Visible runs use Chrome.
        .with_headless_chrome_binary(ChromeBinary::ChromeHeadlessShell)
        // Every case is a test of its own (registered in `ui_tests`): a test's browser is reset and runs the next
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
        // Lookups poll (`pages::lookup`), as long and as often as eventual assertions
        // (`Patience::global()`); an implicit wait would make every lookup of a missing element block, and
        // compound with the polling.
        .with_timeouts(Timeouts::new().with_implicit_wait(Duration::ZERO)))
}
