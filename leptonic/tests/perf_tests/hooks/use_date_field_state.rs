// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_date_field_state` (`/perf/hooks/use_date_field_state`, a
//! date field's state rendered as text): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_date_field_state.json`): creating and disposing of it, its methods
//! (stepping a segment by one and by a page, to its ends, setting and clearing a segment,
//! confirming the shown date, setting and clearing the value), and changes of its reactive
//! inputs (disabled, a minimum, leading zeros, the placeholder).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_date_field_state";

/// Registers the performance tests of `use_date_field_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(StepASegment))
        .with(fixture_test(StepByPages))
        .with(fixture_test(StepToTheEnds))
        .with(fixture_test(SetAndClearASegment))
        .with(fixture_test(ConfirmAnIncompleteDate))
        .with(fixture_test(SetAndClearTheValue))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(Minimum))
        .with(fixture_test(LeadingZeros))
        .with(fixture_test(Placeholder))
}

/// Waits until the fixture shows `expected` in `#perf-<output>` (`segments`, `value`,
/// `invalid`).
async fn wait_for(page: &Page<'_>, output: &str, expected: &str) -> Result<(), Report> {
    page.element(format!("#perf-{output}"))
        .await?
        .wait_for_inner_text(expected)
        .await
}

/// Creating the state (rendered) and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count("#perf-segments", 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for(page, "segments", "mm/dd/yyyy").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count("#perf-segments", 0).await?;
    Ok(())
}

/// Stepping the empty month (it takes the placeholder's), up again and down.
#[browser_test]
pub async fn step_a_segment(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure(
        "increment from the placeholder",
        page.control("increment-month"),
    )
    .await?;
    wait_for(page, "segments", "6/dd/yyyy").await?;
    page.measure("increment", page.control("increment-month"))
        .await?;
    wait_for(page, "segments", "7/dd/yyyy").await?;
    page.measure("decrement", page.control("decrement-month"))
        .await?;
    wait_for(page, "segments", "6/dd/yyyy").await?;
    Ok(())
}

/// Stepping the day of a date by a page (7) up and down.
#[browser_test]
pub async fn step_by_pages(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-value").await?;
    wait_for(page, "value", "2024-06-05").await?;
    page.measure("page up", page.control("increment-day-page"))
        .await?;
    wait_for(page, "value", "2024-06-12").await?;
    page.measure("page down", page.control("decrement-day-page"))
        .await?;
    wait_for(page, "value", "2024-06-05").await?;
    Ok(())
}

/// Setting the day of a date to its maximum (31) and its minimum.
#[browser_test]
pub async fn step_to_the_ends(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-value").await?;
    wait_for(page, "value", "2024-06-05").await?;
    page.measure("to max", page.control("day-to-max")).await?;
    // The maximum of any month (react-aria): June 31 isn't a date, the value stays.
    wait_for(page, "segments", "6/31/2024").await?;
    page.measure("to min", page.control("day-to-min")).await?;
    wait_for(page, "value", "2024-06-01").await?;
    Ok(())
}

/// Setting the year of the empty field and clearing it.
#[browser_test]
pub async fn set_and_clear_a_segment(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-year")).await?;
    wait_for(page, "segments", "mm/dd/2030").await?;
    page.measure("clear", page.control("clear-year")).await?;
    wait_for(page, "segments", "mm/dd/yyyy").await?;
    Ok(())
}

/// Confirming the shown date of a field missing segments: nothing to commit.
#[browser_test]
pub async fn confirm_an_incomplete_date(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-year").await?;
    wait_for(page, "segments", "mm/dd/2030").await?;
    page.measure("confirm", page.control("confirm")).await?;
    wait_for(page, "value", "none").await?;
    Ok(())
}

/// Setting the value (June 5, 2024) and clearing it.
#[browser_test]
pub async fn set_and_clear_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-value")).await?;
    wait_for(page, "segments", "6/5/2024").await?;
    page.measure("clear", page.control("clear-value")).await?;
    wait_for(page, "segments", "mm/dd/yyyy").await?;
    Ok(())
}

/// Disabling the state and enabling it.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable")).await?;
    page.measure("enable", page.control("enable")).await?;
    wait_for(page, "segments", "mm/dd/yyyy").await?;
    Ok(())
}

/// A minimum after the value (the state invalid) and clearing it.
#[browser_test]
pub async fn minimum(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-value").await?;
    wait_for(page, "value", "2024-06-05").await?;
    page.measure("set min", page.control("set-min")).await?;
    wait_for(page, "invalid", "true").await?;
    page.measure("clear min", page.control("clear-min")).await?;
    wait_for(page, "invalid", "false").await?;
    Ok(())
}

/// Forcing leading zeros (06/05/2024) and not.
#[browser_test]
pub async fn leading_zeros(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-value").await?;
    wait_for(page, "segments", "6/5/2024").await?;
    page.measure("force", page.control("force-leading-zeros"))
        .await?;
    wait_for(page, "segments", "06/05/2024").await?;
    page.measure("unforce", page.control("no-leading-zeros"))
        .await?;
    wait_for(page, "segments", "6/5/2024").await?;
    Ok(())
}

/// Moving the placeholder (January 20, 2030) and back: stepping the empty month starts there.
#[browser_test]
pub async fn placeholder(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("move", page.control("move-placeholder"))
        .await?;
    page.control("increment-month").await?;
    wait_for(page, "segments", "1/dd/yyyy").await?;
    page.measure("back", page.control("placeholder-back"))
        .await?;
    wait_for(page, "segments", "1/dd/yyyy").await?;
    Ok(())
}
