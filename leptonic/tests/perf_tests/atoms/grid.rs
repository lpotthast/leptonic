// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Grid` (`/perf/atoms/grid`): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget (`tests/budgets/atoms/grid.json`):
//! mounting and unmounting it, keyboard focus, its navigation keys, select all and Escape,
//! type-ahead, and changes of its controlled selection, reactive props (selection mode and
//! behavior, disabled rows) and rows, with 10 rows and with 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/grid";

/// Registers the performance tests of `Grid`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(NavigationKeysSmall))
        .with(fixture_test(NavigationKeysLarge))
        .with(fixture_test(SelectAllAndEscapeSmall))
        .with(fixture_test(SelectAllAndEscapeLarge))
        .with(fixture_test(TypeAhead))
        .with(fixture_test(TypeAheadLarge))
        .with(fixture_test(ControlledSelectionSmall))
        .with(fixture_test(ControlledSelectionLarge))
        .with(fixture_test(SelectionModeSmall))
        .with(fixture_test(SelectionModeLarge))
        .with(fixture_test(SelectionBehavior))
        .with(fixture_test(DisabledRows))
        .with(fixture_test(AddAndRemoveRowSmall))
        .with(fixture_test(AddAndRemoveRowLarge))
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

/// Creates the live announcer's regions: its first announcement waits 100 ms for them, which
/// would fall into a later measurement.
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

/// How many rows are selected (-1: all).
async fn selected(page: &Page<'_>) -> Result<f64, Report> {
    page.control("selected").await
}

/// Mounting the grid with 10 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(GRID, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ROWS, 10).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GRID, 0).await?;
    Ok(())
}

/// Mounting the grid with 1,000 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(GRID, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ROWS, 1_000).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GRID, 0).await?;
    Ok(())
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
    assert_that!(selected(page).await?).is_equal_to(-1.0);
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    page.element(row(0))
        .await?
        .wait_for_attr("aria-selected", Some("false"))
        .await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
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

/// Typing the text of the last row ("Row 9") moves focus to it (type-ahead). Measured last: the
/// search ends after a timeout.
#[browser_test]
pub async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
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

/// The controlled selection changing: the first row, none, all rows, none.
async fn controlled_selection(page: &Page<'_>, last: usize) -> Result<(), Report> {
    let first = page.element(row(0)).await?;
    let last = page.element(row(last)).await?;
    page.measure("select one", page.control("select-first"))
        .await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("clear", page.control("clear-selection"))
        .await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    page.measure("select all", page.control("select-all"))
        .await?;
    last.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("clear", page.control("clear-selection"))
        .await?;
    last.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// The controlled selection of 10 rows changing.
#[browser_test]
pub async fn controlled_selection_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    controlled_selection(page, 9).await
}

/// The controlled selection of 1,000 rows changing.
#[browser_test]
pub async fn controlled_selection_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    controlled_selection(page, 999).await
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

/// The selection behavior changing to replace and back to toggle; then a click on the second
/// row selects it alone (toggle adds to the selection).
#[browser_test]
pub async fn selection_behavior(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("replace", page.control("behavior-replace"))
        .await?;
    page.measure("toggle", page.control("behavior-toggle"))
        .await?;
    prepare_announcer(page).await?;
    page.control("select-first").await?;
    page.element(row(1)).await?.click().await?;
    assert_that!(selected(page).await?).is_equal_to(2.0);
    Ok(())
}

/// Disabling the first row (`aria-disabled`) and enabling it again.
#[browser_test]
pub async fn disabled_rows(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let first = page.element(row(0)).await?;
    page.measure("disable", page.control("disable-first"))
        .await?;
    first.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("enable", page.control("enable-all")).await?;
    first.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}

/// A row added at the end and removed again.
async fn add_and_remove_row(page: &Page<'_>, rows: usize) -> Result<(), Report> {
    page.measure("add", page.control("add-row")).await?;
    page.wait_for_count(ROWS, rows + 1).await?;
    page.measure("remove", page.control("remove-row")).await?;
    page.wait_for_count(ROWS, rows).await?;
    Ok(())
}

/// A row added to the 10 and removed again.
#[browser_test]
pub async fn add_and_remove_row_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    add_and_remove_row(page, 10).await
}

/// A row added to the 1,000 and removed again.
#[browser_test]
pub async fn add_and_remove_row_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    add_and_remove_row(page, 1_000).await
}
