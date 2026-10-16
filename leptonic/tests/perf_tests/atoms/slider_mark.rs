// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `SliderMark` (`/perf/atoms/slider_mark`): what each thing it offers
//! costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/slider_mark.json`): mounting and unmounting one mark in the mounted
//! `SliderMarks` of a `Slider`, the value moving it into range and out again, and the slider's
//! orientation moving it.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/slider_mark";

/// Registers the performance tests of `SliderMark`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(InAndOutOfRange))
        .with(fixture_test(ChangeOrientation))
}
const MARK: &str = "#perf-slider-mark";

/// Mounting the mark (out of range, at 50) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(MARK, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let mark = page.element(MARK).await?;
    assert_that!(mark.style_property("inset-inline-start").await?).is_equal_to("50%".to_owned());
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(MARK, 0).await?;
    Ok(())
}

/// The value moving to 60 (the mark in range) and back to 30 (out of range).
#[browser_test]
pub async fn in_and_out_of_range(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let mark = page.element(MARK).await?;
    page.measure("into range", page.control("set-value"))
        .await?;
    mark.wait_for_attr("data-in-range", Some("true")).await?;
    page.measure("out of range", page.control("reset-value"))
        .await?;
    mark.wait_for_attr("data-in-range", None).await?;
    Ok(())
}

/// The slider laid out vertically (the mark placed from the bottom) and horizontally again.
#[browser_test]
pub async fn change_orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let mark = page.element(MARK).await?;
    page.measure("make vertical", page.control("make-vertical"))
        .await?;
    mark.wait_for_attr("data-orientation", Some("vertical"))
        .await?;
    assert_that!(mark.style_property("bottom").await?).is_equal_to("50%".to_owned());
    page.measure("make horizontal", page.control("make-horizontal"))
        .await?;
    mark.wait_for_attr("data-orientation", Some("horizontal"))
        .await?;
    assert_that!(mark.style_property("inset-inline-start").await?).is_equal_to("50%".to_owned());
    Ok(())
}
