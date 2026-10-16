// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_label` (`leptonic/src/hooks/form/use_label.rs`, fixture
//! `/perf/hooks/use_label`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_label.json`): mounting and unmounting a
//! labelled input, and changes of its reactive props (the visible label, further labels, an
//! `aria-label`), each changing the input's `aria-labelledby`.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_label";
const INPUT: &str = "#perf-input";
const LABEL: &str = "#perf-label";

/// Registers the performance tests of `use_label`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(FurtherLabels))
        .with(fixture_test(VisibleLabel))
        .with(fixture_test(AriaLabel))
}

/// Mounting the label and its input (the label's `for`, the input's `aria-labelledby`) and
/// unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let label = page.element(LABEL).await?;
    label.wait_for_attr("for", Some("perf-input")).await?;
    let input = page.element(INPUT).await?;
    input
        .wait_for_attr("aria-labelledby", Some("perf-label"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// Adding a further label (`aria-labelledby` names it after the visible one) and removing it.
#[browser_test]
pub async fn further_labels(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("add", page.control("add-other-label")).await?;
    input
        .wait_for_attr("aria-labelledby", Some("perf-label perf-other-label"))
        .await?;
    page.measure("remove", page.control("remove-other-label"))
        .await?;
    input
        .wait_for_attr("aria-labelledby", Some("perf-label"))
        .await?;
    Ok(())
}

/// Hiding the visible label (the further label alone names the input) and showing it again.
#[browser_test]
pub async fn visible_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    // Without a visible label, the input needs another label.
    page.control("add-other-label").await?;
    input
        .wait_for_attr("aria-labelledby", Some("perf-label perf-other-label"))
        .await?;
    page.measure("hide", page.control("hide-label")).await?;
    input
        .wait_for_attr("aria-labelledby", Some("perf-other-label"))
        .await?;
    page.measure("show", page.control("show-label")).await?;
    input
        .wait_for_attr("aria-labelledby", Some("perf-label perf-other-label"))
        .await?;
    Ok(())
}

/// Setting an `aria-label` next to the visible label (the input then names itself first) and
/// removing it.
#[browser_test]
pub async fn aria_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("set", page.control("set-aria-label")).await?;
    input.wait_for_attr("aria-label", Some("Full name")).await?;
    input
        .wait_for_attr("aria-labelledby", Some("perf-input perf-label"))
        .await?;
    page.measure("unset", page.control("unset-aria-label"))
        .await?;
    input.wait_for_attr("aria-label", None).await?;
    input
        .wait_for_attr("aria-labelledby", Some("perf-label"))
        .await?;
    Ok(())
}
