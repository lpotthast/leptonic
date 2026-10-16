// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_droppable_collection_state`
//! (`/perf/hooks/use_droppable_collection_state`): the drop state of a grid list of 10 or 1,000
//! rows taking drops (the fixture of `use_droppable_collection`). Measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_droppable_collection_state.json`):
//! mounting and unmounting the list, the drop target set through the state (the list itself, a
//! row, between rows, none), and disabling and enabling it.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::Page,
    perf_tests::hooks::use_droppable_collection::{drop_target, mount_and_unmount, open},
};

const PATH: &str = "/perf/hooks/use_droppable_collection_state";

/// Registers the performance tests of `use_droppable_collection_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(SetTheTargetSmall))
        .with(fixture_test(SetTheTargetLarge))
        .with(fixture_test(DisableAndEnable))
}

/// Mounting and unmounting the state of 10 rows.
#[browser_test]
pub async fn mount_and_unmount_small(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, PATH, 10).await
}

/// Mounting and unmounting the state of 1,000 rows.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, PATH, 1_000).await
}

/// The drop target of `count` rows set to the list itself, the first row, between the first two
/// rows, and none.
async fn set_the_target(page: &Page<'_>, count: u32) -> Result<(), Report> {
    open(page, PATH, count).await?;
    page.measure("the list", page.control("target-root"))
        .await?;
    assert_that!(drop_target(page).await?).is_equal_to("root".to_owned());
    page.measure("a row", page.control("target-first")).await?;
    assert_that!(drop_target(page).await?).is_equal_to("row Item 0".to_owned());
    page.measure("between rows", page.control("target-between"))
        .await?;
    assert_that!(drop_target(page).await?).is_equal_to("before-item-1".to_owned());
    page.measure("none", page.control("target-none")).await?;
    assert_that!(drop_target(page).await?).is_equal_to("none".to_owned());
    Ok(())
}

/// Setting the drop target of 10 rows.
#[browser_test]
pub async fn set_the_target_small(page: &Page<'_>) -> Result<(), Report> {
    set_the_target(page, 10).await
}

/// Setting the drop target of 1,000 rows.
#[browser_test]
pub async fn set_the_target_large(page: &Page<'_>) -> Result<(), Report> {
    set_the_target(page, 1_000).await
}

/// Disabling the state of 10 rows and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page, PATH, 10).await?;
    page.measure("disable", page.control("disable")).await?;
    page.measure("enable", page.control("enable")).await?;
    assert_that!(drop_target(page).await?).is_equal_to("none".to_owned());
    Ok(())
}
