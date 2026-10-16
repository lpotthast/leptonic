// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_overlay_focus_contain`
//! (`/perf/hooks/use_overlay_focus_contain`): what it costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_overlay_focus_contain.json`): mounting and
//! unmounting it in an overlay (making the overlay contain the focus; the measured mount includes
//! the overlay's context and its display) and outside any overlay (nothing to do).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_overlay_focus_contain";
const CONTAIN: &str = "#perf-contain";
const OUTSIDE: &str = "#perf-outside";

/// Registers the performance tests of `use_overlay_focus_contain`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountOutsideAnOverlay))
}

/// Mounting the hook in an overlay (which then contains the focus) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(CONTAIN, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(CONTAIN)
        .await?
        .wait_for_inner_text("true")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(CONTAIN, 0).await?;
    Ok(())
}

/// Mounting the hook outside any overlay and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_outside_an_overlay(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount-outside").await?;
    page.wait_for_count(OUTSIDE, 0).await?;
    page.measure("mount", page.control("mount-outside")).await?;
    page.element(OUTSIDE).await?;
    page.measure("unmount", page.control("unmount-outside"))
        .await?;
    page.wait_for_count(OUTSIDE, 0).await?;
    Ok(())
}
