// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Select` (`/perf/atoms/select`): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget (`tests/budgets/atoms/select.json`):
//! mounting and unmounting it, opening it and selecting an option with the mouse and the keyboard,
//! dismissing it, changing the value on the closed trigger, the trigger's pressed, hovered and
//! focused states, and changes of its reactive props (value, open state, disabled, invalid,
//! required, disabled options), with 10 options and with 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/select";

/// Registers the performance tests of `Select`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(OpenAndSelectWithTheMouse))
        .with(fixture_test(OpenAndSelectWithTheMouseLarge))
        .with(fixture_test(Dismiss))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(OpenAndSelectWithTheKeyboard))
        .with(fixture_test(OpenAndSelectWithTheKeyboardLarge))
        .with(fixture_test(ChangeOnTheTrigger))
        .with(fixture_test(ChangeOnTheTriggerLarge))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ControlledValue))
        .with(fixture_test(ControlledValueLarge))
        .with(fixture_test(ControlledOpen))
        .with(fixture_test(ControlledOpenLarge))
        .with(fixture_test(Invalid))
        .with(fixture_test(Required))
        .with(fixture_test(DisabledOption))
}
const SELECT: &str = ".leptonic-Select";
const TRIGGER: &str = ".leptonic-SelectTrigger";
const VALUE: &str = ".leptonic-SelectValue";
const POPOVER: &str = ".leptonic-SelectPopover";
/// The fixture's options after the control `large`.
const LARGE: u32 = 1_000;

/// The option "Option `i`".
fn option(i: u32) -> String {
    format!("[data-perf-option='{i}']")
}

/// The select's value: the selected option's number, `-1` for none.
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Switches to [`LARGE`] options (unmeasured).
async fn large(page: &Page<'_>) -> Result<(), Report> {
    page.control("large").await?;
    page.settle().await
}

/// Focuses the trigger with the keyboard (Tab from the plain button before it).
async fn focus_trigger(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let trigger = page.element(TRIGGER).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(trigger)
}

/// Mounting the select (10 options) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SELECT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TRIGGER).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SELECT, 0).await?;
    Ok(())
}

/// Mounting the select with 1,000 options and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SELECT, 0).await?;
    large(page).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TRIGGER).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SELECT, 0).await?;
    Ok(())
}

/// A click on the trigger opening the popover with `last + 1` options, and a click on an option
/// selecting it (closing the popover).
async fn open_and_select_with_the_mouse_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    let trigger = page.element(TRIGGER).await?;
    page.measure("open", trigger.click()).await?;
    page.element(option(last)).await?;
    let third = page.element(option(3)).await?;
    page.measure("select", third.click()).await?;
    page.wait_for_count(POPOVER, 0).await?;
    assert_that!(value(page).await?).is_equal_to(3.0);
    page.element(VALUE)
        .await?
        .wait_for_inner_text("Option 3")
        .await?;
    Ok(())
}

/// Opening the select of 10 options and selecting one with the mouse.
#[browser_test]
pub async fn open_and_select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_and_select_with_the_mouse_in(page, 9).await
}

/// Opening the select of 1,000 options and selecting one with the mouse.
#[browser_test]
pub async fn open_and_select_with_the_mouse_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    open_and_select_with_the_mouse_in(page, LARGE - 1).await
}

/// Escape closing the open popover (focus returns to the trigger), and a click outside closing
/// it.
#[browser_test]
pub async fn dismiss(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    trigger.click().await?;
    page.element(option(0)).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    page.wait_for_focus(&trigger).await?;
    trigger.click().await?;
    page.element(option(0)).await?;
    page.measure("outside click", page.click_at(5, 5)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

/// Tab moving focus to the trigger (with a focus ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let select = page.element(SELECT).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    select
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    select.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// Enter opening the popover of `last + 1` options (focus on the first), ArrowDown moving the
/// focus and Enter selecting the focused option (closing the popover).
async fn open_and_select_with_the_keyboard_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    focus_trigger(page).await?;
    page.measure("open", page.send_keys(Key::Enter)).await?;
    page.element(option(last)).await?;
    page.wait_for_focus(&page.element(option(0)).await?).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&page.element(option(1)).await?).await?;
    page.measure("select", page.send_keys(Key::Enter)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    assert_that!(value(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Opening the select of 10 options and selecting one with the keyboard.
#[browser_test]
pub async fn open_and_select_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_and_select_with_the_keyboard_in(page, 9).await
}

/// Opening the select of 1,000 options and selecting one with the keyboard.
#[browser_test]
pub async fn open_and_select_with_the_keyboard_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    open_and_select_with_the_keyboard_in(page, LARGE - 1).await
}

/// ArrowRight and ArrowLeft on the closed trigger selecting the next and the previous option,
/// and type-ahead ("o", matching the selected option). (Type-ahead last: the search resets a second
/// later.)
async fn change_on_the_trigger_in(page: &Page<'_>) -> Result<(), Report> {
    focus_trigger(page).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    assert_that!(value(page).await?).is_equal_to(0.0);
    page.send_keys(Key::Right).await?;
    page.element(VALUE)
        .await?
        .wait_for_inner_text("Option 1")
        .await?;
    page.measure("arrow left", page.send_keys(Key::Left))
        .await?;
    assert_that!(value(page).await?).is_equal_to(0.0);
    page.measure("type-ahead", page.send_keys("o")).await?;
    assert_that!(value(page).await?).is_equal_to(0.0);
    assert_that!(page.count(POPOVER).await?).is_equal_to(0);
    Ok(())
}

/// Changing the value on the closed trigger with 10 options.
#[browser_test]
pub async fn change_on_the_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    change_on_the_trigger_in(page).await
}

/// Changing the value on the closed trigger with 1,000 options.
#[browser_test]
pub async fn change_on_the_trigger_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    change_on_the_trigger_in(page).await
}

/// Holding the pointer down on the trigger (its pressed state; the popover opens) and releasing
/// it (it stays open and pressed).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    let held = page.measure("hold", trigger.press_and_hold()).await?;
    trigger.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    page.element(option(0)).await?;
    assert_that!(trigger)
        .has_attribute("data-open")
        .await
        .is_equal_to("true");
    Ok(())
}

/// The pointer entering and leaving the trigger.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", trigger.hover()).await?;
    trigger.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    trigger.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Disabling the select, a click on the trigger it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("disable", page.control("disable")).await?;
    trigger.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", trigger.click()).await?;
    assert_that!(page.count(POPOVER).await?).is_equal_to(0);
    page.measure("enable", page.control("enable")).await?;
    trigger.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// The controlled value changing: the first option, the last of `last + 1`, none.
async fn controlled_value_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    let shown = page.element(VALUE).await?;
    page.measure("select first", page.control("select-first"))
        .await?;
    shown.wait_for_inner_text("Option 0").await?;
    page.measure("select last", page.control("select-last"))
        .await?;
    shown.wait_for_inner_text(&format!("Option {last}")).await?;
    page.measure("clear", page.control("clear-value")).await?;
    shown
        .wait_for_attr("data-placeholder", Some("true"))
        .await?;
    Ok(())
}

/// The controlled value of a select of 10 options changing.
#[browser_test]
pub async fn controlled_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    controlled_value_in(page, 9).await
}

/// The controlled value of a select of 1,000 options changing.
#[browser_test]
pub async fn controlled_value_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    controlled_value_in(page, LARGE - 1).await
}

/// The controlled open state opening the popover of `last + 1` options and closing it.
async fn controlled_open_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    page.measure("open", page.control("open")).await?;
    page.element(option(last)).await?;
    page.measure("close", page.control("close")).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

/// The controlled open state of a select of 10 options changing.
#[browser_test]
pub async fn controlled_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    controlled_open_in(page, 9).await
}

/// The controlled open state of a select of 1,000 options changing.
#[browser_test]
pub async fn controlled_open_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    controlled_open_in(page, LARGE - 1).await
}

/// Marking the select invalid and valid again.
#[browser_test]
pub async fn invalid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let select = page.element(SELECT).await?;
    page.measure("invalidate", page.control("invalidate"))
        .await?;
    select.wait_for_attr("data-invalid", Some("true")).await?;
    page.measure("validate", page.control("validate")).await?;
    select.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// Making the select required and optional again.
#[browser_test]
pub async fn required(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let select = page.element(SELECT).await?;
    page.measure("require", page.control("require")).await?;
    select.wait_for_attr("data-required", Some("true")).await?;
    page.measure("unrequire", page.control("unrequire")).await?;
    select.wait_for_attr("data-required", None).await?;
    Ok(())
}

/// Disabling the first option of the open popover, a click on it that it ignores, and enabling
/// it again.
#[browser_test]
pub async fn disabled_option(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("open").await?;
    let first = page.element(option(0)).await?;
    page.measure("disable", page.control("disable-first"))
        .await?;
    first.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("ignored click", first.click()).await?;
    assert_that!(value(page).await?).is_equal_to(-1.0);
    page.measure("enable", page.control("enable-all")).await?;
    first.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}
