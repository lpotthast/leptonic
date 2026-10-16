// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_date_picker_state` (`/perf/hooks/use_date_picker_state`, a
//! date picker's state of a date and time, rendered as text): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_date_picker_state.json`): creating and disposing of it, its methods
//! (opening and closing, selecting a date (closing) and a time, setting and clearing the value),
//! and changes of its reactive inputs (closing on selection, a minimum).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_date_picker_state";

/// Registers the performance tests of `use_date_picker_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(SelectADateAndATime))
        .with(fixture_test(SetAndClearTheValue))
        .with(fixture_test(StayOpenOnSelect))
        .with(fixture_test(Minimum))
}

/// Waits until the fixture shows `expected` in `#perf-<output>` (`value`, `open`, `invalid`).
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
    page.wait_for_count("#perf-value", 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for(page, "value", "none").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count("#perf-value", 0).await?;
    Ok(())
}

/// Opening and closing.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("open", page.control("open")).await?;
    wait_for(page, "open", "true").await?;
    page.measure("close", page.control("close")).await?;
    wait_for(page, "open", "false").await?;
    Ok(())
}

/// Selecting a date while open (it takes the placeholder's time and closes), then a time.
#[browser_test]
pub async fn select_a_date_and_a_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("open").await?;
    wait_for(page, "open", "true").await?;
    page.measure("select a date", page.control("select-june-14"))
        .await?;
    wait_for(page, "value", "2024-06-14T00:00:00").await?;
    wait_for(page, "open", "false").await?;
    page.measure("select a time", page.control("select-9-30"))
        .await?;
    wait_for(page, "value", "2024-06-14T09:30:00").await?;
    Ok(())
}

/// Setting the value (June 5, 2024, 8 AM) and clearing it.
#[browser_test]
pub async fn set_and_clear_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-value")).await?;
    wait_for(page, "value", "2024-06-05T08:00:00").await?;
    page.measure("clear", page.control("clear-value")).await?;
    wait_for(page, "value", "none").await?;
    Ok(())
}

/// Staying open on selection (a date of an empty value waits for its time) and closing again.
#[browser_test]
pub async fn stay_open_on_select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("stay open", page.control("stay-open-on-select"))
        .await?;
    page.control("open").await?;
    page.control("select-june-14").await?;
    wait_for(page, "open", "true").await?;
    wait_for(page, "value", "none").await?;
    page.measure("close on select", page.control("close-on-select"))
        .await?;
    Ok(())
}

/// A minimum after the value (the state invalid) and clearing it.
#[browser_test]
pub async fn minimum(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-value").await?;
    wait_for(page, "value", "2024-06-05T08:00:00").await?;
    page.measure("set min", page.control("set-min")).await?;
    wait_for(page, "invalid", "true").await?;
    page.measure("clear min", page.control("clear-min")).await?;
    wait_for(page, "invalid", "false").await?;
    Ok(())
}
