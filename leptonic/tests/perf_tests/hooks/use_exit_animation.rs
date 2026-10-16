// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_exit_animation` (`/perf/hooks/use_exit_animation`, on a
//! plain `<div>`): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_exit_animation.json`): mounting and unmounting
//! it, closing (it awaits the animation) and the animation finishing (a script finishes it, so
//! that no timer decides when), opening again during the exit (interrupting it), and closing
//! without an animation.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_exit_animation";

/// Registers the performance tests of `use_exit_animation`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ExitWithAnimation))
        .with(fixture_test(InterruptExit))
        .with(fixture_test(ExitWithoutAnimation))
}
const EXITING: &str = "#perf-exit";

/// Finishes the element's animations.
async fn finish_animations(page: &Page<'_>) -> Result<(), Report> {
    let element = page.element(EXITING).await?;
    page.low_level()
        .eval(
            "arguments[0].getAnimations().forEach(animation => animation.finish());",
            vec![element.to_json()?],
        )
        .await
}

/// Mounting the element (open) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(EXITING, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_attr(EXITING, "data-state", Some("open"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(EXITING, 0).await?;
    Ok(())
}

/// Closing (exiting: it awaits the running animation) and the animation finishing (closed).
#[browser_test]
pub async fn exit_with_animation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let exiting = page.element(EXITING).await?;
    page.measure("close", page.control("close")).await?;
    exiting.wait_for_attr("data-state", Some("exiting")).await?;
    page.measure("finish", finish_animations(page)).await?;
    exiting.wait_for_attr("data-state", Some("closed")).await?;
    Ok(())
}

/// Closing, and opening again while the exit animation runs.
#[browser_test]
pub async fn interrupt_exit(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let exiting = page.element(EXITING).await?;
    page.control("close").await?;
    exiting.wait_for_attr("data-state", Some("exiting")).await?;
    page.measure("open", page.control("open")).await?;
    exiting.wait_for_attr("data-state", Some("open")).await?;
    exiting.wait_for_attr("data-exiting", None).await?;
    Ok(())
}

/// Closing without an animation: closed at once.
#[browser_test]
pub async fn exit_without_animation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let exiting = page.element(EXITING).await?;
    page.control("unanimate").await?;
    page.measure("close", page.control("close")).await?;
    exiting.wait_for_attr("data-state", Some("closed")).await?;
    Ok(())
}
