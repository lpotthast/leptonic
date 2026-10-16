// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_calendar_cell` (`/perf/hooks/use_calendar_cell`, the cells
//! of a calendar of hooks on June 2024): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget (`tests/budgets/hooks/use_calendar_cell.json`):
//! creating and disposing of the cells (with their calendar), selecting by a click and with Enter,
//! the pressed state, the browser's focus following the focused date, disabling the cells (and a
//! click they ignore), and in a range calendar: starting a range, highlighting by hovering,
//! finishing it, and dragging.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_calendar_cell";

/// Registers the performance tests of `use_calendar_cell`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(SelectWithTheKeyboard))
        .with(fixture_test(FocusFollowsTheFocusedDate))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(SelectARangeWithClicks))
        .with(fixture_test(SelectARangeByDragging))
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

/// Switches the fixture to a range calendar.
async fn use_range(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(CELLS, 0).await?;
    page.control("use-range").await?;
    page.control("mount").await?;
    page.wait_for_count(CELLS, 42).await
}

/// The button of the date labelled `label` ("Friday, June 14, 2024") in the grid's month. A
/// selected range's first and last dates are labelled with the range first.
async fn date(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    let button = format!("{CELLS} > [role=button]:not([data-outside-month])");
    page.element(format!(
        "{button}[aria-label^='{label}'], {button}[aria-label*=', {label}']"
    ))
    .await
}

/// Creating the calendar's cells (the 42 dates of June's six weeks) and disposing of them.
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

/// A click selects the date.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    page.measure("click", june_14.click()).await?;
    june_14.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(page.control("value").await?).is_equal_to(20_240_614.0);
    Ok(())
}

/// Holding the pointer down (the pressed state) and releasing it (the selection).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    let held = page.measure("hold", june_14.press_and_hold()).await?;
    june_14.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    june_14.wait_for_attr("data-pressed", None).await?;
    assert_that!(page.control("value").await?).is_equal_to(20_240_614.0);
    Ok(())
}

/// Enter on the focused date selects it.
#[browser_test]
pub async fn select_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-after").await?.click().await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    let june_12 = date(page, "Wednesday, June 12, 2024").await?;
    page.wait_for_focus(&june_12).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    june_12.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(page.control("value").await?).is_equal_to(20_240_612.0);
    Ok(())
}

/// The browser's focus following the focused date (Arrow Right in the grid): the next cell takes
/// it.
#[browser_test]
pub async fn focus_follows_the_focused_date(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-after").await?.click().await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&date(page, "Wednesday, June 12, 2024").await?)
        .await?;
    page.measure("right", page.send_keys(Key::Right)).await?;
    page.wait_for_focus(&date(page, "Thursday, June 13, 2024").await?)
        .await?;
    Ok(())
}

/// Disabling the cells, a click they ignore, and enabling them.
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

/// In a range calendar: a click on the first date, the pointer over the last (highlighting the
/// range), and a click on it.
#[browser_test]
pub async fn select_a_range_with_clicks(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    use_range(page).await?;
    let june_10 = date(page, "Monday, June 10, 2024").await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    page.measure("click start", june_10.click()).await?;
    june_10.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("hover end", june_14.hover()).await?;
    june_14.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("click end", june_14.click()).await?;
    assert_that!((page.control("start").await?, page.control("end").await?))
        .is_equal_to((20_240_610.0, 20_240_614.0));
    Ok(())
}

/// In a range calendar: pressing the first date, dragging to the last and releasing there.
#[browser_test]
pub async fn select_a_range_by_dragging(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    use_range(page).await?;
    let june_10 = date(page, "Monday, June 10, 2024").await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    let held = page.measure("hold", june_10.press_and_hold()).await?;
    june_10.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("drag", held.move_to(&june_14)).await?;
    june_14.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("release", held.release()).await?;
    assert_that!((page.control("start").await?, page.control("end").await?))
        .is_equal_to((20_240_610.0, 20_240_614.0));
    Ok(())
}
