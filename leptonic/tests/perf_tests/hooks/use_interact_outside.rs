// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_interact_outside` (`/perf/hooks/use_interact_outside`, on a
//! plain `<div>`): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_interact_outside.json`): mounting and unmounting
//! it, a click outside (pressing starts the interaction, releasing completes it), a click inside
//! (which it ignores), and disabling it (a click outside it ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_interact_outside";

/// Registers the performance tests of `use_interact_outside`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ClickOutside))
        .with(fixture_test(ClickInside))
        .with(fixture_test(DisableAndEnable))
}
const INSIDE: &str = "#perf-inside";

/// The counts of interactions outside started and completed.
async fn counts(page: &Page<'_>) -> Result<(f64, f64), Report> {
    Ok((
        page.control("starts").await?,
        page.control("interactions").await?,
    ))
}

/// Mounting the element with its outside detection and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INSIDE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(INSIDE).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INSIDE, 0).await?;
    Ok(())
}

/// Pressing outside (the start) and releasing there (the interaction).
#[browser_test]
pub async fn click_outside(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let outside = page.element("#perf-outside").await?;
    let held = page.measure("press", outside.press_and_hold()).await?;
    assert_that!(counts(page).await?).is_equal_to((1.0, 0.0));
    page.measure("release", held.release()).await?;
    assert_that!(counts(page).await?).is_equal_to((1.0, 1.0));
    Ok(())
}

/// A click inside, which it ignores.
#[browser_test]
pub async fn click_inside(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let inner = page.element("#perf-inner").await?;
    page.measure("click inside", inner.click()).await?;
    assert_that!(counts(page).await?).is_equal_to((0.0, 0.0));
    Ok(())
}

/// Disabling the detection, a click outside it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let outside = page.element("#perf-outside").await?;
    page.measure("disable", page.control("disable")).await?;
    page.measure("ignored click", outside.click()).await?;
    assert_that!(counts(page).await?).is_equal_to((0.0, 0.0));
    page.measure("enable", page.control("enable")).await?;
    outside.click().await?;
    assert_that!(counts(page).await?).is_equal_to((1.0, 1.0));
    Ok(())
}
