// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ProgressBarFill` (`/perf/atoms/progress_bar_fill`): what each thing
//! it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/progress_bar_fill.json`): mounting and unmounting it in a mounted
//! `ProgressBar`, and the value changing its width (and becoming unknown).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/progress_bar_fill";

/// Registers the performance tests of `ProgressBarFill`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ValueChanges))
        .with(fixture_test(Indeterminate))
}
const FILL: &str = "#perf-progress-bar-fill";

/// Mounting the fill and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(FILL, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let fill = page.element(FILL).await?;
    assert_that!(fill.style_property("width").await?).is_equal_to("25%".to_owned());
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FILL, 0).await?;
    Ok(())
}

/// The value rising from 25 to 75 (the fill widening) and falling back.
#[browser_test]
pub async fn value_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let fill = page.element(FILL).await?;
    page.measure("raise", page.control("raise")).await?;
    assert_that!(fill.style_property("width").await?).is_equal_to("75%".to_owned());
    page.measure("lower", page.control("lower")).await?;
    assert_that!(fill.style_property("width").await?).is_equal_to("25%".to_owned());
    Ok(())
}

/// The value becoming unknown (the fill marked indeterminate, without a width) and known again.
#[browser_test]
pub async fn indeterminate(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let fill = page.element(FILL).await?;
    page.measure("make indeterminate", page.control("make-indeterminate"))
        .await?;
    fill.wait_for_attr("data-indeterminate", Some("true"))
        .await?;
    assert_that!(fill.style_property("width").await?).is_equal_to(String::new());
    page.measure("make determinate", page.control("lower"))
        .await?;
    fill.wait_for_attr("data-indeterminate", None).await?;
    assert_that!(fill.style_property("width").await?).is_equal_to("25%".to_owned());
    Ok(())
}
