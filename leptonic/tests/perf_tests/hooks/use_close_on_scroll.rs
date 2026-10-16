// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_close_on_scroll` (`/perf/hooks/use_close_on_scroll`): what each
//! thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_close_on_scroll.json`): mounting and unmounting it, opening (listening
//! to scrolling) and closing (no longer listening), scrolling the trigger's container closing it,
//! and scrolling elsewhere (ignored).
use std::time::Duration;

use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_close_on_scroll";
const OPEN: &str = "#perf-open";
const TRIGGER: &str = "#perf-trigger";

/// Registers the performance tests of `use_close_on_scroll`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(ScrollingCloses))
        .with(fixture_test(ScrollingElsewhere))
}

/// Mounting the hook (closed) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TRIGGER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TRIGGER).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TRIGGER, 0).await?;
    Ok(())
}

/// Opening (the hook listening to scrolling) and closing (no longer listening).
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let open = page.element(OPEN).await?;
    page.measure("open", page.control("open")).await?;
    open.wait_for_inner_text("true").await?;
    page.measure("close", page.control("close")).await?;
    open.wait_for_inner_text("false").await?;
    Ok(())
}

/// Scrolling the trigger's container while open: closed.
#[browser_test]
pub async fn scrolling_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let open = page.element(OPEN).await?;
    page.control("open").await?;
    open.wait_for_inner_text("true").await?;
    let scroller = page.element("#perf-scroller").await?;
    page.measure("scroll", scroller.scroll_to_top(20.0)).await?;
    open.wait_for_inner_text("false").await?;
    Ok(())
}

/// Scrolling a container not around the trigger while open: ignored.
#[browser_test]
pub async fn scrolling_elsewhere(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let open = page.element(OPEN).await?;
    page.control("open").await?;
    open.wait_for_inner_text("true").await?;
    let other = page.element("#perf-other-scroller").await?;
    page.measure("ignored scroll", other.scroll_to_top(20.0))
        .await?;
    open.inner_text_stays("true", Duration::from_millis(100))
        .await?;
    Ok(())
}
