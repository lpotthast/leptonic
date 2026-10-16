// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `GridRowGroup` (`/perf/atoms/grid_row_group`): what it costs,
//! measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/grid_row_group.json`): mounting and unmounting a grid whose rows are in
//! a row group, with 10 rows and with 1,000 (the group renders an element with its role and
//! nothing reactive).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/grid_row_group";

/// Registers the performance tests of `GridRowGroup`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
}

const GRID: &str = "[role=grid]";
const ROWS: &str = "[role=grid] > [role=rowgroup] > [role=row]";

/// Mounting the grid with its row group of `rows` rows and unmounting it.
async fn mount_and_unmount_rows(page: &Page<'_>, rows: usize) -> Result<(), Report> {
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ROWS, rows).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GRID, 0).await?;
    Ok(())
}

/// Mounting a row group of 10 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ROWS, 10).await?;
    page.control("unmount").await?;
    page.wait_for_count(GRID, 0).await?;
    mount_and_unmount_rows(page, 10).await
}

/// Mounting a row group of 1,000 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ROWS, 10).await?;
    page.control("unmount").await?;
    page.wait_for_count(GRID, 0).await?;
    page.control("large").await?;
    mount_and_unmount_rows(page, 1_000).await
}
