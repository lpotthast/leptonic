// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_slider_marks` (`/perf/hooks/use_slider_marks`, alone, its
//! marks rendered as plain elements): what each thing it offers costs, measured (`Page::measure`)
//! and checked against its budget (`tests/budgets/hooks/use_slider_marks.json`): mounting and
//! unmounting it (21 automatic named marks, and 3 custom ones; the automatic marks are at most 21,
//! whatever the range), the values moving which marks are in range, and the range and step
//! changing the marks.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_slider_marks";

/// Registers the performance tests of `use_slider_marks`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountCustom))
        .with(fixture_test(ValueChanges))
        .with(fixture_test(RangeChanges))
        .with(fixture_test(StepChanges))
}
const MARKS: &str = "#perf-slider-marks";
const MARK: &str = "#perf-slider-marks .perf-mark";
const IN_RANGE: &str = "#perf-slider-marks .perf-mark[data-in-range]";

/// Waits until there are `marks` marks, `in_range` of them in range.
async fn wait_for_marks(page: &Page<'_>, marks: usize, in_range: usize) -> Result<(), Report> {
    page.wait_for_count(MARK, marks).await?;
    page.wait_for_count(IN_RANGE, in_range).await?;
    Ok(())
}

/// Mounting and unmounting the marks with the control `mount` (`mount-custom`): `marks` marks,
/// `in_range` of them in range.
async fn mount_and_unmount_with(
    page: &Page<'_>,
    mount: &str,
    marks: usize,
    in_range: usize,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(MARKS, 0).await?;
    page.measure("mount", page.control(mount)).await?;
    wait_for_marks(page, marks, in_range).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(MARKS, 0).await?;
    Ok(())
}

/// Mounting 21 automatic marks (every fifth step, 7 of them up to the value 30) and unmounting
/// them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, "mount", 21, 7).await
}

/// Mounting 3 custom marks (Low, Middle, High: one up to the value 30) and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_custom(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, "mount-custom", 3, 1).await
}

/// The value moving to 60 (13 marks in range) and back to 30 (7).
#[browser_test]
pub async fn value_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-value")).await?;
    wait_for_marks(page, 21, 13).await?;
    page.measure("reset", page.control("reset-value")).await?;
    wait_for_marks(page, 21, 7).await?;
    Ok(())
}

/// The maximum growing from 100 to 200 (21 marks, every tenth step: 4 in range) and shrinking
/// back (every fifth step again: 7 in range).
#[browser_test]
pub async fn range_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("widen", page.control("widen")).await?;
    wait_for_marks(page, 21, 4).await?;
    page.measure("narrow", page.control("narrow")).await?;
    wait_for_marks(page, 21, 7).await?;
    Ok(())
}

/// The step growing from 1 to 10 (11 marks, 4 in range) and shrinking back (21, 7 in range).
#[browser_test]
pub async fn step_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("coarsen", page.control("coarsen")).await?;
    wait_for_marks(page, 11, 4).await?;
    page.measure("refine", page.control("refine")).await?;
    wait_for_marks(page, 21, 7).await?;
    Ok(())
}
