// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_form_validation`
//! (`leptonic/src/hooks/form/use_form_validation.rs`, fixture `/perf/hooks/use_form_validation`):
//! what each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_form_validation.json`), with native validation: mounting and
//! unmounting it, typing (the custom validity follows the value), the `change` event committing
//! the validation, a submission (the invalid field takes focus), a form reset, and changes of
//! the input's constraint attributes.
use assertr::{matchers::eq, prelude::*};
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_form_validation";
const FORM: &str = "#perf-form";
const INPUT: &str = "#perf-input";

/// Registers the performance tests of `use_form_validation`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Typing))
        .with(fixture_test(CommitOnChange))
        .with(fixture_test(SubmitInvalid))
        .with(fixture_test(ResetAfterASubmission))
        .with(fixture_test(ConstraintAttributes))
}

/// Focuses the input with the caret at the end of its value.
async fn focus_at_end(page: &Page<'_>) -> Result<WebElement, Report> {
    let input = page.element(INPUT).await?;
    input.type_keys(Key::End).await?;
    page.wait_for_focus(&input).await?;
    Ok(input)
}

/// Whether the field shows itself invalid (1) or not (0).
async fn invalid(page: &Page<'_>) -> Result<f64, Report> {
    page.control("invalid").await
}

/// Mounting the input (the hook syncs its custom validity and listens to it and its form) and
/// unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let input = page.element(INPUT).await?;
    input.wait_for_attr("title", Some("")).await?;
    assert_that!(input.check_validity().await?).is_true();
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// Typing a digit, which makes the value invalid (the input's custom validity fails, the field
/// shows nothing yet), and deleting it again.
#[browser_test]
pub async fn typing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_at_end(page).await?;
    page.measure("invalidating key", page.send_keys("1"))
        .await?;
    input.wait_for_prop("value", "Ada1").await?;
    // Its validation message, not `checkValidity()`, whose `invalid` event commits.
    input
        .wait_for_prop("validationMessage", "Letters only.")
        .await?;
    assert_that!(invalid(page).await?).is_equal_to(0.0);
    page.measure("validating key", page.send_keys(Key::Backspace))
        .await?;
    input.wait_for_prop("value", "Ada").await?;
    input.wait_for_prop("validationMessage", "").await?;
    Ok(())
}

/// Leaving the input after an invalid edit: its `change` event commits the validation, and the
/// field shows itself invalid.
#[browser_test]
pub async fn commit_on_change(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_at_end(page).await?;
    page.send_keys("1").await?;
    input.wait_for_prop("value", "Ada1").await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    assert_that!(invalid(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Submitting the form with an invalid value: the field shows itself invalid and takes focus.
#[browser_test]
pub async fn submit_invalid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.control("set-invalid").await?;
    input.wait_for_prop("value", "Ada1").await?;
    let form = page.element(FORM).await?;
    page.measure("submit", form.submit()).await?;
    page.wait_for_focus(&input).await?;
    assert_that!(invalid(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Resetting the form after an invalid submission: the field no longer shows itself invalid.
#[browser_test]
pub async fn reset_after_a_submission(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.control("set-invalid").await?;
    input.wait_for_prop("value", "Ada1").await?;
    let form = page.element(FORM).await?;
    form.submit().await?;
    assert_that!(|| invalid(page))
        .eventually_ok()
        .matches(eq(1.0))
        .await;
    page.measure("reset", form.reset()).await?;
    assert_that!(invalid(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Making the input `required` and optional again: the hook observes its constraint attributes
/// and syncs its validity.
#[browser_test]
pub async fn constraint_attributes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("require", page.control("require")).await?;
    input.wait_for_attr("required", Some("true")).await?;
    assert_that!(input.check_validity().await?).is_true();
    page.measure("unrequire", page.control("unrequire")).await?;
    input.wait_for_attr("required", None).await?;
    Ok(())
}
