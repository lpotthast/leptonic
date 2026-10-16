// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `GridListHeader` (`/perf/atoms/grid_list_header`, the headers of
//! the sections of a list): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/grid_list_header.json`): mounting and
//! unmounting a list of sections with headers, and a heading (from the collection) changing and
//! back, with 2 sections (10 items) and with 200 (1,000 items).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/grid_list_header";

/// Registers the performance tests of `GridListHeader`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(RenameSmall))
        .with(fixture_test(RenameLarge))
}

const LIST: &str = "[role=grid]";
const HEADERS: &str = "[role=grid] > [role=rowgroup] [role=rowheader]";
const FIRST_HEADER: &str = "[role=grid] > [role=rowgroup]:nth-child(1) [role=rowheader]";

/// Opens the fixture with its 2 sections.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(HEADERS, 2).await?;
    Ok(())
}

/// Opens the fixture and mounts the list with 200 sections (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(HEADERS, 200).await?;
    Ok(())
}

/// Mounting the list with its `sections` headers and unmounting it.
async fn mount_and_unmount_headers(page: &Page<'_>, sections: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(HEADERS, sections).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LIST, 0).await?;
    Ok(())
}

/// Mounting 2 sections with headers and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_headers(page, 2).await
}

/// Mounting 200 sections with headers and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_headers(page, 200).await
}

/// The first section's heading changing in the collection, and back.
async fn rename(page: &Page<'_>) -> Result<(), Report> {
    let first = page.element(FIRST_HEADER).await?;
    page.measure("rename", page.control("rename")).await?;
    first.wait_for_inner_text("Renamed 0").await?;
    page.measure("restore", page.control("restore-name"))
        .await?;
    first.wait_for_inner_text("Section 0").await?;
    Ok(())
}

/// A heading of 2 changing.
#[browser_test]
pub async fn rename_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    rename(page).await
}

/// A heading of 200 changing.
#[browser_test]
pub async fn rename_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    rename(page).await
}
