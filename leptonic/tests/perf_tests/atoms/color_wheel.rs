// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ColorWheel` (`/perf/atoms/color_wheel`, an HSV wheel with its
//! track and thumb): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/atoms/color_wheel.json`): mounting and unmounting it,
//! pressing the track (a hue, then the change's end), dragging the thumb, each key it handles,
//! keyboard focus, and changes of its reactive props (disabled, with a press it ignores, the
//! bound value, the radii, the label).
use std::time::Duration;

use assertr::prelude::*;
use browser_test::{
    TestGroup, browser_test,
    thirtyfour::{TypingData, prelude::*},
};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/color_wheel";

/// Registers the performance tests of `ColorWheel`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressTheTrack))
        .with(fixture_test(DragTheThumb))
        .with(fixture_test(Keys))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ChangeTheValue))
        .with(fixture_test(ChangeTheRadii))
        .with(fixture_test(ChangeTheLabel))
}
const WHEEL: &str = ".leptonic-ColorWheel";
const TRACK: &str = ".leptonic-ColorWheelTrack";
const THUMB: &str = ".leptonic-ColorThumb";
const INPUT: &str = ".leptonic-ColorThumb input";

/// The bound hue.
async fn hue(page: &Page<'_>) -> Result<f64, Report> {
    page.control("hue").await
}

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

/// Mounting the wheel and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(WHEEL, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(THUMB).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(WHEEL, 0).await?;
    Ok(())
}

/// Pressing the bottom of the ring (hue 90) and releasing the pointer (the change's end).
#[browser_test]
pub async fn press_the_track(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let track = page.element(TRACK).await?;
    let held = page
        .measure("press", track.press_and_hold_at(0, 87))
        .await?;
    assert_that!(hue(page).await?).is_equal_to(90.0);
    assert_that!(page.control("changes").await?).is_equal_to(1.0);
    page.measure("release", held.release()).await?;
    assert_that!(page.control("change-ends").await?).is_equal_to(1.0);
    Ok(())
}

/// Pressing the thumb (dragging starts), moving it along the ring (one pointer move) and
/// releasing it.
#[browser_test]
pub async fn drag_the_thumb(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element(THUMB).await?;
    let held = page.measure("press", thumb.press_and_hold()).await?;
    thumb.wait_for_attr("data-dragging", Some("true")).await?;
    page.measure("move", move_pressed_pointer_by(page, 40, 60))
        .await?;
    assert_that!(hue(page).await?).is_not_equal_to(180.0);
    page.measure("release", held.release()).await?;
    thumb.wait_for_attr("data-dragging", None).await?;
    assert_that!(page.control("change-ends").await?).is_equal_to(1.0);
    Ok(())
}

/// Each key the wheel handles, its input focused: the arrows step the hue, with Shift and
/// PageUp/PageDown by a page. Every key changes the hue.
#[browser_test]
pub async fn keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(INPUT).await?).await?;
    let steps: [(&str, TypingData); 8] = [
        ("arrow right", Key::Right.into()),
        ("arrow left", Key::Left.into()),
        ("arrow up", Key::Up.into()),
        ("arrow down", Key::Down.into()),
        ("shift arrow right", Key::Shift + Key::Right),
        ("shift arrow left", Key::Shift + Key::Left),
        ("page up", Key::PageUp.into()),
        ("page down", Key::PageDown.into()),
    ];
    for (name, keys) in steps {
        let before = hue(page).await?;
        page.measure(name, page.send_keys(keys)).await?;
        assert_that!(hue(page).await?).is_not_equal_to(before);
    }
    Ok(())
}

/// Tab moving focus to the wheel's input (a focus ring on the thumb) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element(THUMB).await?;
    page.element("#perf-before").await?.click().await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    thumb
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    thumb.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// Disabling the wheel, a press on its ring it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheel = page.element(WHEEL).await?;
    page.measure("disable", page.control("disable")).await?;
    wheel.wait_for_attr("data-disabled", Some("true")).await?;
    let track = page.element(TRACK).await?;
    page.measure(
        "ignored click",
        page.low_level()
            .driver()
            .action_chain()
            .move_to_element_with_offset(&track, 0, 87)
            .click()
            .perform(),
    )
    .await?;
    assert_that!(hue(page).await?).is_equal_to(180.0);
    assert_that!(page.control("changes").await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    wheel.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// Changing the bound value (the thumb moves around the ring) and back.
#[browser_test]
pub async fn change_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    input.wait_for_prop("value", "180").await?;
    page.measure("set hue 90", page.control("set-hue-90"))
        .await?;
    input.wait_for_prop("value", "90").await?;
    page.measure("set hue 180", page.control("set-hue-180"))
        .await?;
    input.wait_for_prop("value", "180").await?;
    Ok(())
}

/// Growing the wheel's radii and shrinking them again.
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

/// Labeling the wheel (its input) and removing the label again (the channel's name).
#[browser_test]
pub async fn change_the_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("set", page.control("set-label")).await?;
    input.wait_for_attr("aria-label", Some("Tint")).await?;
    page.measure("unset", page.control("unset-label")).await?;
    input.wait_for_attr("aria-label", Some("Hue")).await?;
    Ok(())
}
