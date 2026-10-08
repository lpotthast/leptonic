// Upstream: react-aria/test/overlays/DismissButton.test.tsx @ 99e6102368
//! `DismissButton`: named "Dismiss" by default, by its `aria_label`, by `aria_labelledby` alone
//! (no `aria-label` then), or by itself and the referenced elements when given both; activating
//! it calls `on_dismiss`.
use assertr::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

const PATH: &str = "/atoms/dismiss-button";

/// "should have a default aria-label"; not in the tab order.
pub async fn default_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element("#test-dismiss-default button").await?;
    assert_that!(button.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Dismiss");
    assert_that!(button.attr("tabindex").await?)
        .get_some()
        .is_equal_to("-1");
    Ok(())
}

/// "should accept an aria-label".
pub async fn aria_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element("#test-dismiss-label button").await?;
    assert_that!(button.attr("aria-label").await?)
        .get_some()
        .is_equal_to("foo");
    Ok(())
}

/// "should accept an aria-labelledby": no `aria-label` then.
pub async fn aria_labelledby(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element("#test-dismiss-labelledby button").await?;
    assert_that!(button.attr("aria-labelledby").await?)
        .get_some()
        .is_equal_to("test-dismiss-span");
    assert_that!(button.attr("aria-label").await?).is_none();
    Ok(())
}

/// "should accept an aria-labelledby and aria-label": named by itself and the referenced element.
pub async fn aria_labelledby_and_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element("#test-dismiss-both button").await?;
    assert_that!(button.attr("aria-labelledby").await?)
        .get_some()
        .is_equal_to("self test-dismiss-span");
    assert_that!(button.attr("aria-label").await?)
        .get_some()
        .is_equal_to("foo");
    assert_that!(button.id().await?)
        .get_some()
        .is_equal_to("self");
    Ok(())
}

/// Activating it (as a screen reader does; it is visually hidden) dismisses.
pub async fn activating_dismisses(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dismissed = page.element("#test-dismiss-count").await?;
    page.element("#test-dismiss-default button")
        .await?
        .virtual_click()
        .await?;
    dismissed.wait_for_inner_text("1").await?;
    Ok(())
}
