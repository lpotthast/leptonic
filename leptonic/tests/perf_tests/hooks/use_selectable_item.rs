// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_selectable_item` (`/perf/hooks/use_selectable_item`, the plain
//! items of the list of `use_selectable_list`'s performance test): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_selectable_item.json`): mounting and unmounting 10 and 1,000 items,
//! selecting one with the mouse (at both sizes) and the keyboard, its pressed state, the focus
//! moving from item to item, disabling it, and the state's selection changing (all items, at both
//! sizes).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use super::use_selectable_list::{ITEMS, item, large, mount_and_unmount_in, tab_in};
use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_selectable_item";

/// Registers the performance tests of `use_selectable_item`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(SelectWithTheMouseLarge))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(KeyboardFocus))
        .with(fixture_test(SelectWithTheKeyboard))
        .with(fixture_test(Disabled))
        .with(fixture_test(StateSelection))
        .with(fixture_test(StateSelectionLarge))
}

/// Mounting 10 items and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_and_unmount_in(page, ITEMS, 10).await
}

/// Mounting 1,000 items and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, ITEMS, 1_000).await
}

/// A click selecting an item (it takes focus) and one deselecting it.
async fn select_with_the_mouse_in(page: &Page<'_>) -> Result<(), Report> {
    let third = page.element(item(3)).await?;
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
    large(page).await?;
    select_with_the_mouse_in(page).await
}

/// Holding the pointer down on an item (its pressed state; it selects) and releasing it.
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let third = page.element(item(3)).await?;
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

/// Keyboard focus coming to the first item (Tab) and moving to the next (ArrowDown).
#[browser_test]
pub async fn keyboard_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let first = page.element(item(0)).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    first.wait_for_attr("data-focused", Some("true")).await?;
    let second = page.element(item(1)).await?;
    page.measure("next", page.send_keys(Key::Down)).await?;
    second.wait_for_attr("data-focused", Some("true")).await?;
    assert_that!(first)
        .attribute("data-focused")
        .await
        .is_none();
    Ok(())
}

/// Space selecting the focused item and Enter deselecting it.
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

/// The first item disabled, a click it ignores, and enabled again.
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let first = page.element(item(0)).await?;
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

/// The state selecting all `last + 1` items and clearing the selection.
async fn state_selection_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    let last = page.element(item(last)).await?;
    page.measure("select all", page.control("select-all"))
        .await?;
    last.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("clear", page.control("clear-selection"))
        .await?;
    last.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// The selection of 10 items changing.
#[browser_test]
pub async fn state_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    state_selection_in(page, 9).await
}

/// The selection of 1,000 items changing.
#[browser_test]
pub async fn state_selection_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    state_selection_in(page, 999).await
}
