// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_collection` (`/perf/hooks/use_collection`, a collection shown
//! in a plain list): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_collection.json`): building the collection
//! (mounting) and dropping it, and rebuilding it when its data changes (an item appended and
//! removed, renamed, the order reversed, and a change the collection doesn't depend on, which
//! notifies nobody), with 10 items and with 1,000.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_collection";

/// Registers the performance tests of `use_collection`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(AppendAndRemove))
        .with(fixture_test(AppendAndRemoveLarge))
        .with(fixture_test(Rename))
        .with(fixture_test(RenameLarge))
        .with(fixture_test(Reverse))
        .with(fixture_test(ReverseLarge))
        .with(fixture_test(UnchangedRebuild))
        .with(fixture_test(UnchangedRebuildLarge))
}
const LIST: &str = "#perf-list";
const ITEMS: &str = "#perf-list > li";

/// The list entry of item `i`.
fn item(i: u32) -> String {
    format!("[data-perf-item='{i}']")
}

/// Switches to 1,000 items (unmeasured).
async fn large(page: &Page<'_>) -> Result<(), Report> {
    page.control("large").await?;
    page.wait_for_count(ITEMS, 1_000).await?;
    Ok(())
}

/// Building the collection of `count` items (mounting) and dropping it.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ITEMS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LIST, 0).await?;
    Ok(())
}

/// An item appended to `count` and removed again.
async fn append_and_remove_in(page: &Page<'_>, count: u32) -> Result<(), Report> {
    page.measure("append", page.control("append")).await?;
    page.element(item(count)).await?;
    page.measure("remove", page.control("remove-last")).await?;
    page.wait_for_count(item(count), 0).await?;
    Ok(())
}

/// The first item renamed and back.
async fn rename_in(page: &Page<'_>) -> Result<(), Report> {
    page.measure("rename", page.control("rename-first")).await?;
    page.element(item(0))
        .await?
        .wait_for_inner_text("Renamed 0")
        .await?;
    page.measure("restore", page.control("restore-first"))
        .await?;
    page.element(item(0))
        .await?
        .wait_for_inner_text("Item 0")
        .await?;
    Ok(())
}

/// The order of `count` items reversed and back.
async fn reverse_in(page: &Page<'_>, count: u32) -> Result<(), Report> {
    page.measure("reverse", page.control("reverse")).await?;
    page.element(format!("{ITEMS}:first-child{}", item(count - 1)))
        .await?;
    page.measure("restore", page.control("reverse")).await?;
    page.element(format!("{ITEMS}:first-child{}", item(0)))
        .await?;
    Ok(())
}

/// A change of data the build reads but the collection doesn't depend on: the build runs, the
/// equal collection notifies nobody (the list keeps its `count` entries).
async fn unchanged_rebuild_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.measure("touch", page.control("touch")).await?;
    page.count_stays(ITEMS, count, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Building and dropping a collection of 10 items.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_and_unmount_in(page, 10).await
}

/// Building and dropping a collection of 1,000 items.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, 1_000).await
}

/// An item appended to 10 and removed.
#[browser_test]
pub async fn append_and_remove(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    append_and_remove_in(page, 10).await
}

/// An item appended to 1,000 and removed.
#[browser_test]
pub async fn append_and_remove_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    append_and_remove_in(page, 1_000).await
}

/// One of 10 items renamed and back.
#[browser_test]
pub async fn rename(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    rename_in(page).await
}

/// One of 1,000 items renamed and back.
#[browser_test]
pub async fn rename_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    rename_in(page).await
}

/// The order of 10 items reversed and back.
#[browser_test]
pub async fn reverse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    reverse_in(page, 10).await
}

/// The order of 1,000 items reversed and back.
#[browser_test]
pub async fn reverse_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    reverse_in(page, 1_000).await
}

/// A rebuild of 10 items without a change.
#[browser_test]
pub async fn unchanged_rebuild(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    unchanged_rebuild_in(page, 10).await
}

/// A rebuild of 1,000 items without a change.
#[browser_test]
pub async fn unchanged_rebuild_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    unchanged_rebuild_in(page, 1_000).await
}
