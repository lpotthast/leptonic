// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ColorWheelTrack` (`/perf/atoms/color_wheel_track`, in a
//! `ColorWheel` with its thumb): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/color_wheel_track.json`): mounting and
//! unmounting it, pressing it (a hue, then the change's end), the wheel's radii changing, and the
//! wheel disabling it (with a press it ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/color_wheel_track";

/// Registers the performance tests of `ColorWheelTrack`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressAndRelease))
        .with(fixture_test(ChangeTheRadii))
        .with(fixture_test(DisabledWheel))
}
const TRACK: &str = ".leptonic-ColorWheelTrack";

/// The wheel's hue.
async fn hue(page: &Page<'_>) -> Result<f64, Report> {
    page.control("hue").await
}

/// Mounting the track and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TRACK, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TRACK).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TRACK, 0).await?;
    Ok(())
}

/// Pressing the bottom of the ring (hue 90) and releasing the pointer.
#[browser_test]
pub async fn press_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let track = page.element(TRACK).await?;
    let held = page
        .measure("press", track.press_and_hold_at(0, 87))
        .await?;
    assert_that!(hue(page).await?).is_equal_to(90.0);
    page.measure("release", held.release()).await?;
    Ok(())
}

/// The wheel's radii growing and shrinking again (the track's size).
#[browser_test]
pub async fn change_the_radii(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let track = page.element(TRACK).await?;
    let size = track.style_property("width").await?;
    page.measure("grow", page.control("grow")).await?;
    assert_that!(track.style_property("width").await?).is_not_equal_to(size.clone());
    page.measure("shrink", page.control("shrink")).await?;
    assert_that!(track.style_property("width").await?).is_equal_to(size);
    Ok(())
}

/// The wheel disabling the track, a press it ignores, and enabling it again.
#[browser_test]
pub async fn disabled_wheel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let track = page.element(TRACK).await?;
    page.measure("disable", page.control("disable")).await?;
    track.wait_for_attr("data-disabled", Some("true")).await?;
    let held = page
        .measure("ignored press", track.press_and_hold_at(0, 87))
        .await?;
    held.release().await?;
    assert_that!(hue(page).await?).is_equal_to(180.0);
    page.measure("enable", page.control("enable")).await?;
    track.wait_for_attr("data-disabled", None).await?;
    Ok(())
}
