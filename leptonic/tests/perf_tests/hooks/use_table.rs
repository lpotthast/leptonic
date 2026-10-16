// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_table` (`/perf/hooks/use_table`, a table of hook rows and
//! cells on plain elements): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_table.json`): mounting and unmounting
//! the table and its rows, keyboard focus, its navigation keys, select all and Escape,
//! type-ahead, the sort changing (its description and announcement), and the table emptied (a
//! tab stop itself), with 10 rows and with 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_table";

/// Registers the performance tests of `use_table`.
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
        .with(fixture_test(SortSmall))
        .with(fixture_test(SortLarge))
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

const TABLE: &str = "[role=grid]";
const ROWS: &str = "[role=grid] > tbody > tr";
/// The column header of the column "Value".
const VALUE_HEADER: &str = "[role=grid] > thead th:nth-child(2)";
/// The assertive announcements (of sort changes, each removed after 500 ms).
const SORT_ANNOUNCEMENTS: &str = "[data-live-announcer] [aria-live=assertive] > div";

/// Row `i` (from 0).
fn row(i: usize) -> String {
    format!("[role=grid] > tbody > tr:nth-child({})", i + 1)
}

/// The first cell of row `i`.
fn first_cell(i: usize) -> String {
    format!("{} > td:nth-child(1)", row(i))
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

/// The sort changing to the column "Value" ascending, then descending (the column
/// header's `aria-sort`, the table's description, an announcement). Each measured after the
/// previous announcement was removed (500 ms).
async fn sort(page: &Page<'_>) -> Result<(), Report> {
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
pub async fn sort_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    sort(page).await
}

/// The sort of the table of 1,000 rows changing.
#[browser_test]
pub async fn sort_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    sort(page).await
}

/// The table losing all its rows (it becomes a tab stop itself) and getting them back.
#[browser_test]
pub async fn empty_and_filled(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let table = page.element(TABLE).await?;
    page.measure("empty", page.control("empty")).await?;
    page.wait_for_count(ROWS, 0).await?;
    table.wait_for_attr("tabindex", Some("0")).await?;
    page.measure("fill", page.control("small")).await?;
    page.wait_for_count(ROWS, 10).await?;
    Ok(())
}
