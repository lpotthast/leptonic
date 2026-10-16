// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ListBoxItem` (`/perf/atoms/list_box_item`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/list_box_item.json`): mounting and unmounting 10 and 1,000 items in
//! their listbox, selecting one with the mouse and the keyboard, its pressed, hovered and focused
//! states, and the changes of its selected and disabled state from the listbox.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/list_box_item";

/// Registers the performance tests of `ListBoxItem`.
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
        .with(fixture_test(ControlledSelection))
        .with(fixture_test(ControlledSelectionLarge))
}
/// The fixture's items after the control `large`.
const LARGE: u32 = 1_000;

/// The option "Option `i`".
fn option(i: u32) -> String {
    format!("[data-perf-option='{i}']")
}

/// Remounts [`LARGE`] items (unmeasured).
async fn mount_large(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(option(0), 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.element(option(LARGE - 1)).await?;
    Ok(())
}

/// Mounting 10 items in the listbox and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(option(0), 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(option(9)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(option(0), 0).await?;
    Ok(())
}

/// Mounting 1,000 items in the listbox and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(option(0), 0).await?;
    page.control("large").await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(option(LARGE - 1)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(option(0), 0).await?;
    Ok(())
}

/// A click selecting an item (it takes focus) and one deselecting it.
async fn select_with_the_mouse_in(page: &Page<'_>) -> Result<(), Report> {
    let third = page.element(option(3)).await?;
    page.measure("select", third.click()).await?;
    third.wait_for_attr("aria-selected", Some("true")).await?;
    page.wait_for_focus(&third).await?;
    page.measure("deselect", third.click()).await?;
    third.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// Selecting and deselecting one of 10 items with clicks.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select_with_the_mouse_in(page).await
}

/// Selecting and deselecting one of 1,000 items with clicks.
#[browser_test]
pub async fn select_with_the_mouse_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    select_with_the_mouse_in(page).await
}

/// Holding the pointer down on an item (its pressed state; it selects) and releasing it.
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

/// The pointer entering and leaving an item.
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

/// Keyboard focus coming to the first item (Tab), moving to the next (ArrowDown) and leaving
/// the listbox (Tab).
#[browser_test]
pub async fn keyboard_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let first = page.element(option(0)).await?;
    let second = page.element(option(1)).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    first
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("next", page.send_keys(Key::Down)).await?;
    second
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    assert_that!(first)
        .attribute("data-focused")
        .await
        .is_none();
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    second.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// Space selecting the focused item and Enter deselecting it.
#[browser_test]
pub async fn select_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(option(0)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("space", page.send_keys(" ")).await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// The first item disabled, a click it ignores, and enabled again.
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let first = page.element(option(0)).await?;
    page.measure("disable", page.control("disable-first"))
        .await?;
    first.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", first.click()).await?;
    assert_that!(first)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("false");
    page.measure("enable", page.control("enable-all")).await?;
    first.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// The first item selected and deselected through the listbox's controlled selection.
async fn controlled_selection_in(page: &Page<'_>) -> Result<(), Report> {
    let first = page.element(option(0)).await?;
    page.measure("select", page.control("select-first")).await?;
    first.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("deselect", page.control("clear-selection"))
        .await?;
    first.wait_for_attr("data-selected", None).await?;
    Ok(())
}

/// The controlled selection of one of 10 items changing.
#[browser_test]
pub async fn controlled_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    controlled_selection_in(page).await
}

/// The controlled selection of one of 1,000 items changing.
#[browser_test]
pub async fn controlled_selection_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    controlled_selection_in(page).await
}
