// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_slider` (`/perf/hooks/use_slider`, on plain elements with its
//! state and a thumb): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_slider.json`): mounting and unmounting it, a click
//! on its track moving the thumb there, a press on the track dragging the thumb along, a click on
//! its label (focusing the thumb), and the click on the track it ignores while disabled. The
//! thumb's own costs are measured in `use_slider_thumb.rs`.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_slider";

/// Registers the performance tests of `use_slider`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ClickOnTheTrack))
        .with(fixture_test(DragOnTheTrack))
        .with(fixture_test(LabelClick))
        .with(fixture_test(IgnoredWhileDisabled))
}
const SLIDER: &str = "#perf-slider";
const TRACK: &str = "#perf-slider-track";
const OUTPUT: &str = "#perf-slider-output";
const INPUT: &str = "#perf-slider-track input";

/// The slider's value.
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// How often a change of the value ended.
async fn change_ends(page: &Page<'_>) -> Result<f64, Report> {
    page.control("change-ends").await
}

/// Clicks the 200 pixel track `x` pixels right of its center.
async fn click_track_at(page: &Page<'_>, x: i64) -> Result<(), Report> {
    let track = page.element(TRACK).await?;
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_with_offset(&track, x, 0)
        .click()
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
    page.element(OUTPUT)
        .await?
        .wait_for_inner_text("30")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SLIDER, 0).await?;
    Ok(())
}

/// A click at three quarters of the track, moving the thumb there (from 30 to 75) and focusing
/// it.
#[browser_test]
pub async fn click_on_the_track(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("click", click_track_at(page, 50)).await?;
    page.element(OUTPUT)
        .await?
        .wait_for_inner_text("75")
        .await?;
    assert_that!(change_ends(page).await?).is_equal_to(1.0);
    let input = page.element(INPUT).await?;
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// A press at a fifth of the track (away from the thumb at 30; moving it to 20), dragging it 50
/// pixels on (to 45), and the release (ending the change).
#[browser_test]
pub async fn drag_on_the_track(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let track = page.element(TRACK).await?;
    let held = page
        .measure("press", track.press_and_hold_at(-60, 0))
        .await?;
    assert_that!(value(page).await?).is_equal_to(20.0);
    page.measure("drag", held.move_by(50, 0)).await?;
    assert_that!(value(page).await?).is_equal_to(45.0);
    assert_that!(change_ends(page).await?).is_equal_to(0.0);
    page.measure("release", held.release()).await?;
    assert_that!(change_ends(page).await?).is_equal_to(1.0);
    Ok(())
}

/// A click on the label, focusing the thumb.
#[browser_test]
pub async fn label_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = page.element("#perf-slider > span").await?;
    page.measure("click", label.click()).await?;
    let input = page.element(INPUT).await?;
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// A click on the track of the disabled slider, which it ignores.
#[browser_test]
pub async fn ignored_while_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("disable").await?;
    page.wait_for_attr(INPUT, "tabindex", None).await?;
    page.measure("ignored click", click_track_at(page, 50))
        .await?;
    assert_that!(value(page).await?).is_equal_to(30.0);
    assert_that!(change_ends(page).await?).is_equal_to(0.0);
    Ok(())
}
