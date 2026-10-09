// Upstream: react-aria-components/test/Button.test.js @ 99e6102368
//! The `Button` atom: presses, the disabled state, ARIA and form props, the state as data
//! attributes (hover, press, focus ring) and the pending state.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/atoms/button";

/// Pressing the button counts presses and focuses it, a disabled button ignores presses, and
/// accessibility and form props reach the `<button>` ("should render a button with default class",
/// "should not have aria-disabled defined by default", "should support disabled state", "should
/// support accessibility props", "should support form props").
#[browser_test]
pub async fn presses_and_props(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    // "should render a button with default class".
    let basic = page.element("#test-button-basic").await?;
    assert_that!(basic)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-Button");
    // "should not have aria-disabled defined by default".
    assert_that!(basic)
        .attribute("aria-disabled")
        .await
        .is_none();

    let basic_count = page.element("#test-button-basic-count").await?;
    assert_that!(basic_count)
        .inner_text()
        .await
        .map_owned(|value| value.parse::<u32>())
        .get_ok()
        .is_equal_to(0);
    basic.click().await?;
    basic_count.wait_for_inner_text("1").await?;
    basic.click().await?;
    basic_count.wait_for_inner_text("2").await?;
    // The pressed button is focused (`data-focused`).
    basic.wait_for_attr("data-focused", Some("true")).await?;

    // "should support disabled state".
    let disabled = page.element("#test-button-disabled").await?;
    let disabled_count = page.element("#test-button-disabled-count").await?;
    assert_that!(disabled_count)
        .inner_text()
        .await
        .map_owned(|value| value.parse::<u32>())
        .get_ok()
        .is_equal_to(0);
    disabled.click().await?;
    disabled_count
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(disabled)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    assert_that!(disabled).has_attribute("disabled").await;
    assert_that!(basic)
        .attribute("data-disabled")
        .await
        .is_none();

    // "should support accessibility props".
    let labelled = page.element("#test-button-labelled").await?;
    assert_that!(labelled)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Page 2");
    assert_that!(labelled)
        .has_attribute("aria-current")
        .await
        .is_equal_to("page");

    // "should support form props".
    let form_props = page.element("#test-button-form-props").await?;
    for (name, value) in [
        ("type", "submit"),
        ("form", "test-button-form"),
        ("formmethod", "post"),
        ("name", "action"),
        ("value", "save"),
    ] {
        assert_that!(form_props)
            .attribute(name)
            .await
            .with_detail_message(format!("attribute {name}"))
            .get_some()
            .is_equal_to(value);
    }
    Ok(())
}

/// Hovering, pressing and keyboard focus set `data-hovered`, `data-pressed` and
/// `data-focus-visible`; focus by the pointer shows no focus ring ("should support hover", "should
/// support focus ring", "should support press state").
#[browser_test]
pub async fn state_attributes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element("#test-button-labelled").await?;
    for name in ["data-hovered", "data-pressed", "data-focus-visible"] {
        assert_that!(button).attribute(name).await.is_none();
    }
    button.hover().await?;
    button.wait_for_attr("data-hovered", Some("true")).await?;
    let held = button.press_and_hold().await?;
    button.wait_for_attr("data-pressed", Some("true")).await?;
    held.release().await?;
    button.wait_for_attr("data-pressed", None).await?;
    // Focused by the pointer: no focus ring.
    button.wait_for_attr("data-focused", Some("true")).await?;
    button
        .attr_stays(
            "data-focus-visible",
            None,
            std::time::Duration::from_millis(100),
        )
        .await?;
    page.element("h1").await?.hover().await?;
    button.wait_for_attr("data-hovered", None).await?;

    // Focused by the keyboard: a focus ring, gone when focus leaves.
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    button
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.send_keys(Key::Tab).await?;
    button.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// A pending button is `aria-disabled` and ignores pointer and keyboard presses and hover, but
/// stays focusable ("displays a spinner when isPending prop is true").
#[browser_test]
pub async fn pending(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element("#test-button-pending").await?;
    assert_that!(button)
        .attribute("aria-disabled")
        .await
        .is_none();
    assert_that!(button)
        .attribute("data-pending")
        .await
        .is_none();
    button.click().await?;
    button.wait_for_attr("aria-disabled", Some("true")).await?;
    button.wait_for_attr("data-pending", Some("true")).await?;
    button
        .attr_stays("disabled", None, std::time::Duration::from_millis(100))
        .await?;
    let count = page.element("#test-button-pending-count").await?;
    button.click().await?;
    count
        .inner_text_stays("1", std::time::Duration::from_millis(100))
        .await?;
    // Still focused, but neither hovered nor pressed.
    button.wait_for_attr("data-focused", Some("true")).await?;
    button
        .attr_stays("data-hovered", None, std::time::Duration::from_millis(100))
        .await?;
    button
        .attr_stays("data-pressed", None, std::time::Duration::from_millis(100))
        .await?;
    // Keyboard presses are ignored too.
    page.send_keys(Key::Enter).await?;
    page.send_keys(" ").await?;
    count
        .inner_text_stays("1", std::time::Duration::from_millis(100))
        .await?;

    page.element("#test-button-pending-reset")
        .await?
        .click()
        .await?;
    button.wait_for_attr("aria-disabled", None).await?;
    button.wait_for_attr("data-pending", None).await?;
    Ok(())
}

/// A pending submit button is a plain button, so neither a click, Enter on it nor Enter in a text
/// field submits the form again ("should prevent explicit mouse form submission when isPending",
/// "should prevent explicit keyboard form submission when isPending", "should prevent implicit
/// form submission when isPending").
#[browser_test]
pub async fn pending_form_submission(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let submit = page.element("#test-button-pending-submit").await?;
    let submits = page.element("#test-button-pending-submits").await?;
    let toggle = page.element("#test-button-pending-submit-toggle").await?;
    let input_1 = page.element("#test-button-pending-input-1").await?;
    assert_that!(submit)
        .has_attribute("type")
        .await
        .is_equal_to("submit");
    // Mouse: the press submits, then the button turns pending.
    submit.click().await?;
    submits.wait_for_inner_text("1").await?;
    submit.wait_for_attr("type", Some("button")).await?;
    submit.click().await?;
    submits
        .inner_text_stays("1", std::time::Duration::from_millis(100))
        .await?;

    // Keyboard: Enter on the button.
    toggle.click().await?;
    submit.wait_for_attr("type", Some("submit")).await?;
    page.element("#test-button-pending-input-2")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&submit).await?;
    page.send_keys(Key::Enter).await?;
    submits.wait_for_inner_text("2").await?;
    submit.wait_for_attr("type", Some("button")).await?;
    page.send_keys(Key::Enter).await?;
    submits
        .inner_text_stays("2", std::time::Duration::from_millis(100))
        .await?;

    // Implicit: Enter in a text field submits through the submit button, unless it is pending.
    toggle.click().await?;
    submit.wait_for_attr("type", Some("submit")).await?;
    input_1.send_keys(Key::Enter).await?;
    submits.wait_for_inner_text("3").await?;
    // The implicit submission clicks the button: a virtual press, which makes it pending.
    submit.wait_for_attr("type", Some("button")).await?;
    input_1.send_keys(Key::Enter).await?;
    submits
        .inner_text_stays("3", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A pending button named by `aria-label` is labelled by itself and its progress bar through
/// `aria-labelledby`.
#[browser_test]
pub async fn pending_labelled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element("#test-button-pending-labelled").await?;
    let progress_id = assert_that!(button.element("[role=progressbar]").await?)
        .has_attribute("id")
        .await
        .is_not_blank()
        .actual()
        .clone();
    button
        .wait_for_attr(
            "aria-labelledby",
            Some(&format!("test-button-pending-labelled {progress_id}")),
        )
        .await?;
    Ok(())
}

/// Pressing a pending dialog trigger focuses it but doesn't open its dialog ("disables press when
/// in pending state for context").
#[browser_test]
pub async fn pending_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-button-pending-trigger")
        .await?
        .click()
        .await?;
    page.element("#test-button-pending-trigger-focused")
        .await?
        .wait_for_inner_text("true")
        .await?;
    page.count_stays("[role=dialog]", 0, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}
