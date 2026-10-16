// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_date_range_picker_state`
//! (`/perf/hooks/use_date_range_picker_state`, a date range picker's state of dates and times,
//! rendered as text): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_date_range_picker_state.json`): creating and
//! disposing of it, its methods (opening and closing, selecting a range (closing) and a time,
//! setting an end, setting and clearing the range), and a minimum changing.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_date_range_picker_state";

/// Registers the performance tests of `use_date_range_picker_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(SelectARangeAndATime))
        .with(fixture_test(SetAnEnd))
        .with(fixture_test(SetAndClearTheRange))
        .with(fixture_test(Minimum))
}

/// Waits until the fixture shows `expected` in `#perf-<output>` (`start`, `end`, `open`,
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
    page.wait_for_count("#perf-start", 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for(page, "start", "none").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count("#perf-start", 0).await?;
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

/// Selecting a range while open (the ends take the placeholder's time; it closes), then the
/// start's time.
#[browser_test]
pub async fn select_a_range_and_a_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("open").await?;
    wait_for(page, "open", "true").await?;
    page.measure("select a range", page.control("select-june-10-to-14"))
        .await?;
    wait_for(page, "end", "2024-06-14T00:00:00").await?;
    wait_for(page, "open", "false").await?;
    page.measure("select a time", page.control("select-start-9-30"))
        .await?;
    wait_for(page, "start", "2024-06-10T09:30:00").await?;
    Ok(())
}

/// Setting the end of a range from its field.
#[browser_test]
pub async fn set_an_end(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-value").await?;
    wait_for(page, "end", "2024-06-09T08:00:00").await?;
    page.measure("set the end", page.control("set-end")).await?;
    wait_for(page, "end", "2024-06-20T18:00:00").await?;
    Ok(())
}

/// Setting the range (June 5 to 9, 2024, 8 AM) and clearing it.
#[browser_test]
pub async fn set_and_clear_the_range(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-value")).await?;
    wait_for(page, "start", "2024-06-05T08:00:00").await?;
    page.measure("clear", page.control("clear-value")).await?;
    wait_for(page, "start", "none").await?;
    Ok(())
}

/// A minimum after the range (the state invalid) and clearing it.
#[browser_test]
pub async fn minimum(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-value").await?;
    wait_for(page, "start", "2024-06-05T08:00:00").await?;
    page.measure("set min", page.control("set-min")).await?;
    wait_for(page, "invalid", "true").await?;
    page.measure("clear min", page.control("clear-min")).await?;
    wait_for(page, "invalid", "false").await?;
    Ok(())
}
