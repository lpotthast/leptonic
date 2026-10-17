// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_option` (`/perf/hooks/use_option`, plain options of a
//! `use_listbox` listbox): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_option.json`): mounting and unmounting 10 and 1,000
//! options, selecting one with the mouse (at both sizes) and the keyboard, its pressed, hovered and
//! focused states, the focus moving from option to option, disabling it, and the state's selection
//! changing (one option, all options, at both sizes).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_option";

/// Registers the performance tests of `use_option`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(SelectWithTheMouseLarge))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocus))
        .with(fixture_test(SelectWithTheKeyboard))
        .with(fixture_test(Disabled))
        .with(fixture_test(StateSelection))
        .with(fixture_test(StateSelectionLarge))
}
const OPTIONS: &str = "[data-perf-option]";
/// The fixture's options after the control `large`.
const LARGE: u32 = 1_000;

/// The option "Option `i`".
fn option(i: u32) -> String {
    format!("[data-perf-option='{i}']")
}

/// Switches to [`LARGE`] options (unmeasured).
async fn large(page: &Page<'_>) -> Result<(), Report> {
    page.control("large").await?;
    page.element(option(LARGE - 1)).await?;
    Ok(())
}

/// Focuses the first option with the keyboard (Tab from the plain button before the listbox).
async fn tab_in(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(option(0)).await?;
    page.wait_for_focus(&first).await?;
    Ok(first)
}

/// Mounting `count` options in the listbox and unmounting them.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(OPTIONS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(OPTIONS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(OPTIONS, 0).await?;
    Ok(())
}

/// Mounting 10 options and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_and_unmount_in(page, 10).await
}

/// Mounting 1,000 options and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, 1_000).await
}

/// A click selecting an option (it takes focus) and one deselecting it.
async fn select_with_the_mouse_in(page: &Page<'_>) -> Result<(), Report> {
    let third = page.element(option(3)).await?;
    page.measure("select", third.click()).await?;
    third.wait_for_attr("aria-selected", Some("true")).await?;
    page.wait_for_focus(&third).await?;
    page.measure("deselect", third.click()).await?;
    third.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// Selecting and deselecting one of 10 options with clicks.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select_with_the_mouse_in(page).await
}

/// Selecting and deselecting one of 1,000 options with clicks.
#[browser_test]
pub async fn select_with_the_mouse_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    select_with_the_mouse_in(page).await
}

/// Holding the pointer down on an option (its pressed state; it selects) and releasing it.
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let third = page.element(option(3)).await?;
    let held = page.measure("hold", third.press_and_hold()).await?;
    third.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    third.wait_for_attr("data-pressed", None).await?;
    assert_that!(third)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    Ok(())
}

/// The pointer entering and leaving an option.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let third = page.element(option(3)).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", third.hover()).await?;
    third.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    third.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Keyboard focus coming to the first option (Tab) and moving to the next (ArrowDown).
#[browser_test]
pub async fn keyboard_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let first = page.element(option(0)).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    first
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    let second = page.element(option(1)).await?;
    page.measure("next", page.send_keys(Key::Down)).await?;
    second
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    assert_that!(first)
        .attribute("data-focus-visible")
        .await
        .is_none();
    Ok(())
}

/// Space selecting the focused option and Enter deselecting it.
#[browser_test]
pub async fn select_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let first = tab_in(page).await?;
    page.measure("space", page.send_keys(" ")).await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// The first option disabled, a click it ignores, and enabled again.
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let first = page.element(option(0)).await?;
    page.measure("disable", page.control("disable-first"))
        .await?;
    first.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("ignored click", first.click()).await?;
    assert_that!(first)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("false");
    page.measure("enable", page.control("enable-all")).await?;
    first.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}

/// The state's selection changing: the first option, all options (`last + 1`), none.
async fn state_selection_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    let first = page.element(option(0)).await?;
    let last = page.element(option(last)).await?;
    page.measure("select one", page.control("select-first"))
        .await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("select all", page.control("select-all"))
        .await?;
    last.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("clear", page.control("clear-selection"))
        .await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    last.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// The selection of 10 options changing.
#[browser_test]
pub async fn state_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    state_selection_in(page, 9).await
}

/// The selection of 1,000 options changing.
#[browser_test]
pub async fn state_selection_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    state_selection_in(page, 999).await
}
