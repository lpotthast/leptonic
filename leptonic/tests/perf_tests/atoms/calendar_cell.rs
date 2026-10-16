// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `CalendarCell` (`/perf/atoms/calendar_cell`, the cells of a
//! calendar of June 2024): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/calendar_cell.json`): mounting and
//! unmounting the cells, selecting a date by a click, disabling the cells (and a click they
//! ignore) and enabling them, and their dates changing when the calendar shows another month.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/calendar_cell";

/// Registers the performance tests of `CalendarCell`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(DatesChange))
}
const CELLS: &str = "#perf-calendar [role=gridcell]";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// The button of the date labelled `label` ("Friday, June 14, 2024") in its own month's grid.
async fn date(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(format!(
        "{CELLS} > [role=button]:not([data-outside-month])[aria-label^='{label}']"
    ))
    .await
}

/// Mounting the cells of the month's grid (42: six weeks) and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(CELLS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(CELLS, 42).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(CELLS, 0).await?;
    Ok(())
}

/// A click on a cell's date selects it.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    page.measure("click", june_14.click()).await?;
    june_14
        .parent()
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    assert_that!(page.control("value").await?).is_equal_to(20_240_614.0);
    Ok(())
}

/// Disabling the cells, a click they ignore, and enabling them again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    let cell = june_14.parent().await?;
    page.measure("disable", page.control("disable")).await?;
    cell.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("ignored click", june_14.click()).await?;
    assert_that!(page.control("value").await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    cell.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}

/// The calendar moving to July and back: the cells stay, their dates change.
#[browser_test]
pub async fn dates_change(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let heading = page.element("#perf-calendar h2").await?;
    page.measure("show july", page.control("focus-july"))
        .await?;
    heading.wait_for_inner_text("July 2024").await?;
    date(page, "Friday, July 12, 2024").await?;
    page.measure("show june", page.control("focus-june"))
        .await?;
    heading.wait_for_inner_text("June 2024").await?;
    date(page, "Wednesday, June 12, 2024").await?;
    Ok(())
}
