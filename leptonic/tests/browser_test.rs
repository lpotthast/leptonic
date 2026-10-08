//! Browser integration tests.
//!
//! Starts `testing/test-app` through `cargo leptos serve` and drives it with Chrome for Testing.
//! Every test gets a fresh WebDriver session. A failing test fails `cargo test`.
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
//! - `BROWSER_TEST_PARALLELISM=<n>`: how many tests run at the same time (default 4, `1`:
//!   sequential, e.g. with `BROWSER_TEST_VISIBLE=1`).
#![cfg(not(target_arch = "wasm32"))]

mod polling;
#[macro_use]
mod cases;
mod common;
mod pages;
mod ui_tests;

use std::{
    path::Path,
    time::{Duration, Instant},
};

use browser_test::{
    BrowserTestRunner, Cancellation, ChromeProfilesDir, DriverOutput, FailurePolicy, Parallelism,
    Pause, StderrSummary, Timeouts, Visibility, thirtyfour::ChromiumLikeCapabilities,
};
use leptos_browser_test::{LeptosTestAppConfig, Report};

#[tokio::test(flavor = "multi_thread")]
async fn browser_tests() -> Result<(), Report> {
    common::tracing::init_subscriber();
    // Chrome for Testing is downloaded through rustls with `ring` (browser-test's `rustls-no-provider`).
    // Installing fails only if a provider is installed already, which is as good.
    let _ = rustls::crypto::ring::default_provider().install_default();

    let app_start = Instant::now();
    // The test app uses no leptonic theme: it declares no `[package.metadata.leptonic]`, so
    // leptonic's build script generates none.
    let app_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../testing/test-app");
    let mut app = LeptosTestAppConfig::new(app_dir).with_app_name("leptonic test app");
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

    // Tests run in parallel (`BROWSER_TEST_PARALLELISM=<n>`, default 4; 1: sequential), each in a
    // fresh browser. They don't share app state: each loads its own page. The checks of the whole
    // run follow, sequentially (see `ui_tests::all`).
    runner()?
        .run(app.base_url(), ui_tests::all(parallelism()?))
        .await
        .map_err(Report::into_dynamic)?;

    Ok(())
}

/// `BROWSER_TEST_PARALLELISM`, default 4. An invalid value is an error.
fn parallelism() -> Result<Parallelism, Report> {
    Ok(Parallelism::from_env()?.unwrap_or(Parallelism::parallel(4)))
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
        .with_visibility(Visibility::from_env()?.unwrap_or_default())
        .with_pause(Pause::from_env()?.unwrap_or_default())
        .with_driver_output(DriverOutput::from_env()?.unwrap_or_default())
        // Lookups and element waits poll with thirtyfour's element queries (`polling`); an implicit
        // wait would make every lookup of a missing element block, and compound with the polling.
        .with_timeouts(
            Timeouts::builder()
                .implicit_wait_timeout(Duration::ZERO)
                .build(),
        )
        .with_element_query_wait(polling::element_query_wait()))
}
