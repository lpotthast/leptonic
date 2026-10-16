// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_scroll_wheel` (`/perf/hooks/use_scroll_wheel`, on a plain
//! `<div>`): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_scroll_wheel.json`): mounting and unmounting it, a wheel in
//! pixels and in lines (converted to pixels), Control+wheel (zooming, which it ignores), and
//! disabling it (a wheel it ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{DeltaMode, ElementActions, Modifier, Page, SyntheticEvent},
};

const PATH: &str = "/perf/hooks/use_scroll_wheel";

/// Registers the performance tests of `use_scroll_wheel`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Wheel))
        .with(fixture_test(ZoomWheel))
        .with(fixture_test(DisableAndEnable))
}
const WHEELED: &str = "#perf-wheel";

/// The count of scrolls and the sum of their vertical deltas.
async fn scrolls(page: &Page<'_>) -> Result<(f64, f64), Report> {
    Ok((
        page.control("scrolls").await?,
        page.control("delta-y").await?,
    ))
}

/// Mounting the element with its wheel handling and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(WHEELED, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(WHEELED).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(WHEELED, 0).await?;
    Ok(())
}

/// A wheel of 30 pixels, and one of 2 lines (32 pixels).
#[browser_test]
pub async fn wheel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheeled = page.element(WHEELED).await?;
    let pixels = page
        .measure(
            "pixels",
            wheeled.dispatch(SyntheticEvent::wheel().delta_y(30.0)),
        )
        .await?;
    assert_that!(pixels.default_prevented).is_true();
    page.measure(
        "lines",
        wheeled.dispatch(
            SyntheticEvent::wheel()
                .delta_y(2.0)
                .delta_mode(DeltaMode::Line),
        ),
    )
    .await?;
    assert_that!(scrolls(page).await?).is_equal_to((2.0, 62.0));
    Ok(())
}

/// Control+wheel, which zooms: left alone.
#[browser_test]
pub async fn zoom_wheel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheeled = page.element(WHEELED).await?;
    let zoom = page
        .measure(
            "ctrl+wheel",
            wheeled.dispatch(
                SyntheticEvent::wheel()
                    .delta_y(30.0)
                    .modifiers(&[Modifier::Control]),
            ),
        )
        .await?;
    assert_that!(zoom.default_prevented).is_false();
    assert_that!(scrolls(page).await?).is_equal_to((0.0, 0.0));
    Ok(())
}

/// Disabling the wheel handling, a wheel it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheeled = page.element(WHEELED).await?;
    page.measure("disable", page.control("disable")).await?;
    page.measure(
        "ignored wheel",
        wheeled.dispatch(SyntheticEvent::wheel().delta_y(30.0)),
    )
    .await?;
    assert_that!(scrolls(page).await?).is_equal_to((0.0, 0.0));
    page.measure("enable", page.control("enable")).await?;
    wheeled
        .dispatch(SyntheticEvent::wheel().delta_y(30.0))
        .await?;
    assert_that!(scrolls(page).await?).is_equal_to((1.0, 30.0));
    Ok(())
}
