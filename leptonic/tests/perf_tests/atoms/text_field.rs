// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `TextField` (`leptonic/src/atoms/text_field.rs`, fixture
//! `/perf/atoms/text_field`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/text_field.json`): mounting and unmounting
//! it with its label, input, description and error, typing (in a short and in a 10,000
//! character value), focus by keyboard and pointer, hover, changes of its reactive props
//! (disabled, read-only, required, invalid) and of its controlled value, and typing it ignores
//! while read-only.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/text_field";
const FIELD: &str = ".leptonic-TextField";
const INPUT: &str = "#perf-input";
/// The length of the fixture's long value (`set-long-value`).
const LONG_VALUE_LENGTH: usize = 10_000;

/// Registers the performance tests of `TextField`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Typing))
        .with(fixture_test(TypingInALongValue))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(PointerFocus))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ReadOnly))
        .with(fixture_test(Required))
        .with(fixture_test(Invalid))
        .with(fixture_test(ControlledValue))
}

/// Focuses the input with the caret at the end of its value.
async fn focus_at_end(page: &Page<'_>) -> Result<WebElement, Report> {
    let input = page.element(INPUT).await?;
    input.type_keys(Key::End).await?;
    page.wait_for_focus(&input).await?;
    Ok(input)
}

/// Mounting the field with its parts and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(FIELD, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(INPUT)
        .await?
        .wait_for_prop("value", "Ada")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FIELD, 0).await?;
    Ok(())
}

/// Typing a character and deleting it again.
#[browser_test]
pub async fn typing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_at_end(page).await?;
    page.measure("type a character", page.send_keys("m"))
        .await?;
    input.wait_for_prop("value", "Adam").await?;
    page.measure("delete a character", page.send_keys(Key::Backspace))
        .await?;
    input.wait_for_prop("value", "Ada").await?;
    Ok(())
}

/// Typing a character at the end of a 10,000 character value.
#[browser_test]
pub async fn typing_in_a_long_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-long-value").await?;
    let input = page.element(INPUT).await?;
    let long_value = "a".repeat(LONG_VALUE_LENGTH);
    input.wait_for_prop("value", &long_value).await?;
    focus_at_end(page).await?;
    page.measure("type a character", page.send_keys("m"))
        .await?;
    input
        .wait_for_prop("value", &format!("{long_value}m"))
        .await?;
    Ok(())
}

/// Tab moving focus to the input (with a focus ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let input = page.element(INPUT).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    input
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    input.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// A click focusing the input (no focus ring) and a click elsewhere blurring it.
#[browser_test]
pub async fn pointer_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    let after = page.element("#perf-after").await?;
    page.measure("focus", input.click()).await?;
    input.wait_for_attr("data-focused", Some("true")).await?;
    input.wait_for_attr("data-focus-visible", None).await?;
    page.measure("blur", after.click()).await?;
    input.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// The pointer entering and leaving the input.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", input.hover()).await?;
    input.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    input.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Disabling the field and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let field = page.element(FIELD).await?;
    let input = page.element(INPUT).await?;
    page.measure("disable", page.control("disable")).await?;
    field.wait_for_attr("data-disabled", Some("true")).await?;
    input.wait_for_attr("disabled", Some("true")).await?;
    page.measure("enable", page.control("enable")).await?;
    field.wait_for_attr("data-disabled", None).await?;
    input.wait_for_attr("disabled", None).await?;
    Ok(())
}

/// Making the field read-only, typing it ignores, and making it editable again.
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let field = page.element(FIELD).await?;
    let input = focus_at_end(page).await?;
    page.measure("set", page.control("set-read-only")).await?;
    field.wait_for_attr("data-readonly", Some("true")).await?;
    input.wait_for_attr("readonly", Some("true")).await?;
    page.measure("ignored typing", page.send_keys("m")).await?;
    input.wait_for_prop("value", "Ada").await?;
    page.measure("unset", page.control("unset-read-only"))
        .await?;
    field.wait_for_attr("data-readonly", None).await?;
    input.wait_for_attr("readonly", None).await?;
    Ok(())
}

/// Making the field required and optional again.
#[browser_test]
pub async fn required(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let field = page.element(FIELD).await?;
    let input = page.element(INPUT).await?;
    page.measure("require", page.control("require")).await?;
    field.wait_for_attr("data-required", Some("true")).await?;
    input.wait_for_attr("required", Some("true")).await?;
    page.measure("unrequire", page.control("unrequire")).await?;
    field.wait_for_attr("data-required", None).await?;
    input.wait_for_attr("required", None).await?;
    Ok(())
}

/// Marking the field invalid (its error appears) and valid again.
#[browser_test]
pub async fn invalid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let field = page.element(FIELD).await?;
    let input = page.element(INPUT).await?;
    page.measure("invalidate", page.control("invalidate"))
        .await?;
    field.wait_for_attr("data-invalid", Some("true")).await?;
    input.wait_for_attr("aria-invalid", Some("true")).await?;
    page.element(".leptonic-FieldError").await?;
    page.measure("uninvalidate", page.control("uninvalidate"))
        .await?;
    field.wait_for_attr("data-invalid", None).await?;
    page.wait_for_count(".leptonic-FieldError", 0).await?;
    Ok(())
}

/// The page changing the controlled value.
#[browser_test]
pub async fn controlled_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("set", page.control("set-value")).await?;
    input.wait_for_prop("value", "Grace").await?;
    Ok(())
}
