// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_color_field_state` (`/perf/hooks/use_color_field_state`, the
//! state of an RGB field alone): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_color_field_state.json`): creating and
//! disposing of it, and its methods (setting the color, setting and committing the text,
//! validating a text, stepping, white and black).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_color_field_state";

/// Registers the performance tests of `use_color_field_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SetTheColor))
        .with(fixture_test(SetAndCommitTheText))
        .with(fixture_test(Validate))
        .with(fixture_test(Step))
        .with(fixture_test(MaxAndMin))
}
const COLOR: &str = "#perf-color";

/// Waits until the committed color is `hex` (`RRGGBB`).
async fn wait_for_color(page: &Page<'_>, hex: &str) -> Result<(), Report> {
    page.element(COLOR).await?.wait_for_inner_text(hex).await
}

/// Creating the state and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(COLOR, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_color(page, "808080").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(COLOR, 0).await?;
    Ok(())
}

/// Setting the color (`set_color_value`; the text follows) and back.
#[browser_test]
pub async fn set_the_color(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set red", page.control("set-red")).await?;
    wait_for_color(page, "FF0000").await?;
    page.element("#perf-text")
        .await?
        .wait_for_inner_text("#FF0000")
        .await?;
    page.measure("set gray", page.control("set-gray")).await?;
    wait_for_color(page, "808080").await?;
    Ok(())
}

/// Setting the text (`set_input_value`) and committing it (`commit`).
#[browser_test]
pub async fn set_and_commit_the_text(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set text", page.control("set-text")).await?;
    page.element("#perf-text")
        .await?
        .wait_for_inner_text("123")
        .await?;
    wait_for_color(page, "808080").await?;
    page.measure("commit", page.control("commit")).await?;
    wait_for_color(page, "112233").await?;
    Ok(())
}

/// Validating a text (`validate`).
#[browser_test]
pub async fn validate(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("validate", page.control("validate")).await?;
    assert_that!(page.control("valid").await?).is_equal_to(1.0);
    Ok(())
}

/// Stepping the color up and down (`increment`, `decrement`).
#[browser_test]
pub async fn step(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("increment", page.control("increment")).await?;
    wait_for_color(page, "808081").await?;
    page.measure("decrement", page.control("decrement")).await?;
    wait_for_color(page, "808080").await?;
    Ok(())
}

/// Setting the color to white and to black (`increment_to_max`, `decrement_to_min`).
#[browser_test]
pub async fn max_and_min(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("white", page.control("to-max")).await?;
    wait_for_color(page, "FFFFFF").await?;
    page.measure("black", page.control("to-min")).await?;
    wait_for_color(page, "000000").await?;
    Ok(())
}
