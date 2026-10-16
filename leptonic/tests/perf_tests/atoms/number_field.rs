// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `NumberField` (`leptonic/src/atoms/number_field.rs`, fixture
//! `/perf/atoms/number_field`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/number_field.json`): mounting and unmounting
//! it with its group and steppers, the keys stepping its value (arrows, Page Up/Down, Home,
//! End), typing (and a character it rejects), committing with Enter and on blur, the scroll
//! wheel, the stepper buttons, and changes of its reactive props (disabled, read-only, maximum)
//! and of its controlled value.
//!
//! A focused number field announces each new value. The first announcement creates the live
//! announcer and shows after 100 ms, a timer no measurement may span: the tests step once
//! unmeasured and wait for that announcement first.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, SyntheticEvent},
};

const PATH: &str = "/perf/atoms/number_field";
const FIELD: &str = ".leptonic-NumberField";
const INPUT: &str = "#perf-input";
const INCREMENT: &str = ".leptonic-NumberFieldIncrementButton";
const DECREMENT: &str = ".leptonic-NumberFieldDecrementButton";
/// The live announcer's announcement.
const ANNOUNCEMENT: &str = "[data-live-announcer] [aria-live=assertive] div";

/// Registers the performance tests of `NumberField`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(StepWithTheKeyboard))
        .with(fixture_test(Typing))
        .with(fixture_test(CommitWithEnter))
        .with(fixture_test(ScrollWheel))
        .with(fixture_test(Steppers))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ReadOnly))
        .with(fixture_test(Maximum))
        .with(fixture_test(ControlledValue))
}

/// The field's value (-1: empty).
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Focuses the input with a click (right of its short text: the caret goes to its end; End
/// would step to the maximum).
async fn focus_with_click(page: &Page<'_>) -> Result<WebElement, Report> {
    let input = page.element(INPUT).await?;
    input.click().await?;
    page.wait_for_focus(&input).await?;
    Ok(input)
}

/// Waits for the announcement `text`.
async fn wait_for_announcement(page: &Page<'_>, text: &str) -> Result<(), Report> {
    page.wait_for_count(ANNOUNCEMENT, 1).await?;
    page.element(ANNOUNCEMENT)
        .await?
        .wait_for_inner_text(text)
        .await
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
        .wait_for_prop("value", "5")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FIELD, 0).await?;
    Ok(())
}

/// The keys stepping the value: the arrows, Page Up and Page Down (by one step), Home (the
/// minimum) and End (the maximum).
#[browser_test]
pub async fn step_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_with_click(page).await?;
    page.send_keys(Key::Up).await?;
    wait_for_announcement(page, "6").await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    input.wait_for_prop("value", "7").await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    input.wait_for_prop("value", "6").await?;
    page.measure("page up", page.send_keys(Key::PageUp)).await?;
    input.wait_for_prop("value", "7").await?;
    page.measure("page down", page.send_keys(Key::PageDown))
        .await?;
    input.wait_for_prop("value", "6").await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    input.wait_for_prop("value", "0").await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    input.wait_for_prop("value", "100").await?;
    wait_for_announcement(page, "100").await?;
    assert_that!(value(page).await?).is_equal_to(100.0);
    Ok(())
}

/// Typing a digit, a letter the field rejects, and leaving the field (it commits the text).
#[browser_test]
pub async fn typing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_with_click(page).await?;
    page.measure("type a digit", page.send_keys("0")).await?;
    input.wait_for_prop("value", "50").await?;
    page.measure("rejected character", page.send_keys("a"))
        .await?;
    input.wait_for_prop("value", "50").await?;
    assert_that!(value(page).await?).is_equal_to(5.0);
    page.measure("commit on blur", page.send_keys(Key::Tab))
        .await?;
    assert_that!(value(page).await?).is_equal_to(50.0);
    Ok(())
}

/// Enter committing the typed text.
#[browser_test]
pub async fn commit_with_enter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_with_click(page).await?;
    page.send_keys("0").await?;
    input.wait_for_prop("value", "50").await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(value(page).await?).is_equal_to(50.0);
    Ok(())
}

/// The scroll wheel over the focused input stepping the value (a positive `deltaY`
/// increments, a negative one decrements).
#[browser_test]
pub async fn scroll_wheel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_with_click(page).await?;
    input
        .dispatch(SyntheticEvent::wheel().delta_y(10.0))
        .await?;
    wait_for_announcement(page, "6").await?;
    page.measure(
        "increment",
        input.dispatch(SyntheticEvent::wheel().delta_y(10.0)),
    )
    .await?;
    input.wait_for_prop("value", "7").await?;
    page.measure(
        "decrement",
        input.dispatch(SyntheticEvent::wheel().delta_y(-10.0)),
    )
    .await?;
    input.wait_for_prop("value", "6").await?;
    Ok(())
}

/// Clicks on the stepper buttons (the input takes focus with the first).
#[browser_test]
pub async fn steppers(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    let increment = page.element(INCREMENT).await?;
    let decrement = page.element(DECREMENT).await?;
    increment.click().await?;
    page.wait_for_focus(&input).await?;
    wait_for_announcement(page, "6").await?;
    page.measure("increment", increment.click()).await?;
    input.wait_for_prop("value", "7").await?;
    page.measure("decrement", decrement.click()).await?;
    input.wait_for_prop("value", "6").await?;
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

/// Making the field read-only, an arrow key it ignores, and making it editable again.
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let field = page.element(FIELD).await?;
    let input = focus_with_click(page).await?;
    page.measure("set", page.control("set-read-only")).await?;
    field.wait_for_attr("data-readonly", Some("true")).await?;
    page.measure("ignored arrow up", page.send_keys(Key::Up))
        .await?;
    input.wait_for_prop("value", "5").await?;
    page.measure("unset", page.control("unset-read-only"))
        .await?;
    field.wait_for_attr("data-readonly", None).await?;
    Ok(())
}

/// Lowering the maximum to the value (the increment button disables) and raising it again.
#[browser_test]
pub async fn maximum(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let increment = page.element(INCREMENT).await?;
    page.measure("limit", page.control("limit-to-5")).await?;
    increment
        .wait_for_attr("data-disabled", Some("true"))
        .await?;
    page.measure("unlimit", page.control("unlimit")).await?;
    increment.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// The page changing the controlled value.
#[browser_test]
pub async fn controlled_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("set", page.control("set-value")).await?;
    input.wait_for_prop("value", "42").await?;
    Ok(())
}
