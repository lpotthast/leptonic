// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_table_state` (`/perf/hooks/use_table_state`, the state
//! alone): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_table_state.json`): creating and disposing it, rows added and
//! removed, the selection checkboxes shown and hidden (a selection column added to the table),
//! the selection mode changing, its sort, and (in a tree table) every row expanded and
//! collapsed, with 10 rows and with 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_table_state";

/// Registers the performance tests of `use_table_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(RowsChangeSmall))
        .with(fixture_test(RowsChangeLarge))
        .with(fixture_test(SelectionCheckboxesSmall))
        .with(fixture_test(SelectionCheckboxesLarge))
        .with(fixture_test(SelectionModeSmall))
        .with(fixture_test(SelectionModeLarge))
        .with(fixture_test(Sort))
        .with(fixture_test(TreeExpansionSmall))
        .with(fixture_test(TreeExpansionLarge))
}

const ROWS: &str = "#perf-rows";
const COLUMNS: &str = "#perf-columns";

/// Opens the fixture with its 10 rows.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(ROWS).await?.wait_for_inner_text("10").await?;
    Ok(())
}

/// Opens the fixture and creates the state of `mount` (`mount` or `mount-tree`) with 1,000 rows
/// (unmeasured).
async fn open_large(page: &Page<'_>, mount: &str) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(ROWS, 0).await?;
    page.control("large").await?;
    page.control(mount).await?;
    page.wait_for_count(ROWS, 1).await?;
    Ok(())
}

/// How many rows are visible.
async fn rows(page: &Page<'_>) -> Result<f64, Report> {
    page.control("rows").await
}

/// Creating the state and disposing it.
async fn mount_and_unmount_state(page: &Page<'_>, rows: f64) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(ROWS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ROWS, 1).await?;
    assert_that!(self::rows(page).await?).is_equal_to(rows);
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(ROWS, 0).await?;
    Ok(())
}

/// The state of 10 rows created and disposed.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_state(page, 10.0).await
}

/// The state of 1,000 rows created and disposed.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page, "mount").await?;
    mount_and_unmount_state(page, 1_000.0).await
}

/// A row added and removed.
async fn rows_change(page: &Page<'_>, count: f64) -> Result<(), Report> {
    page.measure("add", page.control("add-row")).await?;
    assert_that!(rows(page).await?).is_equal_to(count + 1.0);
    page.measure("remove", page.control("remove-row")).await?;
    assert_that!(rows(page).await?).is_equal_to(count);
    Ok(())
}

/// A row added to the 10 and removed.
#[browser_test]
pub async fn rows_change_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    rows_change(page, 10.0).await
}

/// A row added to the 1,000 and removed.
#[browser_test]
pub async fn rows_change_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page, "mount").await?;
    rows_change(page, 1_000.0).await
}

/// The selection checkboxes shown (the table gets a selection column: 4 columns) and hidden.
async fn selection_checkboxes(page: &Page<'_>) -> Result<(), Report> {
    let columns = page.element(COLUMNS).await?;
    page.measure("show", page.control("show-checkboxes"))
        .await?;
    columns.wait_for_inner_text("4").await?;
    page.measure("hide", page.control("hide-checkboxes"))
        .await?;
    columns.wait_for_inner_text("3").await?;
    Ok(())
}

/// The selection column of 10 rows.
#[browser_test]
pub async fn selection_checkboxes_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    selection_checkboxes(page).await
}

/// The selection column of 1,000 rows.
#[browser_test]
pub async fn selection_checkboxes_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page, "mount").await?;
    selection_checkboxes(page).await
}

/// With the selection checkboxes shown, the selection mode changing to none (the selection
/// column goes) and back to multiple.
async fn selection_mode(page: &Page<'_>) -> Result<(), Report> {
    let columns = page.element(COLUMNS).await?;
    page.control("show-checkboxes").await?;
    columns.wait_for_inner_text("4").await?;
    page.measure("none", page.control("selection-none")).await?;
    columns.wait_for_inner_text("3").await?;
    page.measure("multiple", page.control("selection-multiple"))
        .await?;
    columns.wait_for_inner_text("4").await?;
    Ok(())
}

/// The selection mode of 10 rows changing.
#[browser_test]
pub async fn selection_mode_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    selection_mode(page).await
}

/// The selection mode of 1,000 rows changing.
#[browser_test]
pub async fn selection_mode_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page, "mount").await?;
    selection_mode(page).await
}

/// Sorting by Value (ascending), and again (descending).
#[browser_test]
pub async fn sort(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let sort = page.element("#perf-sort").await?;
    page.measure("ascending", page.control("sort")).await?;
    sort.wait_for_inner_text("ascending").await?;
    page.measure("descending", page.control("sort")).await?;
    sort.wait_for_inner_text("descending").await?;
    Ok(())
}

/// In a tree table, every parent row expanded (its child rows become visible) and collapsed.
async fn tree_expansion(page: &Page<'_>, parents: f64) -> Result<(), Report> {
    assert_that!(rows(page).await?).is_equal_to(parents);
    page.measure("expand all", page.control("expand-all"))
        .await?;
    assert_that!(rows(page).await?).is_equal_to(2.0 * parents);
    page.measure("collapse all", page.control("collapse-all"))
        .await?;
    assert_that!(rows(page).await?).is_equal_to(parents);
    Ok(())
}

/// The rows of a tree table of 10 rows expanded and collapsed.
#[browser_test]
pub async fn tree_expansion_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(ROWS, 0).await?;
    page.control("mount-tree").await?;
    page.wait_for_count(ROWS, 1).await?;
    tree_expansion(page, 5.0).await
}

/// The rows of a tree table of 1,000 rows expanded and collapsed.
#[browser_test]
pub async fn tree_expansion_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page, "mount-tree").await?;
    tree_expansion(page, 500.0).await
}
