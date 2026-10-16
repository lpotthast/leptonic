// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `MenuItemLabel` (`/perf/atoms/menu_item_label`): what it costs,
//! measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/menu_item_label.json`): mounting it (naming its item) and unmounting it.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use leptonic::AriaRole;
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, role},
};

const PATH: &str = "/perf/atoms/menu_item_label";
const SLOT: &str = ".leptonic-MenuItemLabel";

/// Registers the performance tests of `MenuItemLabel`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group.with(fixture_test(MountAndUnmount))
}

/// Mounting the slot (referenced by its item's `aria-labelledby`) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let item = page.element(role(AriaRole::Menuitem)).await?;
    page.control("unmount").await?;
    page.wait_for_count(SLOT, 0).await?;
    let without = item.attr("aria-labelledby").await?.unwrap_or_default();
    page.measure("mount", page.control("mount")).await?;
    let id = page
        .element(SLOT)
        .await?
        .attr("id")
        .await?
        .unwrap_or_default();
    assert_that!(id.is_empty()).is_false();
    let expected = id.clone();
    item.wait_for_attr("aria-labelledby", Some(&expected))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SLOT, 0).await?;
    assert_that!(item.attr("aria-labelledby").await?.unwrap_or_default()).is_equal_to(without);
    Ok(())
}
