// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `CalendarGridBody` (`/perf/atoms/calendar_grid_body`, the body of
//! a calendar's grid of June 2024): what each thing it offers costs, measured (`Page::measure`)
//! and checked against its budget (`tests/budgets/atoms/calendar_grid_body.json`): mounting and
//! unmounting it, its week rows following the month when the calendar pages (June's six, July's
//! five), and a fixed number of rows (six) and back.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/calendar_grid_body";

/// Registers the performance tests of `CalendarGridBody`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Paging))
        .with(fixture_test(WeeksInMonth))
}
const BODY: &str = "#perf-calendar tbody";
const ROWS: &str = "#perf-calendar tbody tr";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// Mounting the body (six week rows) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(BODY, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ROWS, 6).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(BODY, 0).await?;
    Ok(())
}

/// The calendar paging from June (six weeks) to July (five).
#[browser_test]
pub async fn paging(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.wait_for_count(ROWS, 6).await?;
    let next = page
        .element("#perf-calendar button[aria-label=Next]")
        .await?;
    page.measure("next", next.click()).await?;
    page.element("#perf-calendar h2")
        .await?
        .wait_for_inner_text("July 2024")
        .await?;
    page.wait_for_count(ROWS, 5).await?;
    Ok(())
}

/// July (five weeks) with six rows, and with its own five again.
#[browser_test]
pub async fn weeks_in_month(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-calendar button[aria-label=Next]")
        .await?
        .click()
        .await?;
    page.wait_for_count(ROWS, 5).await?;
    page.measure("six weeks", page.control("six-weeks")).await?;
    page.wait_for_count(ROWS, 6).await?;
    page.measure("weeks of the month", page.control("weeks-of-month"))
        .await?;
    page.wait_for_count(ROWS, 5).await?;
    Ok(())
}
