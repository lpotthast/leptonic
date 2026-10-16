// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_calendar_month_picker`
//! (`/perf/hooks/use_calendar_month_picker`, in a calendar of June 2024, its months rendered as
//! buttons): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_calendar_month_picker.json`): mounting and unmounting it,
//! picking a month (the calendar shows it), and its month following the calendar when it pages.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, css},
};

const PATH: &str = "/perf/hooks/use_calendar_month_picker";

/// Registers the performance tests of `use_calendar_month_picker`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PickAMonth))
        .with(fixture_test(Paging))
}
const PICKER: &str = "#perf-months";
const HEADING: &str = "#perf-calendar h2";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// The picker's button of the month `name` ("Sep").
async fn month(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(css("#perf-months button").text(name)).await
}

/// Mounting the picker (twelve months) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(PICKER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count("#perf-months button", 12).await?;
    month(page, "Jun")
        .await?
        .wait_for_attr("aria-pressed", Some("true"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(PICKER, 0).await?;
    Ok(())
}

/// Picking September: the calendar shows it.
#[browser_test]
pub async fn pick_a_month(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let september = month(page, "Sep").await?;
    page.measure("pick", september.click()).await?;
    page.element(HEADING)
        .await?
        .wait_for_inner_text("September 2024")
        .await?;
    september
        .wait_for_attr("aria-pressed", Some("true"))
        .await?;
    Ok(())
}

/// The calendar paging to July: the picker's month follows.
#[browser_test]
pub async fn paging(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let next = page
        .element("#perf-calendar button[aria-label=Next]")
        .await?;
    page.measure("next", next.click()).await?;
    month(page, "Jul")
        .await?
        .wait_for_attr("aria-pressed", Some("true"))
        .await?;
    Ok(())
}
