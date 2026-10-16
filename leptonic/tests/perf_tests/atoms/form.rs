// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Form` (`leptonic/src/atoms/form.rs`, fixture `/perf/atoms/form`):
//! what each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/form.json`): mounting and unmounting it with a required text field,
//! submitting it invalid (the field shows its error and takes focus), resetting it, and the
//! server's validation errors it hands to its fields.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/form";
const FORM: &str = "#perf-form";
const INPUT: &str = "#perf-input";
const ERROR: &str = ".leptonic-FieldError";

/// Registers the performance tests of `Form`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SubmitInvalid))
        .with(fixture_test(ResetAfterASubmission))
        .with(fixture_test(ServerErrors))
}

/// Mounting the form with its field and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(FORM, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let form = page.element(FORM).await?;
    form.wait_for_attr("novalidate", None).await?;
    page.element(INPUT).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FORM, 0).await?;
    Ok(())
}

/// Submitting the form while its required field is empty: the browser's validation fails, the
/// field shows its error and takes focus.
#[browser_test]
pub async fn submit_invalid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let form = page.element(FORM).await?;
    let input = page.element(INPUT).await?;
    page.measure("submit", form.submit()).await?;
    page.element(ERROR).await?;
    input.wait_for_attr("aria-invalid", Some("true")).await?;
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// Resetting the form after an invalid submission: the field's error disappears.
#[browser_test]
pub async fn reset_after_a_submission(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let form = page.element(FORM).await?;
    let input = page.element(INPUT).await?;
    form.submit().await?;
    page.element(ERROR).await?;
    page.measure("reset", form.reset()).await?;
    page.wait_for_count(ERROR, 0).await?;
    input.wait_for_attr("aria-invalid", None).await?;
    Ok(())
}

/// The server's validation errors arriving (the field shows them) and being cleared.
#[browser_test]
pub async fn server_errors(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("set", page.control("set-server-errors"))
        .await?;
    page.element(ERROR)
        .await?
        .wait_for_inner_text("Name taken.")
        .await?;
    input.wait_for_attr("aria-invalid", Some("true")).await?;
    page.measure("clear", page.control("clear-server-errors"))
        .await?;
    page.wait_for_count(ERROR, 0).await?;
    input.wait_for_attr("aria-invalid", None).await?;
    Ok(())
}
