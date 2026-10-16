// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `FieldError` (`leptonic/src/atoms/field.rs`, fixture
//! `/perf/atoms/field_error`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/field_error.json`): mounting and unmounting
//! it while its text field is invalid, showing and hiding it as the field becomes invalid and
//! valid again, and its message changing with the validation errors.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/field_error";
const INPUT: &str = "#perf-input";
const ERROR: &str = ".leptonic-FieldError";

/// Registers the performance tests of `FieldError`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ShowAndHide))
        .with(fixture_test(ChangeTheMessage))
}

/// Mounting the error of an invalid field (the input's `aria-describedby` then names it) and
/// unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.control("set-short").await?;
    page.element(ERROR).await?;
    page.control("unmount").await?;
    page.wait_for_count(ERROR, 0).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    page.measure("mount", page.control("mount")).await?;
    let error = page.element(ERROR).await?;
    error.wait_for_inner_text("2 more characters.").await?;
    let id = error.attr("id").await?.unwrap_or_default();
    input.wait_for_attr("aria-describedby", Some(&id)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(ERROR, 0).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

/// The field becoming invalid (the error appears) and valid again (it disappears).
#[browser_test]
pub async fn show_and_hide(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("show", page.control("set-short")).await?;
    page.element(ERROR)
        .await?
        .wait_for_inner_text("2 more characters.")
        .await?;
    page.measure("hide", page.control("set-valid")).await?;
    page.wait_for_count(ERROR, 0).await?;
    Ok(())
}

/// The validation errors of the invalid field changing: the error shows the new message.
#[browser_test]
pub async fn change_the_message(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-short").await?;
    let error = page.element(ERROR).await?;
    error.wait_for_inner_text("2 more characters.").await?;
    page.measure("change", page.control("set-empty")).await?;
    error.wait_for_inner_text("3 more characters.").await?;
    Ok(())
}
