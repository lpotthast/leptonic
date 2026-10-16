// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `SliderThumb` (`/perf/atoms/slider_thumb`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/slider_thumb.json`): mounting and unmounting it on the track of a mounted
//! `Slider`, every key it handles (the arrows, Shift+arrows, PageUp/PageDown, Home/End), keyboard
//! focus, dragging it, hover, and changes of its reactive props (disabled, with the press it
//! ignores; required; invalid).
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

const PATH: &str = "/perf/atoms/slider_thumb";

/// Registers the performance tests of `SliderThumb`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ArrowKeys))
        .with(fixture_test(PageKeys))
        .with(fixture_test(HomeAndEnd))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(Drag))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(IgnoredWhileDisabled))
        .with(fixture_test(RequiredAndOptional))
        .with(fixture_test(InvalidAndValid))
}
const THUMB: &str = "#perf-slider-thumb";
const INPUT: &str = "#perf-slider-thumb input";

/// The slider's value.
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// How often a change of the value ended.
async fn change_ends(page: &Page<'_>) -> Result<f64, Report> {
    page.control("change-ends").await
}

/// Focuses the thumb with the keyboard (Tab from the plain button before it).
async fn focus_with_tab(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let input = page.element(INPUT).await?;
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// Measures the key `keys` (named `name`) on the focused thumb, which moves the value to
/// `expected`.
async fn key(page: &Page<'_>, name: &str, keys: TypingData, expected: f64) -> Result<(), Report> {
    page.measure(name, page.send_keys(keys)).await?;
    assert_that!(value(page).await?).is_equal_to(expected);
    Ok(())
}

/// Measures the control `on` (named after it), waits until the thumb's input has the attribute
/// `attr` set to `true`, then measures the control `off` and waits until it is gone.
async fn input_flag_round_trip(
    page: &Page<'_>,
    attr: &str,
    on: &str,
    off: &str,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure(on, page.control(on)).await?;
    page.wait_for_attr(INPUT, attr, Some("true")).await?;
    page.measure(off, page.control(off)).await?;
    page.wait_for_attr(INPUT, attr, None).await?;
    Ok(())
}

/// Mounting the thumb and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(THUMB, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_attr(INPUT, "value", Some("30")).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(THUMB, 0).await?;
    Ok(())
}

/// The arrow keys stepping the value up and down (by 1), each ending a change.
#[browser_test]
pub async fn arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_with_tab(page).await?;
    key(page, "right", Key::Right.into(), 31.0).await?;
    key(page, "left", Key::Left.into(), 30.0).await?;
    key(page, "up", Key::Up.into(), 31.0).await?;
    key(page, "down", Key::Down.into(), 30.0).await?;
    assert_that!(change_ends(page).await?).is_equal_to(4.0);
    Ok(())
}

/// Shift+arrows and PageUp/PageDown stepping the value by a page (a tenth of the range).
#[browser_test]
pub async fn page_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_with_tab(page).await?;
    key(page, "shift+right", Key::Shift + Key::Right, 40.0).await?;
    key(page, "shift+left", Key::Shift + Key::Left, 30.0).await?;
    key(page, "page up", Key::PageUp.into(), 40.0).await?;
    key(page, "page down", Key::PageDown.into(), 30.0).await?;
    Ok(())
}

/// Home and End moving the value to the minimum and the maximum.
#[browser_test]
pub async fn home_and_end(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_with_tab(page).await?;
    key(page, "home", Key::Home.into(), 0.0).await?;
    key(page, "end", Key::End.into(), 100.0).await?;
    Ok(())
}

/// Tab moving focus to the thumb (with a focus ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let thumb = page.element(THUMB).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    thumb
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    thumb.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// Pressing the thumb (focusing it, the dragging state), dragging it 20 pixels right (from 30 to
/// 40), and releasing it (ending the change).
#[browser_test]
pub async fn drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element(THUMB).await?;
    let held = page.measure("press", thumb.press_and_hold()).await?;
    thumb.wait_for_attr("data-dragging", Some("true")).await?;
    page.measure("drag", held.move_by(20, 0)).await?;
    assert_that!(value(page).await?).is_equal_to(40.0);
    page.measure("release", held.release()).await?;
    thumb.wait_for_attr("data-dragging", None).await?;
    assert_that!(change_ends(page).await?).is_equal_to(1.0);
    Ok(())
}

/// The pointer entering and leaving the thumb.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element(THUMB).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", thumb.hover()).await?;
    thumb.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    thumb.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Disabling the thumb and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element(THUMB).await?;
    page.measure("disable", page.control("disable")).await?;
    thumb.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("enable", page.control("enable")).await?;
    thumb.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// A press and a drag on the disabled thumb, which it ignores.
#[browser_test]
pub async fn ignored_while_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let thumb = page.element(THUMB).await?;
    page.control("disable").await?;
    thumb.wait_for_attr("data-disabled", Some("true")).await?;
    let held = page
        .measure("ignored press", thumb.press_and_hold())
        .await?;
    page.measure("ignored drag", held.move_by(20, 0)).await?;
    held.release().await?;
    assert_that!(value(page).await?).is_equal_to(30.0);
    thumb.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// Requiring the thumb's value and making it optional again.
#[browser_test]
pub async fn required_and_optional(page: &Page<'_>) -> Result<(), Report> {
    input_flag_round_trip(page, "aria-required", "require", "make-optional").await
}

/// Marking the thumb invalid and valid again.
#[browser_test]
pub async fn invalid_and_valid(page: &Page<'_>) -> Result<(), Report> {
    input_flag_round_trip(page, "aria-invalid", "invalidate", "validate").await
}
