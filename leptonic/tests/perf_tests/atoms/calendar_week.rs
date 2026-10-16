// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `CalendarWeek` (`/perf/atoms/calendar_week`, the week rows of a
//! calendar of June 2024): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/calendar_week.json`): mounting and
//! unmounting the six weeks with their dates, and their dates changing when the calendar pages
//! (to July; its last row goes).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/calendar_week";

/// Registers the performance tests of `CalendarWeek`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Paging))
}
const ROWS: &str = "#perf-calendar tbody tr";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// The button of the date labelled `label` ("Friday, July 12, 2024") in the grid's month.
async fn date(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(format!(
        "#perf-calendar [role=gridcell] > [role=button]:not([data-outside-month])[aria-label^='{label}']"
    ))
    .await
}

/// Mounting the six weeks (42 dates) and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(ROWS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ROWS, 6).await?;
    page.wait_for_count("#perf-calendar [role=gridcell]", 42)
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(ROWS, 0).await?;
    Ok(())
}

/// The calendar paging to July: the weeks show its dates.
#[browser_test]
pub async fn paging(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let next = page
        .element("#perf-calendar button[aria-label=Next]")
        .await?;
    page.measure("next", next.click()).await?;
    page.wait_for_count(ROWS, 5).await?;
    date(page, "Friday, July 12, 2024").await?;
    Ok(())
}
