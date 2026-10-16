// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_calendar` (`/perf/hooks/use_calendar`, a calendar of June
//! 2024 on plain elements: its element, title and previous and next buttons): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_calendar.json`): creating and disposing of it, paging with its
//! buttons (it announces the visible range), a selection (it announces it), and changes of its
//! inputs (label, disabled: both buttons; a maximum: the next button).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_calendar";

/// Registers the performance tests of `use_calendar`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PageWithTheButtons))
        .with(fixture_test(Selection))
        .with(fixture_test(Label))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(Maximum))
}
const CALENDAR: &str = "#perf-calendar";
const TITLE: &str = "#perf-title";
const NEXT: &str = "#perf-calendar button[aria-label=Next]";
const PREVIOUS: &str = "#perf-calendar button[aria-label=Previous]";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// Creating the calendar and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(CALENDAR, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(CALENDAR)
        .await?
        .wait_for_attr("aria-label", Some("Event date, June 2024"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(CALENDAR, 0).await?;
    Ok(())
}

/// The next and previous buttons page by a month; the calendar announces each new month.
#[browser_test]
pub async fn page_with_the_buttons(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let title = page.element(TITLE).await?;
    page.measure("next", page.element(NEXT).await?.click())
        .await?;
    title.wait_for_inner_text("July 2024").await?;
    page.wait_for_count(ANNOUNCER_READY, 2).await?;
    page.measure("previous", page.element(PREVIOUS).await?.click())
        .await?;
    title.wait_for_inner_text("June 2024").await?;
    page.wait_for_count(ANNOUNCER_READY, 3).await?;
    Ok(())
}

/// A date selected through the state: the calendar announces it.
#[browser_test]
pub async fn selection(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("select", page.control("select-june-14"))
        .await?;
    page.wait_for_count(ANNOUNCER_READY, 2).await?;
    Ok(())
}

/// Changing the calendar's label and back.
#[browser_test]
pub async fn label(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let calendar = page.element(CALENDAR).await?;
    page.measure("relabel", page.control("relabel")).await?;
    calendar
        .wait_for_attr("aria-label", Some("Delivery date, June 2024"))
        .await?;
    page.measure("label back", page.control("label-back"))
        .await?;
    calendar
        .wait_for_attr("aria-label", Some("Event date, June 2024"))
        .await?;
    Ok(())
}

/// Disabling the calendar (both buttons) and enabling it.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let next = page.element(NEXT).await?;
    let previous = page.element(PREVIOUS).await?;
    page.measure("disable", page.control("disable")).await?;
    next.wait_for_prop("disabled", "true").await?;
    previous.wait_for_prop("disabled", "true").await?;
    page.measure("enable", page.control("enable")).await?;
    next.wait_for_prop("disabled", "false").await?;
    Ok(())
}

/// A maximum in June (no next page: the next button is disabled) and clearing it.
#[browser_test]
pub async fn maximum(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let next = page.element(NEXT).await?;
    page.measure("set max", page.control("set-max")).await?;
    next.wait_for_prop("disabled", "true").await?;
    page.measure("clear max", page.control("clear-max")).await?;
    next.wait_for_prop("disabled", "false").await?;
    Ok(())
}
