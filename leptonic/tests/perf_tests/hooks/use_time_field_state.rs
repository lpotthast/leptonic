// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_time_field_state` (`/perf/hooks/use_time_field_state`, a
//! time field's state rendered as text): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_time_field_state.json`): creating and disposing of it, its field's
//! methods (stepping the hour, setting the day period and a minute, clearing a segment), its value
//! set and cleared by the app, and changes of its reactive inputs (granularity, hour cycle, a
//! minimum, disabled).
use assertr::{matchers::eq, prelude::*};
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_time_field_state";

/// Registers the performance tests of `use_time_field_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(StepTheHour))
        .with(fixture_test(SetSegments))
        .with(fixture_test(SetAndClearTheValue))
        .with(fixture_test(Granularity))
        .with(fixture_test(HourCycle))
        .with(fixture_test(Minimum))
        .with(fixture_test(DisableAndEnable))
}

/// Waits until the fixture shows `expected` in `#perf-<output>` (`value`, `invalid`).
async fn wait_for(page: &Page<'_>, output: &str, expected: &str) -> Result<(), Report> {
    page.element(format!("#perf-{output}"))
        .await?
        .wait_for_inner_text(expected)
        .await
}

/// Waits until the segments' texts are `expected`, without the bidirectional isolation marks
/// around the time (U+2066, U+2069).
async fn wait_for_segments(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    let segments = page.element("#perf-segments").await?;
    assert_that!(|| async {
        segments
            .inner_text()
            .await
            .map(|text| text.replace(['\u{2066}', '\u{2069}'], ""))
    })
    .eventually_ok()
    .try_matches(eq(expected.to_owned()))
    .await?;
    Ok(())
}

/// Creating the state (rendered) and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count("#perf-segments", 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for(page, "value", "none").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count("#perf-segments", 0).await?;
    Ok(())
}

/// Stepping the empty hour (it takes the placeholder's, 12 AM), up again and down.
#[browser_test]
pub async fn step_the_hour(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure(
        "increment from the placeholder",
        page.control("increment-hour"),
    )
    .await?;
    wait_for_segments(page, "12:––\u{202f}AM").await?;
    page.measure("increment", page.control("increment-hour"))
        .await?;
    wait_for_segments(page, "1:––\u{202f}AM").await?;
    page.measure("decrement", page.control("decrement-hour"))
        .await?;
    wait_for_segments(page, "12:––\u{202f}AM").await?;
    Ok(())
}

/// Setting the minute and the day period of 8:30 AM, and clearing the minute.
#[browser_test]
pub async fn set_segments(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-value").await?;
    wait_for(page, "value", "08:30:00").await?;
    page.measure("set the minute", page.control("set-minute"))
        .await?;
    wait_for(page, "value", "08:45:00").await?;
    page.measure("set pm", page.control("set-pm")).await?;
    wait_for(page, "value", "20:45:00").await?;
    page.measure("clear the minute", page.control("clear-minute"))
        .await?;
    wait_for_segments(page, "8:––\u{202f}PM").await?;
    Ok(())
}

/// The app setting the value (8:30 AM) and clearing it.
#[browser_test]
pub async fn set_and_clear_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-value")).await?;
    wait_for_segments(page, "8:30\u{202f}AM").await?;
    page.measure("clear", page.control("clear-value")).await?;
    wait_for(page, "value", "none").await?;
    Ok(())
}

/// Showing seconds (a segment more) and minutes again.
#[browser_test]
pub async fn granularity(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-value").await?;
    wait_for_segments(page, "8:30\u{202f}AM").await?;
    page.measure("show seconds", page.control("show-seconds"))
        .await?;
    wait_for_segments(page, "8:30:00\u{202f}AM").await?;
    page.measure("show minutes", page.control("show-minutes"))
        .await?;
    wait_for_segments(page, "8:30\u{202f}AM").await?;
    Ok(())
}

/// A 24-hour clock and the locale's 12 hours again.
#[browser_test]
pub async fn hour_cycle(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-value").await?;
    wait_for_segments(page, "8:30\u{202f}AM").await?;
    page.measure("use 24 hours", page.control("use-24-hours"))
        .await?;
    wait_for_segments(page, "08:30").await?;
    page.measure("use the locale's hours", page.control("use-locale-hours"))
        .await?;
    wait_for_segments(page, "8:30\u{202f}AM").await?;
    Ok(())
}

/// A minimum after the value (the state invalid) and clearing it.
#[browser_test]
pub async fn minimum(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("set-value").await?;
    wait_for(page, "value", "08:30:00").await?;
    page.measure("set min", page.control("set-min")).await?;
    wait_for(page, "invalid", "true").await?;
    page.measure("clear min", page.control("clear-min")).await?;
    wait_for(page, "invalid", "false").await?;
    Ok(())
}

/// Disabling the state and enabling it.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable")).await?;
    page.measure("enable", page.control("enable")).await?;
    wait_for(page, "value", "none").await?;
    Ok(())
}
