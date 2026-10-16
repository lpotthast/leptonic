// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Label` (`leptonic/src/atoms/field.rs`, fixture
//! `/perf/atoms/label`): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/atoms/label.json`): mounting and unmounting it in a text
//! field (which then references it) and outside a field, and a click on it, which focuses its
//! field.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, css},
};

const PATH: &str = "/perf/atoms/label";
const INPUT: &str = "#perf-input";

/// Registers the performance tests of `Label`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountOutsideAField))
        .with(fixture_test(ClickFocusesTheField))
}

/// Mounting the label of a text field (the input's `aria-labelledby` then names it, after the
/// input itself, which has an `aria-label`) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.control("unmount").await?;
    page.wait_for_count(css("label").text("Name"), 0).await?;
    input.wait_for_attr("aria-labelledby", None).await?;
    page.measure("mount", page.control("mount")).await?;
    let label = page.element(css("label").text("Name")).await?;
    let label_id = label.attr("id").await?.unwrap_or_default();
    assert_that!(label.attr("for").await?).is_equal_to(Some("perf-input".to_owned()));
    input
        .wait_for_attr("aria-labelledby", Some(&format!("perf-input {label_id}")))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(css("label").text("Name"), 0).await?;
    input.wait_for_attr("aria-labelledby", None).await?;
    Ok(())
}

/// Mounting a plain label outside a field and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_outside_a_field(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("mount", page.control("mount-plain")).await?;
    let label = page.element(css("label").text("Plain")).await?;
    assert_that!(label.attr("for").await?).is_none();
    page.measure("unmount", page.control("unmount-plain"))
        .await?;
    page.wait_for_count(css("label").text("Plain"), 0).await?;
    Ok(())
}

/// A click on the label: the browser focuses its field's input.
#[browser_test]
pub async fn click_focuses_the_field(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = page.element(css("label").text("Name")).await?;
    let input = page.element(INPUT).await?;
    page.measure("click", label.click()).await?;
    page.wait_for_focus(&input).await?;
    input.wait_for_attr("data-focused", Some("true")).await?;
    Ok(())
}
