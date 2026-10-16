// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_color_area_state` (`/perf/hooks/use_color_area_state`, the
//! state of an RGB area alone): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_color_area_state.json`): creating and
//! disposing of it, and each of its methods (setting the value, a channel, the color from a
//! point, stepping the channels, dragging).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_color_area_state";

/// Registers the performance tests of `use_color_area_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SetTheValue))
        .with(fixture_test(SetAChannel))
        .with(fixture_test(SetFromAPoint))
        .with(fixture_test(StepTheChannels))
        .with(fixture_test(Drag))
}
const VALUE: &str = "#perf-value";

/// Waits until the state's value is `hex` (`RRGGBB`).
async fn wait_for_value(page: &Page<'_>, hex: &str) -> Result<(), Report> {
    page.element(VALUE).await?.wait_for_inner_text(hex).await
}

/// Creating the state and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(VALUE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_value(page, "8080FF").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(VALUE, 0).await?;
    Ok(())
}

/// Setting the value (`set_value`) and back.
#[browser_test]
pub async fn set_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set blue", page.control("set-blue")).await?;
    wait_for_value(page, "0000FF").await?;
    page.measure("set back", page.control("set-start")).await?;
    wait_for_value(page, "8080FF").await?;
    Ok(())
}

/// Setting the x and the y channel (`set_x_value`, `set_y_value`).
#[browser_test]
pub async fn set_a_channel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set x", page.control("set-x-64")).await?;
    wait_for_value(page, "4080FF").await?;
    page.measure("set y", page.control("set-y-64")).await?;
    wait_for_value(page, "4040FF").await?;
    Ok(())
}

/// Setting the color from a point of the area (`set_color_from_point`).
#[browser_test]
pub async fn set_from_a_point(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let value = page.element(VALUE).await?;
    page.measure("set from a point", page.control("set-from-point"))
        .await?;
    assert_that!(value.inner_text().await?).is_not_equal_to("8080FF".to_owned());
    Ok(())
}

/// Stepping each channel up and down (`increment_x`, `decrement_x`, `increment_y`,
/// `decrement_y`).
#[browser_test]
pub async fn step_the_channels(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("increment x", page.control("increment-x"))
        .await?;
    wait_for_value(page, "8180FF").await?;
    page.measure("decrement x", page.control("decrement-x"))
        .await?;
    wait_for_value(page, "8080FF").await?;
    page.measure("increment y", page.control("increment-y"))
        .await?;
    wait_for_value(page, "8081FF").await?;
    page.measure("decrement y", page.control("decrement-y"))
        .await?;
    wait_for_value(page, "8080FF").await?;
    Ok(())
}

/// Starting and ending dragging (`set_dragging`; the end calls `on_change_end`).
#[browser_test]
pub async fn drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dragging = page.element("#perf-dragging").await?;
    page.measure("start", page.control("start-dragging"))
        .await?;
    dragging.wait_for_inner_text("true").await?;
    page.measure("end", page.control("end-dragging")).await?;
    dragging.wait_for_inner_text("false").await?;
    assert_that!(page.control("change-ends").await?).is_equal_to(1.0);
    Ok(())
}
