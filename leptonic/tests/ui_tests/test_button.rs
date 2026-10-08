// Upstream: react-aria-components/test/Button.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

/// The `Button` atom: presses, the disabled state, ARIA and form props, the state as data
/// attributes (hover, press, focus ring) and the pending state.
pub struct ButtonTests {}

#[async_trait]
impl BrowserTest<str> for ButtonTests {
    fn name(&self) -> Cow<'_, str> {
        "button_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/button").await?;
        cases!(
            presses_and_props(&page),
            state_attributes(&page),
            pending(&page),
            pending_form_submission(&page),
            pending_labelled(&page),
            pending_trigger(&page),
        );
        Ok(())
    }
}

/// "should render a button with default class", "should support disabled state", "should support
/// accessibility props", "should support form props".
async fn presses_and_props(page: &Page<'_>) -> Result<(), Report> {
    // "should render a button with default class".
    let basic = page.element("#test-button-basic").await?;
    assert_that!(basic.attr("class").await?)
        .get_some()
        .is_equal_to("leptonic-Button");
    // "should not have aria-disabled defined by default".
    assert_that!(basic.attr("aria-disabled").await?).is_none();

    let basic_count = page.element("#test-button-basic-count").await?;
    assert_that!(basic_count.inner_text().await?.parse::<u32>()?).is_equal_to(0);
    basic.click().await?;
    basic_count.wait_for_inner_text("1").await?;
    basic.click().await?;
    basic_count.wait_for_inner_text("2").await?;
    // The pressed button is focused (`data-focused`).
    basic.wait_for_attr("data-focused", Some("true")).await?;

    // "should support disabled state".
    let disabled = page.element("#test-button-disabled").await?;
    let disabled_count = page.element("#test-button-disabled-count").await?;
    assert_that!(disabled_count.inner_text().await?.parse::<u32>()?).is_equal_to(0);
    disabled.click().await?;
    disabled_count.inner_text_stays("0").await?;
    assert_that!(disabled.attr("data-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(disabled.attr("disabled").await?).is_some();
    assert_that!(basic.attr("data-disabled").await?).is_none();

    // "should support accessibility props".
    let labelled = page.element("#test-button-labelled").await?;
    assert_that!(labelled.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Page 2");
    assert_that!(labelled.attr("aria-current").await?)
        .get_some()
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
        assert_that!(form_props.attr(name).await?)
            .with_detail_message(format!("attribute {name}"))
            .get_some()
            .is_equal_to(value);
    }
    Ok(())
}

/// "should support hover", "should support focus ring", "should support press state".
async fn state_attributes(page: &Page<'_>) -> Result<(), Report> {
    let button = page.element("#test-button-labelled").await?;
    for name in ["data-hovered", "data-pressed", "data-focus-visible"] {
        assert_that!(button.attr(name).await?).is_none();
    }
    button.hover().await?;
    button.wait_for_attr("data-hovered", Some("true")).await?;
    page.driver
        .action_chain()
        .click_and_hold_element(&button)
        .perform()
        .await?;
    button.wait_for_attr("data-pressed", Some("true")).await?;
    page.driver.action_chain().release().perform().await?;
    button.wait_for_attr("data-pressed", None).await?;
    // Focused by the pointer: no focus ring.
    button.wait_for_attr("data-focused", Some("true")).await?;
    assert_that!(button.attr("data-focus-visible").await?).is_none();
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

/// "displays a spinner when isPending prop is true": pending, the button is `aria-disabled`,
/// ignores presses and hover, and stays focusable.
async fn pending(page: &Page<'_>) -> Result<(), Report> {
    let button = page.element("#test-button-pending").await?;
    assert_that!(button.attr("aria-disabled").await?).is_none();
    assert_that!(button.attr("data-pending").await?).is_none();
    button.click().await?;
    button.wait_for_attr("aria-disabled", Some("true")).await?;
    button.wait_for_attr("data-pending", Some("true")).await?;
    assert_that!(button.attr("disabled").await?).is_none();
    let count = page.element("#test-button-pending-count").await?;
    button.click().await?;
    count.inner_text_stays("1").await?;
    // Still focused, but neither hovered nor pressed.
    button.wait_for_attr("data-focused", Some("true")).await?;
    assert_that!(button.attr("data-hovered").await?).is_none();
    assert_that!(button.attr("data-pressed").await?).is_none();
    // Keyboard presses are ignored too.
    page.send_keys(Key::Enter).await?;
    page.send_keys(" ").await?;
    count.inner_text_stays("1").await?;

    page.element("#test-button-pending-reset")
        .await?
        .click()
        .await?;
    button.wait_for_attr("aria-disabled", None).await?;
    button.wait_for_attr("data-pending", None).await?;
    Ok(())
}

/// "should prevent explicit mouse/keyboard form submission when isPending", "should prevent
/// implicit form submission when isPending": pending, a submit button is a plain button.
async fn pending_form_submission(page: &Page<'_>) -> Result<(), Report> {
    let submit = page.element("#test-button-pending-submit").await?;
    let submits = page.element("#test-button-pending-submits").await?;
    let toggle = page.element("#test-button-pending-submit-toggle").await?;
    let input_1 = page.element("#test-button-pending-input-1").await?;
    assert_that!(submit.attr("type").await?)
        .get_some()
        .is_equal_to("submit");
    // Mouse: the press submits, then the button turns pending.
    submit.click().await?;
    submits.wait_for_inner_text("1").await?;
    submit.wait_for_attr("type", Some("button")).await?;
    submit.click().await?;
    submits.inner_text_stays("1").await?;

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
    submits.inner_text_stays("2").await?;

    // Implicit: Enter in a text field submits through the submit button, unless it is pending.
    toggle.click().await?;
    submit.wait_for_attr("type", Some("submit")).await?;
    input_1.send_keys(Key::Enter).await?;
    submits.wait_for_inner_text("3").await?;
    // The implicit submission clicks the button: a virtual press, which makes it pending.
    submit.wait_for_attr("type", Some("button")).await?;
    input_1.send_keys(Key::Enter).await?;
    submits.inner_text_stays("3").await?;
    Ok(())
}

/// Pending, a button named by `aria-label` is named by itself and its progress bar
/// (`aria-labelledby` wins over `aria-label`).
async fn pending_labelled(page: &Page<'_>) -> Result<(), Report> {
    let button = page.element("#test-button-pending-labelled").await?;
    let progress_id = button
        .element("[role=progressbar]")
        .await?
        .id()
        .await?
        .unwrap_or_default();
    assert_that!(progress_id.as_str()).is_not_blank();
    button
        .wait_for_attr(
            "aria-labelledby",
            Some(&format!("test-button-pending-labelled {progress_id}")),
        )
        .await?;
    Ok(())
}

/// "disables press when in pending state for context": a pending dialog trigger gets focus but
/// doesn't open its dialog.
async fn pending_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-button-pending-trigger")
        .await?
        .click()
        .await?;
    page.element("#test-button-pending-trigger-focused")
        .await?
        .wait_for_inner_text("true")
        .await?;
    page.count_stays("[role=dialog]", 0).await?;
    Ok(())
}
