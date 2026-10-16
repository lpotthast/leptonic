// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_calendar_grid` (`/perf/hooks/use_calendar_grid`, the grid of
//! a calendar of hooks on June 2024): what each thing it offers costs, measured (`Page::measure`)
//! and checked against its budget (`tests/budgets/hooks/use_calendar_grid.json`): creating and
//! disposing of it, the keys it handles (arrows, Page Up/Down with and without Shift, Home/End),
//! the focus entering and leaving it, and its month changing when the calendar pages.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_calendar_grid";

/// Registers the performance tests of `use_calendar_grid`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MoveWithTheArrowKeys))
        .with(fixture_test(PageWithTheKeyboard))
        .with(fixture_test(MoveToTheMonthEnds))
        .with(fixture_test(FocusAndBlur))
        .with(fixture_test(Paging))
}
const GRID: &str = "#perf-calendar [role=grid]";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// The button of the date labelled `label` ("Friday, June 14, 2024") in the grid's month.
async fn date(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(format!(
        "{GRID} [role=gridcell] > [role=button]:not([data-outside-month])[aria-label^='{label}']"
    ))
    .await
}

/// Focuses the focused date with the keyboard (Shift+Tab from the plain button after the
/// calendar) and returns its button.
async fn focus_the_grid(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-after").await?.click().await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    let june_12 = date(page, "Wednesday, June 12, 2024").await?;
    page.wait_for_focus(&june_12).await?;
    Ok(june_12)
}

/// Creating the grid and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(GRID, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(GRID)
        .await?
        .wait_for_attr("aria-label", Some("Event date, June 2024"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GRID, 0).await?;
    Ok(())
}

/// The arrow keys move the focused date by a day and by a week.
#[browser_test]
pub async fn move_with_the_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    focus_the_grid(page).await?;
    page.measure("right", page.send_keys(Key::Right)).await?;
    page.wait_for_focus(&date(page, "Thursday, June 13, 2024").await?)
        .await?;
    page.measure("left", page.send_keys(Key::Left)).await?;
    page.wait_for_focus(&date(page, "Wednesday, June 12, 2024").await?)
        .await?;
    page.measure("down", page.send_keys(Key::Down)).await?;
    page.wait_for_focus(&date(page, "Wednesday, June 19, 2024").await?)
        .await?;
    page.measure("up", page.send_keys(Key::Up)).await?;
    page.wait_for_focus(&date(page, "Wednesday, June 12, 2024").await?)
        .await?;
    assert_that!(page.control("focused").await?).is_equal_to(20_240_612.0);
    Ok(())
}

/// Page Down/Up move the focused date by a month, with Shift by a year.
#[browser_test]
pub async fn page_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    focus_the_grid(page).await?;
    let grid = page.element(GRID).await?;
    page.measure("page down", page.send_keys(Key::PageDown))
        .await?;
    grid.wait_for_attr("aria-label", Some("Event date, July 2024"))
        .await?;
    page.measure("page up", page.send_keys(Key::PageUp)).await?;
    grid.wait_for_attr("aria-label", Some("Event date, June 2024"))
        .await?;
    page.measure(
        "shift+page down",
        page.send_keys(Key::Shift + Key::PageDown),
    )
    .await?;
    grid.wait_for_attr("aria-label", Some("Event date, June 2025"))
        .await?;
    page.measure("shift+page up", page.send_keys(Key::Shift + Key::PageUp))
        .await?;
    grid.wait_for_attr("aria-label", Some("Event date, June 2024"))
        .await?;
    Ok(())
}

/// Home and End move the focused date to the start and the end of the month.
#[browser_test]
pub async fn move_to_the_month_ends(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    focus_the_grid(page).await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&date(page, "Saturday, June 1, 2024").await?)
        .await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    page.wait_for_focus(&date(page, "Sunday, June 30, 2024").await?)
        .await?;
    Ok(())
}

/// The focus entering the grid (Shift+Tab to its focused date) and leaving it (Tab).
#[browser_test]
pub async fn focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-after").await?.click().await?;
    let june_12 = date(page, "Wednesday, June 12, 2024").await?;
    page.measure("focus", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_focus(&june_12).await?;
    june_12.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    june_12.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// The calendar paging to the next month: the grid shows July.
#[browser_test]
pub async fn paging(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let grid = page.element(GRID).await?;
    let next = page
        .element("#perf-calendar button[aria-label=Next]")
        .await?;
    page.measure("next", next.click()).await?;
    grid.wait_for_attr("aria-label", Some("Event date, July 2024"))
        .await?;
    Ok(())
}
