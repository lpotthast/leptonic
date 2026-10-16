// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_list_collection` (`/perf/hooks/use_list_collection`, the
//! fixture of `use_collection`'s performance test, building the collection from a list of
//! values): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_list_collection.json`): building the collection (mounting)
//! and dropping it, and rebuilding it when its values change (a value appended and removed,
//! renamed, the order reversed, and a new list of equal values, which notifies nobody), with 10
//! values and with 1,000.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use super::use_collection::{
    append_and_remove_in, large, mount_and_unmount_in, rename_in, reverse_in, unchanged_rebuild_in,
};
use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_list_collection";

/// Registers the performance tests of `use_list_collection`.
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
