// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `TableBody` (`/perf/atoms/table_body`, the body of a table): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/table_body.json`): mounting and unmounting the body with its rows, and
//! the table emptied (`data-empty`) and filled again, with 10 rows and with 1,000.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/table_body";

/// Registers the performance tests of `TableBody`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(EmptyAndFilledSmall))
        .with(fixture_test(EmptyAndFilledLarge))
}

const TABLE: &str = "[role=grid]";
const BODY: &str = "[role=grid] > tbody";
const ROWS: &str = "[role=grid] > tbody > tr";

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

/// Mounting the body with its `rows` rows and unmounting it.
async fn mount_and_unmount_body(page: &Page<'_>, rows: usize) -> Result<(), Report> {
    page.control("unmount-body").await?;
    page.wait_for_count(BODY, 0).await?;
    page.measure("mount", page.control("mount-body")).await?;
    page.wait_for_count(ROWS, rows).await?;
    page.measure("unmount", page.control("unmount-body"))
        .await?;
    page.wait_for_count(BODY, 0).await?;
    Ok(())
}

/// Mounting a body of 10 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_body(page, 10).await
}

/// Mounting a body of 1,000 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_body(page, 1_000).await
}

/// The table losing all its rows (the body gets `data-empty`) and getting `rows` back.
async fn empty_and_filled(page: &Page<'_>, size: &str, rows: usize) -> Result<(), Report> {
    let body = page.element(BODY).await?;
    page.measure("empty", page.control("empty")).await?;
    body.wait_for_attr("data-empty", Some("true")).await?;
    page.measure("fill", page.control(size)).await?;
    page.wait_for_count(ROWS, rows).await?;
    body.wait_for_attr("data-empty", None).await?;
    Ok(())
}

/// The 10 rows removed and added back.
#[browser_test]
pub async fn empty_and_filled_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    empty_and_filled(page, "small", 10).await
}

/// The 1,000 rows removed and added back.
#[browser_test]
pub async fn empty_and_filled_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    empty_and_filled(page, "large", 1_000).await
}
