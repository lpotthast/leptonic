//! Page context, health checks, and cleanup around browser tests.
use std::{any::Any, borrow::Cow, panic::AssertUnwindSafe};

use browser_test::{BrowserTest, SessionSettings, async_trait, thirtyfour::WebDriver};
use futures::FutureExt as _;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{Page, health::PagePanic};

/// Supply the page context, enforce fixture health policy, and clean up after a browser test.
pub(crate) fn fixture_test(
    test: impl for<'a> BrowserTest<Page<'a>> + 'static,
) -> impl BrowserTest<str> {
    FixtureTest(test)
}

/// Runs a test, then checks the page's health (what it reported: panics, uncaught errors, console
/// errors and warnings; and its DOM: duplicate ids, dangling id references, literal `attr:`
/// attributes; see `fixtures::check_health`; `Page::goto_path` checks the page it leaves):
/// - the test passed: page problems fail it;
/// - the test failed or panicked after the page panicked: the page's panic is the error, with the
///   test's failure after it (usually its consequence: a wait that gave up);
/// - the test failed otherwise: page problems are added to its failure, as they are often the
///   cause;
/// - an assertion panicked otherwise: page problems are logged, then the panic continues.
struct FixtureTest<T>(T);

#[async_trait]
impl<T: for<'a> BrowserTest<Page<'a>>> BrowserTest<str> for FixtureTest<T> {
    fn name(&self) -> Cow<'_, str> {
        self.0.name()
    }

    fn description(&self) -> Option<Cow<'_, str>> {
        self.0.description()
    }

    fn session_settings(&self) -> SessionSettings {
        self.0.session_settings()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page::new(driver, base_url);
        let outcome = AssertUnwindSafe(self.0.run(driver, &page))
            .catch_unwind()
            .await;
        let page_errors = crate::fixtures::check_health(&page).await;
        let cleanup = page.cleanup().await;
        let page_errors = match (page_errors, cleanup) {
            (Ok(()), cleanup) => cleanup,
            (Err(error), Ok(())) => Err(error),
            (Err(error), Err(cleanup)) => Err(error
                .context(format!("cleanup also failed: {cleanup}"))
                .into_dynamic()),
        };
        match (outcome, page_errors) {
            (Ok(Ok(())), page_errors) => {
                page_errors.context("the page reported problems after the test passed")?;
                Ok(())
            }
            (Ok(Err(failure)), Ok(())) => Err(failure),
            (Ok(Err(failure)), Err(page_errors)) if page_errors.page_panicked() => Err(page_errors
                .context(format!(
                    "the page panicked; then the test failed:\n{failure}"
                ))
                .into_dynamic()),
            (Ok(Err(failure)), Err(page_errors)) => Err(failure
                .context(format!(
                    "the page also reported problems, possibly the cause:\n{page_errors}"
                ))
                .into_dynamic()),
            (Err(panic), Err(page_errors)) if page_errors.page_panicked() => Err(page_errors
                .context(format!(
                    "the page panicked; then the test panicked:\n{}",
                    panic_message(panic.as_ref())
                ))
                .into_dynamic()),
            (Err(panic), page_errors) => {
                if let Err(page_errors) = page_errors {
                    tracing::error!(
                        "The page reported problems, possibly the cause of the panic:\n{page_errors}"
                    );
                }
                std::panic::resume_unwind(panic)
            }
        }
    }
}

/// The message of a caught panic (assertr's failure report for a failed check).
fn panic_message(panic: &(dyn Any + Send)) -> &str {
    panic
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| panic.downcast_ref::<&str>().copied())
        .unwrap_or("a panic without a message")
}
