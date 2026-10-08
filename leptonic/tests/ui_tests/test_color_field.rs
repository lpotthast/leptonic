// Upstream: @adobe/react-spectrum/test/color/ColorField.test.js @ 99e6102368
// Upstream: react-aria/test/color/useColorField.test.js @ 99e6102368
//! The `ColorField`/`ColorChannelField` atoms: hex text committed on blur (cleared, typed, invalid
//! characters rejected, incomplete text reverted), stepping by keys and the wheel within
//! #000000–#FFFFFF, flags, forms, and channel fields as number fields.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, SyntheticEvent};

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

/// "handles defaults": a text box without spin button values, labelled by its `Label`.
pub async fn defaults(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let primary = input(page, "test-cf-primary").await?;
    assert_that!(primary.attr("type").await?)
        .get_some()
        .is_equal_to("text");
    assert_that!(primary.attr("autocomplete").await?)
        .get_some()
        .is_equal_to("off");
    assert_that!(primary.attr("spellcheck").await?)
        .get_some()
        .is_equal_to("false");
    assert_that!(primary.attr("role").await?).is_none();
    assert_that!(primary.attr("aria-valuenow").await?).is_none();
    assert_that!(primary.attr("aria-valuetext").await?).is_none();
    assert_that!(primary.value().await?)
        .get_some()
        .is_equal_to("#AABBCC");
    assert_that!(primary.referenced_text("aria-labelledby").await?).is_equal_to("Primary Color");
    Ok(())
}

/// "should handle uncontrolled state".
pub async fn uncontrolled_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let primary = input(page, "test-cf-primary").await?;
    primary.focus().await?;
    page.send_keys(Key::Control + "a").await?;
    page.send_keys(Key::Backspace).await?;
    blur(page).await?;
    log(page).await?.wait_for_inner_text("primary:none").await?;
    assert_that!(primary.value().await?).get_some().is_empty();
    primary.focus().await?;
    page.send_keys("cbacba").await?;
    blur(page).await?;
    log(page)
        .await?
        .wait_for_inner_text("primary:none,primary:CBACBA")
        .await?;
    assert_that!(primary.value().await?)
        .get_some()
        .is_equal_to("#CBACBA");
    Ok(())
}

/// "should disallow invalid characters and revert back to last valid value if left incomplete".
pub async fn invalid_characters(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let empty = input(page, "test-cf-empty").await?;
    empty.focus().await?;
    page.send_keys("abc").await?;
    blur(page).await?;
    log(page).await?.wait_for_inner_text("empty:AABBCC").await?;
    assert_that!(empty.value().await?)
        .get_some()
        .is_equal_to("#AABBCC");
    empty.focus().await?;
    page.send_keys(Key::Control + "a").await?;
    page.send_keys(Key::Backspace).await?;
    page.send_keys("abcxyz8b").await?;
    empty.wait_for_prop("value", "abc8b").await?;
    blur(page).await?;
    empty.wait_for_prop("value", "#AABBCC").await?;
    log(page).await?.inner_text_stays("empty:AABBCC").await?;
    Ok(())
}

/// "increment with arrow up key", "decrement with arrow down key", "not increment beyond max
/// value", "decrement to min value".
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
    assert_that!(max.value().await?)
        .get_some()
        .is_equal_to("#000000");
    Ok(())
}

/// "increment with mouse wheel" (while focused).
pub async fn mouse_wheel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let max = input(page, "test-cf-max").await?;
    max.focus().await?;
    max.dispatch(SyntheticEvent::wheel().with("deltaY", 10))
        .await?;
    log(page).await?.wait_for_inner_text("max:FFFFFF").await?;
    max.wait_for_prop("value", "#FFFFFF").await?;
    Ok(())
}

/// "should be readonly", "should be required".
pub async fn flags(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let flags = input(page, "test-cf-flags").await?;
    assert_that!(flags.attr("readonly").await?).is_some();
    // Native validation (the default): `required`, not `aria-required` (react-aria).
    assert_that!(flags.attr("required").await?).is_some();
    assert_that!(flags.attr("aria-required").await?).is_none();
    Ok(())
}

/// "supports form reset", with the name on a hidden input.
pub async fn form_reset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let form = input(page, "test-cf-form").await?;
    let hidden = page.element("#test-cf-form input[type=hidden]").await?;
    assert_that!(hidden.attr("name").await?)
        .get_some()
        .is_equal_to("color");
    form.focus().await?;
    page.send_keys(Key::Up).await?;
    form.wait_for_prop("value", "#123457").await?;
    page.element("#test-cf-reset").await?.click().await?;
    form.wait_for_prop("value", "#123456").await?;
    Ok(())
}

/// "should support the channel prop": a number field of the channel, named after it.
pub async fn channel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let hue = input(page, "test-cf-hue").await?;
    assert_that!(hue.value().await?)
        .get_some()
        .is_equal_to("10°");
    assert_that!(hue.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Hue");
    hue.focus().await?;
    page.send_keys(Key::Up).await?;
    log(page).await?.wait_for_inner_text("hue:11").await?;
    hue.wait_for_prop("value", "11°").await?;
    let saturation = input(page, "test-cf-saturation").await?;
    assert_that!(saturation.value().await?)
        .get_some()
        .is_equal_to("50%");
    Ok(())
}
