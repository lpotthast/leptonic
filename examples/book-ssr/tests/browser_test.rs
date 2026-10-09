//! Browser integration tests of the book.
//!
//! Starts the book through `cargo leptos serve` (in its own target directory, so it can run next to `just serve`) and
//! checks every documentation page with Chrome for Testing. Every case is a test of its own (`cases`), run in a fresh
//! or reset `WebDriver` session; the tests run in parallel. A failing test fails `cargo test`.
//! Every test's timing is logged when it finishes, and a summary (slowest tests, slowest steps, time spent on
//! sessions) is printed at the end; slow steps (> 2s) of the `BookPage` helpers are logged as warnings.
//!
//! Run with `just book-browser-test`. Useful environment variables:
//! - `BROWSER_TEST_VISIBLE=1`: show the browser window.
//! - `BROWSER_TEST_PAUSE=1`: pause before each test for manual inspection.
//! - `BROWSER_TEST_DRIVER_OUTPUT=1`: forward chromedriver output.
//! - `BROWSER_TEST_FILTER=<text>`: only run the tests whose name contains `<text>` (e.g. `shell::` for the shell's
//!   cases, `/doc/table` for the checks of the table pages; several texts separated by commas: any of them).
//! - `BOOK_TEST_PAGES=<text>`: only check the pages whose path contains `<text>` (e.g. `/doc/table`).
//! - `BROWSER_TEST_SESSION_REUSE=0`: give every test a fresh browser instead of resetting the one of the test before.
//! - `BROWSER_TEST_PARALLELISM=<n>`: how many tests run at the same time (default 8, `1`: sequential, e.g. with
//!   `BROWSER_TEST_VISIBLE=1`).
//! - `BROWSER_TEST_LOG_STEPS=1`: log every step of every test with its duration.
//!
//! The book is built in `target/browser-test`: two runs at the same time share it and break each other.
#![cfg(not(target_arch = "wasm32"))]

mod cases;
mod common;
mod pages;
mod ui_tests;

use std::{
    path::Path,
    time::{Duration, Instant},
};

use browser_test::{
    BrowserTestRunner, Cancellation, ChromeBinary, ChromeProfilesDir, DriverOutput, FailurePolicy,
    Parallelism, Pause, SessionReuse, StderrSummary, Timeouts, Visibility,
    thirtyfour::ChromiumLikeCapabilities,
};
use leptos_browser_test::{LeptosTestAppConfig, Report, SiteScheme};

#[tokio::test(flavor = "multi_thread")]
async fn browser_tests() -> Result<(), Report> {
    common::tracing::init_subscriber();
    // Chrome for Testing is downloaded through rustls with `ring` (browser-test's `rustls-no-provider`). Installing
    // fails only if a provider is installed already, which is as good.
    let _ = rustls::crypto::ring::default_provider().install_default();

    let app_start = Instant::now();
    let app = LeptosTestAppConfig::new(env!("CARGO_MANIFEST_DIR"))
        .with_app_name("leptonic book")
        // The book serves TLS with a self-signed certificate (`certs/`).
        .with_site_scheme(SiteScheme::Https)
        // A separate build and site output, so that a running `just serve` is not disturbed. Relative to the book's
        // directory, where cargo-leptos runs: with an absolute site root, it misses the wasm-bindgen output.
        .with_env("CARGO_TARGET_DIR", "target/browser-test")
        .with_env("LEPTOS_SITE_ROOT", "target/browser-test/site")
        .start()
        .await
        .map_err(Report::into_dynamic)?;
    tracing::info!(
        "Built and started the book in {:.2}s.",
        app_start.elapsed().as_secs_f64()
    );

    // The tests are independent (each loads its pages itself), so they run at the same time
    // (`BROWSER_TEST_PARALLELISM=<n>`, default 8; 1: sequential), each in a fresh or reset browser. The checks of the
    // whole run (links between pages) follow, sequentially (see `ui_tests::all`).
    runner()?
        .run(app.base_url(), ui_tests::all(parallelism()?))
        .await
        .map_err(Report::into_dynamic)?;

    Ok(())
}

/// `BROWSER_TEST_PARALLELISM`, default 8 (every parallel test is a browser; more gains little, see the library's
/// `documentation/browser-tests.md`, "Speed and memory"). An invalid value is an error.
fn parallelism() -> Result<Parallelism, Report> {
    Ok(Parallelism::from_env()?.unwrap_or(Parallelism::parallel(8)))
}

/// A runner with the book's browser settings and those of the `BROWSER_TEST_*` variables. An invalid value is an
/// error.
fn runner() -> Result<BrowserTestRunner, Report> {
    // Ctrl-C (or SIGTERM) cancels the run: running tests stop, and chromedriver and its browsers shut down instead of
    // outliving the process.
    Ok(BrowserTestRunner::new(Cancellation::on_shutdown_signals())
        .with_report_consumer(StderrSummary)
        // The sessions' Chrome profiles (tens of MB each) go into the target dir's `tmp`, on disk, rather than the
        // system's temporary directory (often a RAM disk). Each is removed when its session ends; a run removes those
        // that killed runs left behind.
        .with_chrome_profiles_dir(ChromeProfilesDir::new(
            Path::new(env!("CARGO_TARGET_TMPDIR")).join("browser-test-profiles"),
        ))
        .with_chrome_capabilities(|caps| {
            // Chrome for Testing is extracted in user mode, so its setuid sandbox helper can't be
            // installed. Without these flags, Chrome may exit before chromedriver opens a session.
            caps.add_arg("--no-sandbox")?;
            caps.add_arg("--disable-dev-shm-usage")?;
            caps.add_arg("--ignore-certificate-errors")?;
            Ok(())
        })
        .with_failure_policy(FailurePolicy::RunAll)
        // Chrome Headless Shell: a third less memory than Chrome and faster session resets. Visible runs use Chrome.
        .with_headless_chrome_binary(ChromeBinary::ChromeHeadlessShell)
        // Every case is a test of its own (`cases`): a passed test's browser is reset and runs the next test instead
        // of starting a new one. `BROWSER_TEST_SESSION_REUSE=0`: a fresh browser per test.
        .with_session_reuse(SessionReuse::from_env()?.unwrap_or(SessionReuse::enabled()))
        .with_visibility(Visibility::from_env()?.unwrap_or_default())
        .with_pause(Pause::from_env()?.unwrap_or_default())
        .with_driver_output(DriverOutput::from_env()?.unwrap_or_default())
        .with_timeouts(
            Timeouts::builder()
                .implicit_wait_timeout(Duration::from_secs(3))
                .build(),
        ))
}
