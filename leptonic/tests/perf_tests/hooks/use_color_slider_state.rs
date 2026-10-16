// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_color_slider_state` (`/perf/hooks/use_color_slider_state`,
//! the state of a red channel slider alone): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_color_slider_state.json`): creating and disposing of it, its methods
//! (setting the value and the channel's value, stepping it, dragging), and changes of its inputs
//! (disabled, orientation).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_color_slider_state";

/// Registers the performance tests of `use_color_slider_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SetTheValue))
        .with(fixture_test(SetTheChannel))
        .with(fixture_test(Step))
        .with(fixture_test(Drag))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ChangeTheOrientation))
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

/// Setting the channel's value (the slider state's `set_thumb_value`).
#[browser_test]
pub async fn set_the_channel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-channel-64")).await?;
    wait_for_value(page, "4080FF").await?;
    page.element("#perf-formatted")
        .await?
        .wait_for_inner_text("64")
        .await?;
    Ok(())
}

/// Stepping the channel up and down.
#[browser_test]
pub async fn step(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("increment", page.control("increment")).await?;
    wait_for_value(page, "8180FF").await?;
    page.measure("decrement", page.control("decrement")).await?;
    wait_for_value(page, "8080FF").await?;
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

/// Disabling the state (a step it ignores) and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable")).await?;
    page.measure("ignored step", page.control("increment"))
        .await?;
    wait_for_value(page, "8080FF").await?;
    page.measure("enable", page.control("enable")).await?;
    page.control("increment").await?;
    wait_for_value(page, "8180FF").await?;
    Ok(())
}

/// Making the state vertical and horizontal again.
#[browser_test]
pub async fn change_the_orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("vertical", page.control("make-vertical"))
        .await?;
    page.measure("horizontal", page.control("make-horizontal"))
        .await?;
    wait_for_value(page, "8080FF").await?;
    Ok(())
}
