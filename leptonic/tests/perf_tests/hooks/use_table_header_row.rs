// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_table_header_row` (`/perf/hooks/use_table_header_row`, the
//! header row of a hook table): what it costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_table_header_row.json`): mounting and unmounting the header
//! with its row, in tables of 10 rows and of 1,000 (the hook returns the row's role, nothing
//! reactive).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_table_header_row";

/// Registers the performance tests of `use_table_header_row`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
}

const TABLE: &str = "[role=grid]";
const HEADER_ROWS: &str = "[role=grid] > thead > tr[role=row]";
const ROWS: &str = "[role=grid] > tbody > tr";

/// Mounting the header and its row, and unmounting them.
async fn mount_and_unmount_header(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount-header").await?;
    page.wait_for_count(HEADER_ROWS, 0).await?;
    page.measure("mount", page.control("mount-header")).await?;
    page.wait_for_count(HEADER_ROWS, 1).await?;
    page.measure("unmount", page.control("unmount-header"))
        .await?;
    page.wait_for_count(HEADER_ROWS, 0).await?;
    Ok(())
}

/// The header row of a table of 10 rows mounting and unmounting.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ROWS, 10).await?;
    mount_and_unmount_header(page).await
}

/// The header row of a table of 1,000 rows mounting and unmounting.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ROWS, 10).await?;
    page.control("unmount").await?;
    page.wait_for_count(TABLE, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(ROWS, 1_000).await?;
    mount_and_unmount_header(page).await
}
