// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ColorSlider` (`/perf/atoms/color_slider`, the red channel of an
//! RGB color, with its output, track and thumb): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget (`tests/budgets/atoms/color_slider.json`):
//! mounting and unmounting it, pressing the track (a value, then the change's end), dragging the
//! thumb, each key it handles, keyboard focus, and changes of its reactive props (disabled, with
//! a press it ignores, the bound value, orientation, the label).
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

const PATH: &str = "/perf/atoms/color_slider";

/// Registers the performance tests of `ColorSlider`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressTheTrack))
        .with(fixture_test(DragTheThumb))
        .with(fixture_test(Keys))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ChangeTheValue))
        .with(fixture_test(ChangeTheOrientation))
        .with(fixture_test(ChangeTheLabel))
}
const SLIDER: &str = ".leptonic-ColorSlider";
const TRACK: &str = ".leptonic-ColorSliderTrack";
const THUMB: &str = ".leptonic-ColorThumb";
const INPUT: &str = ".leptonic-ColorThumb input";
const OUTPUT: &str = ".leptonic-ColorSliderOutput";
/// The bound value at first (`#8000FF`).
const PURPLE: f64 = 8_388_863.0;

/// The bound value (`0xRRGGBB`).
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
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

/// Mounting the slider and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SLIDER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(THUMB).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SLIDER, 0).await?;
    Ok(())
}

/// Pressing the track (the value at that point) and releasing the pointer (the change's end).
#[browser_test]
pub async fn press_the_track(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let track = page.element(TRACK).await?;
    let held = page
        .measure("press", track.press_and_hold_at(-50, 0))
        .await?;
    assert_that!(value(page).await?).is_not_equal_to(PURPLE);
    assert_that!(page.control("changes").await?).is_equal_to(1.0);
    page.measure("release", held.release()).await?;
    assert_that!(page.control("change-ends").await?).is_equal_to(1.0);
    Ok(())
}

/// Pressing the thumb (dragging starts), moving it (one pointer move) and releasing it.
#[browser_test]
pub async fn drag_the_thumb(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element(THUMB).await?;
    let held = page.measure("press", thumb.press_and_hold()).await?;
    thumb.wait_for_attr("data-dragging", Some("true")).await?;
    page.measure("move", move_pressed_pointer_by(page, 50, 0))
        .await?;
    assert_that!(value(page).await?).is_not_equal_to(PURPLE);
    page.measure("release", held.release()).await?;
    thumb.wait_for_attr("data-dragging", None).await?;
    assert_that!(page.control("change-ends").await?).is_equal_to(1.0);
    Ok(())
}

/// Each key the slider handles, its input focused: the arrows step it, PageUp/PageDown by a
/// page, Home/End to its ends. Every key changes the value.
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
        ("page up", Key::PageUp.into()),
        ("page down", Key::PageDown.into()),
        ("end", Key::End.into()),
        ("home", Key::Home.into()),
    ];
    for (name, keys) in steps {
        let before = value(page).await?;
        page.measure(name, page.send_keys(keys)).await?;
        assert_that!(value(page).await?).is_not_equal_to(before);
    }
    Ok(())
}

/// Tab moving focus to the slider's input (a focus ring on the thumb) and on.
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

/// Disabling the slider, a press on its track it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let slider = page.element(SLIDER).await?;
    page.measure("disable", page.control("disable")).await?;
    slider.wait_for_attr("data-disabled", Some("true")).await?;
    let track = page.element(TRACK).await?;
    page.measure("ignored click", track.click()).await?;
    assert_that!(value(page).await?).is_equal_to(PURPLE);
    assert_that!(page.control("changes").await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    slider.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// Changing the bound value (the thumb moves, the output and the input change) and back.
#[browser_test]
pub async fn change_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let output = page.element(OUTPUT).await?;
    output.wait_for_inner_text("128").await?;
    page.measure("set blue", page.control("set-blue")).await?;
    output.wait_for_inner_text("0").await?;
    page.measure("set purple", page.control("set-purple"))
        .await?;
    output.wait_for_inner_text("128").await?;
    Ok(())
}

/// Making the slider vertical and horizontal again.
#[browser_test]
pub async fn change_the_orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let slider = page.element(SLIDER).await?;
    page.measure("vertical", page.control("make-vertical"))
        .await?;
    slider
        .wait_for_attr("data-orientation", Some("vertical"))
        .await?;
    page.measure("horizontal", page.control("make-horizontal"))
        .await?;
    slider
        .wait_for_attr("data-orientation", Some("horizontal"))
        .await?;
    Ok(())
}

/// Labeling the slider (its group) and removing the label again.
#[browser_test]
pub async fn change_the_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let track = page.element(TRACK).await?;
    page.measure("set", page.control("set-label")).await?;
    track.wait_for_attr("aria-label", Some("Redness")).await?;
    page.measure("unset", page.control("unset-label")).await?;
    track.wait_for_attr("aria-label", Some("Red")).await?;
    Ok(())
}
