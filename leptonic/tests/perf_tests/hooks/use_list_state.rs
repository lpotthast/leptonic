// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_list_state` (`/perf/hooks/use_list_state`, a state shown in a
//! plain list): what each thing it offers costs, measured (`Page::measure`) and checked against
//! its budget (`tests/budgets/hooks/use_list_state.json`): creating the state (mounting) and
//! dropping it, its selection methods (toggle, replace, extend, select all, toggle all, clear), a
//! focus change, and the focused item leaving the collection (the focus moves on), with 10 items
//! and with 1,000.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_list_state";

/// Registers the performance tests of `use_list_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(Toggle))
        .with(fixture_test(ReplaceAndExtend))
        .with(fixture_test(ReplaceAndExtendLarge))
        .with(fixture_test(SelectAll))
        .with(fixture_test(SelectAllLarge))
        .with(fixture_test(Focus))
        .with(fixture_test(FocusedItemRemoved))
        .with(fixture_test(FocusedItemRemovedLarge))
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

/// Creating the state of `count` items (mounting) and dropping it.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ITEMS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LIST, 0).await?;
    Ok(())
}

/// Creating and dropping the state of 10 items.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_and_unmount_in(page, 10).await
}

/// Creating and dropping the state of 1,000 items.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, 1_000).await
}

/// Toggling the first item's selection on and off.
#[browser_test]
pub async fn toggle(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let first = page.element(item(0)).await?;
    page.measure("select", page.control("toggle-first")).await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("deselect", page.control("toggle-first"))
        .await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// Replacing the selection with the last item (of `last + 1`), and extending it from the first
/// (the anchor) to the last.
async fn replace_and_extend_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    let first = page.element(item(0)).await?;
    let last_item = page.element(item(last)).await?;
    page.measure("replace", page.control("replace-with-last"))
        .await?;
    last_item
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    page.control("toggle-first").await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("extend", page.control("extend-to-last"))
        .await?;
    page.element(item(last / 2))
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    Ok(())
}

/// Replacing and extending the selection of 10 items.
#[browser_test]
pub async fn replace_and_extend(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    replace_and_extend_in(page, 9).await
}

/// Replacing and extending the selection of 1,000 items.
#[browser_test]
pub async fn replace_and_extend_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    replace_and_extend_in(page, 999).await
}

/// Selecting all `last + 1` items, toggling all off and on again, and clearing the selection.
async fn select_all_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    let last = page.element(item(last)).await?;
    page.measure("select all", page.control("select-all"))
        .await?;
    last.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("toggle all off", page.control("toggle-select-all"))
        .await?;
    last.wait_for_attr("aria-selected", Some("false")).await?;
    page.measure("toggle all on", page.control("toggle-select-all"))
        .await?;
    last.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("clear", page.control("clear-selection"))
        .await?;
    last.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// Select all, toggle all and clear with 10 items.
#[browser_test]
pub async fn select_all(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select_all_in(page, 9).await
}

/// Select all, toggle all and clear with 1,000 items.
#[browser_test]
pub async fn select_all_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    select_all_in(page, 999).await
}

/// Focusing the first item.
#[browser_test]
pub async fn focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("focus", page.control("focus-first")).await?;
    page.element(item(0))
        .await?
        .wait_for_attr("data-focused", Some("true"))
        .await?;
    Ok(())
}

/// The focused first item removed from the collection: the focus moves to the next item.
async fn focused_item_removed_in(page: &Page<'_>) -> Result<(), Report> {
    page.control("focus-first").await?;
    page.element(item(0))
        .await?
        .wait_for_attr("data-focused", Some("true"))
        .await?;
    page.measure("remove", page.control("remove-first")).await?;
    page.element(item(1))
        .await?
        .wait_for_attr("data-focused", Some("true"))
        .await?;
    Ok(())
}

/// The focused one of 10 items removed.
#[browser_test]
pub async fn focused_item_removed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focused_item_removed_in(page).await
}

/// The focused one of 1,000 items removed.
#[browser_test]
pub async fn focused_item_removed_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    focused_item_removed_in(page).await
}
