//! Every case of the book's browser tests runs as a test of its own, in a fresh or reset browser session
//! (browser-test's `SessionReuse`): a failing case doesn't hide others, no case depends on what another one left
//! behind, and `BROWSER_TEST_FILTER` selects single cases.

use std::{borrow::Cow, future::Future};

use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use leptos_browser_test::Report;

use crate::pages::BookPage;

/// A case: an `async fn case(page: &BookPage<'_>) -> Result<(), Report>` of a test file, registered in
/// `ui_tests::all()`. It loads its page itself (`goto`), so what it runs on is visible where it is written.
pub trait CaseFn<'a>: Fn(&'a BookPage<'a>) -> Self::Run + Send + Sync + 'static {
    type Run: Future<Output = Result<(), Report>> + Send + 'a;
}

impl<'a, F, Run> CaseFn<'a> for F
where
    F: Fn(&'a BookPage<'a>) -> Run + Send + Sync + 'static,
    Run: Future<Output = Result<(), Report>> + Send + 'a,
{
    type Run = Run;
}

/// A case run as a test of its own, named after its function: `test_shell::search_opens_with_ctrl_k` is
/// `shell::search_opens_with_ctrl_k`.
pub struct Case<F>(pub F);

#[async_trait]
impl<F: for<'a> CaseFn<'a>> BrowserTest<str> for Case<F> {
    fn name(&self) -> Cow<'_, str> {
        case_name(std::any::type_name::<F>()).into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = BookPage { driver, base_url };
        (self.0)(&page).await
    }
}

/// `<module>::<case>` of the path of a case function, without the module's `test_` prefix.
fn case_name(path: &str) -> String {
    let mut segments = path.rsplit("::");
    let case = segments.next().unwrap_or(path);
    let module = segments.next().unwrap_or_default();
    format!("{}::{case}", module.strip_prefix("test_").unwrap_or(module))
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::case_name;

    #[test]
    fn cases_are_named_after_their_function() {
        assert_that!(case_name(
            "browser_test::ui_tests::test_shell::search_opens_with_ctrl_k"
        ))
        .is_equal_to("shell::search_opens_with_ctrl_k");
    }
}
