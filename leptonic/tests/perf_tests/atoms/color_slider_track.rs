// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ColorSliderTrack` (`/perf/atoms/color_slider_track`, with its
//! thumb, in a `ColorSlider` of the red channel): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/color_slider_track.json`): mounting and unmounting it, hover, pressing
//! it (a value, then the change's end), its gradient following the value, and the slider
//! disabling it (with a press it ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/color_slider_track";

/// Registers the performance tests of `ColorSliderTrack`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(PressAndRelease))
        .with(fixture_test(ChangeTheValue))
        .with(fixture_test(DisabledSlider))
}
const TRACK: &str = ".leptonic-ColorSliderTrack";
/// The bound value at first (`#8000FF`).
const PURPLE: f64 = 8_388_863.0;

/// The slider's value (`0xRRGGBB`).
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Mounting the track (with its thumb) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TRACK, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TRACK).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TRACK, 0).await?;
    // The output refers to the input in the track: mounted again, the page is valid at the end.
    page.control("mount").await?;
    page.element(TRACK).await?;
    Ok(())
}

/// The pointer entering and leaving the track.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let track = page.element(TRACK).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", track.hover()).await?;
    track.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    track.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Pressing the track (the value at that point) and releasing the pointer.
#[browser_test]
pub async fn press_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let track = page.element(TRACK).await?;
    let held = page
        .measure("press", track.press_and_hold_at(-50, 0))
        .await?;
    assert_that!(value(page).await?).is_not_equal_to(PURPLE);
    page.measure("release", held.release()).await?;
    Ok(())
}

/// The slider's value changing (green: the track's gradient follows the other channels) and
/// back.
#[browser_test]
pub async fn change_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let track = page.element(TRACK).await?;
    let gradient = track.style_property("background").await?;
    page.measure("set green", page.control("set-green")).await?;
    assert_that!(track.style_property("background").await?).is_not_equal_to(gradient.clone());
    page.measure("set purple", page.control("set-purple"))
        .await?;
    assert_that!(track.style_property("background").await?).is_equal_to(gradient);
    Ok(())
}

/// The slider disabling the track, a press it ignores, and enabling it again.
#[browser_test]
pub async fn disabled_slider(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let track = page.element(TRACK).await?;
    page.measure("disable", page.control("disable")).await?;
    track.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", track.click()).await?;
    assert_that!(value(page).await?).is_equal_to(PURPLE);
    page.measure("enable", page.control("enable")).await?;
    track.wait_for_attr("data-disabled", None).await?;
    Ok(())
}
