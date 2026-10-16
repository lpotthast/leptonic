// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_enter_animation` (`/perf/hooks/use_enter_animation`, on a
//! plain `<div>`, hidden until ready): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_enter_animation.json`): mounting and unmounting it, the element
//! becoming ready (it shows and awaits its animation), the animation finishing (a script
//! finishes it, so that no timer decides when), and an entry without animations.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_enter_animation";

/// Registers the performance tests of `use_enter_animation`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(EnterWithAnimation))
        .with(fixture_test(EnterWithoutAnimation))
}
const ENTERING: &str = "#perf-enter";

/// Finishes the element's animations.
async fn finish_animations(page: &Page<'_>) -> Result<(), Report> {
    let element = page.element(ENTERING).await?;
    page.low_level()
        .eval(
            "arguments[0].getAnimations().forEach(animation => animation.finish());",
            vec![element.to_json()?],
        )
        .await
}

/// Mounting the element (not ready: hidden) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(ENTERING, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(ENTERING).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(ENTERING, 0).await?;
    Ok(())
}

/// The element becoming ready (shown, entering: it awaits the running animation) and its
/// animation finishing (entered).
#[browser_test]
pub async fn enter_with_animation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let entering = page.element(ENTERING).await?;
    page.measure("ready", page.control("ready")).await?;
    entering
        .wait_for_attr("data-entering", Some("true"))
        .await?;
    page.measure("finish", finish_animations(page)).await?;
    entering.wait_for_attr("data-entering", None).await?;
    Ok(())
}

/// The element becoming ready without an animation: entered at once.
#[browser_test]
pub async fn enter_without_animation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let entering = page.element(ENTERING).await?;
    page.control("unanimate").await?;
    page.measure("ready", page.control("ready")).await?;
    entering.wait_for_attr("data-entering", None).await?;
    Ok(())
}
