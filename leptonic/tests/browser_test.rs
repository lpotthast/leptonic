mod common;
mod pages;
mod test_app;
mod ui_tests;

use chrome_for_testing_manager::prelude::*;
use thirtyfour::ChromiumLikeCapabilities;
use ui_tests::UiTest;

/// Set to `true` to pause before running tests, allowing manual inspection
/// of the running test-app. Enter "y" to continue, "n" to tear down.
const DELAY_TEST_EXECUTION: bool = false;

#[tokio::test(flavor = "multi_thread")]
async fn browser_tests() -> anyhow::Result<()> {
    common::tracing::init_subscriber();

    // 1. Start test-app via `cargo leptos watch`.
    let fe = test_app::start_frontend().await;

    // 2. Optional: pause for manual debugging.
    if DELAY_TEST_EXECUTION {
        tracing::info!("Continue with tests? y/n");
        let mut buf = String::new();
        loop {
            buf.clear();
            let input = std::io::stdin().read_line(&mut buf);
            if input.is_ok() {
                match buf.trim() {
                    "y" => break,
                    "n" => return Ok(()),
                    _ => {}
                }
            }
            if let Err(err) = input {
                tracing::error!("Error reading input: {err:?}");
                return Err(err.into());
            }
        }
    }

    // 3. Collect all test implementations.
    let tests: Vec<Box<dyn UiTest>> = vec![
        Box::new(ui_tests::test_button::ButtonTests {}),
        Box::new(ui_tests::test_focus::FocusTests {}),
        Box::new(ui_tests::test_focus_within::FocusWithinTests {}),
        Box::new(ui_tests::test_focus_ring::FocusRingTests {}),
        Box::new(ui_tests::test_focusable::FocusableTests {}),
        Box::new(ui_tests::test_focus_manager::FocusManagerTests {}),
        Box::new(ui_tests::test_focus_visible::FocusVisibleTests {}),
        Box::new(ui_tests::test_has_tabbable_child::HasTabbableChildTests {}),
        Box::new(ui_tests::test_focus_scope::FocusScopeTests {}),
    ];

    // 4. Launch chromedriver (auto-downloads matching Chrome version).
    tracing::info!("Starting webdriver...");
    let chromedriver =
        Chromedriver::run(VersionRequest::LatestIn(Channel::Stable), PortRequest::Any).await?;

    // 5. Run each test with a fresh WebDriver session.
    let base_url = &fe.base_url;
    for test in tests {
        #[allow(clippy::redundant_closure_for_method_calls)]
        chromedriver
            .with_custom_session(
                |caps| {
                    if std::env::var("BROWSER_TEST_VISIBLE").is_ok() {
                        caps.unset_headless()?;
                    }
                    Ok(())
                },
                async |driver| {
                    tracing::info!("Executing test: {}", test.name());
                    match test.run(driver, base_url).await {
                        Ok(()) => {
                            tracing::info!("Test '{}' passed!", test.name());
                        }
                        Err(err) => {
                            tracing::error!("Test '{}' failed: {:?}", test.name(), err);
                        }
                    }
                    Ok(())
                },
            )
            .await?;
    }

    // 6. Teardown.
    chromedriver.terminate().await?;
    drop(fe);
    Ok(())
}
