//! Browser integration tests of the book.
//!
//! Starts the book through `cargo leptos serve` (in its own target directory, so it can run next to `just serve`) and
//! checks every documentation page with Chrome for Testing. Every test gets a fresh `WebDriver` session; the tests
//! run in parallel. A failing
//! test fails `cargo test`.
//! Every test's timing is logged when it finishes, and a summary (slowest tests, slowest steps, time spent on
//! sessions) is printed at the end; slow steps (> 2s) of the `BookPage` helpers are logged as warnings.
//!
//! Run with `just book-browser-test`. Useful environment variables:
//! - `BROWSER_TEST_VISIBLE=1`: show the browser window.
//! - `BROWSER_TEST_PAUSE=1`: pause before each test for manual inspection.
//! - `BROWSER_TEST_DRIVER_OUTPUT=1`: forward chromedriver output.
//! - `BROWSER_TEST_FILTER=<text>`: only run the tests whose name contains `<text>`.
//! - `BOOK_TEST_PAGES=<text>`: only check the pages whose path contains `<text>` (e.g. `/doc/table`).
//! - `BROWSER_TEST_PARALLELISM=<n>`: how many checks run at the same time (default: a quarter of the CPU cores, 2
//!   to 8; `1`: sequential). The walks
//!   over all pages are split into shards.
//! - `BROWSER_TEST_LOG_STEPS=1`: log every step of every test with its duration.
//!
//! The book is built in `target/browser-test`: two runs at the same time share it and break each other.
#![cfg(not(target_arch = "wasm32"))]

mod common;
mod pages;
mod ui_tests;

use std::time::{Duration, Instant};

use browser_test::{
    BrowserTestRunner, DriverOutput, FailurePolicy, Parallelism, Pause, StderrSummary, Timeouts,
    Visibility, thirtyfour::ChromiumLikeCapabilities,
};
use leptos_browser_test::{LeptosTestAppConfig, Report, SiteScheme};

#[tokio::test(flavor = "multi_thread")]
async fn browser_tests() -> Result<(), Report> {
    common::tracing::init_subscriber();

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

    // The checks are independent (each visits its pages itself), so they run at the same time
    // (`BROWSER_TEST_PARALLELISM=<n>`; default: a quarter of the CPU cores, 2 to 8, as every check drives a browser
    // hydrating debug wasm; 1: sequential). The checks of the whole run (links between pages) follow, sequentially
    // (see `ui_tests::all`).
    let parallelism = Parallelism::from_env()?.unwrap_or_else(|| {
        let cores = std::thread::available_parallelism().map_or(4, std::num::NonZero::get);
        Parallelism::parallel((cores / 4).clamp(2, 8))
    });
    runner()?
        .run(app.base_url(), ui_tests::all(parallelism))
        .await
        .map_err(Report::into_dynamic)?;

    Ok(())
}

/// A runner with the book's browser settings and those of the `BROWSER_TEST_*` variables. An invalid value is an
/// error.
fn runner() -> Result<BrowserTestRunner, Report> {
    Ok(BrowserTestRunner::new()
        .with_report_consumer(StderrSummary)
        .with_chrome_capabilities(|caps| {
            // Chrome for Testing is extracted in user mode, so its setuid sandbox helper can't be
            // installed. Without these flags, Chrome may exit before chromedriver opens a session.
            caps.add_arg("--no-sandbox")?;
            caps.add_arg("--disable-dev-shm-usage")?;
            caps.add_arg("--ignore-certificate-errors")?;
            Ok(())
        })
        .with_failure_policy(FailurePolicy::RunAll)
        .with_visibility(Visibility::from_env()?.unwrap_or_default())
        .with_pause(Pause::from_env()?.unwrap_or_default())
        .with_driver_output(DriverOutput::from_env()?.unwrap_or_default())
        .with_timeouts(
            Timeouts::builder()
                .implicit_wait_timeout(Duration::from_secs(3))
                .build(),
        ))
}
