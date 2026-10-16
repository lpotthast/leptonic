// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ProgressBar` (`/perf/atoms/progress_bar`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/progress_bar.json`): mounting and unmounting it, and changes of its
//! reactive props (the value, also unknown: indeterminate, the range, the format, the value label).
//! Its parts are measured in `progress_bar_fill.rs` and `progress_bar_value_text.rs`.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/progress_bar";

/// Registers the performance tests of `ProgressBar`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ValueChanges))
        .with(fixture_test(Indeterminate))
        .with(fixture_test(RangeChanges))
        .with(fixture_test(FormatChanges))
        .with(fixture_test(ValueLabel))
}
const PROGRESS_BAR: &str = "#perf-progress-bar";

/// Mounting the progress bar and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(PROGRESS_BAR, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_attr(PROGRESS_BAR, "aria-valuetext", Some("25%"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(PROGRESS_BAR, 0).await?;
    Ok(())
}

/// The value rising from 25 to 75 and falling back.
#[browser_test]
pub async fn value_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("raise", page.control("raise")).await?;
    page.wait_for_attr(PROGRESS_BAR, "aria-valuenow", Some("75"))
        .await?;
    page.wait_for_attr(PROGRESS_BAR, "aria-valuetext", Some("75%"))
        .await?;
    page.measure("lower", page.control("lower")).await?;
    page.wait_for_attr(PROGRESS_BAR, "aria-valuenow", Some("25"))
        .await?;
    page.wait_for_attr(PROGRESS_BAR, "aria-valuetext", Some("25%"))
        .await?;
    Ok(())
}

/// The value becoming unknown (indeterminate) and known again.
#[browser_test]
pub async fn indeterminate(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("make indeterminate", page.control("make-indeterminate"))
        .await?;
    page.wait_for_attr(PROGRESS_BAR, "data-indeterminate", Some("true"))
        .await?;
    page.wait_for_attr(PROGRESS_BAR, "aria-valuenow", None)
        .await?;
    page.measure("make determinate", page.control("lower"))
        .await?;
    page.wait_for_attr(PROGRESS_BAR, "data-indeterminate", None)
        .await?;
    page.wait_for_attr(PROGRESS_BAR, "aria-valuenow", Some("25"))
        .await?;
    Ok(())
}

/// The maximum growing from 100 to 200 and shrinking back.
#[browser_test]
pub async fn range_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("widen", page.control("widen")).await?;
    page.wait_for_attr(PROGRESS_BAR, "aria-valuemax", Some("200"))
        .await?;
    page.measure("narrow", page.control("narrow")).await?;
    page.wait_for_attr(PROGRESS_BAR, "aria-valuemax", Some("100"))
        .await?;
    page.wait_for_attr(PROGRESS_BAR, "aria-valuetext", Some("25%"))
        .await?;
    Ok(())
}

/// The format changing from a percentage to a decimal and back.
#[browser_test]
pub async fn format_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("format decimal", page.control("format-decimal"))
        .await?;
    page.wait_for_attr(PROGRESS_BAR, "aria-valuetext", Some("25"))
        .await?;
    page.measure("format percent", page.control("format-percent"))
        .await?;
    page.wait_for_attr(PROGRESS_BAR, "aria-valuetext", Some("25%"))
        .await?;
    Ok(())
}

/// A value label replacing the formatted value, and the formatted value again.
#[browser_test]
pub async fn value_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-value-label")).await?;
    page.wait_for_attr(PROGRESS_BAR, "aria-valuetext", Some("A quarter"))
        .await?;
    page.measure("unset", page.control("unset-value-label"))
        .await?;
    page.wait_for_attr(PROGRESS_BAR, "aria-valuetext", Some("25%"))
        .await?;
    Ok(())
}
