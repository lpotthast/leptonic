// Upstream: react-aria/test/button/useButton.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

/// `use_button` on native buttons, inputs, anchors and custom elements: ARIA and native
/// attributes by element type, press by pointer and keyboard, tab order, disabled state, form
/// submission, hover and focus visibility.
pub struct UseButtonTests {}

#[async_trait]
impl BrowserTest<str> for UseButtonTests {
    fn name(&self) -> Cow<'_, str> {
        "use_button_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/button").await?;

        cases!(
            attributes_depend_on_element_type(&page),
            native_and_custom_elements_press(&page),
            tab_order(&page),
            disabled_buttons(&page),
            form_submission(&page),
            hover_and_focus_visible(&page),
        );
        Ok(())
    }
}

/// Native buttons need no role and default to `type="button"`; other elements are announced as
/// buttons and are focusable.
async fn attributes_depend_on_element_type(page: &Page<'_>) -> Result<(), Report> {
    let native = page.element("#test-btn-native").await?;
    assert_that!(native.attr("role").await?).is_none();
    assert_that!(native.attr("type").await?)
        .get_some()
        .is_equal_to("button");

    let div = page.element("#test-btn-div").await?;
    assert_that!(div.attr("role").await?)
        .get_some()
        .is_equal_to("button");
    assert_that!(div.attr("tabindex").await?)
        .get_some()
        .is_equal_to("0");
    assert_that!(div.attr("type").await?).is_none();

    let anchor = page.element("#test-btn-anchor").await?;
    assert_that!(anchor.attr("role").await?)
        .get_some()
        .is_equal_to("button");
    assert_that!(anchor.attr("href").await?)
        .get_some()
        .is_equal_to("#anchor-target");

    // "handles input elements": `type="button"`, `role="button"`.
    let input = page.element("#test-btn-input").await?;
    assert_that!(input.attr("type").await?)
        .get_some()
        .is_equal_to("button");
    assert_that!(input.attr("role").await?)
        .get_some()
        .is_equal_to("button");

    // "handles target and rel": a new tab also gets `noopener` (a leptonic addition).
    let blank = page.element("#test-btn-blank").await?;
    assert_that!(blank.attr("target").await?)
        .get_some()
        .is_equal_to("_blank");
    assert_that!(blank.attr("rel").await?)
        .get_some()
        .is_equal_to("nofollow noopener");

    // RAC Button.test.js "removes href attribute from anchor element when isPending is true".
    let pending_anchor = page.element("#test-btn-pending-anchor").await?;
    assert_that!(pending_anchor.attr("href").await?).is_none();
    assert_that!(pending_anchor.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    Ok(())
}

/// Pointer presses on a native and a custom button, then Enter and Space on the focused custom one.
async fn native_and_custom_elements_press(page: &Page<'_>) -> Result<(), Report> {
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
async fn tab_order(page: &Page<'_>) -> Result<(), Report> {
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

/// Native buttons and inputs use the `disabled` attribute, everything else `aria-disabled`;
/// disabled anchors lose their link and every disabled button its place in the tab order; a
/// disabled button doesn't press.
async fn disabled_buttons(page: &Page<'_>) -> Result<(), Report> {
    let toggle = page.element("#test-btn-toggle-disabled").await?;
    toggle.click().await?;
    let native = page.element("#test-btn-native").await?;
    native.wait_for_attr("disabled", Some("true")).await?;

    page.element("#test-btn-input")
        .await?
        .wait_for_attr("disabled", Some("true"))
        .await?;
    let div = page.element("#test-btn-div").await?;
    assert_that!(div.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(div.attr("disabled").await?).is_none();
    assert_that!(div.attr("tabindex").await?).is_none();
    assert_that!(native.attr("aria-disabled").await?).is_none();
    assert_that!(page.element("#test-btn-anchor").await?.attr("href").await?).is_none();

    let presses = page.element("#test-btn-presses").await?;
    let before = presses.inner_text().await?;
    div.click().await?;
    presses.inner_text_stays(&before).await?;

    toggle.click().await?;
    Ok(())
}

/// A button defaults to `type="button"` and does not submit its form; a submit button does.
async fn form_submission(page: &Page<'_>) -> Result<(), Report> {
    let submits = page.element("#test-btn-submits").await?;
    page.element("#test-btn-in-form").await?.click().await?;
    submits.inner_text_stays("0").await?;

    page.element("#test-btn-submit").await?.click().await?;
    submits.wait_for_inner_text("1").await?;
    Ok(())
}

/// Hovering sets `is_hovered`; pointer focus does not show a focus ring, keyboard focus does.
async fn hover_and_focus_visible(page: &Page<'_>) -> Result<(), Report> {
    let state = page.element("#test-btn-state").await?;
    let focus_visible = page.element("#test-btn-is-focus-visible").await?;
    state.hover().await?;
    page.element("#test-btn-is-hovered")
        .await?
        .wait_for_inner_text("true")
        .await?;

    state.click().await?;
    focus_visible.inner_text_stays("false").await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    focus_visible.wait_for_inner_text("true").await?;
    Ok(())
}
