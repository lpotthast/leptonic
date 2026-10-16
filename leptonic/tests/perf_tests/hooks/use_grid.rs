// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_grid` (`/perf/hooks/use_grid`, a grid of hook rows and
//! cells on plain elements): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_grid.json`): mounting and unmounting
//! the grid and its rows, keyboard focus, its navigation keys, select all and Escape,
//! type-ahead, the selection mode (`aria-multiselectable`), keys while keyboard navigation is
//! disabled, and an empty grid (a tab stop itself), with 10 rows and with 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_grid";

/// Registers the performance tests of `use_grid`.
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
        .with(fixture_test(NavigationDisabled))
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

const GRID: &str = "[role=grid]";
const ROWS: &str = "[role=grid] > [role=row]";

/// Row `i` (from 0).
fn row(i: usize) -> String {
    format!("[role=grid] > [role=row]:nth-child({})", i + 1)
}

/// The first cell of row `i`.
fn first_cell(i: usize) -> String {
    format!("{} > [role=gridcell]:nth-child(1)", row(i))
}

/// Opens the fixture with its 10 rows.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ROWS, 10).await?;
    Ok(())
}

/// Opens the fixture and mounts the grid with 1,000 rows (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GRID, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(ROWS, 1_000).await?;
    Ok(())
}

/// Creates the live announcer's regions: its first announcement (of a selection change while
/// the grid has focus) waits 100 ms for them, which would fall into a later measurement.
async fn prepare_announcer(page: &Page<'_>) -> Result<(), Report> {
    page.control("prepare-announcer").await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    Ok(())
}

/// Focuses the grid with the keyboard (Tab from the plain button before it): its first row.
async fn focus_with_tab(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(row(0)).await?).await?;
    Ok(())
}

/// Mounting the grid with its `rows` rows and unmounting it.
async fn mount_and_unmount_rows(page: &Page<'_>, rows: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(GRID, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ROWS, rows).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GRID, 0).await?;
    Ok(())
}

/// Mounting the grid with 10 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_rows(page, 10).await
}

/// Mounting the grid with 1,000 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_rows(page, 1_000).await
}

/// Tab moving focus into the grid (its first row) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element(row(0)).await?).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    Ok(())
}

/// The navigation keys, from the first row: ArrowDown, ArrowUp, ArrowRight (into the row's
/// first cell), ArrowLeft (back to the row), End, Home, PageDown and PageUp.
async fn navigation_keys(page: &Page<'_>, last: usize) -> Result<(), Report> {
    focus_with_tab(page).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&page.element(row(1)).await?).await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    page.wait_for_focus(&page.element(row(0)).await?).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    page.wait_for_focus(&page.element(first_cell(0)).await?)
        .await?;
    page.measure("arrow left", page.send_keys(Key::Left))
        .await?;
    page.wait_for_focus(&page.element(row(0)).await?).await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    page.wait_for_focus(&page.element(row(last)).await?).await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&page.element(row(0)).await?).await?;
    page.measure("page down", page.send_keys(Key::PageDown))
        .await?;
    page.measure("page up", page.send_keys(Key::PageUp)).await?;
    page.wait_for_focus(&page.element(row(0)).await?).await?;
    Ok(())
}

/// The navigation keys in the grid of 10 rows.
#[browser_test]
pub async fn navigation_keys_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    navigation_keys(page, 9).await
}

/// The navigation keys in the grid of 1,000 rows.
#[browser_test]
pub async fn navigation_keys_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    navigation_keys(page, 999).await
}

/// Select all (Ctrl+A, Cmd+A on a Mac) and Escape clearing the selection, the grid focused.
async fn select_all_and_escape(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    focus_with_tab(page).await?;
    let select_all = page.primary_modifier().await? + "a";
    page.measure("select all", page.send_keys(select_all))
        .await?;
    page.element(row(0))
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    assert_that!(page.control("selected").await?).is_equal_to(-1.0);
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    page.element(row(0))
        .await?
        .wait_for_attr("aria-selected", Some("false"))
        .await?;
    assert_that!(page.control("selected").await?).is_equal_to(0.0);
    Ok(())
}

/// Select all and Escape with 10 rows.
#[browser_test]
pub async fn select_all_and_escape_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    select_all_and_escape(page).await
}

/// Select all and Escape with 1,000 rows.
#[browser_test]
pub async fn select_all_and_escape_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    select_all_and_escape(page).await
}

/// Typing the text of the last of 10 rows ("Row 9") moves focus to it (type-ahead). Measured
/// last: the search ends after a timeout.
#[browser_test]
pub async fn type_ahead_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    focus_with_tab(page).await?;
    type_each_key(page, "Row 9").await?;
    page.wait_for_focus(&page.element(row(9)).await?).await?;
    Ok(())
}

/// Typing the text of the last of 1,000 rows ("Row 999") moves focus to it.
#[browser_test]
pub async fn type_ahead_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    focus_with_tab(page).await?;
    type_each_key(page, "Row 999").await?;
    page.wait_for_focus(&page.element(row(999)).await?).await?;
    Ok(())
}

/// The selection mode changing to none (the rows lose `aria-selected`, the grid
/// `aria-multiselectable`) and back to multiple.
async fn selection_mode(page: &Page<'_>) -> Result<(), Report> {
    let grid = page.element(GRID).await?;
    let first = page.element(row(0)).await?;
    page.measure("none", page.control("selection-none")).await?;
    first.wait_for_attr("aria-selected", None).await?;
    grid.wait_for_attr("aria-multiselectable", None).await?;
    page.measure("multiple", page.control("selection-multiple"))
        .await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    grid.wait_for_attr("aria-multiselectable", Some("true"))
        .await?;
    Ok(())
}

/// The selection mode of the grid of 10 rows changing.
#[browser_test]
pub async fn selection_mode_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    selection_mode(page).await
}

/// The selection mode of the grid of 1,000 rows changing.
#[browser_test]
pub async fn selection_mode_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    selection_mode(page).await
}

/// Keyboard navigation disabled (as while a table column is resized with the arrow keys): an
/// ArrowDown the grid ignores; then enabled again.
#[browser_test]
pub async fn navigation_disabled(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    focus_with_tab(page).await?;
    page.measure("disable", page.control("disable-navigation"))
        .await?;
    page.measure("ignored arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&page.element(row(0)).await?).await?;
    page.measure("enable", page.control("enable-navigation"))
        .await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&page.element(row(1)).await?).await?;
    Ok(())
}

/// The grid losing all its rows (it becomes a tab stop itself) and getting them back.
#[browser_test]
pub async fn empty_and_filled(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let grid = page.element(GRID).await?;
    page.measure("empty", page.control("empty")).await?;
    page.wait_for_count(ROWS, 0).await?;
    grid.wait_for_attr("tabindex", Some("0")).await?;
    page.measure("fill", page.control("small")).await?;
    page.wait_for_count(ROWS, 10).await?;
    Ok(())
}
