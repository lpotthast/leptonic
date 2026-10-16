// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_range_calendar` (`/perf/hooks/use_range_calendar`, a range
//! calendar of hooks on June 2024): what each thing it offers costs, measured (`Page::measure`)
//! and checked against its budget (`tests/budgets/hooks/use_range_calendar.json`): creating and
//! disposing of it, committing a range being selected when the focus leaves it and when a
//! pointer is released outside it, paging with its buttons, selecting a range with clicks (it
//! announces it), and changes of its inputs (label, disabled).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_range_calendar";

/// Registers the performance tests of `use_range_calendar`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(CommitOnLeaving))
        .with(fixture_test(CommitOnAPointerOutside))
        .with(fixture_test(PageWithTheButtons))
        .with(fixture_test(SelectWithClicks))
        .with(fixture_test(Label))
        .with(fixture_test(DisableAndEnable))
}
const CALENDAR: &str = "#perf-calendar";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// The button of the date labelled `label` ("Friday, June 14, 2024") in the grid's month. A
/// selected range's first and last dates are labelled with the range first.
async fn date(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    let button = format!("{CALENDAR} [role=gridcell] > [role=button]:not([data-outside-month])");
    page.element(format!(
        "{button}[aria-label^='{label}'], {button}[aria-label*=', {label}']"
    ))
    .await
}

/// The range (`yyyymmdd`, 0 without).
async fn range(page: &Page<'_>) -> Result<(f64, f64), Report> {
    Ok((page.control("start").await?, page.control("end").await?))
}

/// Starts a range at June 12 with the keyboard (Shift+Tab into the grid, Enter); the focus moves
/// on to June 13.
async fn start_a_range(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-after").await?.click().await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&date(page, "Wednesday, June 12, 2024").await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(&date(page, "Thursday, June 13, 2024").await?)
        .await
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
        .wait_for_attr("aria-label", Some("Trip, June 2024"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(CALENDAR, 0).await?;
    Ok(())
}

/// The focus leaving the calendar (Tab) while selecting finishes the range at the focused date.
#[browser_test]
pub async fn commit_on_leaving(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    start_a_range(page).await?;
    page.measure("leave", page.send_keys(Key::Tab)).await?;
    assert_that!(range(page).await?).is_equal_to((20_240_612.0, 20_240_613.0));
    Ok(())
}

/// A click outside the calendar while selecting finishes the range at the focused date.
#[browser_test]
pub async fn commit_on_a_pointer_outside(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    start_a_range(page).await?;
    let before = page.element("#perf-before").await?;
    page.measure("click outside", before.click()).await?;
    assert_that!(range(page).await?).is_equal_to((20_240_612.0, 20_240_613.0));
    Ok(())
}

/// The next and previous buttons page by a month.
#[browser_test]
pub async fn page_with_the_buttons(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let title = page.element("#perf-title").await?;
    let next = page
        .element("#perf-calendar button[aria-label=Next]")
        .await?;
    let previous = page
        .element("#perf-calendar button[aria-label=Previous]")
        .await?;
    page.measure("next", next.click()).await?;
    title.wait_for_inner_text("July 2024").await?;
    page.measure("previous", previous.click()).await?;
    title.wait_for_inner_text("June 2024").await?;
    Ok(())
}

/// A range by clicks: the first date and the last.
#[browser_test]
pub async fn select_with_clicks(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let june_10 = date(page, "Monday, June 10, 2024").await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    page.measure("click start", june_10.click()).await?;
    june_10.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("click end", june_14.click()).await?;
    assert_that!(range(page).await?).is_equal_to((20_240_610.0, 20_240_614.0));
    Ok(())
}

/// Changing the calendar's label and back.
#[browser_test]
pub async fn label(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let calendar = page.element(CALENDAR).await?;
    page.measure("relabel", page.control("relabel")).await?;
    calendar
        .wait_for_attr("aria-label", Some("Holiday, June 2024"))
        .await?;
    page.measure("label back", page.control("label-back"))
        .await?;
    calendar
        .wait_for_attr("aria-label", Some("Trip, June 2024"))
        .await?;
    Ok(())
}

/// Disabling the calendar (its buttons and dates) and enabling it.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let next = page
        .element("#perf-calendar button[aria-label=Next]")
        .await?;
    page.measure("disable", page.control("disable")).await?;
    next.wait_for_prop("disabled", "true").await?;
    page.measure("enable", page.control("enable")).await?;
    next.wait_for_prop("disabled", "false").await?;
    Ok(())
}
