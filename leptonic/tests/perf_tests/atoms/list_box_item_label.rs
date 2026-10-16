// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ListBoxItemLabel` (`/perf/atoms/list_box_item_label`): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/list_box_item_label.json`): mounting and unmounting it in 10 and in
//! 1,000 options (it labels its option), and a click on it (selecting its option).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/list_box_item_label";

/// Registers the performance tests of `ListBoxItemLabel`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(Click))
}
/// Every label of the fixture.
const LABELS: &str = "[data-perf-label]";
/// The fixture's options after the control `large`.
const LARGE: usize = 1_000;

/// The option "Option `i`".
fn option(i: usize) -> String {
    format!("[data-perf-option='{i}']")
}

/// The label of the option "Option `i`".
fn label(i: usize) -> String {
    format!("[data-perf-label='{i}']")
}

/// Mounts the labels of `count` options, measured, checks that the first one
/// labels its option, and unmounts them, measured.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(LABELS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(LABELS, count).await?;
    let first = page.element(label(0)).await?;
    let id = first.attr("id").await?;
    assert_that!(
        page.element(option(0))
            .await?
            .attr("aria-labelledby")
            .await?
    )
    .is_equal_to(id);
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LABELS, 0).await?;
    Ok(())
}

/// Mounting the labels of 10 options and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_and_unmount_in(page, 10).await
}

/// Mounting the labels of 1,000 options and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("large").await?;
    page.element(label(LARGE - 1)).await?;
    mount_and_unmount_in(page, LARGE).await
}

/// A click on the label of an option, selecting the option.
#[browser_test]
pub async fn click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let third = page.element(label(3)).await?;
    page.measure("click", third.click()).await?;
    page.element(option(3))
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    Ok(())
}
