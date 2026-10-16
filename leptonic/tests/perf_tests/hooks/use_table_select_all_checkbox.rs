// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_table_select_all_checkbox`
//! (`/perf/hooks/use_table_select_all_checkbox`, the select all checkbox of a hook table): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_table_select_all_checkbox.json`): mounting and unmounting the table
//! with it, clicks selecting every row and none, its indeterminate state (some rows selected),
//! the selection mode changing (single: disabled) and the table emptied (disabled), in tables of
//! 10 rows and of 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_table_select_all_checkbox";

/// Registers the performance tests of `use_table_select_all_checkbox`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(ClickSmall))
        .with(fixture_test(ClickLarge))
        .with(fixture_test(IndeterminateSmall))
        .with(fixture_test(IndeterminateLarge))
        .with(fixture_test(SelectionMode))
        .with(fixture_test(EmptyAndFilled))
}

const TABLE: &str = "[role=grid]";
const ROWS: &str = "[role=grid] > tbody > tr";
const CHECKBOX: &str = "#perf-select-all input";

/// Opens the fixture with its 10 rows.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ROWS, 10).await?;
    Ok(())
}

/// Opens the fixture and mounts the table with 1,000 rows (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(TABLE, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(ROWS, 1_000).await?;
    Ok(())
}

/// Mounting the table with its checkbox and `rows` rows, and unmounting it.
async fn mount_and_unmount_table(page: &Page<'_>, rows: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(TABLE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ROWS, rows).await?;
    page.wait_for_count(CHECKBOX, 1).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TABLE, 0).await?;
    Ok(())
}

/// Mounting a table of 10 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_table(page, 10).await
}

/// Mounting a table of 1,000 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_table(page, 1_000).await
}

/// A click selecting every row, and one deselecting them.
async fn click(page: &Page<'_>) -> Result<(), Report> {
    let checkbox = page.element(CHECKBOX).await?;
    page.measure("select all", checkbox.click()).await?;
    checkbox.wait_for_prop("checked", "true").await?;
    assert_that!(page.control("selected").await?).is_equal_to(-1.0);
    page.measure("deselect all", checkbox.click()).await?;
    checkbox.wait_for_prop("checked", "false").await?;
    assert_that!(page.control("selected").await?).is_equal_to(0.0);
    Ok(())
}

/// Select all on 10 rows.
#[browser_test]
pub async fn click_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    click(page).await
}

/// Select all on 1,000 rows.
#[browser_test]
pub async fn click_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    click(page).await
}

/// One row selected (the checkbox is indeterminate), then none.
async fn indeterminate(page: &Page<'_>) -> Result<(), Report> {
    let checkbox = page.element(CHECKBOX).await?;
    page.measure("select one", page.control("select-first"))
        .await?;
    checkbox.wait_for_prop("indeterminate", "true").await?;
    page.measure("clear", page.control("clear-selection"))
        .await?;
    checkbox.wait_for_prop("indeterminate", "false").await?;
    Ok(())
}

/// The indeterminate state with 10 rows.
#[browser_test]
pub async fn indeterminate_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    indeterminate(page).await
}

/// The indeterminate state with 1,000 rows.
#[browser_test]
pub async fn indeterminate_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    indeterminate(page).await
}

/// The selection mode changing to single (the checkbox is disabled, labelled "Select") and back
/// to multiple.
#[browser_test]
pub async fn selection_mode(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let checkbox = page.element(CHECKBOX).await?;
    page.measure("single", page.control("single")).await?;
    checkbox.wait_for_prop("disabled", "true").await?;
    checkbox.wait_for_attr("aria-label", Some("Select")).await?;
    page.measure("multiple", page.control("multiple")).await?;
    checkbox.wait_for_prop("disabled", "false").await?;
    checkbox
        .wait_for_attr("aria-label", Some("Select All"))
        .await?;
    Ok(())
}

/// The table losing all its rows (the checkbox is disabled) and getting them back.
#[browser_test]
pub async fn empty_and_filled(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let checkbox = page.element(CHECKBOX).await?;
    page.measure("empty", page.control("empty")).await?;
    checkbox.wait_for_prop("disabled", "true").await?;
    page.measure("fill", page.control("small")).await?;
    checkbox.wait_for_prop("disabled", "false").await?;
    page.wait_for_count(ROWS, 10).await?;
    Ok(())
}
