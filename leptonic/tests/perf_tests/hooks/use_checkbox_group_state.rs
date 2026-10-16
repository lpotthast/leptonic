// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_checkbox_group_state`
//! (`leptonic/src/hooks/form/use_checkbox_group_state.rs`, fixture
//! `/perf/hooks/use_checkbox_group_state`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_checkbox_group_state.json`): creating and disposing of it (the
//! number of checked values shown), checking and unchecking a value (and an addition it
//! ignores: already checked), toggling one with few and with 1,000 values checked, setting
//! 1,000 values, recording a checkbox's validity, and a change it ignores while read-only.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_checkbox_group_state";
const COUNT: &str = "#perf-count";

/// Registers the performance tests of `use_checkbox_group_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(AddAndRemove))
        .with(fixture_test(Toggle))
        .with(fixture_test(SetManyValues))
        .with(fixture_test(ToggleWithManyValues))
        .with(fixture_test(RecordValidity))
        .with(fixture_test(IgnoredWhileReadOnly))
}

/// Creating the state (nothing checked) and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(COUNT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(COUNT).await?.wait_for_inner_text("0").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(COUNT, 0).await?;
    Ok(())
}

/// Checking a value, checking it again (ignored), and unchecking it.
#[browser_test]
pub async fn add_and_remove(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let count = page.element(COUNT).await?;
    page.measure("add", page.control("add-a")).await?;
    count.wait_for_inner_text("1").await?;
    page.measure("ignored add", page.control("add-a")).await?;
    count.wait_for_inner_text("1").await?;
    page.measure("remove", page.control("remove-a")).await?;
    count.wait_for_inner_text("0").await?;
    Ok(())
}

/// Toggling a value on and off.
#[browser_test]
pub async fn toggle(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let count = page.element(COUNT).await?;
    page.measure("on", page.control("toggle-a")).await?;
    count.wait_for_inner_text("1").await?;
    page.measure("off", page.control("toggle-a")).await?;
    count.wait_for_inner_text("0").await?;
    Ok(())
}

/// Setting 1,000 checked values.
#[browser_test]
pub async fn set_many_values(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let count = page.element(COUNT).await?;
    page.measure("set", page.control("check-1000")).await?;
    count.wait_for_inner_text("1000").await?;
    Ok(())
}

/// Toggling a value on and off while 1,000 others are checked.
#[browser_test]
pub async fn toggle_with_many_values(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let count = page.element(COUNT).await?;
    page.control("check-1000").await?;
    count.wait_for_inner_text("1000").await?;
    page.measure("on", page.control("toggle-a")).await?;
    count.wait_for_inner_text("1001").await?;
    page.measure("off", page.control("toggle-a")).await?;
    count.wait_for_inner_text("1000").await?;
    Ok(())
}

/// Recording that a checkbox of the group is invalid: the group shows itself invalid.
#[browser_test]
pub async fn record_validity(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("invalid checkbox", page.control("invalidate-a"))
        .await?;
    assert_that!(page.control("invalid").await?).is_equal_to(1.0);
    Ok(())
}

/// Checking a value while read-only: ignored.
#[browser_test]
pub async fn ignored_while_read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let count = page.element(COUNT).await?;
    page.control("set-read-only").await?;
    page.measure("ignored add", page.control("add-a")).await?;
    count.wait_for_inner_text("0").await?;
    Ok(())
}
