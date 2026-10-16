// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_virtualizer_item` (`/perf/hooks/use_virtualizer_item`): 10 or
//! 1,000 items positioned at their layout infos (25px rows) and measured by one measurer while
//! their sizes are estimated. Measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_virtualizer_item.json`): mounting and unmounting the items, new
//! layout infos for all of them (moved by 10px and back), and their sizes becoming estimated
//! (measured) and fixed again, at both sizes.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_virtualizer_item";
const ITEMS: &str = "#perf-items > div";

/// Registers the performance tests of `use_virtualizer_item`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(MoveSmall))
        .with(fixture_test(MoveLarge))
        .with(fixture_test(EstimateAndFixSmall))
        .with(fixture_test(EstimateAndFixLarge))
}

/// Opens the fixture with `count` items (10 or 1,000).
async fn open(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    if count != 10 {
        page.control("unmount").await?;
        page.wait_for_count(ITEMS, 0).await?;
        page.control("large").await?;
        page.control("mount").await?;
    }
    page.wait_for_count(ITEMS, count).await
}

/// The top of the last of `count` items, as its styles place it.
async fn last_top(page: &Page<'_>, count: usize) -> Result<String, Report> {
    page.element(format!("#perf-items > div:nth-child({count})"))
        .await?
        .style_property("top")
        .await
}

/// Mounting and unmounting `count` items of fixed size.
async fn mount_and_unmount(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(ITEMS, 0).await?;
    if count != 10 {
        page.control("large").await?;
    }
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ITEMS, count).await?;
    assert_that!(page.control("measurements").await?).is_equal_to(0.0);
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(ITEMS, 0).await?;
    Ok(())
}

/// Mounting and unmounting 10 items.
#[browser_test]
pub async fn mount_and_unmount_small(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, 10).await
}

/// Mounting and unmounting 1,000 items.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, 1_000).await
}

/// New layout infos for `count` items: all moved down by 10px and back.
async fn move_items(page: &Page<'_>, count: usize) -> Result<(), Report> {
    open(page, count).await?;
    let top = (count - 1) * 25;
    page.measure("move", page.control("move")).await?;
    assert_that!(last_top(page, count).await?).is_equal_to(format!("{}px", top + 10));
    page.measure("move back", page.control("move-back")).await?;
    assert_that!(last_top(page, count).await?).is_equal_to(format!("{top}px"));
    Ok(())
}

/// Moving 10 items.
#[browser_test]
pub async fn move_small(page: &Page<'_>) -> Result<(), Report> {
    move_items(page, 10).await
}

/// Moving 1,000 items.
#[browser_test]
pub async fn move_large(page: &Page<'_>) -> Result<(), Report> {
    move_items(page, 1_000).await
}

/// The sizes of `count` items becoming estimated (each is measured) and fixed again.
async fn estimate_and_fix(page: &Page<'_>, count: usize) -> Result<(), Report> {
    open(page, count).await?;
    page.measure("estimate", page.control("estimate")).await?;
    assert_that!(page.control("measurements").await?).is_equal_to(f64::from(u32::try_from(count)?));
    page.measure("fix", page.control("fix")).await?;
    assert_that!(page.control("measurements").await?).is_equal_to(f64::from(u32::try_from(count)?));
    Ok(())
}

/// Estimating and fixing the sizes of 10 items.
#[browser_test]
pub async fn estimate_and_fix_small(page: &Page<'_>) -> Result<(), Report> {
    estimate_and_fix(page, 10).await
}

/// Estimating and fixing the sizes of 1,000 items.
#[browser_test]
pub async fn estimate_and_fix_large(page: &Page<'_>) -> Result<(), Report> {
    estimate_and_fix(page, 1_000).await
}
