// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `GridListItemDescription`
//! (`/perf/atoms/grid_list_item_description`): what it costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/grid_list_item_description.json`): mounting
//! and unmounting a list whose items have descriptions (part of their item's name), with 10 items
//! and with 1,000 (the description renders its slot's element and nothing reactive).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/grid_list_item_description";

/// Registers the performance tests of `GridListItemDescription`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
}

const LIST: &str = "[role=grid]";
const DESCRIPTIONS: &str = "[role=grid] .leptonic-GridListItemDescription";

/// Mounting the list with `items` described items and unmounting it; the first item's
/// description is part of its accessible name (as react-aria labels a row by its text and
/// description).
async fn mount_and_unmount_descriptions(page: &Page<'_>, items: usize) -> Result<(), Report> {
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(DESCRIPTIONS, items).await?;
    let first = page
        .element("[role=grid] > [role=row]:nth-child(1)")
        .await?;
    assert_that!(first.accessible_name().await?.contains("Description 0")).is_true();
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LIST, 0).await?;
    Ok(())
}

/// Mounting 10 described items and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(DESCRIPTIONS, 10).await?;
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    mount_and_unmount_descriptions(page, 10).await
}

/// Mounting 1,000 described items and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(DESCRIPTIONS, 10).await?;
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.control("large").await?;
    mount_and_unmount_descriptions(page, 1_000).await
}
