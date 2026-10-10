//! Browser integration tests of the book.
//!
//! Starts the book through `cargo leptos serve` (in its own target directory, so it can run next to `just serve`) and
//! checks every documentation page with Chrome for Testing. Every case is a test of its own (`cases`), run in a fresh
//! or reset `WebDriver` session; the tests run in parallel. A failing test fails `cargo test`.
//! The output is short: the run's milestones, warnings and errors, each failing test's failure report, and a summary at
//! the end (slowest tests, slowest steps, time spent on sessions). Each test's start and end (with its timing) and
//! every step of the `BookPage` helpers (with its duration) are logged at `debug` level.
//!
//! Run with `just book-browser-test`. Useful environment variables:
//! - `BROWSER_TEST_LOG=<filter>`: which logs to show (default `info`): a level (`debug`) or levels per target
//!   (`info,browser_test::step=debug`: every step). The `just` recipes set it, to edit there.
//! - `BROWSER_TEST_APP_OUTPUT=1`: forward the book's output (its build and its server's logs). A startup failure's
//!   report carries that output anyway.
//! - `BROWSER_TEST_VISIBLE=1`: show the browser window.
//! - `BROWSER_TEST_PAUSE=1`: pause before each test for manual inspection.
//! - `BROWSER_TEST_DRIVER_OUTPUT=1`: forward chromedriver output.
//! - `BROWSER_TEST_FILTER=<text>`: only run the tests whose name contains `<text>` (e.g. `shell::` for the shell's
//!   cases, `/doc/table` for the checks of the table pages; several texts separated by commas: any of them).
//! - `BOOK_TEST_PAGES=<text>`: only check the pages whose path contains `<text>` (e.g. `/doc/table`).
//! - `BROWSER_TEST_SESSION_REUSE=0`: give every test a fresh browser instead of resetting the one of the test before.
//! - `BROWSER_TEST_PARALLELISM=<n>`: how many tests run at the same time (default 8, `1`: sequential, e.g. with
//!   `BROWSER_TEST_VISIBLE=1`).
//!
//! - `BOOK_TARGET_DIR=<dir>`: build the book and its site there (default: inherited
//!   `CARGO_TARGET_DIR`, else `target/browser-test`). Two runs must not share one target directory.
#![cfg(not(target_arch = "wasm32"))]

mod cases;
mod common;
mod pages;
mod ui_tests;

use std::{
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use browser_test::{
    BrowserTestRunner, Cancellation, ChromeBinary, DriverOutput, FailurePolicy, Parallelism, Pause,
    ProgressWarnings, SessionReuse, StderrSummary, Timeouts, Visibility,
    thirtyfour::ChromiumLikeCapabilities,
};
use leptos_browser_test::{LeptosTestAppConfig, Report, SiteScheme, report};

#[tokio::test(flavor = "multi_thread")]
async fn browser_tests() -> Result<(), Report> {
    common::tracing::init_subscriber()?;
    // Chrome for Testing is downloaded through rustls with `ring` (browser-test's `rustls-no-provider`). Installing
    // fails only if a provider is installed already, which is as good.
    let _ = rustls::crypto::ring::default_provider().install_default();

    let app_start = Instant::now();
    let app_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let target_dir = std::env::var_os("BOOK_TARGET_DIR")
        .or_else(|| std::env::var_os("CARGO_TARGET_DIR"))
        .map_or_else(|| app_dir.join("target/browser-test"), PathBuf::from);
    let target_dir = std::path::absolute(target_dir)?;
    let app = LeptosTestAppConfig::new(app_dir)
        .with_app_name("leptonic book")
        .with_forward_logs(app_output()?)
        // The book serves TLS with a self-signed certificate (`certs/`).
        .with_site_scheme(SiteScheme::Https)
        // Keep the build and site output together, separate from a running `just serve`.
        .with_env("CARGO_TARGET_DIR", &target_dir)
        .with_env("LEPTOS_SITE_ROOT", target_dir.join("browser-test-site"))
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

/// `BROWSER_TEST_APP_OUTPUT`: `1` forwards the book's output (its build and its server's logs), `0` (the default)
/// doesn't.
fn app_output() -> Result<bool, Report> {
    match std::env::var("BROWSER_TEST_APP_OUTPUT").as_deref() {
        Err(_) | Ok("" | "0") => Ok(false),
        Ok("1") => Ok(true),
        Ok(other) => {
            Err(report!("BROWSER_TEST_APP_OUTPUT is {other:?}, expected `1` or `0`").into_dynamic())
        }
    }
}

/// `BROWSER_TEST_PARALLELISM`, default 8 (every parallel test is a browser; more gains little, see the library's
/// `documentation/testing.md`, "Speed and memory"). An invalid value is an error.
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
        .with_chrome_profiles_dir(
            Path::new(env!("CARGO_TARGET_TMPDIR")).join("browser-test-profiles"),
        )
        .with_chrome_capabilities(|caps| {
            // Chrome for Testing is extracted in user mode, so its setuid sandbox helper can't be
            // installed. Without these flags, Chrome may exit before chromedriver opens a session.
            caps.add_arg("--no-sandbox")?;
            caps.add_arg("--disable-dev-shm-usage")?;
            caps.add_arg("--ignore-certificate-errors")?;
            Ok(())
        })
        .with_failure_policy(FailurePolicy::RunAll)
        // No warning per slow step: steps are logged at `debug` level with their duration, and the summary lists the
        // slowest.
        .with_progress_warnings(ProgressWarnings::default().with_slow_step(None))
        // Chrome Headless Shell: a third less memory than Chrome and faster session resets. Visible runs use Chrome.
        .with_headless_chrome_binary(ChromeBinary::ChromeHeadlessShell)
        // Every case is a test of its own (`cases`): a passed test's browser is reset and runs the next test instead
        // of starting a new one. `BROWSER_TEST_SESSION_REUSE=0`: a fresh browser per test.
        .with_session_reuse(SessionReuse::from_env()?.unwrap_or(SessionReuse::enabled()))
        .with_visibility(Visibility::from_env()?.unwrap_or_default())
        .with_pause(Pause::from_env()?.unwrap_or_default())
        .with_driver_output(DriverOutput::from_env()?.unwrap_or_default())
        .with_timeouts(Timeouts::new().with_implicit_wait(Duration::from_secs(3))))
}
