// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_tree_expansion` (`/perf/hooks/use_tree_expansion`, the state
//! alone, read per item): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_tree_expansion.json`): creating and disposing it,
//! an item expanded and collapsed (only its reader runs), every item expanded and collapsed, and
//! setting the expanded items unchanged (ignored), with 10 items read and with 1,000.
use std::time::Duration;

use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_tree_expansion";

/// Registers the performance tests of `use_tree_expansion`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(Toggle))
        .with(fixture_test(ToggleLarge))
        .with(fixture_test(ExpandAll))
        .with(fixture_test(ExpandAllLarge))
        .with(fixture_test(SameKeys))
}

const EXPANDED: &str = "#perf-expanded";
const FIRST: &str = "#perf-item-0";

/// Opens the fixture with its 10 items.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(EXPANDED)
        .await?
        .wait_for_inner_text("0")
        .await?;
    Ok(())
}

/// Opens the fixture and reads 1,000 items (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(EXPANDED, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count("li[id^=perf-item-]", 1_000).await?;
    Ok(())
}

/// Waits until `expected` items are expanded.
async fn wait_for_expanded(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    page.element(EXPANDED)
        .await?
        .wait_for_inner_text(expected)
        .await?;
    Ok(())
}

/// Creating the state with its readers and disposing it.
async fn mount_and_unmount_state(page: &Page<'_>, items: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(EXPANDED, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count("li[id^=perf-item-]", items).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(EXPANDED, 0).await?;
    Ok(())
}

/// The state read by 10 items created and disposed.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_state(page, 10).await
}

/// The state read by 1,000 items created and disposed.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_state(page, 1_000).await
}

/// The first item expanded and collapsed.
async fn toggle_first(page: &Page<'_>) -> Result<(), Report> {
    let first = page.element(FIRST).await?;
    page.measure("expand", page.control("toggle-first")).await?;
    first.wait_for_inner_text("expanded").await?;
    wait_for_expanded(page, "1").await?;
    page.measure("collapse", page.control("toggle-first"))
        .await?;
    first.wait_for_inner_text("collapsed").await?;
    wait_for_expanded(page, "0").await?;
    Ok(())
}

/// One of 10 items expanded and collapsed.
#[browser_test]
pub async fn toggle(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    toggle_first(page).await
}

/// One of 1,000 items expanded and collapsed.
#[browser_test]
pub async fn toggle_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    toggle_first(page).await
}

/// Every item expanded, then none.
async fn expand_and_collapse_all(page: &Page<'_>, items: &str) -> Result<(), Report> {
    page.measure("expand all", page.control("expand-all"))
        .await?;
    wait_for_expanded(page, items).await?;
    page.measure("collapse all", page.control("collapse-all"))
        .await?;
    wait_for_expanded(page, "0").await?;
    page.element(FIRST)
        .await?
        .wait_for_inner_text("collapsed")
        .await?;
    Ok(())
}

/// Every one of 10 items expanded, then none.
#[browser_test]
pub async fn expand_all(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    expand_and_collapse_all(page, "10").await
}

/// Every one of 1,000 items expanded, then none.
#[browser_test]
pub async fn expand_all_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    expand_and_collapse_all(page, "1000").await
}

/// Setting the expanded items to what they are: nothing changes.
#[browser_test]
pub async fn same_keys(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("unchanged", page.control("collapse-all"))
        .await?;
    page.element(EXPANDED)
        .await?
        .inner_text_stays("0", Duration::from_millis(100))
        .await?;
    Ok(())
}
