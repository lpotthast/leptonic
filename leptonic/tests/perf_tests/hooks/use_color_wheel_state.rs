// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_color_wheel_state` (`/perf/hooks/use_color_wheel_state`, the
//! state of an HSV wheel alone): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_color_wheel_state.json`): creating and
//! disposing of it, its methods (setting the value, the hue, the hue from a point, stepping it,
//! dragging), and disabling it.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_color_wheel_state";

/// Registers the performance tests of `use_color_wheel_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SetTheValue))
        .with(fixture_test(SetTheHue))
        .with(fixture_test(SetFromAPoint))
        .with(fixture_test(Step))
        .with(fixture_test(Drag))
        .with(fixture_test(DisableAndEnable))
}
const HUE: &str = "#perf-hue";

/// Waits until the state's hue is `hue`.
async fn wait_for_hue(page: &Page<'_>, hue: &str) -> Result<(), Report> {
    page.element(HUE).await?.wait_for_inner_text(hue).await
}

/// Creating the state and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(HUE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_hue(page, "180").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(HUE, 0).await?;
    Ok(())
}

/// Setting the value (`set_value`) and back.
#[browser_test]
pub async fn set_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set 90", page.control("set-value-90")).await?;
    wait_for_hue(page, "90").await?;
    page.measure("set 180", page.control("set-value-180"))
        .await?;
    wait_for_hue(page, "180").await?;
    Ok(())
}

/// Setting the hue (`set_hue`).
#[browser_test]
pub async fn set_the_hue(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-hue-45")).await?;
    wait_for_hue(page, "45").await?;
    Ok(())
}

/// Setting the hue from a point (`set_hue_from_point`).
#[browser_test]
pub async fn set_from_a_point(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-from-point")).await?;
    wait_for_hue(page, "90").await?;
    Ok(())
}

/// Stepping the hue up and down.
#[browser_test]
pub async fn step(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("increment", page.control("increment")).await?;
    wait_for_hue(page, "181").await?;
    page.measure("decrement", page.control("decrement")).await?;
    wait_for_hue(page, "180").await?;
    Ok(())
}

/// Starting and ending dragging (the end calls `on_change_end`).
#[browser_test]
pub async fn drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("start", page.control("start-dragging"))
        .await?;
    page.measure("end", page.control("end-dragging")).await?;
    assert_that!(page.control("change-ends").await?).is_equal_to(1.0);
    Ok(())
}

/// Disabling the state and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable")).await?;
    page.measure("enable", page.control("enable")).await?;
    wait_for_hue(page, "180").await?;
    Ok(())
}
