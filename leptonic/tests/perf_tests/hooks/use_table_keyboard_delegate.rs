// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_table_keyboard_delegate`
//! (`/perf/hooks/use_table_keyboard_delegate`, the delegate of a hook table): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_table_keyboard_delegate.json`): mounting the table with it, every
//! navigation key it computes (also onto the column headers, and the page keys, which measure the
//! rendered rows), type-ahead, and a change of the reading direction, with 10 rows and with
//! 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_table_keyboard_delegate";

/// Registers the performance tests of `use_table_keyboard_delegate`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(NavigationKeysSmall))
        .with(fixture_test(NavigationKeysLarge))
        .with(fixture_test(TypeAheadSmall))
        .with(fixture_test(TypeAheadLarge))
        .with(fixture_test(ReadingDirection))
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

/// Focuses the table with the keyboard (Tab from the plain button before it): its first row.
async fn focus_with_tab(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(row(0)).await?).await?;
    Ok(())
}

/// Mounting the table (and its delegate) with its `rows` rows and unmounting it.
async fn mount_and_unmount_rows(page: &Page<'_>, rows: usize) -> Result<(), Report> {
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
    mount_and_unmount_rows(page, 10).await
}

/// Mounting the table with 1,000 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_rows(page, 1_000).await
}

/// The navigation keys, from the first row: ArrowDown, ArrowUp, ArrowRight (into the row's
/// first cell), ArrowUp (onto its
/// column header), ArrowDown (back), ArrowLeft (back to the row), End, Home, PageDown and PageUp.
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
    page.measure("arrow up to the header", page.send_keys(Key::Up))
        .await?;
    page.wait_for_focus(&page.element("[role=grid] > thead th:nth-child(1)").await?)
        .await?;
    page.measure("arrow down from the header", page.send_keys(Key::Down))
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

/// Typing the text of the last of 10 rows ("Row 9") moves focus to it (the delegate's search).
/// Measured last: the search ends after a timeout.
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

/// The locale switching to a right-to-left one (the delegate is rebuilt: ArrowLeft now moves
/// into a row's cells) and back.
#[browser_test]
pub async fn reading_direction(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("rtl", page.control("rtl")).await?;
    focus_with_tab(page).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&page.element(first_cell(0)).await?)
        .await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&page.element(row(0)).await?).await?;
    page.measure("ltr", page.control("ltr")).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&page.element(first_cell(0)).await?)
        .await?;
    Ok(())
}
