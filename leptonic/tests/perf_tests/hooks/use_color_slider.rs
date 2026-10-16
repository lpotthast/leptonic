// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_color_slider` (`/perf/hooks/use_color_slider`, on plain
//! elements, with its state): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_color_slider.json`): mounting and
//! unmounting it, pressing the track (a value, then the change's end), dragging the thumb, each
//! key it handles, keyboard focus, disabling it (with a press it ignores), and changes of its
//! label.
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

const PATH: &str = "/perf/hooks/use_color_slider";

/// Registers the performance tests of `use_color_slider`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressTheTrack))
        .with(fixture_test(DragTheThumb))
        .with(fixture_test(Keys))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ChangeTheLabel))
}
const TRACK: &str = ".perf-track";
const THUMB: &str = ".perf-thumb";
const INPUT: &str = ".perf-thumb input";

/// How often the value changed.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
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
    page.wait_for_count(TRACK, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(THUMB).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TRACK, 0).await?;
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
    assert_that!(changes(page).await?).is_equal_to(1.0);
    page.measure("release", held.release()).await?;
    assert_that!(page.control("change-ends").await?).is_equal_to(1.0);
    Ok(())
}

/// Pressing the thumb, moving it (one pointer move) and releasing it.
#[browser_test]
pub async fn drag_the_thumb(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element(THUMB).await?;
    let held = page.measure("press", thumb.press_and_hold()).await?;
    page.measure("move", move_pressed_pointer_by(page, 50, 0))
        .await?;
    assert_that!(changes(page).await?).is_equal_to(1.0);
    page.measure("release", held.release()).await?;
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
        let before = page.control("value").await?;
        page.measure(name, page.send_keys(keys)).await?;
        assert_that!(page.control("value").await?).is_not_equal_to(before);
    }
    Ok(())
}

/// Tab moving focus to the slider's input and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let input = page.element(INPUT).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&input).await?;
    let after = page.element("#perf-after").await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&after).await?;
    Ok(())
}

/// Disabling the slider (its state), a press on its track it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("disable", page.control("disable")).await?;
    input.wait_for_attr("disabled", Some("true")).await?;
    let track = page.element(TRACK).await?;
    page.measure("ignored click", track.click()).await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    input.wait_for_attr("disabled", None).await?;
    Ok(())
}

/// Labeling the slider (its group) and removing the label again (the channel's name).
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
