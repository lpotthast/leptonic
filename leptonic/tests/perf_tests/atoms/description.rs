// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Description` (`leptonic/src/atoms/field.rs`, fixture
//! `/perf/atoms/description`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/description.json`): mounting and unmounting
//! it in a text field, whose input references it (`aria-describedby`) while it is rendered.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/description";
const INPUT: &str = "#perf-input";
const DESCRIPTION: &str = ".leptonic-Description";

/// Registers the performance tests of `Description`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group.with(fixture_test(MountAndUnmount))
}

/// Mounting the description (the input's `aria-describedby` then names it) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.control("unmount").await?;
    page.wait_for_count(DESCRIPTION, 0).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    page.measure("mount", page.control("mount")).await?;
    let description = page.element(DESCRIPTION).await?;
    description.wait_for_inner_text("Shown to others.").await?;
    let id = description.attr("id").await?.unwrap_or_default();
    input.wait_for_attr("aria-describedby", Some(&id)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(DESCRIPTION, 0).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}
