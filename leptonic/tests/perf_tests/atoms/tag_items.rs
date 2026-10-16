// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `TagItems` (`/perf/atoms/tag_items`, the fixture of `TagGroup`'s
//! performance test, rendering its tags with `TagItems`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget (`tests/budgets/atoms/tag_items.json`):
//! mounting and unmounting the tags of 10 and of 1,000 collection items, and following the
//! collection's changes (a tag appended and removed, a tag's text changing) at both sizes.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use super::tag_group::{TAGS, goto, large, tag};
use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/tag_items";

/// Registers the performance tests of `TagItems`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(AppendAndRemove))
        .with(fixture_test(AppendAndRemoveLarge))
        .with(fixture_test(Rename))
        .with(fixture_test(RenameLarge))
}

/// Mounting the tags of `count` items and unmounting them.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(TAGS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(TAGS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TAGS, 0).await?;
    Ok(())
}

/// Mounting the tags of 10 items and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    mount_and_unmount_in(page, 10).await
}

/// Mounting the tags of 1,000 items and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, 1_000).await
}

/// A tag appended to `count` and removed again.
async fn append_and_remove_in(page: &Page<'_>, count: u32) -> Result<(), Report> {
    page.measure("append", page.control("append")).await?;
    page.element(tag(count)).await?;
    page.measure("remove", page.control("remove-last")).await?;
    page.wait_for_count(tag(count), 0).await?;
    Ok(())
}

/// A tag appended to 10 and removed again.
#[browser_test]
pub async fn append_and_remove(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    append_and_remove_in(page, 10).await
}

/// A tag appended to 1,000 and removed again.
#[browser_test]
pub async fn append_and_remove_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    append_and_remove_in(page, 1_000).await
}

/// The first tag's text changing and back: its tag renders the new text.
async fn rename_in(page: &Page<'_>) -> Result<(), Report> {
    page.measure("rename", page.control("rename-first")).await?;
    page.element("[data-perf-tag='0']")
        .await?
        .wait_for_inner_text("Renamed 0")
        .await?;
    page.measure("restore", page.control("restore-names"))
        .await?;
    page.element("[data-perf-tag='0']")
        .await?
        .wait_for_inner_text("Tag 0")
        .await?;
    Ok(())
}

/// One of 10 tags renamed and back.
#[browser_test]
pub async fn rename(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    rename_in(page).await
}

/// One of 1,000 tags renamed and back.
#[browser_test]
pub async fn rename_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    rename_in(page).await
}
