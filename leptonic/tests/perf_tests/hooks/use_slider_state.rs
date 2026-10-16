// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_slider_state` (`/perf/hooks/use_slider_state`, alone, its
//! values shown in an `<output>`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_slider_state.json`): mounting and
//! unmounting it, its methods (increment, decrement, setting a value or a percentage, dragging,
//! focusing a thumb), changes of its inputs (maximum, step, disabled) and the change it ignores
//! while disabled.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_slider_state";

/// Registers the performance tests of `use_slider_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(IncrementAndDecrement))
        .with(fixture_test(SetValueAndPercent))
        .with(fixture_test(DragStartAndEnd))
        .with(fixture_test(FocusAndBlurTheThumb))
        .with(fixture_test(RangeChanges))
        .with(fixture_test(StepChanges))
        .with(fixture_test(DisableAndEnable))
}
const OUTPUT: &str = "#perf-slider-state";

/// The value of the slider's thumb.
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Mounting the state and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(OUTPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(OUTPUT)
        .await?
        .wait_for_inner_text("30")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(OUTPUT, 0).await?;
    Ok(())
}

/// Incrementing the value by a step and decrementing it again.
#[browser_test]
pub async fn increment_and_decrement(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let output = page.element(OUTPUT).await?;
    page.measure("increment", page.control("increment")).await?;
    output.wait_for_inner_text("31").await?;
    page.measure("decrement", page.control("decrement")).await?;
    output.wait_for_inner_text("30").await?;
    Ok(())
}

/// Setting the value (60), and the percentage (a quarter: 25).
#[browser_test]
pub async fn set_value_and_percent(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let output = page.element(OUTPUT).await?;
    page.measure("set value", page.control("set-value")).await?;
    output.wait_for_inner_text("60").await?;
    page.measure("set percent", page.control("set-percent"))
        .await?;
    output.wait_for_inner_text("25").await?;
    Ok(())
}

/// The thumb starting to be dragged and stopping (ending the change).
#[browser_test]
pub async fn drag_start_and_end(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("start", page.control("start-drag")).await?;
    page.wait_for_attr(OUTPUT, "data-dragging", Some("true"))
        .await?;
    page.measure("end", page.control("end-drag")).await?;
    page.wait_for_attr(OUTPUT, "data-dragging", None).await?;
    assert_that!(page.control("change-ends").await?).is_equal_to(1.0);
    Ok(())
}

/// The thumb becoming the focused one and none again.
#[browser_test]
pub async fn focus_and_blur_the_thumb(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("focus", page.control("focus-thumb")).await?;
    page.wait_for_attr(OUTPUT, "data-focused", Some("true"))
        .await?;
    page.measure("blur", page.control("blur-thumb")).await?;
    page.wait_for_attr(OUTPUT, "data-focused", None).await?;
    Ok(())
}

/// The maximum growing from 100 to 200 (the thumb at 15%) and shrinking back (30%).
#[browser_test]
pub async fn range_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("widen", page.control("widen")).await?;
    assert_that!(page.control("percent").await?).is_equal_to(15.0);
    page.measure("narrow", page.control("narrow")).await?;
    assert_that!(page.control("percent").await?).is_equal_to(30.0);
    Ok(())
}

/// The step growing from 1 to 5 (an increment then adds 5) and shrinking back.
#[browser_test]
pub async fn step_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("coarsen", page.control("coarsen")).await?;
    page.control("increment").await?;
    assert_that!(value(page).await?).is_equal_to(35.0);
    page.measure("refine", page.control("refine")).await?;
    page.control("increment").await?;
    assert_that!(value(page).await?).is_equal_to(36.0);
    Ok(())
}

/// Disabling the state, an increment it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable")).await?;
    page.wait_for_attr(OUTPUT, "data-disabled", Some("true"))
        .await?;
    page.measure("ignored increment", page.control("increment"))
        .await?;
    assert_that!(value(page).await?).is_equal_to(30.0);
    page.measure("enable", page.control("enable")).await?;
    page.wait_for_attr(OUTPUT, "data-disabled", None).await?;
    Ok(())
}
