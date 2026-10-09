// Upstream: react-aria/test/button/useButton.test.js @ 99e6102368
//! `use_button` on native buttons, inputs, anchors and custom elements: ARIA and native
//! attributes by element type, press by pointer and keyboard, tab order, disabled state, form
//! submission, hover and focus visibility.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/hooks/button";

/// Native buttons get `type="button"` and no role, while inputs, anchors and other elements get the
/// button role, and a pending anchor loses its link ("handles defaults", "handles elements other
/// than button", "handles input elements").
#[browser_test]
pub async fn attributes_depend_on_element_type(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let native = page.element("#test-btn-native").await?;
    assert_that!(native).attribute("role").await.is_none();
    assert_that!(native)
        .has_attribute("type")
        .await
        .is_equal_to("button");

    let div = page.element("#test-btn-div").await?;
    assert_that!(div)
        .has_attribute("role")
        .await
        .is_equal_to("button");
    assert_that!(div)
        .has_attribute("tabindex")
        .await
        .is_equal_to("0");
    assert_that!(div).attribute("type").await.is_none();

    let anchor = page.element("#test-btn-anchor").await?;
    assert_that!(anchor)
        .has_attribute("role")
        .await
        .is_equal_to("button");
    assert_that!(anchor)
        .has_attribute("href")
        .await
        .is_equal_to("#anchor-target");

    // "handles input elements": `type="button"`, `role="button"`.
    let input = page.element("#test-btn-input").await?;
    assert_that!(input)
        .has_attribute("type")
        .await
        .is_equal_to("button");
    assert_that!(input)
        .has_attribute("role")
        .await
        .is_equal_to("button");

    // "handles target and rel": a new tab also gets `noopener` (a leptonic addition).
    let blank = page.element("#test-btn-blank").await?;
    assert_that!(blank)
        .has_attribute("target")
        .await
        .is_equal_to("_blank");
    assert_that!(blank)
        .has_attribute("rel")
        .await
        .is_equal_to("nofollow noopener");

    // RAC Button.test.js "removes href attribute from anchor element when isPending is true".
    let pending_anchor = page.element("#test-btn-pending-anchor").await?;
    assert_that!(pending_anchor)
        .attribute("href")
        .await
        .is_none();
    assert_that!(pending_anchor)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    Ok(())
}

/// Clicking a native or a custom button presses it, and Enter and Space press the focused custom
/// button.
#[browser_test]
pub async fn native_and_custom_elements_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let presses = page.element("#test-btn-presses").await?;
    page.element("#test-btn-native").await?.click().await?;
    presses.wait_for_inner_text("1").await?;
    page.element("#test-btn-div").await?.click().await?;
    presses.wait_for_inner_text("2").await?;
    // Keyboard activation of the custom element (it has focus after the click).
    page.send_keys(Key::Enter).await?;
    presses.wait_for_inner_text("3").await?;
    page.send_keys(" ").await?;
    presses.wait_for_inner_text("4").await?;
    Ok(())
}

/// Tab visits every button but the one with `exclude_from_tab_order`.
#[browser_test]
pub async fn tab_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-btn-before").await?.click().await?;
    for id in [
        "#test-btn-native",
        "#test-btn-div",
        "#test-btn-anchor",
        "#test-btn-after",
    ] {
        page.send_keys(Key::Tab).await?;
        page.wait_for_focus(&page.element(id).await?).await?;
    }
    Ok(())
}

/// Disabled native buttons and inputs get `disabled`, other elements `aria-disabled` and no tab
/// stop, anchors lose their link, and a disabled button doesn't press ("handles elements other than
/// button disabled").
#[browser_test]
pub async fn disabled_buttons(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page.element("#test-btn-toggle-disabled").await?;
    toggle.click().await?;
    let native = page.element("#test-btn-native").await?;
    native.wait_for_attr("disabled", Some("true")).await?;

    page.element("#test-btn-input")
        .await?
        .wait_for_attr("disabled", Some("true"))
        .await?;
    let div = page.element("#test-btn-div").await?;
    assert_that!(div)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    assert_that!(div).attribute("disabled").await.is_none();
    assert_that!(div).attribute("tabindex").await.is_none();
    assert_that!(native)
        .attribute("aria-disabled")
        .await
        .is_none();
    assert_that!(page.element("#test-btn-anchor").await?)
        .attribute("href")
        .await
        .is_none();

    let presses = page.element("#test-btn-presses").await?;
    let before = presses.inner_text().await?;
    div.click().await?;
    presses
        .inner_text_stays(&before, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A button defaults to `type="button"` and does not submit its form; a submit button does.
#[browser_test]
pub async fn form_submission(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let submits = page.element("#test-btn-submits").await?;
    page.element("#test-btn-in-form").await?.click().await?;
    submits
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;

    page.element("#test-btn-submit").await?.click().await?;
    submits.wait_for_inner_text("1").await?;
    Ok(())
}

/// Hovering sets `is_hovered`; pointer focus does not show a focus ring, keyboard focus does.
#[browser_test]
pub async fn hover_and_focus_visible(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let state = page.element("#test-btn-state").await?;
    let focus_visible = page.element("#test-btn-is-focus-visible").await?;
    state.hover().await?;
    page.element("#test-btn-is-hovered")
        .await?
        .wait_for_inner_text("true")
        .await?;

    state.click().await?;
    focus_visible
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    focus_visible.wait_for_inner_text("true").await?;
    Ok(())
}
