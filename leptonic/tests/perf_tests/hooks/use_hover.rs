// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_hover` (`/perf/hooks/use_hover`, on a plain `<div>`): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_hover.json`): mounting and unmounting it, the pointer entering and
//! leaving, and disabling it (a hover it ignores), also while hovered.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_hover";

/// Registers the performance tests of `use_hover`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(DisableWhileHovered))
}
const HOVERED: &str = "#perf-hover";

/// How often a hover started.
async fn hovers(page: &Page<'_>) -> Result<f64, Report> {
    page.control("hovers").await
}

/// Mounting the element with its hover handling and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(HOVERED, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(HOVERED).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(HOVERED, 0).await?;
    Ok(())
}

/// The pointer entering and leaving the element.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let hovered = page.element(HOVERED).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", hovered.hover()).await?;
    hovered.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    hovered.wait_for_attr("data-hovered", None).await?;
    assert_that!(hovers(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Disabling the hover handling, a hover it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let hovered = page.element(HOVERED).await?;
    let after = page.element("#perf-after").await?;
    page.measure("disable", page.control("disable")).await?;
    page.measure("ignored hover", hovered.hover()).await?;
    after.hover().await?;
    assert_that!(hovers(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    hovered.hover().await?;
    hovered.wait_for_attr("data-hovered", Some("true")).await?;
    Ok(())
}

/// Disabling the hover handling while the element is hovered: the hover ends.
#[browser_test]
pub async fn disable_while_hovered(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let hovered = page.element(HOVERED).await?;
    hovered.hover().await?;
    hovered.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("disable", page.control("disable")).await?;
    hovered.wait_for_attr("data-hovered", None).await?;
    Ok(())
}
