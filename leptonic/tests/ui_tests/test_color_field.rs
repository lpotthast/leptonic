// Upstream: @adobe/react-spectrum/test/color/ColorField.test.js @ 99e6102368
// Upstream: react-aria/test/color/useColorField.test.js @ 99e6102368
//! The `ColorField`/`ColorChannelField` atoms: hex text committed on blur (cleared, typed, invalid
//! characters rejected, incomplete text reverted), stepping by keys and the wheel within
//! #000000–#FFFFFF, flags, forms, and channel fields as number fields.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, SyntheticEvent};

const PATH: &str = "/atoms/color-field";

/// The text input of the field `#id`.
async fn input(page: &Page<'_>, id: &str) -> Result<WebElement, Report> {
    page.element(format!("#{id} input:not([type=hidden])"))
        .await
}

/// Moves focus to the button before the fields (committing the focused field).
async fn blur(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-cf-before").await?.focus().await?;
    Ok(())
}

/// The change log of the fields: `<field>:<hex or value>` entries, comma-separated.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-cf-log").await
}

/// The field is a text input without spin button values that shows the hex color and is labelled
/// by its `Label` ("handles defaults").
#[browser_test]
pub async fn defaults(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let primary = input(page, "test-cf-primary").await?;
    assert_that!(primary)
        .has_attribute("type")
        .await
        .is_equal_to("text");
    assert_that!(primary)
        .has_attribute("autocomplete")
        .await
        .is_equal_to("off");
    assert_that!(primary)
        .has_attribute("spellcheck")
        .await
        .is_equal_to("false");
    assert_that!(primary).attribute("role").await.is_none();
    assert_that!(primary)
        .attribute("aria-valuenow")
        .await
        .is_none();
    assert_that!(primary)
        .attribute("aria-valuetext")
        .await
        .is_none();
    assert_that!(primary)
        .property("value")
        .await
        .some()
        .is_equal_to("#AABBCC");
    assert_that!(primary)
        .accessible_name()
        .await
        .is_equal_to("Primary Color");
    Ok(())
}

/// Clearing the text commits no color on blur, and typing hex digits commits that color on blur,
/// shown as `#CBACBA` ("should handle uncontrolled state").
#[browser_test]
pub async fn uncontrolled_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let primary = input(page, "test-cf-primary").await?;
    primary.focus().await?;
    page.send_keys(page.primary_modifier().await? + "a").await?;
    page.send_keys(Key::Backspace).await?;
    blur(page).await?;
    log(page).await?.wait_for_inner_text("primary:none").await?;
    assert_that!(primary)
        .property("value")
        .await
        .some()
        .is_empty();
    primary.focus().await?;
    page.send_keys("cbacba").await?;
    blur(page).await?;
    log(page)
        .await?
        .wait_for_inner_text("primary:none,primary:CBACBA")
        .await?;
    assert_that!(primary)
        .property("value")
        .await
        .some()
        .is_equal_to("#CBACBA");
    Ok(())
}

/// Three hex digits commit their expanded color on blur, other characters can't be typed, and
/// incomplete text reverts to the last valid color ("should disallow invalid characters and revert
/// back to last valid value if left incomplete").
#[browser_test]
pub async fn invalid_characters(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let empty = input(page, "test-cf-empty").await?;
    empty.focus().await?;
    page.send_keys("abc").await?;
    blur(page).await?;
    log(page).await?.wait_for_inner_text("empty:AABBCC").await?;
    assert_that!(empty)
        .property("value")
        .await
        .some()
        .is_equal_to("#AABBCC");
    empty.focus().await?;
    page.send_keys(page.primary_modifier().await? + "a").await?;
    page.send_keys(Key::Backspace).await?;
    page.send_keys("abcxyz8b").await?;
    empty.wait_for_prop("value", "abc8b").await?;
    blur(page).await?;
    empty.wait_for_prop("value", "#AABBCC").await?;
    log(page)
        .await?
        .inner_text_stays("empty:AABBCC", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// The arrow keys step the color by one but not beyond #FFFFFF, and Home goes to #000000
/// ("increment with arrow up key", "decrement with arrow down key", "not increment beyond max
/// value", "decrement to min value").
#[browser_test]
pub async fn stepping(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let max = input(page, "test-cf-max").await?;
    max.focus().await?;
    page.send_keys(Key::Up).await?;
    log(page).await?.wait_for_inner_text("max:FFFFFF").await?;
    page.send_keys(Key::Up).await?;
    page.send_keys(Key::Down).await?;
    log(page)
        .await?
        .wait_for_inner_text("max:FFFFFF,max:FFFFFE")
        .await?;
    page.send_keys(Key::Home).await?;
    log(page)
        .await?
        .wait_for_inner_text("max:FFFFFF,max:FFFFFE,max:000000")
        .await?;
    assert_that!(max)
        .property("value")
        .await
        .some()
        .is_equal_to("#000000");
    Ok(())
}

/// Scrolling the wheel over the focused field increments the color ("increment with mouse wheel").
#[browser_test]
pub async fn mouse_wheel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let max = input(page, "test-cf-max").await?;
    max.focus().await?;
    max.dispatch(SyntheticEvent::wheel().delta_y(10.0)).await?;
    log(page).await?.wait_for_inner_text("max:FFFFFF").await?;
    max.wait_for_prop("value", "#FFFFFF").await?;
    Ok(())
}

/// A read-only, required field's input is `readonly` and natively `required`, without
/// `aria-required` ("should be readonly", "should be required").
#[browser_test]
pub async fn flags(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let flags = input(page, "test-cf-flags").await?;
    assert_that!(flags).has_attribute("readonly").await;
    // Native validation (the default): `required`, not `aria-required` (react-aria).
    assert_that!(flags).has_attribute("required").await;
    assert_that!(flags)
        .attribute("aria-required")
        .await
        .is_none();
    Ok(())
}

/// A hidden input submits the color under the field's name, and resetting the form restores the
/// default color ("supports form reset").
#[browser_test]
pub async fn form_reset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let form = input(page, "test-cf-form").await?;
    let hidden = page.element("#test-cf-form input[type=hidden]").await?;
    assert_that!(hidden)
        .has_attribute("name")
        .await
        .is_equal_to("color");
    // The form submits the hidden input's value.
    hidden.wait_for_prop("value", "#123456").await?;
    form.focus().await?;
    page.send_keys(Key::Up).await?;
    form.wait_for_prop("value", "#123457").await?;
    hidden.wait_for_prop("value", "#123457").await?;
    page.element("#test-cf-reset").await?.click().await?;
    form.wait_for_prop("value", "#123456").await?;
    hidden.wait_for_prop("value", "#123456").await?;
    Ok(())
}

/// A channel field is a number field labelled with its channel that shows the value with the
/// channel's unit (degrees, percent) and steps it ("should support the channel prop").
#[browser_test]
pub async fn channel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let hue = input(page, "test-cf-hue").await?;
    assert_that!(hue)
        .property("value")
        .await
        .some()
        .is_equal_to("10°");
    assert_that!(hue)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Hue");
    hue.focus().await?;
    page.send_keys(Key::Up).await?;
    log(page).await?.wait_for_inner_text("hue:11").await?;
    hue.wait_for_prop("value", "11°").await?;
    let saturation = input(page, "test-cf-saturation").await?;
    assert_that!(saturation)
        .property("value")
        .await
        .some()
        .is_equal_to("50%");
    Ok(())
}
