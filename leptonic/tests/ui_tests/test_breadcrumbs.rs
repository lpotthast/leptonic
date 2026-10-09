// Upstream: react-aria-components/test/Breadcrumbs.test.js @ 99e6102368
// Upstream: react-aria/test/breadcrumbs/useBreadcrumbs.test.js @ 99e6102368
// Upstream: react-aria/test/breadcrumbs/useBreadcrumbItem.test.js @ 99e6102368
//! The breadcrumbs atoms: a labelled list whose current item is a disabled link with
//! `aria-current="page"` and no `href`, also while the trail grows and shrinks; pressed items
//! report their id; the whole trail can be disabled. The hooks: the navigation's default label,
//! items as anchors and spans, disabled and current.
use assertr::{matchers::eq, prelude::*};
use browser_test::browser_test;
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, role};

const PATH: &str = "/atoms/breadcrumbs";

/// The texts of the current items of the trail named "Breadcrumbs".
async fn current_items(page: &Page<'_>) -> Result<Vec<String>, Report> {
    page.inner_texts("ol[aria-label=Breadcrumbs] li[data-current]")
        .await
}

/// The last item is the current page: a disabled link with `aria-current="page"` and no `href`,
/// while the other items link to their pages.
#[browser_test]
pub async fn current_item(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("ol[aria-label=Breadcrumbs]").await?;
    assert_that!(current_items(page).await?).contains_exactly(["Item 3"]);
    let current = page.element(role(AriaRole::Link).text("Item 3")).await?;
    assert_that!(current)
        .has_attribute("aria-current")
        .await
        .is_equal_to("page");
    assert_that!(current)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    assert_that!(current).attribute("href").await.is_none();
    let first = page.element(role(AriaRole::Link).text("Item 1")).await?;
    assert_that!(first)
        .attribute("aria-current")
        .await
        .is_none();
    assert_that!(first)
        .has_attribute("href")
        .await
        .ends_with("/atoms/toolbar?item=1");
    Ok(())
}

/// An added item becomes the current one and the previous last item a link; removing it makes that
/// item current again ("should support dynamic collections").
#[browser_test]
pub async fn dynamic_collections(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-bc-add").await?.click().await?;
    page.element("#test-bc-count")
        .await?
        .wait_for_inner_text("4")
        .await?;
    page.element("ol[aria-label=Breadcrumbs] li[data-current] [aria-current=page]")
        .await?;
    assert_that!(current_items(page).await?).contains_exactly(["Item 4"]);
    let item_3 = page.element(role(AriaRole::Link).text("Item 3")).await?;
    assert_that!(item_3).has_attribute("href").await;

    page.element("#test-bc-remove").await?.click().await?;
    page.element("#test-bc-count")
        .await?
        .wait_for_inner_text("3")
        .await?;
    assert_that!(|| current_items(page))
        .eventually_ok()
        .matches(eq(vec!["Item 3".to_owned()]))
        .await;
    Ok(())
}

/// Disabling the breadcrumbs marks the trail `data-disabled` and removes every item's `href`.
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-bc-disable").await?.click().await?;
    page.element("ol[aria-label=Breadcrumbs][data-disabled]")
        .await?;
    page.wait_for_count("ol[aria-label=Breadcrumbs] a[href]", 0)
        .await?;
    Ok(())
}

/// The hooks label the navigation "Breadcrumbs" by default and render items as links, disabled
/// spans, or the current page with `aria-current="page"` and no `href` (useBreadcrumbs.test.js
/// "handles defaults"; useBreadcrumbItem.test.js "handles span elements", "handles isCurrent",
/// "handles isDisabled").
#[browser_test]
pub async fn hooks(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let nav = page.element("#test-bc-hook").await?;
    assert_that!(nav)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Breadcrumbs");
    let home = page.element(role(AriaRole::Link).text("Hook home")).await?;
    assert_that!(home)
        .has_attribute("href")
        .await
        .ends_with("/atoms");
    assert_that!(home).attribute("aria-current").await.is_none();

    let section = page
        .element(role(AriaRole::Link).text("Hook section"))
        .await?;
    assert_that!(section.tag_name().await?).is_equal_to("span");
    assert_that!(section)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    assert_that!(section).attribute("tabindex").await.is_none();

    // The current item: announced as the page, not followed (no `href`, so `role="link"`).
    let current = page
        .element(role(AriaRole::Link).text("Hook current"))
        .await?;
    assert_that!(current)
        .has_attribute("aria-current")
        .await
        .is_equal_to("page");
    assert_that!(current)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    assert_that!(current).attribute("href").await.is_none();
    assert_that!(current)
        .has_attribute("role")
        .await
        .is_equal_to("link");
    Ok(())
}

/// Pressing an item reports its id.
#[browser_test]
pub async fn press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(role(AriaRole::Link).text("Action 1"))
        .await?
        .click()
        .await?;
    page.element("#test-bc-action")
        .await?
        .wait_for_inner_text("action-1")
        .await?;
    Ok(())
}
