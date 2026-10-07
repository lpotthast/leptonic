// Upstream: react-aria/test/utils/getScrollParents.test.ts @ 99e6102368
// Upstream: react-aria/test/utils/scrollIntoView.test.ts @ 99e6102368
use std::borrow::Cow;

use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The scroll utilities: the root counts among the scroll parents ("includes root as a scroll
/// parent for a node in the document", "includes a scrollable intermediate parent", "excludes
/// non-scrollable ancestors"); scrolling the root into view ignores its border and scrollbar
/// ("excludes root border from scroll port when scrolling to start/end").
pub struct ScrollTests {}

#[async_trait]
impl BrowserTest<str> for ScrollTests {
    fn name(&self) -> Cow<'_, str> {
        "scroll_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/scroll").await?;

        // The child of the scrolling box: the box, then the root; the child of the plain box: the
        // root only (neither the plain box nor the body).
        page.click_element_with_id("test-scroll-list-parents")
            .await?;
        page.wait_for_text("test-scroll-parents", "test-scroll-box,HTML | HTML")
            .await?;

        // The root's border (100px) isn't part of its scroll port: the target ends up at the
        // viewport's top edge, and at its bottom edge.
        page.click_element_with_id("test-scroll-to-start").await?;
        page.wait_for_text("test-scroll-offset", "top: 0").await?;
        page.click_element_with_id("test-scroll-to-end").await?;
        page.wait_for_text("test-scroll-offset", "bottom: 0")
            .await?;

        page.expect_no_page_errors().await
    }
}
