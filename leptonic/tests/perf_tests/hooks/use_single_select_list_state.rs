// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_single_select_list_state`
//! (`/perf/hooks/use_single_select_list_state`, a state shown in a plain list): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_single_select_list_state.json`): creating the state (mounting) and
//! dropping it, selecting an item (the first, then the last), selecting the selected item again
//! (reported, the selection stays) and clearing the selection, with 10 items and with 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_single_select_list_state";

/// Registers the performance tests of `use_single_select_list_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(Select))
        .with(fixture_test(SelectLarge))
        .with(fixture_test(Reselect))
}
const LIST: &str = "#perf-list";
const ITEMS: &str = "#perf-list > li";
const SELECTED: &str = "#perf-selected";

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

/// Selecting the first item, then the last of `last + 1` (deselecting the first), then none.
async fn select_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    let selected = page.element(SELECTED).await?;
    page.measure("select first", page.control("select-first"))
        .await?;
    selected.wait_for_inner_text("Item 0").await?;
    page.measure("select last", page.control("select-last"))
        .await?;
    selected
        .wait_for_inner_text(&format!("Item {last}"))
        .await?;
    page.element(item(0))
        .await?
        .wait_for_attr("aria-selected", Some("false"))
        .await?;
    page.measure("clear", page.control("clear")).await?;
    selected.wait_for_inner_text("").await?;
    Ok(())
}

/// Selecting among 10 items.
#[browser_test]
pub async fn select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select_in(page, 9).await
}

/// Selecting among 1,000 items.
#[browser_test]
pub async fn select_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    select_in(page, 999).await
}

/// Selecting the selected item again: reported as a selection, it stays selected.
#[browser_test]
pub async fn reselect(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("reselect-first").await?;
    let first = page.element(item(0)).await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    assert_that!(page.control("changes").await?).is_equal_to(1.0);
    page.measure("reselect", page.control("reselect-first"))
        .await?;
    assert_that!(page.control("changes").await?).is_equal_to(2.0);
    assert_that!(first)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    Ok(())
}
