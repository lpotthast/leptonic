// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_tree_state` (`/perf/hooks/use_tree_state`, the state alone):
//! what each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_tree_state.json`): creating and disposing it, an item expanded and
//! collapsed, every item expanded and collapsed, and the selection changing, with 5 parent items
//! (10 items) and with 500 (1,000 items).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_tree_state";

/// Registers the performance tests of `use_tree_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(ToggleSmall))
        .with(fixture_test(ToggleLarge))
        .with(fixture_test(ExpandAllSmall))
        .with(fixture_test(ExpandAllLarge))
        .with(fixture_test(SelectionSmall))
        .with(fixture_test(SelectionLarge))
}

const VISIBLE: &str = "#perf-visible";

/// Opens the fixture with its 5 parents.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(VISIBLE)
        .await?
        .wait_for_inner_text("5")
        .await?;
    Ok(())
}

/// Opens the fixture and creates the state of 500 parents (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(VISIBLE, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.element(VISIBLE)
        .await?
        .wait_for_inner_text("500")
        .await?;
    Ok(())
}

/// How many items are visible.
async fn visible(page: &Page<'_>) -> Result<f64, Report> {
    page.control("visible").await
}

/// Creating the state and disposing it.
async fn mount_and_unmount_state(page: &Page<'_>, parents: &str) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(VISIBLE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(VISIBLE)
        .await?
        .wait_for_inner_text(parents)
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(VISIBLE, 0).await?;
    Ok(())
}

/// The state of 5 parents created and disposed.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_state(page, "5").await
}

/// The state of 500 parents created and disposed.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_state(page, "500").await
}

/// The first parent expanded (its child becomes visible) and collapsed.
async fn toggle(page: &Page<'_>, parents: f64) -> Result<(), Report> {
    page.measure("expand", page.control("toggle-first")).await?;
    assert_that!(visible(page).await?).is_equal_to(parents + 1.0);
    page.measure("collapse", page.control("toggle-first"))
        .await?;
    assert_that!(visible(page).await?).is_equal_to(parents);
    Ok(())
}

/// A parent of 5 expanded and collapsed.
#[browser_test]
pub async fn toggle_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    toggle(page, 5.0).await
}

/// A parent of 500 expanded and collapsed.
#[browser_test]
pub async fn toggle_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    toggle(page, 500.0).await
}

/// Every parent expanded, then none.
async fn expand_all(page: &Page<'_>, parents: f64) -> Result<(), Report> {
    page.measure("expand all", page.control("expand-all"))
        .await?;
    assert_that!(visible(page).await?).is_equal_to(2.0 * parents);
    page.measure("collapse all", page.control("collapse-all"))
        .await?;
    assert_that!(visible(page).await?).is_equal_to(parents);
    Ok(())
}

/// All of 5 parents expanded and collapsed.
#[browser_test]
pub async fn expand_all_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    expand_all(page, 5.0).await
}

/// All of 500 parents expanded and collapsed.
#[browser_test]
pub async fn expand_all_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    expand_all(page, 500.0).await
}

/// The first parent selected, then none.
async fn selection(page: &Page<'_>) -> Result<(), Report> {
    page.measure("select", page.control("select-first")).await?;
    assert_that!(page.control("selected").await?).is_equal_to(1.0);
    page.measure("clear", page.control("clear-selection"))
        .await?;
    assert_that!(page.control("selected").await?).is_equal_to(0.0);
    Ok(())
}

/// The selection of a tree of 5 parents changing.
#[browser_test]
pub async fn selection_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    selection(page).await
}

/// The selection of a tree of 500 parents changing.
#[browser_test]
pub async fn selection_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    selection(page).await
}
