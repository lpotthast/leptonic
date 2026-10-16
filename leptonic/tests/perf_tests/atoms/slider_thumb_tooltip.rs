// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `SliderThumbTooltip` (`/perf/atoms/slider_thumb_tooltip`): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/slider_thumb_tooltip.json`): mounting and unmounting it in a mounted
//! `SliderThumb`, showing it on hover and on drag (its popover setting is no reactive prop:
//! `OnHoverOrDrag` covers both), and the value changing its text.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/slider_thumb_tooltip";

/// Registers the performance tests of `SliderThumbTooltip`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(Drag))
        .with(fixture_test(ValueChanges))
}
const TOOLTIP: &str = "#perf-slider-thumb-tooltip";
const THUMB: &str = "#perf-slider-thumb";

/// Mounting the tooltip (hidden) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TOOLTIP, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_attr(TOOLTIP, "data-visible", None).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TOOLTIP, 0).await?;
    Ok(())
}

/// The pointer entering the thumb (showing the tooltip) and leaving it (hiding it).
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element(THUMB).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", thumb.hover()).await?;
    page.wait_for_attr(TOOLTIP, "data-visible", Some("true"))
        .await?;
    page.element(TOOLTIP)
        .await?
        .wait_for_inner_text("30")
        .await?;
    page.measure("leave", after.hover()).await?;
    page.wait_for_attr(TOOLTIP, "data-visible", None).await?;
    Ok(())
}

/// Pressing the thumb (showing the tooltip), dragging it 20 pixels right (the tooltip showing 40),
/// and releasing it (the pointer still on the thumb, so the tooltip stays).
#[browser_test]
pub async fn drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element(THUMB).await?;
    let held = page.measure("press", thumb.press_and_hold()).await?;
    page.wait_for_attr(TOOLTIP, "data-visible", Some("true"))
        .await?;
    page.measure("drag", held.move_by(20, 0)).await?;
    page.element(TOOLTIP)
        .await?
        .wait_for_inner_text("40")
        .await?;
    page.measure("release", held.release()).await?;
    thumb.wait_for_attr("data-dragging", None).await?;
    assert_that!(page.control("value").await?).is_equal_to(40.0);
    Ok(())
}

/// The value changing to 60 and back to 30 (the tooltip's text).
#[browser_test]
pub async fn value_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tooltip = page.element(TOOLTIP).await?;
    page.measure("set", page.control("set-value")).await?;
    tooltip.wait_for_inner_text("60").await?;
    page.measure("reset", page.control("reset-value")).await?;
    tooltip.wait_for_inner_text("30").await?;
    Ok(())
}
