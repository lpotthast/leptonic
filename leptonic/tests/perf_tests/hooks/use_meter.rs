// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_meter` (`/perf/hooks/use_meter`, on a plain element): what each
//! thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_meter.json`): mounting and unmounting it (with its value text), and
//! changes of its reactive inputs (the value, the range, the format, the value label).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_meter";

/// Registers the performance tests of `use_meter`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ValueChanges))
        .with(fixture_test(RangeChanges))
        .with(fixture_test(FormatChanges))
        .with(fixture_test(ValueLabel))
}
const METER: &str = "#perf-meter";
const TEXT: &str = "#perf-meter-text";

/// Mounting the meter and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(METER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TEXT).await?.wait_for_inner_text("25%").await?;
    page.wait_for_attr(METER, "aria-valuetext", Some("25%"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(METER, 0).await?;
    Ok(())
}

/// The value rising from 25 to 75 and falling back.
#[browser_test]
pub async fn value_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("raise", page.control("raise")).await?;
    page.wait_for_attr(METER, "aria-valuenow", Some("75"))
        .await?;
    page.wait_for_attr(METER, "aria-valuetext", Some("75%"))
        .await?;
    page.measure("lower", page.control("lower")).await?;
    page.wait_for_attr(METER, "aria-valuenow", Some("25"))
        .await?;
    page.wait_for_attr(METER, "aria-valuetext", Some("25%"))
        .await?;
    Ok(())
}

/// The maximum growing from 100 to 200 and shrinking back.
#[browser_test]
pub async fn range_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("widen", page.control("widen")).await?;
    page.wait_for_attr(METER, "aria-valuemax", Some("200"))
        .await?;
    page.measure("narrow", page.control("narrow")).await?;
    page.wait_for_attr(METER, "aria-valuemax", Some("100"))
        .await?;
    page.wait_for_attr(METER, "aria-valuetext", Some("25%"))
        .await?;
    Ok(())
}

/// The format changing from a percentage to a decimal and back.
#[browser_test]
pub async fn format_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("format decimal", page.control("format-decimal"))
        .await?;
    page.wait_for_attr(METER, "aria-valuetext", Some("25"))
        .await?;
    page.measure("format percent", page.control("format-percent"))
        .await?;
    page.wait_for_attr(METER, "aria-valuetext", Some("25%"))
        .await?;
    Ok(())
}

/// A value label replacing the formatted value, and the formatted value again.
#[browser_test]
pub async fn value_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-value-label")).await?;
    page.wait_for_attr(METER, "aria-valuetext", Some("A quarter"))
        .await?;
    page.measure("unset", page.control("unset-value-label"))
        .await?;
    page.wait_for_attr(METER, "aria-valuetext", Some("25%"))
        .await?;
    Ok(())
}
