// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_toggle_state`
//! (`leptonic/src/hooks/form/use_toggle_state.rs`, fixture `/perf/hooks/use_toggle_state`): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_toggle_state.json`): creating and disposing of it (its selection
//! shown), toggling it on and off, selecting it (and a selection it ignores: already
//! selected), and a toggle it ignores while read-only.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_toggle_state";
const SELECTION: &str = "#perf-selection";

/// Registers the performance tests of `use_toggle_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Toggle))
        .with(fixture_test(Select))
        .with(fixture_test(IgnoredWhileReadOnly))
}

/// How often the state reported a change.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
}

/// Creating the state (its selection shown) and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SELECTION, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(SELECTION)
        .await?
        .wait_for_inner_text("not selected")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SELECTION, 0).await?;
    Ok(())
}

/// Toggling the selection on and off.
#[browser_test]
pub async fn toggle(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let selection = page.element(SELECTION).await?;
    page.measure("on", page.control("toggle")).await?;
    selection.wait_for_inner_text("selected").await?;
    page.measure("off", page.control("toggle")).await?;
    selection.wait_for_inner_text("not selected").await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// Selecting it, and selecting it again (ignored: no change).
#[browser_test]
pub async fn select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let selection = page.element(SELECTION).await?;
    page.measure("select", page.control("select")).await?;
    selection.wait_for_inner_text("selected").await?;
    page.measure("ignored select", page.control("select"))
        .await?;
    assert_that!(changes(page).await?).is_equal_to(1.0);
    Ok(())
}

/// A toggle while read-only: ignored.
#[browser_test]
pub async fn ignored_while_read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let selection = page.element(SELECTION).await?;
    page.control("set-read-only").await?;
    page.measure("ignored toggle", page.control("toggle"))
        .await?;
    selection.wait_for_inner_text("not selected").await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    Ok(())
}
