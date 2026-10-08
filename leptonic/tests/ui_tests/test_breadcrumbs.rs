// Upstream: react-aria-components/test/Breadcrumbs.test.js @ 99e6102368
// Upstream: react-aria/test/breadcrumbs/useBreadcrumbs.test.js @ 99e6102368
// Upstream: react-aria/test/breadcrumbs/useBreadcrumbItem.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions, role},
    polling::wait_for,
};

/// The breadcrumbs atoms: a labelled list whose current item is a disabled link with
/// `aria-current="page"` and no `href`, also while the trail grows and shrinks; pressed items
/// report their id; the whole trail can be disabled. The hooks: the navigation's default label,
/// items as anchors and spans, disabled and current.
pub struct BreadcrumbsTests {}

#[async_trait]
impl BrowserTest<str> for BreadcrumbsTests {
    fn name(&self) -> Cow<'_, str> {
        "breadcrumbs_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/breadcrumbs").await?;
        cases!(
            current_item(&page),
            dynamic_collections(&page),
            disabled(&page),
            hooks(&page),
            // Last: following the in-page link navigates.
            press(&page),
        );
        Ok(())
    }
}

/// The texts of the current items of the trail named "Breadcrumbs".
async fn current_items(page: &Page<'_>) -> Result<Vec<String>, Report> {
    page.inner_texts("ol[aria-label=Breadcrumbs] li[data-current]")
        .await
}

/// The last item is the current one: a disabled link to the page without `href`.
async fn current_item(page: &Page<'_>) -> Result<(), Report> {
    page.element("ol[aria-label=Breadcrumbs]").await?;
    assert_that!(current_items(page).await?).contains_exactly(["Item 3"]);
    let current = page.element(role("link").text("Item 3")).await?;
    assert_that!(current.attr("aria-current").await?)
        .get_some()
        .is_equal_to("page");
    assert_that!(current.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(current.attr("href").await?).is_none();
    let first = page.element(role("link").text("Item 1")).await?;
    assert_that!(first.attr("aria-current").await?).is_none();
    assert_that!(first.attr("href").await?)
        .get_some()
        .ends_with("/atoms/toolbar?item=1");
    Ok(())
}

/// "should support dynamic collections": the marked item is the current one.
async fn dynamic_collections(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-bc-add").await?.click().await?;
    page.element("#test-bc-count")
        .await?
        .wait_for_inner_text("4")
        .await?;
    page.element("ol[aria-label=Breadcrumbs] li[data-current] [aria-current=page]")
        .await?;
    assert_that!(current_items(page).await?).contains_exactly(["Item 4"]);
    let item_3 = page.element(role("link").text("Item 3")).await?;
    assert_that!(item_3.attr("href").await?).is_some();

    page.element("#test-bc-remove").await?.click().await?;
    page.element("#test-bc-count")
        .await?
        .wait_for_inner_text("3")
        .await?;
    wait_for("the current items")
        .observing(|| current_items(page))
        .to_be_equal_to(vec!["Item 3".to_owned()])
        .await?;
    Ok(())
}

/// Disabled breadcrumbs: no item can be followed.
async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-bc-disable").await?.click().await?;
    page.element("ol[aria-label=Breadcrumbs][data-disabled]")
        .await?;
    page.wait_for_count("ol[aria-label=Breadcrumbs] a[href]", 0)
        .await?;
    Ok(())
}

/// useBreadcrumbs.test.js "handles defaults"; useBreadcrumbItem.test.js "handles span elements",
/// "handles isCurrent", "handles isDisabled".
async fn hooks(page: &Page<'_>) -> Result<(), Report> {
    let nav = page.element("#test-bc-hook").await?;
    assert_that!(nav.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Breadcrumbs");
    let home = page.element(role("link").text("Hook home")).await?;
    assert_that!(home.attr("href").await?)
        .get_some()
        .ends_with("/atoms");
    assert_that!(home.attr("aria-current").await?).is_none();

    let section = page.element(role("link").text("Hook section")).await?;
    assert_that!(section.tag_name().await?).is_equal_to("span");
    assert_that!(section.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(section.attr("tabindex").await?).is_none();

    // The current item: announced as the page, not followed (no `href`, so `role="link"`).
    let current = page.element(role("link").text("Hook current")).await?;
    assert_that!(current.attr("aria-current").await?)
        .get_some()
        .is_equal_to("page");
    assert_that!(current.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(current.attr("href").await?).is_none();
    assert_that!(current.attr("role").await?)
        .get_some()
        .is_equal_to("link");
    Ok(())
}

/// Pressing an item reports its id.
async fn press(page: &Page<'_>) -> Result<(), Report> {
    page.element(role("link").text("Action 1"))
        .await?
        .click()
        .await?;
    page.element("#test-bc-action")
        .await?
        .wait_for_inner_text("action-1")
        .await?;
    Ok(())
}
