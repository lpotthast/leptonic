// Upstream: react-aria-components/test/Breadcrumbs.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The breadcrumbs atoms: a labelled list whose current item is a disabled link with
/// `aria-current="page"` and no `href`, also while the trail grows and shrinks; pressed items
/// report their id; the whole trail can be disabled.
pub struct BreadcrumbsTests {}

/// The text of the current items of the trail named "Breadcrumbs".
async fn current_items(page: &Page<'_>) -> Result<Vec<String>, Report> {
    Ok(page
        .driver
        .execute(
            "return [...document.querySelectorAll('ol[aria-label=Breadcrumbs] li[data-current]')]
                .map(li => li.textContent);",
            vec![],
        )
        .await?
        .convert()?)
}

#[async_trait]
impl BrowserTest<str> for BreadcrumbsTests {
    fn name(&self) -> Cow<'_, str> {
        "breadcrumbs_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/breadcrumbs").await?;

        page.wait_for_selector("ol[aria-label=Breadcrumbs]").await?;
        assert_that!(current_items(&page).await?).is_equal_to(vec!["Item 3".to_owned()]);
        let current = page.by_role_and_text("link", "Item 3").await?;
        assert_that!(current.attr("aria-current").await?).is_equal_to(Some("page".to_owned()));
        assert_that!(current.attr("aria-disabled").await?).is_equal_to(Some("true".to_owned()));
        assert_that!(current.attr("href").await?).is_none();
        let first = page.by_role_and_text("link", "Item 1").await?;
        assert_that!(first.attr("aria-current").await?).is_none();
        assert_that!(first.attr("href").await?.unwrap_or_default())
            .ends_with("/atoms/toolbar?item=1");

        // "should support dynamic collections": the marked item is the current one.
        page.click_element_with_id("test-bc-add").await?;
        page.wait_for_text("test-bc-count", "4").await?;
        page.wait_for_selector("ol[aria-label=Breadcrumbs] li[data-current] [aria-current=page]")
            .await?;
        assert_that!(current_items(&page).await?).is_equal_to(vec!["Item 4".to_owned()]);
        let item_3 = page.by_role_and_text("link", "Item 3").await?;
        assert_that!(item_3.attr("href").await?).is_some();
        page.click_element_with_id("test-bc-remove").await?;
        page.wait_for_text("test-bc-count", "3").await?;
        assert_that!(current_items(&page).await?).is_equal_to(vec!["Item 3".to_owned()]);

        // Disabled breadcrumbs: no item can be followed.
        page.click_element_with_id("test-bc-disable").await?;
        page.wait_for_selector("ol[aria-label=Breadcrumbs][data-disabled]")
            .await?;
        assert_that!(
            page.count_matching("ol[aria-label=Breadcrumbs] a[href]")
                .await?
        )
        .is_equal_to(0);

        // Pressing an item reports its id (last: following the in-page link navigates).
        page.by_role_and_text("link", "Action 1")
            .await?
            .click()
            .await?;
        page.wait_for_text("test-bc-action", "action-1").await?;

        page.expect_no_page_errors().await
    }
}
