// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_radio_group_state`
//! (`leptonic/src/hooks/form/use_radio_group_state.rs`, fixture
//! `/perf/hooks/use_radio_group_state`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_radio_group_state.json`): creating and disposing of it (its
//! selected value shown), selecting another value (and the selected one again), recording the
//! radio focused last, and a selection it ignores while read-only.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_radio_group_state";
const SELECTED: &str = "#perf-selected";

/// Registers the performance tests of `use_radio_group_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Select))
        .with(fixture_test(LastFocused))
        .with(fixture_test(IgnoredWhileReadOnly))
}

/// Creating the state ("a" selected) and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SELECTED, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(SELECTED)
        .await?
        .wait_for_inner_text("a")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SELECTED, 0).await?;
    Ok(())
}

/// Selecting another value, and selecting it again (no change).
#[browser_test]
pub async fn select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let selected = page.element(SELECTED).await?;
    page.measure("select", page.control("select-b")).await?;
    selected.wait_for_inner_text("b").await?;
    page.measure("select again", page.control("select-b"))
        .await?;
    selected.wait_for_inner_text("b").await?;
    Ok(())
}

/// Recording the radio focused last.
#[browser_test]
pub async fn last_focused(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("focus", page.control("focus-b")).await?;
    assert_that!(page.control("b-focused-last").await?).is_equal_to(1.0);
    Ok(())
}

/// Selecting another value while read-only: ignored.
#[browser_test]
pub async fn ignored_while_read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let selected = page.element(SELECTED).await?;
    page.control("set-read-only").await?;
    page.measure("ignored select", page.control("select-b"))
        .await?;
    selected.wait_for_inner_text("a").await?;
    Ok(())
}
