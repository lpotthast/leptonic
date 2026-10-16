// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_color_channel_field_state`
//! (`/perf/hooks/use_color_channel_field_state`, the state of a hue field alone): what each thing
//! it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_color_channel_field_state.json`): creating and disposing of it, its
//! methods (setting the color, setting and committing the text, stepping, to the ends), and
//! disabling it.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_color_channel_field_state";

/// Registers the performance tests of `use_color_channel_field_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SetTheColor))
        .with(fixture_test(SetAndCommitTheText))
        .with(fixture_test(Step))
        .with(fixture_test(MaxAndMin))
        .with(fixture_test(DisableAndEnable))
}
const HUE: &str = "#perf-hue";

/// Waits until the color's hue is `hue`.
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

/// Setting the color (`set_color_value`; the text follows) and back.
#[browser_test]
pub async fn set_the_color(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set 90", page.control("set-hue-90")).await?;
    wait_for_hue(page, "90").await?;
    page.element("#perf-text")
        .await?
        .wait_for_inner_text("90°")
        .await?;
    page.measure("set 180", page.control("set-hue-180")).await?;
    wait_for_hue(page, "180").await?;
    Ok(())
}

/// Setting the text and committing it.
#[browser_test]
pub async fn set_and_commit_the_text(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set text", page.control("set-text")).await?;
    page.element("#perf-text")
        .await?
        .wait_for_inner_text("45")
        .await?;
    page.measure("commit", page.control("commit")).await?;
    wait_for_hue(page, "45").await?;
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

/// Setting the hue to its ends.
#[browser_test]
pub async fn max_and_min(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("min", page.control("to-min")).await?;
    wait_for_hue(page, "0").await?;
    page.measure("max", page.control("to-max")).await?;
    page.element("#perf-text")
        .await?
        .wait_for_inner_text("360°")
        .await?;
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
