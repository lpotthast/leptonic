// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ColorThumb` (`/perf/atoms/color_thumb`, the thumbs of a
//! `ColorArea`, a `ColorSlider` and a `ColorWheel`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget (`tests/budgets/atoms/color_thumb.json`):
//! mounting and unmounting it in each parent, hover, its dragging state (pressing, moving and
//! releasing it), keyboard focus (its focus ring), and its parent disabling it.
use std::time::Duration;

use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/color_thumb";

/// Registers the performance tests of `ColorThumb`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountInAnArea))
        .with(fixture_test(MountAndUnmountInASlider))
        .with(fixture_test(MountAndUnmountInAWheel))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(Drag))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisabledParent))
}
const AREA_THUMB: &str = "#perf-area .leptonic-ColorThumb";
const SLIDER_THUMB: &str = "#perf-slider .leptonic-ColorThumb";
const WHEEL_THUMB: &str = "#perf-wheel .leptonic-ColorThumb";

/// Moves the pressed pointer by (`x`, `y`) in one step: a single `pointermove` (WebDriver's
/// default moves take 250 ms, in a number of steps that varies).
async fn move_pressed_pointer_by(page: &Page<'_>, x: i64, y: i64) -> Result<(), Report> {
    page.low_level()
        .driver()
        .action_chain_with_delay(None, Some(Duration::ZERO))
        .move_by_offset(x, y)
        .perform()
        .await?;
    Ok(())
}

/// Unmounts the thumb `selector` with the control `unmount`, then measures mounting and
/// unmounting it.
async fn mount_and_unmount_with(
    page: &Page<'_>,
    selector: &str,
    mount: &str,
    unmount: &str,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control(unmount).await?;
    page.wait_for_count(selector, 0).await?;
    page.measure("mount", page.control(mount)).await?;
    page.element(selector).await?;
    page.measure("unmount", page.control(unmount)).await?;
    page.wait_for_count(selector, 0).await?;
    Ok(())
}

/// Mounting the thumb of an area (with its two inputs) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_in_an_area(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, AREA_THUMB, "mount-area", "unmount-area").await
}

/// Mounting the thumb of a slider (with its input) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_in_a_slider(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, SLIDER_THUMB, "mount-slider", "unmount-slider").await
}

/// Mounting the thumb of a wheel (with its input) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_in_a_wheel(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, WHEEL_THUMB, "mount-wheel", "unmount-wheel").await
}

/// The pointer entering and leaving the area's thumb.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element(AREA_THUMB).await?;
    let before = page.element("#perf-before").await?;
    page.measure("hover", thumb.hover()).await?;
    thumb.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", before.hover()).await?;
    thumb.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Pressing the area's thumb (dragging), moving it (one pointer move) and releasing it.
#[browser_test]
pub async fn drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element(AREA_THUMB).await?;
    let held = page.measure("press", thumb.press_and_hold()).await?;
    thumb.wait_for_attr("data-dragging", Some("true")).await?;
    page.measure("move", move_pressed_pointer_by(page, -50, 50))
        .await?;
    page.measure("release", held.release()).await?;
    thumb.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// Tab moving focus into the area's thumb (its focus ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element(AREA_THUMB).await?;
    page.element("#perf-before").await?.click().await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    thumb
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    thumb.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    thumb.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// The area disabling its thumb and enabling it again.
#[browser_test]
pub async fn disabled_parent(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element(AREA_THUMB).await?;
    page.measure("disable", page.control("disable")).await?;
    thumb.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("enable", page.control("enable")).await?;
    thumb.wait_for_attr("data-disabled", None).await?;
    Ok(())
}
