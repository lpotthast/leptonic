// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_grid_list` (`/perf/hooks/use_grid_list`, a list of hook items
//! on plain elements): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_grid_list.json`): mounting and unmounting the
//! list and its items, keyboard focus, its navigation keys, select all and Escape, type-ahead,
//! the selection mode (`aria-multiselectable`), and the list emptied (a tab stop itself), with 10
//! items and with 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_grid_list";

/// Registers the performance tests of `use_grid_list`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(NavigationKeysSmall))
        .with(fixture_test(NavigationKeysLarge))
        .with(fixture_test(SelectAllAndEscapeSmall))
        .with(fixture_test(SelectAllAndEscapeLarge))
        .with(fixture_test(TypeAheadSmall))
        .with(fixture_test(TypeAheadLarge))
        .with(fixture_test(SelectionModeSmall))
        .with(fixture_test(SelectionModeLarge))
        .with(fixture_test(EmptyAndFilled))
}

/// Types `text` one key at a time, each key measured as "type" from an idle page (keys sent at
/// once fall into frames differently from run to run).
async fn type_each_key(page: &Page<'_>, text: &str) -> Result<(), Report> {
    for key in text.chars() {
        page.measure("type", page.send_keys(key.to_string()))
            .await?;
    }
    Ok(())
}

const LIST: &str = "[role=grid]";
const ITEMS: &str = "[role=grid] > [role=row]";

/// Item `i` (from 0).
fn item(i: usize) -> String {
    format!("[role=grid] > [role=row]:nth-child({})", i + 1)
}

/// Opens the fixture with its 10 items.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ITEMS, 10).await?;
    Ok(())
}

/// Opens the fixture and mounts the list with 1,000 items (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(ITEMS, 1_000).await?;
    Ok(())
}

/// Creates the live announcer's regions: its first announcement (of a selection change while
/// the list has focus) waits 100 ms for them, which would fall into a later measurement.
async fn prepare_announcer(page: &Page<'_>) -> Result<(), Report> {
    page.control("prepare-announcer").await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    Ok(())
}

/// Focuses the list with the keyboard (Tab from the plain button before it): its first item.
async fn focus_with_tab(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(item(0)).await?).await?;
    Ok(())
}

/// How many items are selected (-1: all).
async fn selected(page: &Page<'_>) -> Result<f64, Report> {
    page.control("selected").await
}

/// Mounting the list with its `items` items and unmounting it.
async fn mount_and_unmount_list(page: &Page<'_>, items: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ITEMS, items).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LIST, 0).await?;
    Ok(())
}

/// Mounting the list with 10 items and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_list(page, 10).await
}

/// Mounting the list with 1,000 items and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_list(page, 1_000).await
}

/// Tab moving focus into the list (its first item) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    let first = page.element(item(0)).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    Ok(())
}

/// The navigation keys, from the first item: ArrowDown, ArrowUp, End, Home, PageDown and
/// PageUp.
async fn navigation_keys(page: &Page<'_>, last: usize) -> Result<(), Report> {
    focus_with_tab(page).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&page.element(item(1)).await?).await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    page.wait_for_focus(&page.element(item(0)).await?).await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    page.wait_for_focus(&page.element(item(last)).await?)
        .await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&page.element(item(0)).await?).await?;
    page.measure("page down", page.send_keys(Key::PageDown))
        .await?;
    page.measure("page up", page.send_keys(Key::PageUp)).await?;
    page.wait_for_focus(&page.element(item(0)).await?).await?;
    Ok(())
}

/// The navigation keys in the list of 10 items.
#[browser_test]
pub async fn navigation_keys_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    navigation_keys(page, 9).await
}

/// The navigation keys in the list of 1,000 items.
#[browser_test]
pub async fn navigation_keys_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    navigation_keys(page, 999).await
}

/// Select all (Ctrl+A, Cmd+A on a Mac) and Escape clearing the selection, the list focused.
async fn select_all_and_escape(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    focus_with_tab(page).await?;
    let select_all = page.primary_modifier().await? + "a";
    page.measure("select all", page.send_keys(select_all))
        .await?;
    page.element(item(0))
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    assert_that!(selected(page).await?).is_equal_to(-1.0);
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    page.element(item(0))
        .await?
        .wait_for_attr("aria-selected", Some("false"))
        .await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Select all and Escape with 10 items.
#[browser_test]
pub async fn select_all_and_escape_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    select_all_and_escape(page).await
}

/// Select all and Escape with 1,000 items.
#[browser_test]
pub async fn select_all_and_escape_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    select_all_and_escape(page).await
}

/// Typing the text of the last of 10 items ("Item 9") moves focus to it (type-ahead). Measured
/// last: the search ends after a timeout.
#[browser_test]
pub async fn type_ahead_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    focus_with_tab(page).await?;
    type_each_key(page, "Item 9").await?;
    page.wait_for_focus(&page.element(item(9)).await?).await?;
    Ok(())
}

/// Typing the text of the last of 1,000 items ("Item 999") moves focus to it.
#[browser_test]
pub async fn type_ahead_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    focus_with_tab(page).await?;
    type_each_key(page, "Item 999").await?;
    page.wait_for_focus(&page.element(item(999)).await?).await?;
    Ok(())
}

/// The selection mode changing to none (the items lose `aria-selected`) and back to multiple.
async fn selection_mode(page: &Page<'_>) -> Result<(), Report> {
    let list = page.element(LIST).await?;
    let first = page.element(item(0)).await?;
    page.measure("none", page.control("selection-none")).await?;
    first.wait_for_attr("aria-selected", None).await?;
    list.wait_for_attr("aria-multiselectable", None).await?;
    page.measure("multiple", page.control("selection-multiple"))
        .await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    list.wait_for_attr("aria-multiselectable", Some("true"))
        .await?;
    Ok(())
}

/// The selection mode of the list of 10 items changing.
#[browser_test]
pub async fn selection_mode_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    selection_mode(page).await
}

/// The selection mode of the list of 1,000 items changing.
#[browser_test]
pub async fn selection_mode_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    selection_mode(page).await
}

/// The list losing all its items (it becomes a tab stop itself) and getting them back.
#[browser_test]
pub async fn empty_and_filled(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let list = page.element(LIST).await?;
    page.measure("empty", page.control("empty")).await?;
    page.wait_for_count(ITEMS, 0).await?;
    list.wait_for_attr("tabindex", Some("0")).await?;
    page.measure("fill", page.control("small")).await?;
    page.wait_for_count(ITEMS, 10).await?;
    Ok(())
}
