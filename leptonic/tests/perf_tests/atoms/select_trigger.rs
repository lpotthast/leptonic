// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `SelectTrigger` (`/perf/atoms/select_trigger`, the fixture of
//! `Select`'s performance test): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/select_trigger.json`): mounting and unmounting
//! it in its select, opening the popover with the mouse and with each key that does (Enter, Space,
//! ArrowDown, ArrowUp), changing the value with ArrowRight, ArrowLeft and type-ahead (10 and 1,000
//! options), its pressed, hovered and focused states, and the select's disabled and open states
//! changing.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use super::select::{POPOVER, TRIGGER, change_on_the_trigger_in, focus_trigger, large, option};
use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/select_trigger";

/// Registers the performance tests of `SelectTrigger`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenWithTheMouse))
        .with(fixture_test(OpenWithEnter))
        .with(fixture_test(OpenWithSpace))
        .with(fixture_test(OpenWithArrowDown))
        .with(fixture_test(OpenWithArrowUp))
        .with(fixture_test(ChangeTheValue))
        .with(fixture_test(ChangeTheValueLarge))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(OpenState))
}

/// Mounting the trigger in its select and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TRIGGER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TRIGGER).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TRIGGER, 0).await?;
    Ok(())
}

/// A click on the trigger opening the popover (10 options).
#[browser_test]
pub async fn open_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("click", trigger.click()).await?;
    page.element(option(9)).await?;
    trigger.wait_for_attr("data-open", Some("true")).await?;
    Ok(())
}

/// Enter on the focused trigger opening the popover. (Each key in a test of its own: what an
/// opening costs depends on the popover's state from an earlier opening, e.g. its scroll position.)
#[browser_test]
pub async fn open_with_enter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_trigger(page).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    page.element(option(9)).await?;
    Ok(())
}

/// Space on the focused trigger opening the popover.
#[browser_test]
pub async fn open_with_space(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_trigger(page).await?;
    page.measure("space", page.send_keys(" ")).await?;
    page.element(option(9)).await?;
    Ok(())
}

/// ArrowDown on the focused trigger opening the popover, the first option focused.
#[browser_test]
pub async fn open_with_arrow_down(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_trigger(page).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&page.element(option(0)).await?).await?;
    Ok(())
}

/// ArrowUp on the focused trigger opening the popover, the last option focused.
#[browser_test]
pub async fn open_with_arrow_up(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_trigger(page).await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    page.wait_for_focus(&page.element(option(9)).await?).await?;
    Ok(())
}

/// Changing the value on the closed trigger with 10 options.
#[browser_test]
pub async fn change_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    change_on_the_trigger_in(page).await
}

/// Changing the value on the closed trigger with 1,000 options.
#[browser_test]
pub async fn change_the_value_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    change_on_the_trigger_in(page).await
}

/// Holding the pointer down on the trigger (its pressed state; the popover opens) and releasing
/// it.
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    let held = page.measure("hold", trigger.press_and_hold()).await?;
    trigger.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    page.element(option(0)).await?;
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

/// Tab moving focus to the trigger and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&trigger).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    Ok(())
}

/// The select disabled (the trigger too), a click on the trigger it ignores, and enabled again.
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

/// The select's controlled open state changing: the trigger shows it.
#[browser_test]
pub async fn open_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("open", page.control("open")).await?;
    trigger.wait_for_attr("data-open", Some("true")).await?;
    page.measure("close", page.control("close")).await?;
    trigger.wait_for_attr("data-open", None).await?;
    Ok(())
}
