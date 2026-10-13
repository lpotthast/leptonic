// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! `VirtualList` performance (`/perf/virtual-list`, a long log): mounting it, appending lines
//! while it follows its end or shows its start, and changing its width once it grew. Each case
//! reports its timings (`crate::perf::report`) and checks that the scenario did its work.
use assertr::prelude::*;
use browser_test::browser_test;
use rootcause::Report;

use crate::pages::{Page, PerfResults, css};

const PATH: &str = "/perf/virtual-list";

/// Runs `scenario`, reports it and checks that every round was measured.
async fn run(page: &Page<'_>, scenario: &str, rounds: &str) -> Result<PerfResults, Report> {
    page.goto_path(PATH).await?;
    let results = page.run_perf_scenario(scenario).await?;
    crate::perf::report(&results)?;
    let rounds = usize::try_from(results.parameter(rounds)?)?;
    for metric in &results.metrics {
        assert_that!((metric.name.as_str(), metric.samples.len()))
            .is_equal_to((metric.name.as_str(), rounds));
    }
    Ok(results)
}

/// Line `i` of the log.
fn line(i: u64) -> String {
    format!("#perf-vl-log [data-line='{i}']")
}

/// Mounting the list with its lines.
#[browser_test]
pub async fn mount(page: &Page<'_>) -> Result<(), Report> {
    let results = run(page, "mount", "mounts").await?;
    assert_that!(results.samples("mount.render")?.len()).is_greater_than(0);
    page.element(css(line(results.parameter("rows")? - 1)))
        .await?;
    Ok(())
}

/// Appending lines while the list follows its end: the last line is in view afterwards.
#[browser_test]
pub async fn append_following_end(page: &Page<'_>) -> Result<(), Report> {
    let results = run(page, "append-following-end", "rounds").await?;
    let appends = results.parameter("warmup")? + results.parameter("rounds")?;
    let last = results.parameter("rows")? + results.parameter("batch")? * appends - 1;
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
    let results = run(page, "resize-grown", "rounds").await?;
    page.element(css(line(results.parameter("rows")? - 1)))
        .await?;
    Ok(())
}
