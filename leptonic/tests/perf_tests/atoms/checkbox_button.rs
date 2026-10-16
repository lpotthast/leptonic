// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `CheckboxButton` (`/perf/atoms/checkbox_button`): what each thing
//! it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/checkbox_button.json`): mounting and unmounting it (in its
//! `CheckboxField`), checking and unchecking it with the mouse, the keyboard and a virtual click,
//! its pressed state, hover, keyboard focus, and the presses it ignores while disabled or read-only.
//! The field's props are measured in `checkbox_field.rs`.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/checkbox_button";

/// Registers the performance tests of `CheckboxButton`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(CheckWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(CheckWithTheKeyboard))
        .with(fixture_test(CheckWithAVirtualClick))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(IgnoredWhileDisabled))
        .with(fixture_test(IgnoredWhileReadOnly))
}
const CHECKBOX: &str = "#perf-checkbox";

/// Whether the checkbox is checked (`1`) or not (`0`), as its `on_change` reported.
async fn selected(page: &Page<'_>) -> Result<f64, Report> {
    page.control("selected").await
}

/// How often the checkbox changed.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
}

/// Mounting the checkbox (its field and button) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(CHECKBOX, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(CHECKBOX).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(CHECKBOX, 0).await?;
    Ok(())
}

/// Clicks checking and unchecking the checkbox (the first click also focuses it).
#[browser_test]
pub async fn check_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let checkbox = page.element(CHECKBOX).await?;
    page.measure("check", checkbox.click()).await?;
    checkbox
        .wait_for_attr("data-selected", Some("true"))
        .await?;
    assert_that!(selected(page).await?).is_equal_to(1.0);
    page.measure("uncheck", checkbox.click()).await?;
    checkbox.wait_for_attr("data-selected", None).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Holding the pointer down (the pressed state) and releasing it (checking the checkbox).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let checkbox = page.element(CHECKBOX).await?;
    let held = page.measure("hold", checkbox.press_and_hold()).await?;
    checkbox.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    checkbox.wait_for_attr("data-pressed", None).await?;
    assert_that!(selected(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Space checking and unchecking the focused checkbox.
#[browser_test]
pub async fn check_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let checkbox = page.element(CHECKBOX).await?;
    checkbox
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("check", page.send_keys(" ")).await?;
    checkbox
        .wait_for_attr("data-selected", Some("true"))
        .await?;
    page.measure("uncheck", page.send_keys(" ")).await?;
    checkbox.wait_for_attr("data-selected", None).await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// Virtual clicks (as assistive technology clicks, without a pointer) checking and unchecking it.
#[browser_test]
pub async fn check_with_a_virtual_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let checkbox = page.element(CHECKBOX).await?;
    page.measure("check", checkbox.virtual_click()).await?;
    checkbox
        .wait_for_attr("data-selected", Some("true"))
        .await?;
    page.measure("uncheck", checkbox.virtual_click()).await?;
    checkbox.wait_for_attr("data-selected", None).await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// The pointer entering and leaving the checkbox.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let checkbox = page.element(CHECKBOX).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", checkbox.hover()).await?;
    checkbox.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    checkbox.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Tab moving focus to the checkbox (with a focus ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let checkbox = page.element(CHECKBOX).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    checkbox
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    checkbox.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// A click the disabled checkbox ignores.
#[browser_test]
pub async fn ignored_while_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let checkbox = page.element(CHECKBOX).await?;
    page.control("disable").await?;
    checkbox
        .wait_for_attr("data-disabled", Some("true"))
        .await?;
    page.measure("ignored click", checkbox.click()).await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    checkbox.wait_for_attr("data-selected", None).await?;
    Ok(())
}

/// A click the read-only checkbox ignores (it still takes focus).
#[browser_test]
pub async fn ignored_while_read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let checkbox = page.element(CHECKBOX).await?;
    page.control("make-read-only").await?;
    checkbox
        .wait_for_attr("data-readonly", Some("true"))
        .await?;
    page.measure("ignored click", checkbox.click()).await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    checkbox.wait_for_attr("data-selected", None).await?;
    Ok(())
}
