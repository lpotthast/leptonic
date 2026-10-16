// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_overlay_position` (`/perf/hooks/use_overlay_position`): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_overlay_position.json`): mounting and unmounting it, opening
//! (positioning the overlay) and closing, its placement and offset changing while open, the
//! target resizing (repositioned in the next frame), an explicit update, and scrolling the target's
//! container closing the overlay.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_overlay_position";
const OVERLAY: &str = ".perf-overlay";
const TARGET: &str = "#perf-target";

/// Registers the performance tests of `use_overlay_position`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(PlacementChanges))
        .with(fixture_test(OffsetChanges))
        .with(fixture_test(TargetResizes))
        .with(fixture_test(Update))
        .with(fixture_test(ScrollingCloses))
}

/// Opens the overlay and waits until it is placed below the target.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.control("open").await?;
    wait_for_placement(page, "bottom").await
}

/// Waits until the overlay is placed on `side` of the target.
async fn wait_for_placement(page: &Page<'_>, side: &str) -> Result<(), Report> {
    page.element(OVERLAY)
        .await?
        .wait_for_attr("data-placement", Some(side))
        .await
}

/// The overlay's distance below the target, and the horizontal distance of their centers.
async fn distances(page: &Page<'_>) -> Result<(f64, f64), Report> {
    let overlay = page.element(OVERLAY).await?.client_rect().await?;
    let target = page.element(TARGET).await?.client_rect().await?;
    let center = |left: f64, width: f64| left + width / 2.0;
    Ok((
        overlay.top - target.bottom,
        center(overlay.left, overlay.width) - center(target.left, target.width),
    ))
}

/// Mounting the hook (closed) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TARGET, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TARGET).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TARGET, 0).await?;
    Ok(())
}

/// Opening the overlay (positioned below the target, centered) and closing it.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("open", page.control("open")).await?;
    wait_for_placement(page, "bottom").await?;
    let (gap, center) = distances(page).await?;
    assert_that!(gap).is_close_to(8.0, 1.0);
    assert_that!(center).is_close_to(0.0, 1.0);
    page.measure("close", page.control("close")).await?;
    page.wait_for_count(OVERLAY, 0).await?;
    Ok(())
}

/// The open overlay moving above the target and back below it (`placement`).
#[browser_test]
pub async fn placement_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    page.measure("place above", page.control("place-above"))
        .await?;
    wait_for_placement(page, "top").await?;
    page.measure("place below", page.control("place-below"))
        .await?;
    wait_for_placement(page, "bottom").await?;
    Ok(())
}

/// The open overlay's distance from the target growing and shrinking (`offset`).
#[browser_test]
pub async fn offset_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    page.measure("offset far", page.control("offset-far"))
        .await?;
    assert_that!(distances(page).await?.0).is_close_to(24.0, 1.0);
    page.measure("offset near", page.control("offset-near"))
        .await?;
    assert_that!(distances(page).await?.0).is_close_to(8.0, 1.0);
    Ok(())
}

/// The target widening and narrowing while the overlay is open: repositioned (centered again) in
/// the next frame.
#[browser_test]
pub async fn target_resizes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    page.measure("widen", page.control("widen-target")).await?;
    assert_that!(distances(page).await?.1).is_close_to(0.0, 1.0);
    page.measure("narrow", page.control("narrow-target"))
        .await?;
    assert_that!(distances(page).await?.1).is_close_to(0.0, 1.0);
    Ok(())
}

/// An explicit update of the open overlay's position (nothing moved).
#[browser_test]
pub async fn update(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    page.measure("update", page.control("update")).await?;
    let (gap, center) = distances(page).await?;
    assert_that!(gap).is_close_to(8.0, 1.0);
    assert_that!(center).is_close_to(0.0, 1.0);
    Ok(())
}

/// Scrolling the target's container closing the open overlay (`on_close`).
#[browser_test]
pub async fn scrolling_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    let scroller = page.element("#perf-scroller").await?;
    page.measure("scroll", scroller.scroll_to_top(20.0)).await?;
    page.wait_for_count(OVERLAY, 0).await?;
    Ok(())
}
