//! Browser integration tests.
//!
//! Starts `testing/test-app` through `cargo leptos serve` and drives it with Chrome for Testing.
//! Every test gets a fresh WebDriver session. A failing test fails `cargo test`.
//!
//! Useful environment variables:
//! - `BROWSER_TEST_VISIBLE=1`: show the browser window.
//! - `BROWSER_TEST_PAUSE=1`: pause before each test for manual inspection.
//! - `BROWSER_TEST_DRIVER_OUTPUT=1`: forward chromedriver output.
#![cfg(not(target_arch = "wasm32"))]

mod common;
mod pages;
mod ui_tests;

use std::time::Duration;

use browser_test::{
    BrowserTestFailurePolicy, BrowserTestRunner, BrowserTestVisibility, BrowserTimeouts,
    DriverOutputConfig, PauseConfig, thirtyfour::ChromiumLikeCapabilities,
};
use leptos_browser_test::{LeptosTestAppConfig, Report};

#[tokio::test(flavor = "multi_thread")]
async fn browser_tests() -> Result<(), Report> {
    common::tracing::init_subscriber();

    let app = LeptosTestAppConfig::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../testing/test-app"))
        .with_app_name("leptonic test app")
        .start()
        .await
        .map_err(Report::into_dynamic)?;

    BrowserTestRunner::new()
        .with_chrome_capabilities(|caps| {
            // Chrome for Testing is extracted in user mode, so its setuid sandbox helper can't be
            // installed. Without these flags, Chrome may exit before chromedriver opens a session.
            caps.add_arg("--no-sandbox")?;
            caps.add_arg("--disable-dev-shm-usage")?;
            Ok(())
        })
        .with_failure_policy(BrowserTestFailurePolicy::RunAll)
        .with_visibility(BrowserTestVisibility::from_env())
        .with_pause(PauseConfig::from_env())
        .with_driver_output(DriverOutputConfig::from_env())
        .with_timeouts(
            BrowserTimeouts::builder()
                .implicit_wait_timeout(Duration::from_secs(3))
                .build(),
        )
        .run(app.base_url(), ui_tests::all())
        .await
        .map_err(Report::into_dynamic)?;

    Ok(())
}
