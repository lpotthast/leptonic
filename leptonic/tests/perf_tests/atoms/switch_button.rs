// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `SwitchButton` (`/perf/atoms/switch_button`): what each thing
//! it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/switch_button.json`): mounting and unmounting it (in its
//! `SwitchField`), switching it on and off with the mouse, the keyboard and a virtual click,
//! its pressed state, hover, keyboard focus, and the presses it ignores while disabled or read-only.
//! The field's props are measured in `switch_field.rs`.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/switch_button";

/// Registers the performance tests of `SwitchButton`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SwitchWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(SwitchWithTheKeyboard))
        .with(fixture_test(SwitchWithAVirtualClick))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(IgnoredWhileDisabled))
        .with(fixture_test(IgnoredWhileReadOnly))
}
const SWITCH: &str = "#perf-switch";

/// Whether the switch is on (`1`) or not (`0`), as its `on_change` reported.
async fn selected(page: &Page<'_>) -> Result<f64, Report> {
    page.control("selected").await
}

/// How often the switch changed.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
}

/// Mounting the switch (its field and button) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SWITCH, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(SWITCH).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SWITCH, 0).await?;
    Ok(())
}

/// Clicks switching the switch on and off (the first click also focuses it).
#[browser_test]
pub async fn switch_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let switch = page.element(SWITCH).await?;
    page.measure("switch on", switch.click()).await?;
    switch.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(selected(page).await?).is_equal_to(1.0);
    page.measure("switch off", switch.click()).await?;
    switch.wait_for_attr("data-selected", None).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Holding the pointer down (the pressed state) and releasing it (switching it on).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let switch = page.element(SWITCH).await?;
    let held = page.measure("hold", switch.press_and_hold()).await?;
    switch.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    switch.wait_for_attr("data-pressed", None).await?;
    assert_that!(selected(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Space switching on and off the focused switch.
#[browser_test]
pub async fn switch_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let switch = page.element(SWITCH).await?;
    switch
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("switch on", page.send_keys(" ")).await?;
    switch.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("switch off", page.send_keys(" ")).await?;
    switch.wait_for_attr("data-selected", None).await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// Virtual clicks (as assistive technology clicks, without a pointer) switching it on and off.
#[browser_test]
pub async fn switch_with_a_virtual_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let switch = page.element(SWITCH).await?;
    page.measure("switch on", switch.virtual_click()).await?;
    switch.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("switch off", switch.virtual_click()).await?;
    switch.wait_for_attr("data-selected", None).await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// The pointer entering and leaving the switch.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let switch = page.element(SWITCH).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", switch.hover()).await?;
    switch.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    switch.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Tab moving focus to the switch (with a focus ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let switch = page.element(SWITCH).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    switch
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    switch.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// A click the disabled switch ignores.
#[browser_test]
pub async fn ignored_while_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let switch = page.element(SWITCH).await?;
    page.control("disable").await?;
    switch.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", switch.click()).await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    switch.wait_for_attr("data-selected", None).await?;
    Ok(())
}

/// A click the read-only switch ignores (it still takes focus).
#[browser_test]
pub async fn ignored_while_read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let switch = page.element(SWITCH).await?;
    page.control("make-read-only").await?;
    switch.wait_for_attr("data-readonly", Some("true")).await?;
    page.measure("ignored click", switch.click()).await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    switch.wait_for_attr("data-selected", None).await?;
    Ok(())
}
