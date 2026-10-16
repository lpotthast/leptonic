// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `VirtualList` (`/perf/atoms/virtual_list`, a long log): mounting it, appending lines
//! while it follows its end or shows its start, and changing its width once it grew. The regular
//! suite counts each action (`Page::count_perf_scenario`, checked against the budgets), the
//! performance suite times them (`crate::perf::report`); both check that the scenario did its
//! work.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;
use serde_json::{Map, Value};

use crate::{
    harness::fixture_test,
    pages::{Page, css},
    ui_tests::Suite,
};

const FIXTURE: &str = "atoms/virtual_list";
const PATH: &str = "/perf/atoms/virtual_list";

/// Registers the performance tests of `VirtualList` (in both suites, see the module
/// documentation).
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(Mount))
        .with(fixture_test(AppendFollowingEnd))
        .with(fixture_test(AppendAtStart))
        .with(fixture_test(ResizeGrown))
}
/// Counts `scenario` (the regular suite) or times it (the performance suite), and checks that
/// every round was measured. Returns its parameters.
async fn run(page: &Page<'_>, scenario: &str, rounds: &str) -> Result<Map<String, Value>, Report> {
    page.goto_path(PATH).await?;
    if crate::ui_tests::suite() == Suite::Performance {
        let results = page.run_perf_scenario(scenario).await?;
        crate::perf::report(FIXTURE, &results)?;
        let rounds = usize::try_from(results.parameter(rounds)?)?;
        for metric in &results.metrics {
            assert_that!((metric.name.as_str(), metric.samples.len()))
                .is_equal_to((metric.name.as_str(), rounds));
        }
        return Ok(results.parameters);
    }
    // The budgets check how often each action was measured.
    page.count_perf_scenario(scenario).await
}

/// The integer parameter `name` of a scenario.
fn parameter(parameters: &Map<String, Value>, name: &str) -> Result<u64, Report> {
    match parameters.get(name).and_then(Value::as_u64) {
        Some(value) => Ok(value),
        None => rootcause::bail!("the scenario has no integer parameter {name:?}"),
    }
}

/// Line `i` of the log.
fn line(i: u64) -> String {
    format!("#perf-vl-log [data-line='{i}']")
}

/// Mounting the list with its lines.
#[browser_test]
pub async fn mount(page: &Page<'_>) -> Result<(), Report> {
    let parameters = run(page, "mount", "mounts").await?;
    page.element(css(line(parameter(&parameters, "rows")? - 1)))
        .await?;
    Ok(())
}

/// Appending lines while the list follows its end: the last line is in view afterwards.
#[browser_test]
pub async fn append_following_end(page: &Page<'_>) -> Result<(), Report> {
    let parameters = run(page, "append-following-end", "rounds").await?;
    let appends = parameter(&parameters, "warmup")? + parameter(&parameters, "rounds")?;
    let last = parameter(&parameters, "rows")? + parameter(&parameters, "batch")? * appends - 1;
    page.element(css(line(last))).await?;
    Ok(())
}

/// Appending lines while the list shows its start: the first line stays in view.
#[browser_test]
pub async fn append_at_start(page: &Page<'_>) -> Result<(), Report> {
    run(page, "append-at-start", "rounds").await?;
    page.element(css(line(0))).await?;
    Ok(())
}

/// Changing the width of a log that grew while following its end: the last line stays in view.
#[browser_test]
pub async fn resize_grown(page: &Page<'_>) -> Result<(), Report> {
    let parameters = run(page, "resize-grown", "rounds").await?;
    page.element(css(line(parameter(&parameters, "rows")? - 1)))
        .await?;
    Ok(())
}
