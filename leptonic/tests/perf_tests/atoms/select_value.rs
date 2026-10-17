// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `SelectValue` (`/perf/atoms/select_value`, in the trigger of a
//! `Select`): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/atoms/select_value.json`): mounting and unmounting it in its trigger, the
//! select's value changing (10 and 1,000 options), and its placeholder changing.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/select_value";

/// Registers the performance tests of `SelectValue`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ValueChange))
        .with(fixture_test(ValueChangeLarge))
        .with(fixture_test(Placeholder))
}
const VALUE: &str = ".leptonic-SelectValue";
/// The fixture's options after the control `large`.
const LARGE: u32 = 1_000;

/// Switches to [`LARGE`] options (unmeasured).
async fn large(page: &Page<'_>) -> Result<(), Report> {
    page.control("large").await?;
    page.settle().await
}

/// Mounting the value in its trigger and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(VALUE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(VALUE)
        .await?
        .wait_for_inner_text("Pick an option")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(VALUE, 0).await?;
    // Mounted again: the trigger refers to it (`aria-labelledby`).
    page.control("mount").await?;
    page.element(VALUE).await?;
    Ok(())
}

/// The select's value changing: the first option, the last of `last + 1`, none.
async fn value_change_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    let shown = page.element(VALUE).await?;
    page.measure("select first", page.control("select-first"))
        .await?;
    shown.wait_for_inner_text("Option 0").await?;
    page.measure("select last", page.control("select-last"))
        .await?;
    shown.wait_for_inner_text(&format!("Option {last}")).await?;
    page.measure("clear", page.control("clear-value")).await?;
    shown
        .wait_for_attr("data-placeholder", Some("true"))
        .await?;
    Ok(())
}

/// The value of a select of 10 options changing: the first option, the last, none.
#[browser_test]
pub async fn value_change(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    value_change_in(page, 9).await
}

/// The value of a select of 1,000 options changing.
#[browser_test]
pub async fn value_change_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    value_change_in(page, LARGE - 1).await
}

/// The placeholder changing while nothing is selected.
#[browser_test]
pub async fn placeholder(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let shown = page.element(VALUE).await?;
    page.measure("rename", page.control("rename-placeholder"))
        .await?;
    shown.wait_for_inner_text("Choose an option").await?;
    Ok(())
}
