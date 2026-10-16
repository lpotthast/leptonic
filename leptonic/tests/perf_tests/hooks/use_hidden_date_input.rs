// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_hidden_date_input` (`/perf/hooks/use_hidden_date_input`, the
//! hidden autofill input of a date and time field's state): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_hidden_date_input.json`): creating and disposing of it, its value
//! following the field's, a browser's autofill (an `input` event) setting the field's value, and
//! changes of its inputs (granularity: its input type; disabled).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_hidden_date_input";

/// Registers the performance tests of `use_hidden_date_input`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(FollowTheValue))
        .with(fixture_test(Autofill))
        .with(fixture_test(Granularity))
        .with(fixture_test(DisableAndEnable))
}
const INPUT: &str = "#perf-hidden";

/// Creating the input and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(INPUT)
        .await?
        .wait_for_attr("type", Some("datetime-local"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// The field's value set (June 5, 2024, 9:45 AM) and cleared: the input follows.
#[browser_test]
pub async fn follow_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("set", page.control("set-value")).await?;
    input.wait_for_prop("value", "2024-06-05T09:45").await?;
    page.measure("clear", page.control("clear-value")).await?;
    input.wait_for_prop("value", "").await?;
    Ok(())
}

/// A browser's autofill (May 30, 2000, 8:15 AM and its `input` event) sets the field's value.
#[browser_test]
pub async fn autofill(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("autofill", input.virtual_input("2000-05-30T08:15"))
        .await?;
    page.element("#perf-value")
        .await?
        .wait_for_inner_text("2000-05-30T08:15:00")
        .await?;
    Ok(())
}

/// Days only (a date input) and minutes again (a date and time input).
#[browser_test]
pub async fn granularity(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("show days", page.control("show-days")).await?;
    input.wait_for_attr("type", Some("date")).await?;
    page.measure("show minutes", page.control("show-minutes"))
        .await?;
    input.wait_for_attr("type", Some("datetime-local")).await?;
    Ok(())
}

/// Disabling the input with its field and enabling it.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("disable", page.control("disable")).await?;
    input.wait_for_prop("disabled", "true").await?;
    page.measure("enable", page.control("enable")).await?;
    input.wait_for_prop("disabled", "false").await?;
    Ok(())
}
