// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `SliderFill` (`/perf/atoms/slider_fill`): what each thing it offers
//! costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/slider_fill.json`): mounting and unmounting it in a mounted `Slider`, the
//! value changing its width, its `offset`, the slider's orientation, and hover.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/slider_fill";

/// Registers the performance tests of `SliderFill`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ValueChanges))
        .with(fixture_test(Offset))
        .with(fixture_test(ChangeOrientation))
        .with(fixture_test(HoverAndLeave))
}
const FILL: &str = "#perf-slider-fill";

/// The fill's inline style property `name`.
async fn style(page: &Page<'_>, name: &str) -> Result<String, Report> {
    page.element(FILL).await?.style_property(name).await
}

/// Mounting the fill and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(FILL, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    assert_that!(style(page, "width").await?).is_equal_to("30%".to_owned());
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FILL, 0).await?;
    Ok(())
}

/// The value rising from 30 to 60 (the fill widening) and falling back.
#[browser_test]
pub async fn value_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("raise", page.control("set-value")).await?;
    assert_that!(style(page, "width").await?).is_equal_to("60%".to_owned());
    page.measure("lower", page.control("reset-value")).await?;
    assert_that!(style(page, "width").await?).is_equal_to("30%".to_owned());
    Ok(())
}

/// The fill starting at an offset (50: from the thumb at 30 to 50) and from the start again.
#[browser_test]
pub async fn offset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set offset", page.control("set-offset"))
        .await?;
    assert_that!(style(page, "inset-inline-start").await?).is_equal_to("30%".to_owned());
    assert_that!(style(page, "width").await?).is_equal_to("20%".to_owned());
    page.measure("unset offset", page.control("unset-offset"))
        .await?;
    assert_that!(style(page, "inset-inline-start").await?).is_equal_to("0%".to_owned());
    assert_that!(style(page, "width").await?).is_equal_to("30%".to_owned());
    Ok(())
}

/// The slider laid out vertically (the fill's height and bottom) and horizontally again.
#[browser_test]
pub async fn change_orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("make vertical", page.control("make-vertical"))
        .await?;
    assert_that!(style(page, "height").await?).is_equal_to("30%".to_owned());
    assert_that!(style(page, "width").await?).is_equal_to(String::new());
    page.measure("make horizontal", page.control("make-horizontal"))
        .await?;
    assert_that!(style(page, "width").await?).is_equal_to("30%".to_owned());
    Ok(())
}

/// The pointer entering and leaving the fill.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let fill = page.element(FILL).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", fill.hover()).await?;
    fill.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    fill.wait_for_attr("data-hovered", None).await?;
    Ok(())
}
