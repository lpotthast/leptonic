// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_item_measurer` (`/perf/hooks/use_item_measurer`): measuring 10
//! or 1,000 items of estimated size (registered by `use_virtualizer_item`), all at once after they
//! rendered, and with `should_observe_item_size` re-measuring those whose content resizes.
//! Measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_item_measurer.json`): mounting (with the first measurements) and
//! unmounting, with and without observing, at both sizes; an item's content growing and
//! shrinking; the items hidden (`display: none`, nothing reported) and shown again.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_item_measurer";
const ITEMS: &str = "#perf-items > div";

/// Registers the performance tests of `use_item_measurer`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(MountObservingSmall))
        .with(fixture_test(MountObservingLarge))
        .with(fixture_test(ContentResize))
        .with(fixture_test(HideAndShow))
}

/// How many sizes the measurer reported.
async fn measurements(page: &Page<'_>) -> Result<f64, Report> {
    page.control("measurements").await
}

/// Mounting `count` items (10 or 1,000; observing their sizes if `observe`) with their first
/// measurements, and unmounting them. Returns how many sizes were reported while mounting.
async fn mount_and_unmount(page: &Page<'_>, count: u32, observe: bool) -> Result<f64, Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(ITEMS, 0).await?;
    if count != 10 {
        page.control("large").await?;
    }
    if observe {
        page.control("observe").await?;
    }
    let before = measurements(page).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ITEMS, usize::try_from(count)?).await?;
    let reported = measurements(page).await? - before;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(ITEMS, 0).await?;
    Ok(reported)
}

/// Mounting and unmounting 10 items: each is measured once.
#[browser_test]
pub async fn mount_and_unmount_small(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(mount_and_unmount(page, 10, false).await?).is_equal_to(10.0);
    Ok(())
}

/// Mounting and unmounting 1,000 items: each is measured once.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(mount_and_unmount(page, 1_000, false).await?).is_equal_to(1_000.0);
    Ok(())
}

/// Mounting and unmounting 10 items observed for resizes: measured once rendered and again when
/// observed.
#[browser_test]
pub async fn mount_observing_small(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(mount_and_unmount(page, 10, true).await?).is_greater_or_equal_to(20.0);
    Ok(())
}

/// Mounting and unmounting 1,000 items observed for resizes.
#[browser_test]
pub async fn mount_observing_large(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(mount_and_unmount(page, 1_000, true).await?).is_greater_or_equal_to(2_000.0);
    Ok(())
}

/// Opens the fixture with 1,000 or 10 items observed for resizes, all measured (once rendered,
/// again when observed). Returns how many sizes were reported by then.
async fn open_observing(page: &Page<'_>, count: u32) -> Result<f64, Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(ITEMS, 0).await?;
    if count != 10 {
        page.control("large").await?;
    }
    page.control("observe").await?;
    page.control("mount").await?;
    page.wait_for_count(ITEMS, usize::try_from(count)?).await?;
    assert_that!(|| measurements(page))
        .eventually_ok()
        .satisfies(|reported| {
            reported.is_greater_or_equal_to(f64::from(2 * count));
        })
        .await;
    page.settle().await?;
    measurements(page).await
}

/// The content of the first of 1,000 observed items growing (40px) and shrinking (25px): it is
/// measured again each time.
#[browser_test]
pub async fn content_resize(page: &Page<'_>) -> Result<(), Report> {
    let before = open_observing(page, 1_000).await?;
    page.measure("grow", page.control("grow-first")).await?;
    assert_that!(measurements(page).await?).is_equal_to(before + 1.0);
    page.measure("shrink", page.control("shrink-first")).await?;
    assert_that!(measurements(page).await?).is_equal_to(before + 2.0);
    Ok(())
}

/// Hiding 10 observed items (`display: none`: their observations report nothing) and showing
/// them again (each measured again).
#[browser_test]
pub async fn hide_and_show(page: &Page<'_>) -> Result<(), Report> {
    let before = open_observing(page, 10).await?;
    page.measure("hide", page.control("hide")).await?;
    assert_that!(measurements(page).await?).is_equal_to(before);
    page.measure("show", page.control("show")).await?;
    assert_that!(measurements(page).await?).is_equal_to(before + 10.0);
    Ok(())
}
