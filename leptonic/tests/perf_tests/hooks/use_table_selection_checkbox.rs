// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_table_selection_checkbox`
//! (`/perf/hooks/use_table_selection_checkbox`, the selection checkboxes of a hook table): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_table_selection_checkbox.json`): mounting and unmounting the table
//! with them, selecting and deselecting a row with a click, the selection changing elsewhere (all
//! rows, none), and a disabled row's checkbox ignoring a click, in tables of 10 rows and of
//! 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_table_selection_checkbox";

/// Registers the performance tests of `use_table_selection_checkbox`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(ClickSmall))
        .with(fixture_test(ClickLarge))
        .with(fixture_test(SelectionChangeSmall))
        .with(fixture_test(SelectionChangeLarge))
        .with(fixture_test(DisableAndEnable))
}

const TABLE: &str = "[role=grid]";
const CHECKBOXES: &str = "[role=grid] > tbody input";

/// The checkbox of row `i` (from 0).
fn checkbox(i: usize) -> String {
    format!("[role=grid] > tbody > tr:nth-child({}) input", i + 1)
}

/// Opens the fixture with its 10 rows.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(CHECKBOXES, 10).await?;
    Ok(())
}

/// Opens the fixture and mounts the table with 1,000 rows (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TABLE, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(CHECKBOXES, 1_000).await?;
    Ok(())
}

/// How many rows are selected (-1: all).
async fn selected(page: &Page<'_>) -> Result<f64, Report> {
    page.control("selected").await
}

/// Mounting the checkboxes of `rows` rows and unmounting them.
async fn mount_and_unmount_checkboxes(page: &Page<'_>, rows: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(TABLE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(CHECKBOXES, rows).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TABLE, 0).await?;
    Ok(())
}

/// Mounting the checkboxes of 10 rows and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_checkboxes(page, 10).await
}

/// Mounting the checkboxes of 1,000 rows and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_checkboxes(page, 1_000).await
}

/// Creates the live announcer's regions: its first announcement (of a selection change while
/// the table has focus) waits 100 ms for them, which would fall into a later measurement.
async fn prepare_announcer(page: &Page<'_>) -> Result<(), Report> {
    page.control("prepare-announcer").await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    Ok(())
}

/// A click on the first row's checkbox selecting the row, and one deselecting it.
async fn click(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    let first = page.element(checkbox(0)).await?;
    page.measure("select", first.click()).await?;
    first.wait_for_prop("checked", "true").await?;
    assert_that!(selected(page).await?).is_equal_to(1.0);
    page.measure("deselect", first.click()).await?;
    first.wait_for_prop("checked", "false").await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Clicks on a checkbox of 10.
#[browser_test]
pub async fn click_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    click(page).await
}

/// Clicks on a checkbox of 1,000.
#[browser_test]
pub async fn click_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    click(page).await
}

/// The selection changing elsewhere: every checkbox checked (select all), then none.
async fn selection_change(page: &Page<'_>, last: usize) -> Result<(), Report> {
    let last = page.element(checkbox(last)).await?;
    page.measure("select all", page.control("select-all"))
        .await?;
    last.wait_for_prop("checked", "true").await?;
    page.measure("clear", page.control("clear-selection"))
        .await?;
    last.wait_for_prop("checked", "false").await?;
    Ok(())
}

/// The selection of 10 rows changing elsewhere.
#[browser_test]
pub async fn selection_change_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    selection_change(page, 9).await
}

/// The selection of 1,000 rows changing elsewhere.
#[browser_test]
pub async fn selection_change_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    selection_change(page, 999).await
}

/// Disabling the first row (its checkbox is disabled), a click it ignores, and enabling it.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let first = page.element(checkbox(0)).await?;
    page.measure("disable", page.control("disable-first"))
        .await?;
    first.wait_for_prop("disabled", "true").await?;
    page.measure("ignored click", first.click()).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable-all")).await?;
    first.wait_for_prop("disabled", "false").await?;
    Ok(())
}
