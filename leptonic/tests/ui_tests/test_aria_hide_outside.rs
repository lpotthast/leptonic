// Upstream: react-aria/test/overlays/ariaHideOutside.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// `aria_hide_outside`: hides everything under the root except the targets (not traversing into
/// hidden containers), keeps author-set `aria-hidden`, hides the cells of a hidden row as well,
/// stacks hides restored in any order, and hides a root that doesn't contain a target.
pub struct AriaHideOutsideTests {}

async fn expect_hidden(page: &Page<'_>, hidden: &[&str], visible: &[&str]) -> Result<(), Report> {
    for id in hidden {
        page.wait_for_attr(&page.element(id).await?, "aria-hidden", Some("true"))
            .await?;
    }
    for id in visible {
        page.wait_for_attr(&page.element(id).await?, "aria-hidden", None)
            .await?;
    }
    Ok(())
}

#[async_trait]
impl BrowserTest<str> for AriaHideOutsideTests {
    fn name(&self) -> Cow<'_, str> {
        "aria_hide_outside_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/aria-hide-outside").await?;

        // "should hide everything except the provided element", "should not traverse into an
        // already hidden container", "should not overwrite an existing aria-hidden prop".
        page.click_element_with_id("test-aho-hide-basic").await?;
        expect_hidden(
            &page,
            &["test-aho-c1", "test-aho-wrap", "test-aho-author"],
            &["test-aho-c2", "test-aho-target", "test-aho-basic"],
        )
        .await?;
        page.click_element_with_id("test-aho-revert-basic").await?;
        expect_hidden(
            &page,
            &["test-aho-author"],
            &[
                "test-aho-c1",
                "test-aho-wrap",
                "test-aho-c2",
                "test-aho-target",
            ],
        )
        .await?;

        // "should hide everything except the provided element [row]": the hidden row's cell is
        // hidden as well (VoiceOver on iOS), not the cell's content.
        page.click_element_with_id("test-aho-hide-row").await?;
        expect_hidden(
            &page,
            &["test-aho-row-1", "test-aho-cell-1"],
            &[
                "test-aho-span",
                "test-aho-row-2",
                "test-aho-cell-2",
                "test-aho-grid",
            ],
        )
        .await?;
        page.click_element_with_id("test-aho-revert-row").await?;
        expect_hidden(&page, &[], &["test-aho-row-1", "test-aho-cell-1"]).await?;

        // "work when called multiple times and restored out of order".
        let checkboxes = ["test-aho-n-c1", "test-aho-n-c2"];
        let radios = ["test-aho-n-r1", "test-aho-n-r2"];
        page.click_element_with_id("test-aho-hide-nested-1").await?;
        expect_hidden(&page, &checkboxes, &radios).await?;
        page.click_element_with_id("test-aho-hide-nested-2").await?;
        expect_hidden(
            &page,
            &[checkboxes, radios].concat(),
            &["test-aho-n-button"],
        )
        .await?;
        page.click_element_with_id("test-aho-revert-nested-1")
            .await?;
        expect_hidden(
            &page,
            &[checkboxes, radios].concat(),
            &["test-aho-n-button"],
        )
        .await?;
        page.click_element_with_id("test-aho-revert-nested-2")
            .await?;
        expect_hidden(&page, &[], &[checkboxes, radios].concat()).await?;

        // "work when called multiple times", restored in order.
        page.click_element_with_id("test-aho-hide-nested-1").await?;
        page.click_element_with_id("test-aho-hide-nested-2").await?;
        page.click_element_with_id("test-aho-revert-nested-2")
            .await?;
        expect_hidden(&page, &checkboxes, &radios).await?;
        page.click_element_with_id("test-aho-revert-nested-1")
            .await?;
        expect_hidden(&page, &[], &[checkboxes, radios].concat()).await?;

        // The root itself is hidden when the target is outside it.
        page.click_element_with_id("test-aho-hide-outer").await?;
        expect_hidden(&page, &["test-aho-outer-root"], &[]).await?;
        page.click_element_with_id("test-aho-revert-outer").await?;
        expect_hidden(&page, &[], &["test-aho-outer-root"]).await?;

        assert_that!(page.count_matching("#test-aho-basic [aria-hidden]").await?).is_equal_to(1);
        page.expect_no_page_errors().await
    }
}
