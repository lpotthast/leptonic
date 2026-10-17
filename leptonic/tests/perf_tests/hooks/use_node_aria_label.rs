// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_node_aria_label` (`/perf/hooks/use_node_aria_label`, read by each item of a keyed
//! list): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_node_aria_label.json`): mounting and unmounting it per item, an item
//! renamed (label changes), an item added in front (the other items move, their nodes don't
//! change) and the first item removed, with 10 items and with 1,000.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_node_aria_label";

/// Registers the performance tests of `use_node_aria_label`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(RenameFirst))
        .with(fixture_test(Prepend))
        .with(fixture_test(PrependLarge))
        .with(fixture_test(RemoveFirst))
}

const ITEMS: &str = "#perf-list > li";
const FIRST: &str = "#perf-list > li:first-child";

/// Opens the fixture with its 10 items.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ITEMS, 10).await?;
    Ok(())
}

/// Opens the fixture and renders 1,000 items (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(ITEMS, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(ITEMS, 1_000).await?;
    Ok(())
}

/// Mounting the list and unmounting it.
async fn mount_and_unmount_list(page: &Page<'_>, items: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(ITEMS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ITEMS, items).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(ITEMS, 0).await?;
    Ok(())
}

/// 10 items mounted and unmounted.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_list(page, 10).await
}

/// 1,000 items mounted and unmounted.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_list(page, 1_000).await
}

/// The first of 10 items renamed: its label follows.
#[browser_test]
pub async fn rename_first(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let first = page.element(FIRST).await?;
    page.measure("rename", page.control("rename-first")).await?;
    first
        .wait_for_attr("aria-label", Some("Label Item 0 renamed"))
        .await?;
    Ok(())
}

/// An item added in front of `items` items.
async fn prepend_item(page: &Page<'_>, items: usize) -> Result<(), Report> {
    page.measure("prepend", page.control("prepend")).await?;
    page.wait_for_count(ITEMS, items + 1).await?;
    page.element(FIRST)
        .await?
        .wait_for_attr("data-perf-item", Some(&items.to_string()))
        .await?;
    Ok(())
}

/// An item added in front of 10.
#[browser_test]
pub async fn prepend(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    prepend_item(page, 10).await
}

/// An item added in front of 1,000.
#[browser_test]
pub async fn prepend_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    prepend_item(page, 1_000).await
}

/// The first of 10 items removed.
#[browser_test]
pub async fn remove_first(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("remove", page.control("remove-first")).await?;
    page.wait_for_count(ITEMS, 9).await?;
    page.element(FIRST)
        .await?
        .wait_for_attr("data-perf-item", Some("1"))
        .await?;
    Ok(())
}
