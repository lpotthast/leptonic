// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `CalendarYearPicker` (`/perf/atoms/calendar_year_picker`, in a
//! calendar of June 2024, its 20 years rendered as buttons): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/calendar_year_picker.json`): mounting and unmounting it, picking a year
//! (the calendar shows it), and its years following the focused date (to 2040, other years, and
//! back).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, css},
};

const PATH: &str = "/perf/atoms/calendar_year_picker";

/// Registers the performance tests of `CalendarYearPicker`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PickAYear))
        .with(fixture_test(OtherYears))
}
const PICKER: &str = "#perf-years";
const HEADING: &str = "#perf-calendar h2";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// The picker's button of the year `year` ("2030").
async fn year(page: &Page<'_>, year: &str) -> Result<WebElement, Report> {
    page.element(css("#perf-years button").text(year)).await
}

/// Mounting the picker (20 years) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(PICKER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count("#perf-years button", 20).await?;
    year(page, "2024")
        .await?
        .wait_for_attr("aria-pressed", Some("true"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(PICKER, 0).await?;
    Ok(())
}

/// Picking 2030: the calendar shows June 2030.
#[browser_test]
pub async fn pick_a_year(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("pick", year(page, "2030").await?.click())
        .await?;
    page.element(HEADING)
        .await?
        .wait_for_inner_text("June 2030")
        .await?;
    year(page, "2030")
        .await?
        .wait_for_attr("aria-pressed", Some("true"))
        .await?;
    Ok(())
}

/// The app moving the focused date to 2040 (the picker offers the years around it) and back.
#[browser_test]
pub async fn other_years(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("focus 2040", page.control("focus-2040"))
        .await?;
    year(page, "2040")
        .await?
        .wait_for_attr("aria-pressed", Some("true"))
        .await?;
    page.measure("focus 2024", page.control("focus-2024"))
        .await?;
    year(page, "2024")
        .await?
        .wait_for_attr("aria-pressed", Some("true"))
        .await?;
    Ok(())
}
