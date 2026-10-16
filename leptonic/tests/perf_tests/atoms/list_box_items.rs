// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ListBoxItems` (`/perf/atoms/list_box_items`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/list_box_items.json`): mounting and unmounting the options of 10 and of
//! 1,000 collection items, and following the collection's changes (an item appended and
//! removed, an item's text changing, the order reversing) at both sizes; a click on an option it
//! rendered.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/list_box_items";

/// Registers the performance tests of `ListBoxItems`.
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
        .with(fixture_test(SelectWithTheMouse))
}
const OPTIONS: &str = "[role=option]";
/// The fixture's items after the control `large`.
const LARGE: usize = 1_000;

/// The content of the option of item `i`.
fn text(i: usize) -> String {
    format!("[data-perf-text='{i}']")
}

/// The option of item `i`.
fn option(i: usize) -> String {
    format!("[role=option]:has(> {})", text(i))
}

/// Remounts the options of [`LARGE`] items (unmeasured).
async fn mount_large(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(OPTIONS, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(OPTIONS, LARGE).await?;
    Ok(())
}

/// Mounting the options of `count` items and unmounting them.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(OPTIONS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(OPTIONS, 0).await?;
    Ok(())
}

/// Mounting the options of 10 items and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(OPTIONS, 0).await?;
    mount_and_unmount_in(page, 10).await
}

/// Mounting the options of 1,000 items and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(OPTIONS, 0).await?;
    page.control("large").await?;
    mount_and_unmount_in(page, LARGE).await
}

/// An item appended to `count` and removed again.
async fn append_and_remove_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.measure("append", page.control("append")).await?;
    page.element(option(count)).await?;
    page.measure("remove", page.control("remove-last")).await?;
    page.wait_for_count(text(count), 0).await?;
    Ok(())
}

/// An item appended to 10 and removed again.
#[browser_test]
pub async fn append_and_remove(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    append_and_remove_in(page, 10).await
}

/// An item appended to 1,000 and removed again.
#[browser_test]
pub async fn append_and_remove_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    append_and_remove_in(page, LARGE).await
}

/// The first item's text changing and back: its option renders the new text.
async fn rename_in(page: &Page<'_>) -> Result<(), Report> {
    page.measure("rename", page.control("rename-first")).await?;
    page.element(text(0))
        .await?
        .wait_for_inner_text("Renamed 0")
        .await?;
    page.measure("restore", page.control("restore-names"))
        .await?;
    page.element(text(0))
        .await?
        .wait_for_inner_text("Option 0")
        .await?;
    Ok(())
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
    mount_large(page).await?;
    rename_in(page).await
}

/// The order of `count` items reversed and back: the options move.
async fn reverse_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    let first = format!("{OPTIONS}:first-child > {}", text(count - 1));
    page.measure("reverse", page.control("reverse")).await?;
    page.element(first).await?;
    page.measure("restore", page.control("reverse")).await?;
    page.element(format!("{OPTIONS}:first-child > {}", text(0)))
        .await?;
    Ok(())
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
    mount_large(page).await?;
    reverse_in(page, LARGE).await
}

/// A click selecting an option it rendered.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let third = page.element(option(3)).await?;
    page.measure("select", third.click()).await?;
    third.wait_for_attr("aria-selected", Some("true")).await?;
    assert_that!(page.count("[aria-selected=true]").await?).is_equal_to(1);
    Ok(())
}
