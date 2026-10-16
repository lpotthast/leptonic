// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `MenuItemDescription` (`/perf/atoms/menu_item_description`): what it
//! costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/menu_item_description.json`): mounting it (describing its item) and
//! unmounting it.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use leptonic::AriaRole;
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, role},
};

const PATH: &str = "/perf/atoms/menu_item_description";
const SLOT: &str = ".leptonic-MenuItemDescription";

/// Registers the performance tests of `MenuItemDescription`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group.with(fixture_test(MountAndUnmount))
}

/// Mounting the slot (referenced by its item's `aria-describedby`) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let item = page.element(role(AriaRole::Menuitem)).await?;
    page.control("unmount").await?;
    page.wait_for_count(SLOT, 0).await?;
    let without = item.attr("aria-describedby").await?.unwrap_or_default();
    page.measure("mount", page.control("mount")).await?;
    let id = page
        .element(SLOT)
        .await?
        .attr("id")
        .await?
        .unwrap_or_default();
    assert_that!(id.is_empty()).is_false();
    let expected = id.clone();
    item.wait_for_attr("aria-describedby", Some(&expected))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SLOT, 0).await?;
    assert_that!(item.attr("aria-describedby").await?.unwrap_or_default()).is_equal_to(without);
    Ok(())
}
