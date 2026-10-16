// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `CalendarHeaderCell` (`/perf/atoms/calendar_header_cell`, the
//! weekday names of a calendar's grid of June 2024): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/calendar_header_cell.json`): mounting and unmounting the seven cells,
//! and the cells of a new weekday order (the weeks starting on Monday, and on the locale's Sunday
//! again).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/calendar_header_cell";

/// Registers the performance tests of `CalendarHeaderCell`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(FirstDayOfWeek))
}
const CELLS: &str = "#perf-calendar thead th";
const FIRST: &str = "#perf-calendar thead th:first-child";

/// Mounting the seven header cells and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(CELLS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(CELLS, 7).await?;
    page.element(FIRST).await?.wait_for_inner_text("S").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(CELLS, 0).await?;
    Ok(())
}

/// The cells of the weeks starting on Monday, and on the locale's first day (Sunday) again.
#[browser_test]
pub async fn first_day_of_week(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("start on monday", page.control("start-on-monday"))
        .await?;
    page.element(FIRST).await?.wait_for_inner_text("M").await?;
    page.measure(
        "start on the locale's day",
        page.control("start-on-locale-day"),
    )
    .await?;
    page.element(FIRST).await?.wait_for_inner_text("S").await?;
    Ok(())
}
