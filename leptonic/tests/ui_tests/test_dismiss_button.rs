// Upstream: react-aria/test/overlays/DismissButton.test.tsx @ 99e6102368
//! `DismissButton`: named "Dismiss" by default, by its `aria_label`, by `aria_labelledby` alone
//! (no `aria-label` then), or by itself and the referenced elements when given both; activating
//! it calls `on_dismiss`, and it never submits an enclosing form.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::Key};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/atoms/dismiss-button";

/// The dismiss button is named "Dismiss" by default and is out of the tab order ("should have a
/// default aria-label").
#[browser_test]
pub async fn default_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element("#test-dismiss-default button").await?;
    assert_that!(button)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Dismiss");
    assert_that!(button)
        .has_attribute("tabindex")
        .await
        .is_equal_to("-1");
    Ok(())
}

/// The dismiss button is named by its `aria_label` ("should accept an aria-label").
#[browser_test]
pub async fn aria_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element("#test-dismiss-label button").await?;
    assert_that!(button)
        .has_attribute("aria-label")
        .await
        .is_equal_to("foo");
    Ok(())
}

/// With `aria_labelledby` alone, the dismiss button is labelled by the referenced element and has
/// no `aria-label` ("should accept an aria-labelledby").
#[browser_test]
pub async fn aria_labelledby(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element("#test-dismiss-labelledby button").await?;
    assert_that!(button)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to("test-dismiss-span");
    assert_that!(button).attribute("aria-label").await.is_none();
    Ok(())
}

/// With both `aria_label` and `aria_labelledby`, the dismiss button is named by itself and the
/// referenced element ("should accept an aria-labelledby and aria-label").
#[browser_test]
pub async fn aria_labelledby_and_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element("#test-dismiss-both button").await?;
    assert_that!(button)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to("self test-dismiss-span");
    assert_that!(button)
        .has_attribute("aria-label")
        .await
        .is_equal_to("foo");
    assert_that!(button)
        .has_attribute("id")
        .await
        .is_equal_to("self");
    Ok(())
}

/// Activating the visually hidden dismiss button, as a screen reader does, calls `on_dismiss`.
#[browser_test]
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

/// Enter in a field of a form around a dismiss button submits the form without dismissing: the
/// dismiss button is `type="button"`, no submit button (an addition: upstream's dismiss button
/// submits enclosing forms).
#[browser_test]
pub async fn doesnt_submit_an_enclosing_form(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element("#test-dismiss-form button").await?;
    assert_that!(button)
        .has_attribute("type")
        .await
        .is_equal_to("button");
    page.element("#test-dismiss-field").await?.focus().await?;
    page.send_keys(Key::Enter).await?;
    page.element("#test-dismiss-submits")
        .await?
        .wait_for_inner_text("1")
        .await?;
    page.element("#test-dismiss-count")
        .await?
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}
