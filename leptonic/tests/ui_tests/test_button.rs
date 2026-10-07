// Upstream: react-aria-components/test/Button.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, button::ButtonPage};

/// The `Button` atom: presses, the disabled state, ARIA and form props, the state as data
/// attributes (hover, press, focus ring) and the pending state.
pub struct ButtonTests {}

#[async_trait]
impl BrowserTest<str> for ButtonTests {
    fn name(&self) -> Cow<'_, str> {
        "button_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = ButtonPage { driver, base_url };
        page.goto().await?;

        presses_and_props(&page).await?;
        state_attributes(&page).await?;
        pending(&page).await?;
        pending_form_submission(&page).await?;
        pending_labelled(&page).await?;
        pending_trigger(&page).await?;

        Ok(())
    }
}

async fn presses_and_props(page: &ButtonPage<'_>) -> Result<(), Report> {
    // "should render a button with default class".
    let basic = page.css("#test-button-basic").await?;
    assert_that!(basic.attr("class").await?).is_equal_to(Some("leptonic-Button".to_owned()));
    // "should not have aria-disabled defined by default".
    assert_that!(basic.attr("aria-disabled").await?).is_none();

    assert_that!(page.read_basic_count().await?).is_equal_to(0);
    page.click_basic_button().await?;
    assert_that!(page.read_basic_count().await?).is_equal_to(1);
    page.click_basic_button().await?;
    assert_that!(page.read_basic_count().await?).is_equal_to(2);
    // The pressed button is focused (`data-focused`).
    page.wait_for_selector("#test-button-basic[data-focused=true]")
        .await?;

    // "should support disabled state".
    assert_that!(page.read_disabled_count().await?).is_equal_to(0);
    page.click_disabled_button().await?;
    assert_that!(page.read_disabled_count().await?).is_equal_to(0);
    let disabled = page.css("#test-button-disabled").await?;
    assert_that!(disabled.attr("data-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(disabled.attr("disabled").await?).is_some();
    assert_that!(basic.attr("data-disabled").await?).is_none();

    // "should support accessibility props".
    let labelled = page.css("#test-button-labelled").await?;
    assert_that!(labelled.attr("aria-label").await?).is_equal_to(Some("Page 2".to_owned()));
    assert_that!(labelled.attr("aria-current").await?).is_equal_to(Some("page".to_owned()));

    // "should support form props".
    let form_props = page.css("#test-button-form-props").await?;
    for (name, value) in [
        ("type", "submit"),
        ("form", "test-button-form"),
        ("formmethod", "post"),
        ("name", "action"),
        ("value", "save"),
    ] {
        let actual: Option<String> = page
            .driver
            .execute(
                "return arguments[0].getAttribute(arguments[1]);",
                vec![form_props.to_json()?, serde_json::Value::from(name)],
            )
            .await?
            .convert()?;
        assert_that!(actual)
            .with_detail_message(format!("attribute {name}"))
            .is_equal_to(Some(value.to_owned()));
    }
    Ok(())
}

/// "should support hover", "should support focus ring", "should support press state".
async fn state_attributes(page: &ButtonPage<'_>) -> Result<(), Report> {
    let button = page.css("#test-button-labelled").await?;
    for name in ["data-hovered", "data-pressed", "data-focus-visible"] {
        assert_that!(button.attr(name).await?).is_none();
    }
    page.driver
        .action_chain()
        .move_to_element_center(&button)
        .perform()
        .await?;
    page.wait_for_attr(&button, "data-hovered", Some("true"))
        .await?;
    page.driver
        .action_chain()
        .click_and_hold_element(&button)
        .perform()
        .await?;
    page.wait_for_attr(&button, "data-pressed", Some("true"))
        .await?;
    page.driver.action_chain().release().perform().await?;
    page.wait_for_attr(&button, "data-pressed", None).await?;
    // Focused by the pointer: no focus ring.
    page.wait_for_attr(&button, "data-focused", Some("true"))
        .await?;
    assert_that!(button.attr("data-focus-visible").await?).is_none();
    page.driver
        .action_chain()
        .move_to_element_center(&page.css("h1").await?)
        .perform()
        .await?;
    page.wait_for_attr(&button, "data-hovered", None).await?;

    // Focused by the keyboard: a focus ring, gone when focus leaves.
    page.press_shift_tab().await?;
    page.press_tab().await?;
    page.wait_for_attr(&button, "data-focus-visible", Some("true"))
        .await?;
    page.press_tab().await?;
    page.wait_for_attr(&button, "data-focus-visible", None)
        .await
}

/// "displays a spinner when isPending prop is true": pending, the button is `aria-disabled`,
/// ignores presses and hover, and stays focusable.
async fn pending(page: &ButtonPage<'_>) -> Result<(), Report> {
    let button = page.css("#test-button-pending").await?;
    assert_that!(button.attr("aria-disabled").await?).is_none();
    assert_that!(button.attr("data-pending").await?).is_none();
    button.click().await?;
    page.wait_for_attr(&button, "aria-disabled", Some("true"))
        .await?;
    page.wait_for_attr(&button, "data-pending", Some("true"))
        .await?;
    assert_that!(button.attr("disabled").await?).is_none();
    button.click().await?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_that!(page.read_text_of("test-button-pending-count").await?).is_equal_to("1".to_owned());
    // Still focused, but neither hovered nor pressed.
    page.wait_for_attr(&button, "data-focused", Some("true"))
        .await?;
    assert_that!(button.attr("data-hovered").await?).is_none();
    assert_that!(button.attr("data-pressed").await?).is_none();
    // Keyboard presses are ignored too.
    page.send_keys_to_active(Key::Enter).await?;
    page.send_keys_to_active(" ").await?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_that!(page.read_text_of("test-button-pending-count").await?).is_equal_to("1".to_owned());

    page.click_element_with_id("test-button-pending-reset")
        .await?;
    page.wait_for_attr(&button, "aria-disabled", None).await?;
    page.wait_for_attr(&button, "data-pending", None).await
}

/// "should prevent explicit mouse/keyboard form submission when isPending", "should prevent
/// implicit form submission when isPending": pending, a submit button is a plain button.
async fn pending_form_submission(page: &ButtonPage<'_>) -> Result<(), Report> {
    let submit = page.css("#test-button-pending-submit").await?;
    assert_that!(submit.attr("type").await?).is_equal_to(Some("submit".to_owned()));
    // Mouse: the press submits, then the button turns pending.
    submit.click().await?;
    page.wait_for_text("test-button-pending-submits", "1")
        .await?;
    page.wait_for_attr(&submit, "type", Some("button")).await?;
    submit.click().await?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_that!(page.read_text_of("test-button-pending-submits").await?)
        .is_equal_to("1".to_owned());

    // Keyboard: Enter on the button.
    page.click_element_with_id("test-button-pending-submit-toggle")
        .await?;
    page.wait_for_attr(&submit, "type", Some("submit")).await?;
    page.element("test-button-pending-input-2")
        .await?
        .click()
        .await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&submit, "the submit button").await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-button-pending-submits", "2")
        .await?;
    page.wait_for_attr(&submit, "type", Some("button")).await?;
    page.send_keys_to_active(Key::Enter).await?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_that!(page.read_text_of("test-button-pending-submits").await?)
        .is_equal_to("2".to_owned());

    // Implicit: Enter in a text field submits through the submit button, unless it is pending.
    page.click_element_with_id("test-button-pending-submit-toggle")
        .await?;
    page.wait_for_attr(&submit, "type", Some("submit")).await?;
    page.element("test-button-pending-input-1")
        .await?
        .send_keys(Key::Enter)
        .await?;
    page.wait_for_text("test-button-pending-submits", "3")
        .await?;
    // The implicit submission clicks the button, whose press may have made it pending already.
    tokio::time::sleep(Duration::from_millis(200)).await;
    if submit.attr("type").await?.as_deref() == Some("submit") {
        page.click_element_with_id("test-button-pending-submit-toggle")
            .await?;
    }
    page.wait_for_attr(&submit, "type", Some("button")).await?;
    page.element("test-button-pending-input-1")
        .await?
        .send_keys(Key::Enter)
        .await?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_that!(page.read_text_of("test-button-pending-submits").await?)
        .is_equal_to("3".to_owned());
    Ok(())
}

/// Pending, a button named by `aria-label` is named by itself and its progress bar
/// (`aria-labelledby` wins over `aria-label`).
async fn pending_labelled(page: &ButtonPage<'_>) -> Result<(), Report> {
    let button = page.css("#test-button-pending-labelled").await?;
    let progress_id = button
        .find(browser_test::thirtyfour::By::Css("[role=progressbar]"))
        .await?
        .attr("id")
        .await?
        .unwrap_or_default();
    assert_that!(progress_id.is_empty()).is_false();
    page.wait_for_attr(
        &button,
        "aria-labelledby",
        Some(&format!("test-button-pending-labelled {progress_id}")),
    )
    .await
}

/// "disables press when in pending state for context": a pending dialog trigger gets focus but
/// doesn't open its dialog.
async fn pending_trigger(page: &ButtonPage<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-button-pending-trigger")
        .await?;
    page.wait_for_text("test-button-pending-trigger-focused", "true")
        .await?;
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert_that!(page.count_matching("[role=dialog]").await?).is_equal_to(0);
    Ok(())
}
