// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `SliderOutput` (`/perf/atoms/slider_output`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/slider_output.json`): mounting and unmounting it in a mounted `Slider`,
//! and the value and the slider's format changing its text.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/slider_output";

/// Registers the performance tests of `SliderOutput`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ValueChanges))
        .with(fixture_test(FormatChanges))
}
const OUTPUT: &str = "#perf-slider-output";

/// Mounting the output and unmounting it.
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

/// The value changing to 60 and back to 30.
#[browser_test]
pub async fn value_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let output = page.element(OUTPUT).await?;
    page.measure("set", page.control("set-value")).await?;
    output.wait_for_inner_text("60").await?;
    page.measure("reset", page.control("reset-value")).await?;
    output.wait_for_inner_text("30").await?;
    Ok(())
}

/// The slider's format changing to one fraction digit and back.
#[browser_test]
pub async fn format_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let output = page.element(OUTPUT).await?;
    page.measure("format fraction", page.control("format-fraction"))
        .await?;
    output.wait_for_inner_text("30.0").await?;
    page.measure("format integer", page.control("format-integer"))
        .await?;
    output.wait_for_inner_text("30").await?;
    Ok(())
}
