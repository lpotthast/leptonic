// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ToggleButton` (`/perf/atoms/toggle_button`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/toggle_button.json`): mounting and unmounting it, selecting and
//! deselecting it with the mouse and the keyboard, its pressed state, hover, keyboard focus,
//! disabling it, and a controlled one's selection changing from outside and by a click.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/toggle_button";

/// Registers the performance tests of `ToggleButton`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ToggleWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(ToggleWithTheKeyboard))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(Controlled))
}
const TOGGLE: &str = "#perf-toggle";
const CONTROLLED: &str = "#perf-toggle-controlled";

/// How often the selection changed.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
}

/// Mounting the toggle button and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TOGGLE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TOGGLE).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TOGGLE, 0).await?;
    Ok(())
}

/// A click selecting it, and one deselecting it.
#[browser_test]
pub async fn toggle_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page.element(TOGGLE).await?;
    page.measure("select", toggle.click()).await?;
    toggle.wait_for_attr("aria-pressed", Some("true")).await?;
    page.measure("deselect", toggle.click()).await?;
    toggle.wait_for_attr("aria-pressed", Some("false")).await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// Holding the pointer down (the pressed state) and releasing it (selecting it).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page.element(TOGGLE).await?;
    let held = page.measure("hold", toggle.press_and_hold()).await?;
    toggle.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    toggle.wait_for_attr("data-pressed", None).await?;
    toggle.wait_for_attr("data-selected", Some("true")).await?;
    Ok(())
}

/// Space selecting it and Enter deselecting it, focused.
#[browser_test]
pub async fn toggle_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let toggle = page.element(TOGGLE).await?;
    page.wait_for_focus(&toggle).await?;
    page.measure("space", page.send_keys(" ")).await?;
    toggle.wait_for_attr("aria-pressed", Some("true")).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    toggle.wait_for_attr("aria-pressed", Some("false")).await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// The pointer entering and leaving the toggle button.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page.element(TOGGLE).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", toggle.hover()).await?;
    toggle.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    toggle.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Tab moving focus to the toggle button (with a focus ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let toggle = page.element(TOGGLE).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    toggle
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    toggle.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// Disabling the toggle button, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page.element(TOGGLE).await?;
    page.measure("disable", page.control("disable")).await?;
    toggle.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", toggle.click()).await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    toggle.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// A controlled toggle button: selected and deselected from outside, and selected by a click.
#[browser_test]
pub async fn controlled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page.element(CONTROLLED).await?;
    page.measure("select", page.control("select-controlled"))
        .await?;
    toggle.wait_for_attr("aria-pressed", Some("true")).await?;
    page.measure("deselect", page.control("deselect-controlled"))
        .await?;
    toggle.wait_for_attr("aria-pressed", Some("false")).await?;
    page.measure("click", toggle.click()).await?;
    toggle.wait_for_attr("aria-pressed", Some("true")).await?;
    Ok(())
}
