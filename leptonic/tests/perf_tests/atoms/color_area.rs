// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ColorArea` (`/perf/atoms/color_area`, an RGB area with its
//! `ColorThumb`): what each thing it offers costs, measured (`Page::measure`) and checked against
//! its budget (`tests/budgets/atoms/color_area.json`): mounting and unmounting it, pressing the
//! area (a color, then the change's end), dragging the thumb, each key it handles, keyboard
//! focus, and changes of its reactive props (disabled, with a press it ignores, the bound value,
//! the label).
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

const PATH: &str = "/perf/atoms/color_area";

/// Registers the performance tests of `ColorArea`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressTheArea))
        .with(fixture_test(DragTheThumb))
        .with(fixture_test(Keys))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ChangeTheValue))
        .with(fixture_test(ChangeTheLabel))
}
const AREA: &str = ".leptonic-ColorArea";
const THUMB: &str = ".leptonic-ColorThumb";
const X_INPUT: &str = ".leptonic-ColorThumb input:first-of-type";
/// The bound value at first.
const MAGENTA: f64 = 16_711_935.0;

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

/// Mounting the area and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(AREA, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(THUMB).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(AREA, 0).await?;
    Ok(())
}

/// Pressing the area (the color at that point) and releasing the pointer (the change's end).
#[browser_test]
pub async fn press_the_area(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let area = page.element(AREA).await?;
    let held = page
        .measure("press", area.press_and_hold_at(-50, 50))
        .await?;
    assert_that!(value(page).await?).is_not_equal_to(MAGENTA);
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
    page.measure("move", move_pressed_pointer_by(page, -50, 50))
        .await?;
    assert_that!(value(page).await?).is_not_equal_to(MAGENTA);
    page.measure("release", held.release()).await?;
    thumb.wait_for_attr("data-dragging", None).await?;
    assert_that!(page.control("change-ends").await?).is_equal_to(1.0);
    Ok(())
}

/// Each key the area handles, the x input focused: arrows step a channel, with Shift and
/// PageUp/PageDown/Home/End by a page. Every key changes the value.
#[browser_test]
pub async fn keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-blue").await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(X_INPUT).await?).await?;
    let steps: [(&str, TypingData); 10] = [
        ("arrow right", Key::Right.into()),
        ("arrow left", Key::Left.into()),
        ("arrow up", Key::Up.into()),
        ("arrow down", Key::Down.into()),
        ("shift arrow right", Key::Shift + Key::Right),
        ("shift arrow left", Key::Shift + Key::Left),
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

/// Tab moving focus to the area (its x input; a focus ring on the thumb) and on.
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

/// Disabling the area, a press it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let area = page.element(AREA).await?;
    page.measure("disable", page.control("disable")).await?;
    area.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", area.click()).await?;
    assert_that!(value(page).await?).is_equal_to(MAGENTA);
    assert_that!(page.control("changes").await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    area.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// Changing the bound value (the thumb moves, the inputs' values change) and back.
#[browser_test]
pub async fn change_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let x_input = page.element(X_INPUT).await?;
    x_input.wait_for_prop("value", "255").await?;
    page.measure("set blue", page.control("set-blue")).await?;
    x_input.wait_for_prop("value", "0").await?;
    page.measure("set magenta", page.control("set-magenta"))
        .await?;
    x_input.wait_for_prop("value", "255").await?;
    Ok(())
}

/// Labeling the area and removing the label again.
#[browser_test]
pub async fn change_the_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let area = page.element(AREA).await?;
    page.measure("set", page.control("set-label")).await?;
    area.wait_for_attr("aria-label", Some("Color, Color picker"))
        .await?;
    page.measure("unset", page.control("unset-label")).await?;
    area.wait_for_attr("aria-label", None).await?;
    Ok(())
}
