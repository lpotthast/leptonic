// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `OverlayArrow` (`/perf/atoms/overlay_arrow`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/overlay_arrow.json`): mounting it in an open popover (positioned to point
//! at the trigger) and unmounting it, and the popover's placement changing (the arrow moving to the
//! other edge).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/overlay_arrow";
const ARROW: &str = ".leptonic-OverlayArrow";
const POPOVER: &str = ".leptonic-Popover";

/// Registers the performance tests of `OverlayArrow`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PlacementChanges))
}

/// Waits until the popover is placed on `side` of the trigger, done entering.
async fn wait_for_popover(page: &Page<'_>, side: &str) -> Result<(), Report> {
    let popover = page.element(POPOVER).await?;
    popover.wait_for_attr("data-placement", Some(side)).await?;
    popover.wait_for_attr("data-entering", None).await
}

/// Mounting the arrow in the open popover and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_popover(page, "bottom").await?;
    page.control("unmount").await?;
    page.wait_for_count(ARROW, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(ARROW)
        .await?
        .wait_for_attr("data-placement", Some("bottom"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(ARROW, 0).await?;
    Ok(())
}

/// The popover moving above the trigger (the arrow to its bottom edge) and back below it.
#[browser_test]
pub async fn placement_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_popover(page, "bottom").await?;
    let arrow = page.element(ARROW).await?;
    page.measure("place above", page.control("place-above"))
        .await?;
    arrow.wait_for_attr("data-placement", Some("top")).await?;
    page.measure("place below", page.control("place-below"))
        .await?;
    arrow
        .wait_for_attr("data-placement", Some("bottom"))
        .await?;
    Ok(())
}
