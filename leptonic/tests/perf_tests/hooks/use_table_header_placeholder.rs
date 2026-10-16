// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_table_header_placeholder`
//! (`/perf/hooks/use_table_header_placeholder`, the placeholders in the column group row of a
//! hook table): what each thing it offers costs, measured (`Page::measure`) and checked against
//! its budget (`tests/budgets/hooks/use_table_header_placeholder.json`): mounting and unmounting
//! the header with its placeholders, and rows added and removed (the placeholders follow the
//! table's columns), in tables of 10 rows and of 1,000.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_table_header_placeholder";

/// Registers the performance tests of `use_table_header_placeholder`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(RowsChangeSmall))
        .with(fixture_test(RowsChangeLarge))
}

const TABLE: &str = "[role=grid]";
const PLACEHOLDERS: &str = "[role=grid] > thead th[role=gridcell]";
const ROWS: &str = "[role=grid] > tbody > tr";

/// Opens the fixture with its 10 rows.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ROWS, 10).await?;
    page.wait_for_count(PLACEHOLDERS, 2).await?;
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

/// Mounting the header with its two placeholders and unmounting it.
async fn mount_and_unmount_header(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount-header").await?;
    page.wait_for_count(PLACEHOLDERS, 0).await?;
    page.measure("mount", page.control("mount-header")).await?;
    page.wait_for_count(PLACEHOLDERS, 2).await?;
    page.measure("unmount", page.control("unmount-header"))
        .await?;
    page.wait_for_count(PLACEHOLDERS, 0).await?;
    Ok(())
}

/// The header of a table of 10 rows mounting and unmounting.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_header(page).await
}

/// The header of a table of 1,000 rows mounting and unmounting.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_header(page).await
}

/// A row added and removed: the table (and with it each placeholder's position) is rebuilt.
async fn rows_change(page: &Page<'_>, rows: usize) -> Result<(), Report> {
    page.measure("add", page.control("add-row")).await?;
    page.wait_for_count(ROWS, rows + 1).await?;
    page.measure("remove", page.control("remove-row")).await?;
    page.wait_for_count(ROWS, rows).await?;
    page.wait_for_count(PLACEHOLDERS, 2).await?;
    Ok(())
}

/// A row added to the 10 and removed.
#[browser_test]
pub async fn rows_change_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    rows_change(page, 10).await
}

/// A row added to the 1,000 and removed.
#[browser_test]
pub async fn rows_change_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    rows_change(page, 1_000).await
}
