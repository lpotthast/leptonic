// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Table` (`/perf/atoms/table`): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget (`tests/budgets/atoms/table.json`):
//! mounting and unmounting it, keyboard focus, its navigation keys, select all and Escape,
//! type-ahead, and changes of its controlled selection and sort, reactive props (selection mode,
//! selection checkboxes, disabled rows) and rows, with 10 rows and with 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/table";

/// Registers the performance tests of `Table`.
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
        .with(fixture_test(ControlledSelectionSmall))
        .with(fixture_test(ControlledSelectionLarge))
        .with(fixture_test(ControlledSortSmall))
        .with(fixture_test(ControlledSortLarge))
        .with(fixture_test(SelectionModeSmall))
        .with(fixture_test(SelectionModeLarge))
        .with(fixture_test(SelectionCheckboxesSmall))
        .with(fixture_test(SelectionCheckboxesLarge))
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

const TABLE: &str = "[role=grid]";
const ROWS: &str = "[role=grid] > tbody > tr";
/// The column header of the column "Value" (after the selection checkboxes' and "Name").
const VALUE_HEADER: &str = "[role=grid] > thead th:nth-child(3)";
/// The assertive announcements (of sort changes, each removed after 500 ms).
const SORT_ANNOUNCEMENTS: &str = "[data-live-announcer] [aria-live=assertive] > div";

/// Row `i` (from 0).
fn row(i: usize) -> String {
    format!("[role=grid] > tbody > tr:nth-child({})", i + 1)
}

/// The selection checkbox in the first cell of row `i` (the cell's child, which takes its
/// focus).
fn first_cell(i: usize) -> String {
    format!("{} > td:nth-child(1) input", row(i))
}

/// Opens the fixture with its 10 rows.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ROWS, 10).await?;
    Ok(())
}

/// Opens the fixture and mounts the table with 1,000 rows (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TABLE, 0).await?;
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

/// Focuses the table with the keyboard (Tab from the plain button before it): its first row.
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

/// Mounting the table with its `rows` rows and unmounting it.
async fn mount_and_unmount_table(page: &Page<'_>, rows: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(TABLE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ROWS, rows).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TABLE, 0).await?;
    Ok(())
}

/// Mounting the table with 10 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_table(page, 10).await
}

/// Mounting the table with 1,000 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_table(page, 1_000).await
}

/// Tab moving focus into the table (its first row) and on.
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
/// first cell's
/// selection checkbox), ArrowLeft (back to the row), End, Home, PageDown and PageUp.
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
    // Where the page keys land depends on the layout (a page above the first rows is the
    // column header row): focus stays in the table.
    assert_that!(page.focused_element().await?.is_within(TABLE).await?).is_true();
    Ok(())
}

/// The navigation keys in the table of 10 rows.
#[browser_test]
pub async fn navigation_keys_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    navigation_keys(page, 9).await
}

/// The navigation keys in the table of 1,000 rows.
#[browser_test]
pub async fn navigation_keys_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    navigation_keys(page, 999).await
}

/// Select all (Ctrl+A, Cmd+A on a Mac) and Escape clearing the selection, the table focused.
async fn select_all_and_escape(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    focus_with_tab(page).await?;
    let select_all = page.primary_modifier().await? + "a";
    page.measure("select all", page.send_keys(select_all))
        .await?;
    page.element(row(0))
        .await?
        .wait_for_attr("data-selected", Some("true"))
        .await?;
    assert_that!(selected(page).await?).is_equal_to(-1.0);
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    page.element(row(0))
        .await?
        .wait_for_attr("data-selected", None)
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

/// The controlled selection changing: the first row, none, all rows, none.
async fn controlled_selection(page: &Page<'_>, last: usize) -> Result<(), Report> {
    let first = page.element(row(0)).await?;
    let last = page.element(row(last)).await?;
    page.measure("select one", page.control("select-first"))
        .await?;
    first.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("clear", page.control("clear-selection"))
        .await?;
    first.wait_for_attr("data-selected", None).await?;
    page.measure("select all", page.control("select-all"))
        .await?;
    last.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("clear", page.control("clear-selection"))
        .await?;
    last.wait_for_attr("data-selected", None).await?;
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

/// The controlled sort changing to the column "Value" ascending, then descending (the column
/// header's `aria-sort`, the table's description, an announcement). Each measured after the
/// previous announcement was removed (500 ms).
async fn controlled_sort(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    let header = page.element(VALUE_HEADER).await?;
    page.measure("ascending", page.control("sort-ascending"))
        .await?;
    header.wait_for_attr("aria-sort", Some("ascending")).await?;
    page.wait_for_count(SORT_ANNOUNCEMENTS, 0).await?;
    page.measure("descending", page.control("sort-descending"))
        .await?;
    header
        .wait_for_attr("aria-sort", Some("descending"))
        .await?;
    page.wait_for_count(SORT_ANNOUNCEMENTS, 0).await?;
    Ok(())
}

/// The sort of the table of 10 rows changing.
#[browser_test]
pub async fn controlled_sort_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    controlled_sort(page).await
}

/// The sort of the table of 1,000 rows changing.
#[browser_test]
pub async fn controlled_sort_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    controlled_sort(page).await
}

/// The selection mode changing to none (the rows lose `aria-selected`, the selection
/// checkboxes go) and back to multiple.
async fn selection_mode(page: &Page<'_>) -> Result<(), Report> {
    let first = page.element(row(0)).await?;
    page.measure("none", page.control("selection-none")).await?;
    first.wait_for_attr("aria-selected", None).await?;
    page.wait_for_count("[role=grid] input[type=checkbox]", 0)
        .await?;
    page.measure("multiple", page.control("selection-multiple"))
        .await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// The selection mode of the table of 10 rows changing.
#[browser_test]
pub async fn selection_mode_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    selection_mode(page).await
}

/// The selection mode of the table of 1,000 rows changing.
#[browser_test]
pub async fn selection_mode_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    selection_mode(page).await
}

/// The selection checkboxes hidden (their column goes) and shown again.
async fn selection_checkboxes(page: &Page<'_>, rows: usize) -> Result<(), Report> {
    let checkboxes = "[role=grid] input[type=checkbox]";
    page.measure("hide", page.control("hide-checkboxes"))
        .await?;
    page.wait_for_count(checkboxes, 0).await?;
    page.measure("show", page.control("show-checkboxes"))
        .await?;
    // A checkbox per row, and select all.
    page.wait_for_count(checkboxes, rows + 1).await?;
    Ok(())
}

/// The selection checkboxes of 10 rows hidden and shown.
#[browser_test]
pub async fn selection_checkboxes_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    selection_checkboxes(page, 10).await
}

/// The selection checkboxes of 1,000 rows hidden and shown.
#[browser_test]
pub async fn selection_checkboxes_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    selection_checkboxes(page, 1_000).await
}

/// Disabling the first row (with the table's `DisabledBehavior::Selection`, only its selection:
/// its checkbox is disabled) and enabling it again.
#[browser_test]
pub async fn disabled_rows(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let checkbox = page.element(first_cell(0)).await?;
    page.measure("disable", page.control("disable-first"))
        .await?;
    checkbox.wait_for_prop("disabled", "true").await?;
    page.measure("enable", page.control("enable-all")).await?;
    checkbox.wait_for_prop("disabled", "false").await?;
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
