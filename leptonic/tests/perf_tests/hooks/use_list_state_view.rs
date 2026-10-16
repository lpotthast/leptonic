// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_list_state_view` (`/perf/hooks/use_list_state_view`, a view of
//! a list state shown in a plain list): what each thing it offers costs, measured (`Page::measure`)
//! and checked against its budget (`tests/budgets/hooks/use_list_state_view.json`): creating the
//! view (mounting) and dropping it, its collection changing (filtered to the even items and back),
//! the focused item leaving the view (the focus moves to a neighbor), and selecting all through the
//! view, with 10 items and with 1,000.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_list_state_view";

/// Registers the performance tests of `use_list_state_view`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(Filter))
        .with(fixture_test(FilterLarge))
        .with(fixture_test(FocusedItemLeaves))
        .with(fixture_test(FocusedItemLeavesLarge))
        .with(fixture_test(SelectAll))
        .with(fixture_test(SelectAllLarge))
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

/// Creating the view of `count` items (mounting) and dropping it.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ITEMS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LIST, 0).await?;
    Ok(())
}

/// Creating and dropping the view of 10 items.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_and_unmount_in(page, 10).await
}

/// Creating and dropping the view of 1,000 items.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, 1_000).await
}

/// The view filtered to the even ones of `count` items and back.
async fn filter_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.measure("even", page.control("even")).await?;
    page.wait_for_count(ITEMS, count / 2).await?;
    page.measure("all", page.control("all")).await?;
    page.wait_for_count(ITEMS, count).await?;
    Ok(())
}

/// Filtering the view of 10 items.
#[browser_test]
pub async fn filter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    filter_in(page, 10).await
}

/// Filtering the view of 1,000 items.
#[browser_test]
pub async fn filter_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    filter_in(page, 1_000).await
}

/// The focused second item filtered out of the view: the focus moves to the third.
async fn focused_item_leaves_in(page: &Page<'_>) -> Result<(), Report> {
    page.measure("focus", page.control("focus-second")).await?;
    page.element(item(1))
        .await?
        .wait_for_attr("data-focused", Some("true"))
        .await?;
    page.measure("filter", page.control("even")).await?;
    page.element(item(2))
        .await?
        .wait_for_attr("data-focused", Some("true"))
        .await?;
    Ok(())
}

/// The focused one of 10 items leaving the view.
#[browser_test]
pub async fn focused_item_leaves(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focused_item_leaves_in(page).await
}

/// The focused one of 1,000 items leaving the view.
#[browser_test]
pub async fn focused_item_leaves_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    focused_item_leaves_in(page).await
}

/// Selecting all `last + 1` items through the view.
async fn select_all_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    page.measure("select all", page.control("select-all"))
        .await?;
    page.element(item(last))
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    Ok(())
}

/// Selecting all of 10 items.
#[browser_test]
pub async fn select_all(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select_all_in(page, 9).await
}

/// Selecting all of 1,000 items.
#[browser_test]
pub async fn select_all_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    select_all_in(page, 999).await
}
