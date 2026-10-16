// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `CalendarHeading` (`/perf/atoms/calendar_heading`, the heading of
//! a calendar of June 2024): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/calendar_heading.json`): mounting and
//! unmounting it, and its text following the visible month when the calendar pages (its
//! buttons) and when the app moves the focused date to another year.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/calendar_heading";

/// Registers the performance tests of `CalendarHeading`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Paging))
        .with(fixture_test(AnotherYear))
}
const HEADING: &str = "#perf-calendar h2";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// Mounting the heading and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(HEADING, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(HEADING)
        .await?
        .wait_for_inner_text("June 2024")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(HEADING, 0).await?;
    Ok(())
}

/// The calendar paging forward and back: the heading follows.
#[browser_test]
pub async fn paging(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let heading = page.element(HEADING).await?;
    let next = page
        .element("#perf-calendar button[aria-label=Next]")
        .await?;
    let previous = page
        .element("#perf-calendar button[aria-label=Previous]")
        .await?;
    page.measure("next", next.click()).await?;
    heading.wait_for_inner_text("July 2024").await?;
    page.measure("previous", previous.click()).await?;
    heading.wait_for_inner_text("June 2024").await?;
    Ok(())
}

/// The app moving the focused date to October 2030 and back.
#[browser_test]
pub async fn another_year(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let heading = page.element(HEADING).await?;
    page.measure("focus 2030", page.control("focus-2030"))
        .await?;
    heading.wait_for_inner_text("October 2030").await?;
    page.measure("focus june", page.control("focus-june"))
        .await?;
    heading.wait_for_inner_text("June 2024").await?;
    Ok(())
}
