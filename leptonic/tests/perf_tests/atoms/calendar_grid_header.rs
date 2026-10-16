// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `CalendarGridHeader` (`/perf/atoms/calendar_grid_header`, the
//! header of a calendar's grid of June 2024): what it offers costs, measured (`Page::measure`)
//! and checked against its budget (`tests/budgets/atoms/calendar_grid_header.json`): mounting and
//! unmounting it with its row of weekday names. It has no state or reactive props of its own.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/calendar_grid_header";

/// Registers the performance tests of `CalendarGridHeader`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group.with(fixture_test(MountAndUnmount))
}
const HEADER: &str = "#perf-calendar thead";

/// Mounting the header (hidden from assistive technology, with seven weekday names) and
/// unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(HEADER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count("#perf-calendar thead th", 7).await?;
    assert_that!(page.element(HEADER).await?.attr("aria-hidden").await?)
        .is_equal_to(Some("true".to_owned()));
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(HEADER, 0).await?;
    Ok(())
}
