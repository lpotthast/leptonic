// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_number_field_state`
//! (`leptonic/src/hooks/form/use_number_field_state.rs`, fixture
//! `/perf/hooks/use_number_field_state`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_number_field_state.json`): creating and disposing of it (its text
//! shown), stepping (by one, to the maximum and the minimum, and a step it ignores at the
//! maximum), typed text and its commit, setting the value, and a change of its format.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_number_field_state";
const TEXT: &str = "#perf-text";

/// Registers the performance tests of `use_number_field_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Step))
        .with(fixture_test(StepToTheLimits))
        .with(fixture_test(TypeAndCommit))
        .with(fixture_test(SetTheValue))
        .with(fixture_test(Format))
}

/// The state's committed value (-1: empty).
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Creating the state (its text shown) and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TEXT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TEXT).await?.wait_for_inner_text("5").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TEXT, 0).await?;
    Ok(())
}

/// Incrementing and decrementing by one step.
#[browser_test]
pub async fn step(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let text = page.element(TEXT).await?;
    page.measure("increment", page.control("increment")).await?;
    text.wait_for_inner_text("6").await?;
    page.measure("decrement", page.control("decrement")).await?;
    text.wait_for_inner_text("5").await?;
    Ok(())
}

/// Stepping to the maximum, a step it ignores there, and stepping to the minimum.
#[browser_test]
pub async fn step_to_the_limits(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let text = page.element(TEXT).await?;
    page.measure("to the maximum", page.control("increment-to-max"))
        .await?;
    text.wait_for_inner_text("10").await?;
    page.measure("ignored increment", page.control("increment"))
        .await?;
    assert_that!(value(page).await?).is_equal_to(10.0);
    page.measure("to the minimum", page.control("decrement-to-min"))
        .await?;
    text.wait_for_inner_text("0").await?;
    Ok(())
}

/// Typed text (the value stays until committed) and its commit, clamped to the maximum.
#[browser_test]
pub async fn type_and_commit(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let text = page.element(TEXT).await?;
    page.measure("type", page.control("type-12")).await?;
    text.wait_for_inner_text("12").await?;
    assert_that!(value(page).await?).is_equal_to(5.0);
    page.measure("commit", page.control("commit")).await?;
    text.wait_for_inner_text("10").await?;
    assert_that!(value(page).await?).is_equal_to(10.0);
    Ok(())
}

/// Setting a number value (as it is: only commits clamp).
#[browser_test]
pub async fn set_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let text = page.element(TEXT).await?;
    page.measure("set", page.control("set-42")).await?;
    text.wait_for_inner_text("42").await?;
    assert_that!(value(page).await?).is_equal_to(42.0);
    Ok(())
}

/// Changing the format: the text shows the value in US dollars.
#[browser_test]
pub async fn format(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let text = page.element(TEXT).await?;
    page.measure("as dollars", page.control("format-as-dollars"))
        .await?;
    text.wait_for_inner_text("$5.00").await?;
    Ok(())
}
