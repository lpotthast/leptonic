// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_disclosure_state` (`/perf/hooks/use_disclosure_state`): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_disclosure_state.json`): mounting and unmounting it, expanding and
//! collapsing, toggling, and a change to the state it already has (which it ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_disclosure_state";
const EXPANDED: &str = "#perf-expanded";

/// Registers the performance tests of `use_disclosure_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ExpandAndCollapse))
        .with(fixture_test(Toggle))
        .with(fixture_test(UnchangedState))
}

/// How often the state reported a change.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
}

/// Mounting the state and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(EXPANDED, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(EXPANDED)
        .await?
        .wait_for_inner_text("false")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(EXPANDED, 0).await?;
    Ok(())
}

/// `expand()` and `collapse()`.
#[browser_test]
pub async fn expand_and_collapse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let expanded = page.element(EXPANDED).await?;
    page.measure("expand", page.control("expand")).await?;
    expanded.wait_for_inner_text("true").await?;
    page.measure("collapse", page.control("collapse")).await?;
    expanded.wait_for_inner_text("false").await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// `toggle()` twice: expanding, then collapsing.
#[browser_test]
pub async fn toggle(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let expanded = page.element(EXPANDED).await?;
    page.measure("toggle", page.control("toggle")).await?;
    expanded.wait_for_inner_text("true").await?;
    page.measure("toggle", page.control("toggle")).await?;
    expanded.wait_for_inner_text("false").await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// `collapse()` while collapsed: ignored, nothing reported.
#[browser_test]
pub async fn unchanged_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("ignored collapse", page.control("collapse"))
        .await?;
    page.element(EXPANDED)
        .await?
        .wait_for_inner_text("false")
        .await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    Ok(())
}
