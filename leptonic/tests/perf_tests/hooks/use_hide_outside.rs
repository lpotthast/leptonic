// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_hide_outside` (`/perf/hooks/use_hide_outside`, a plain
//! overlay among other elements): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_hide_outside.json`): mounting and
//! unmounting it, closed and open (the page made inert and restored); opening and closing it,
//! with 10 elements outside the overlay and with 1,000; opening and closing it while it only
//! keeps the overlay visible (nothing hidden it: nothing to do); and switching its behavior while
//! open.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_hide_outside";
const OVERLAY: &str = "#perf-overlay";
const FIRST_OUTSIDE: &str = ".perf-outside:first-child";

/// Registers the performance tests of `use_hide_outside`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountOpen))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(OpenAndCloseLarge))
        .with(fixture_test(OpenAndCloseKeepingVisible))
        .with(fixture_test(SwitchBehaviorWhileOpen))
}

/// Waits until the first element outside the overlay is inert (`true`) or not.
async fn wait_for_inert(page: &Page<'_>, inert: bool) -> Result<(), Report> {
    page.element(FIRST_OUTSIDE)
        .await?
        .wait_for_attr("inert", inert.then_some(""))
        .await?;
    Ok(())
}

/// Mounting the closed hook and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(OVERLAY, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(OVERLAY).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(OVERLAY, 0).await?;
    Ok(())
}

/// Mounting the open hook (the page made inert) and unmounting it (restored).
#[browser_test]
pub async fn mount_and_unmount_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(OVERLAY, 0).await?;
    page.control("open").await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_inert(page, true).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(OVERLAY, 0).await?;
    page.element("h1")
        .await?
        .wait_for_attr("inert", None)
        .await?;
    Ok(())
}

/// Opening (the page made inert) and closing (restored).
async fn open_and_close_overlay(page: &Page<'_>) -> Result<(), Report> {
    page.measure("open", page.control("open")).await?;
    wait_for_inert(page, true).await?;
    page.element(OVERLAY)
        .await?
        .wait_for_attr("inert", None)
        .await?;
    page.measure("close", page.control("close")).await?;
    wait_for_inert(page, false).await?;
    Ok(())
}

/// Opening and closing among 10 elements.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(OVERLAY).await?;
    open_and_close_overlay(page).await
}

/// Opening and closing among 1,000 elements.
#[browser_test]
pub async fn open_and_close_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(OVERLAY, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(".perf-outside", 1_000).await?;
    open_and_close_overlay(page).await
}

/// Opening and closing while it only keeps the overlay visible: the page stays as it is.
#[browser_test]
pub async fn open_and_close_keeping_visible(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(OVERLAY).await?;
    page.control("keep-visible").await?;
    page.measure("open", page.control("open")).await?;
    page.element(FIRST_OUTSIDE)
        .await?
        .attr_stays("inert", None, std::time::Duration::from_millis(100))
        .await?;
    page.measure("close", page.control("close")).await?;
    Ok(())
}

/// Switching to keeping the overlay visible while open (the page restored) and back (inert
/// again).
#[browser_test]
pub async fn switch_behavior_while_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(OVERLAY).await?;
    page.control("open").await?;
    wait_for_inert(page, true).await?;
    page.measure("keep visible", page.control("keep-visible"))
        .await?;
    wait_for_inert(page, false).await?;
    page.measure("inert", page.control("inert")).await?;
    wait_for_inert(page, true).await?;
    Ok(())
}
