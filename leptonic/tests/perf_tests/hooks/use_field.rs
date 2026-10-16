// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_field` (`leptonic/src/hooks/form/use_field.rs`, fixture
//! `/perf/hooks/use_field`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_field.json`): mounting and unmounting a
//! field, its description and error message appearing and disappearing (the input's
//! `aria-describedby` follows them), and a further description (its `aria_describedby` prop).
use browser_test::{TestGroup, browser_test, thirtyfour::WebElement};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_field";
const INPUT: &str = "#perf-input";
const DESCRIPTION: &str = "#perf-description-slot p";
const ERROR: &str = "#perf-error-slot p";

/// Registers the performance tests of `use_field`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Description))
        .with(fixture_test(ErrorMessage))
        .with(fixture_test(FurtherDescription))
}

/// The id of `element`.
async fn id(element: &WebElement) -> Result<String, Report> {
    Ok(element.attr("id").await?.unwrap_or_default())
}

/// Mounting the field (the input described by its description) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let description = id(&page.element(DESCRIPTION).await?).await?;
    page.element(INPUT)
        .await?
        .wait_for_attr("aria-describedby", Some(&description))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// The description disappearing (no longer referenced) and appearing again.
#[browser_test]
pub async fn description(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    let description = id(&page.element(DESCRIPTION).await?).await?;
    input
        .wait_for_attr("aria-describedby", Some(&description))
        .await?;
    page.measure("hide", page.control("hide-description"))
        .await?;
    page.wait_for_count(DESCRIPTION, 0).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    page.measure("show", page.control("show-description"))
        .await?;
    input
        .wait_for_attr("aria-describedby", Some(&description))
        .await?;
    Ok(())
}

/// The error message appearing (referenced after the description) and disappearing.
#[browser_test]
pub async fn error_message(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    let description = id(&page.element(DESCRIPTION).await?).await?;
    page.measure("show", page.control("show-error")).await?;
    let error = id(&page.element(ERROR).await?).await?;
    input
        .wait_for_attr("aria-describedby", Some(&format!("{description} {error}")))
        .await?;
    page.measure("hide", page.control("hide-error")).await?;
    page.wait_for_count(ERROR, 0).await?;
    input
        .wait_for_attr("aria-describedby", Some(&description))
        .await?;
    Ok(())
}

/// Describing the input by a further element (after its own description) and no longer.
#[browser_test]
pub async fn further_description(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    let description = id(&page.element(DESCRIPTION).await?).await?;
    page.measure("add", page.control("add-hint")).await?;
    input
        .wait_for_attr(
            "aria-describedby",
            Some(&format!("{description} perf-hint")),
        )
        .await?;
    page.measure("remove", page.control("remove-hint")).await?;
    input
        .wait_for_attr("aria-describedby", Some(&description))
        .await?;
    Ok(())
}
