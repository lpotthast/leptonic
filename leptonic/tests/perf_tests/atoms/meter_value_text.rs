// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `MeterValueText` (`/perf/atoms/meter_value_text`): what each thing
//! it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/meter_value_text.json`): mounting and unmounting it in a mounted `Meter`,
//! and the value and the format changing its text.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/meter_value_text";

/// Registers the performance tests of `MeterValueText`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ValueChanges))
        .with(fixture_test(FormatChanges))
}
const TEXT: &str = "#perf-meter-value-text";

/// Mounting the text and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TEXT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TEXT).await?.wait_for_inner_text("25%").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TEXT, 0).await?;
    Ok(())
}

/// The value rising from 25 to 75 and falling back.
#[browser_test]
pub async fn value_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let text = page.element(TEXT).await?;
    page.measure("raise", page.control("raise")).await?;
    text.wait_for_inner_text("75%").await?;
    page.measure("lower", page.control("lower")).await?;
    text.wait_for_inner_text("25%").await?;
    Ok(())
}

/// The format changing from a percentage to a decimal and back.
#[browser_test]
pub async fn format_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let text = page.element(TEXT).await?;
    page.measure("format decimal", page.control("format-decimal"))
        .await?;
    text.wait_for_inner_text("25").await?;
    page.measure("format percent", page.control("format-percent"))
        .await?;
    text.wait_for_inner_text("25%").await?;
    Ok(())
}
