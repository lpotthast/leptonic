// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_form_reset` (`leptonic/src/hooks/form/use_form_reset.rs`,
//! fixture `/perf/hooks/use_form_reset`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget (`tests/budgets/hooks/use_form_reset.json`):
//! mounting and unmounting it (it listens to the form), a form reset restoring the initial value,
//! and a reset it ignores (canceled).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_form_reset";
const FORM: &str = "#perf-form";
const INPUT: &str = "#perf-input";

/// Registers the performance tests of `use_form_reset`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Reset))
        .with(fixture_test(IgnoredReset))
}

/// How often the hook restored the value.
async fn resets(page: &Page<'_>) -> Result<f64, Report> {
    page.control("resets").await
}

/// Mounting the input (the hook starts listening to its form) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(INPUT)
        .await?
        .wait_for_prop("value", "Ada")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// A form reset after an edit: the hook restores the initial value.
#[browser_test]
pub async fn reset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    input.type_keys("m").await?;
    input.wait_for_prop("value", "Adam").await?;
    let form = page.element(FORM).await?;
    page.measure("reset", form.reset()).await?;
    input.wait_for_prop("value", "Ada").await?;
    assert_that!(resets(page).await?).is_equal_to(1.0);
    Ok(())
}

/// A reset the form cancels: the hook leaves the value alone.
#[browser_test]
pub async fn ignored_reset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    input.type_keys("m").await?;
    input.wait_for_prop("value", "Adam").await?;
    page.control("prevent-resets").await?;
    let form = page.element(FORM).await?;
    page.measure("ignored reset", form.reset()).await?;
    input.wait_for_prop("value", "Adam").await?;
    assert_that!(resets(page).await?).is_equal_to(0.0);
    Ok(())
}
