// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_tooltip` (`/perf/hooks/use_tooltip`): what each thing it offers
//! costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_tooltip.json`): mounting and unmounting it, the pointer entering the
//! tooltip (opening its state: hovering keeps a tooltip open) and leaving it (closing it), and
//! hovering while disabled (ignored). Its state has no delays: each step ends with it open or
//! closed, no timer pending (one opening per test: the warm-up's cooldown after a close is a
//! timer).
use std::time::Duration;

use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_tooltip";
const TOOLTIP: &str = "#perf-tooltip";
const OPEN: &str = "#perf-open";

/// Registers the performance tests of `use_tooltip`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(IgnoredWhileDisabled))
}

/// Mounting the tooltip and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TOOLTIP, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TOOLTIP)
        .await?
        .wait_for_attr("role", Some("tooltip"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TOOLTIP, 0).await?;
    Ok(())
}

/// The pointer entering the tooltip (opening its state) and leaving it (closing it).
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let open = page.element(OPEN).await?;
    let tooltip = page.element(TOOLTIP).await?;
    page.measure("hover", tooltip.hover()).await?;
    open.wait_for_inner_text("true").await?;
    page.measure("leave", page.element("#perf-after").await?.hover())
        .await?;
    open.wait_for_inner_text("false").await?;
    Ok(())
}

/// The pointer entering the tooltip while it is disabled: its state stays closed.
#[browser_test]
pub async fn ignored_while_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("disable").await?;
    let tooltip = page.element(TOOLTIP).await?;
    page.measure("ignored hover", tooltip.hover()).await?;
    page.element(OPEN)
        .await?
        .inner_text_stays("false", Duration::from_millis(100))
        .await?;
    Ok(())
}
