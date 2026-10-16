// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ColorPicker` (`/perf/atoms/color_picker`, sharing its color with
//! a swatch, a hue slider and a hex field): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget (`tests/budgets/atoms/color_picker.json`):
//! mounting and unmounting it with its parts, a part changing the shared color (a slider key, a
//! committed field), and the bound value changing (every part follows).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/color_picker";

/// Registers the performance tests of `ColorPicker`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ChangeWithTheSlider))
        .with(fixture_test(ChangeWithTheField))
        .with(fixture_test(ChangeTheValue))
}
const SWATCH: &str = ".leptonic-ColorSwatch";
const SLIDER_INPUT: &str = ".leptonic-ColorSlider input";
const FIELD_INPUT: &str = ".leptonic-ColorField input";
/// The live announcer's announcements (the field announces its value changes).
const ANNOUNCEMENTS: &str = "[data-live-announcer] [aria-live=assertive] > *";
/// The bound value at first (`#00FFFF`).
const CYAN: f64 = 65_535.0;

/// The bound value (`0xRRGGBB`).
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Mounting the picker with its parts and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SWATCH, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(FIELD_INPUT)
        .await?
        .wait_for_prop("value", "#00FFFF")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SWATCH, 0).await?;
    Ok(())
}

/// The right arrow key on the hue slider: the shared color changes, the field follows.
#[browser_test]
pub async fn change_with_the_slider(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(SLIDER_INPUT).await?)
        .await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    let changed = value(page).await?;
    assert_that!(changed).is_not_equal_to(CYAN);
    assert_that!(page.control("changes").await?).is_equal_to(1.0);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let hex = format!("#{:06X}", changed as u32);
    page.element(FIELD_INPUT)
        .await?
        .wait_for_prop("value", &hex)
        .await?;
    Ok(())
}

/// Enter committing a color typed into the field: the shared color changes.
#[browser_test]
pub async fn change_with_the_field(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(FIELD_INPUT).await?;
    input.click().await?;
    page.wait_for_focus(&input).await?;
    // The field announces its changes: its live announcer's first announcement comes 100 ms
    // late (a timer), later ones at once. A step up and back, unmeasured, creates it.
    page.send_keys(Key::Up).await?;
    page.wait_for_count(ANNOUNCEMENTS, 1).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_count(ANNOUNCEMENTS, 1).await?;
    page.send_keys(page.primary_modifier().await? + "a").await?;
    page.send_keys("f00").await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(value(page).await?).is_equal_to(f64::from(0xFF_00_00));
    Ok(())
}

/// Changing the bound value (every part follows) and back.
#[browser_test]
pub async fn change_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(FIELD_INPUT).await?;
    page.measure("set red", page.control("set-red")).await?;
    input.wait_for_prop("value", "#FF0000").await?;
    page.measure("set cyan", page.control("set-cyan")).await?;
    input.wait_for_prop("value", "#00FFFF").await?;
    Ok(())
}
