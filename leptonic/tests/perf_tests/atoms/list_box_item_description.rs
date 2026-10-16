// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ListBoxItemDescription` (`/perf/atoms/list_box_item_description`): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/list_box_item_description.json`): mounting and unmounting it in 10 and in
//! 1,000 options (it describes its option), and a click on it (selecting its option).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/list_box_item_description";

/// Registers the performance tests of `ListBoxItemDescription`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(Click))
}
/// Every description of the fixture.
const DESCRIPTIONS: &str = "[data-perf-description]";
/// The fixture's options after the control `large`.
const LARGE: usize = 1_000;

/// The option "Option `i`".
fn option(i: usize) -> String {
    format!("[data-perf-option='{i}']")
}

/// The description of the option "Option `i`".
fn description(i: usize) -> String {
    format!("[data-perf-description='{i}']")
}

/// Mounts the descriptions of `count` options, measured, checks that the first one
/// describes its option, and unmounts them, measured.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(DESCRIPTIONS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(DESCRIPTIONS, count).await?;
    let first = page.element(description(0)).await?;
    let id = first.attr("id").await?;
    assert_that!(
        page.element(option(0))
            .await?
            .attr("aria-describedby")
            .await?
    )
    .is_equal_to(id);
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(DESCRIPTIONS, 0).await?;
    Ok(())
}

/// Mounting the descriptions of 10 options and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_and_unmount_in(page, 10).await
}

/// Mounting the descriptions of 1,000 options and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("large").await?;
    page.element(description(LARGE - 1)).await?;
    mount_and_unmount_in(page, LARGE).await
}

/// A click on the description of an option, selecting the option.
#[browser_test]
pub async fn click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let third = page.element(description(3)).await?;
    page.measure("click", third.click()).await?;
    page.element(option(3))
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    Ok(())
}
