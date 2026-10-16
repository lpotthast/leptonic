// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Calendar` (`/perf/atoms/calendar`): what each thing it offers
//! costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/calendar.json`): mounting and unmounting it with one month and with
//! twelve, selecting with the mouse and the keyboard, moving the focused date with the arrow,
//! page and Home/End keys, paging with its buttons, hovering a date, keyboard focus, the dates it
//! ignores (unavailable, disabled, read-only), and changes of its reactive props (value, focused
//! date, limits, states, visible duration, page behavior, first day of the week, selection
//! alignment, weeks per month, label). The calendar shows June 2024 (fixed dates: the counts
//! don't depend on the day the tests run).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/calendar";

/// Registers the performance tests of `Calendar`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountTwelveMonths))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(SelectWithTheMouseInTwelveMonths))
        .with(fixture_test(SelectWithTheKeyboard))
        .with(fixture_test(MoveWithTheArrowKeys))
        .with(fixture_test(MoveWithTheArrowKeysInTwelveMonths))
        .with(fixture_test(PageWithTheKeyboard))
        .with(fixture_test(MoveToTheMonthEnds))
        .with(fixture_test(PageWithTheButtons))
        .with(fixture_test(PageWithTheButtonsInTwelveMonths))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(UnavailableDate))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ReadOnly))
        .with(fixture_test(Invalid))
        .with(fixture_test(Limits))
        .with(fixture_test(ControlledValue))
        .with(fixture_test(ControlledFocusedDate))
        .with(fixture_test(VisibleDuration))
        .with(fixture_test(PageBehavior))
        .with(fixture_test(FirstDayOfWeek))
        .with(fixture_test(SelectionAlignment))
        .with(fixture_test(WeeksInMonth))
        .with(fixture_test(Label))
}
const CALENDAR: &str = "#perf-calendar";
const HEADING: &str = "#perf-calendar h2";
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
        "{CALENDAR} [role=gridcell] > [role=button]:not([data-outside-month])[aria-label^='{label}']"
    ))
    .await
}

/// The calendar's value (`yyyymmdd`, 0 without).
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// The calendar's focused date (`yyyymmdd`).
async fn focused(page: &Page<'_>) -> Result<f64, Report> {
    page.control("focused").await
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

/// Shows twelve months (January to December 2024: the focused date centered).
async fn show_twelve_months(page: &Page<'_>) -> Result<(), Report> {
    page.control("show-twelve-months").await?;
    page.element(HEADING)
        .await?
        .wait_for_inner_text("January 2024")
        .await
}

/// Mounting the calendar (one month) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(CALENDAR, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(HEADING)
        .await?
        .wait_for_inner_text("June 2024")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(CALENDAR, 0).await?;
    Ok(())
}

/// Mounting the calendar with twelve months (twelve grids) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_twelve_months(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(CALENDAR, 0).await?;
    page.control("show-twelve-months").await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count("#perf-calendar [role=grid]", 12)
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(CALENDAR, 0).await?;
    Ok(())
}

/// A click on a date selects it.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    page.measure("click", june_14.click()).await?;
    june_14.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(value(page).await?).is_equal_to(20_240_614.0);
    Ok(())
}

/// A click on a date of twelve visible months selects it.
#[browser_test]
pub async fn select_with_the_mouse_in_twelve_months(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    show_twelve_months(page).await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    page.measure("click", june_14.click()).await?;
    june_14.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(value(page).await?).is_equal_to(20_240_614.0);
    Ok(())
}

/// Enter and Space select the focused date.
#[browser_test]
pub async fn select_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let june_12 = focus_the_grid(page).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    june_12.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(value(page).await?).is_equal_to(20_240_612.0);
    page.send_keys(Key::Right).await?;
    let june_13 = date(page, "Thursday, June 13, 2024").await?;
    page.wait_for_focus(&june_13).await?;
    page.measure("space", page.send_keys(" ")).await?;
    june_13.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(value(page).await?).is_equal_to(20_240_613.0);
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
    assert_that!(focused(page).await?).is_equal_to(20_240_612.0);
    Ok(())
}

/// The arrow keys in twelve visible months.
#[browser_test]
pub async fn move_with_the_arrow_keys_in_twelve_months(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    show_twelve_months(page).await?;
    focus_the_grid(page).await?;
    page.measure("right", page.send_keys(Key::Right)).await?;
    page.wait_for_focus(&date(page, "Thursday, June 13, 2024").await?)
        .await?;
    page.measure("down", page.send_keys(Key::Down)).await?;
    page.wait_for_focus(&date(page, "Thursday, June 20, 2024").await?)
        .await?;
    assert_that!(focused(page).await?).is_equal_to(20_240_620.0);
    Ok(())
}

/// Page Down/Up move the focused date by a month (paging the calendar), with Shift by a year.
#[browser_test]
pub async fn page_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    focus_the_grid(page).await?;
    let heading = page.element(HEADING).await?;
    page.measure("page down", page.send_keys(Key::PageDown))
        .await?;
    heading.wait_for_inner_text("July 2024").await?;
    page.wait_for_focus(&date(page, "Friday, July 12, 2024").await?)
        .await?;
    page.measure("page up", page.send_keys(Key::PageUp)).await?;
    heading.wait_for_inner_text("June 2024").await?;
    page.measure(
        "shift+page down",
        page.send_keys(Key::Shift + Key::PageDown),
    )
    .await?;
    heading.wait_for_inner_text("June 2025").await?;
    page.measure("shift+page up", page.send_keys(Key::Shift + Key::PageUp))
        .await?;
    heading.wait_for_inner_text("June 2024").await?;
    page.wait_for_focus(&date(page, "Wednesday, June 12, 2024").await?)
        .await?;
    Ok(())
}

/// Home and End move the focused date to the start and the end of its month.
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

/// The next and previous buttons page by the visible month.
#[browser_test]
pub async fn page_with_the_buttons(page: &Page<'_>) -> Result<(), Report> {
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

/// The next and previous buttons page by twelve visible months.
#[browser_test]
pub async fn page_with_the_buttons_in_twelve_months(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    show_twelve_months(page).await?;
    let heading = page.element(HEADING).await?;
    let next = page
        .element("#perf-calendar button[aria-label=Next]")
        .await?;
    let previous = page
        .element("#perf-calendar button[aria-label=Previous]")
        .await?;
    page.measure("next", next.click()).await?;
    heading.wait_for_inner_text("January 2025").await?;
    page.measure("previous", previous.click()).await?;
    heading.wait_for_inner_text("January 2024").await?;
    Ok(())
}

/// The pointer entering and leaving a date.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", june_14.hover()).await?;
    june_14.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    june_14.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Shift+Tab moving focus into the grid (to the focused date, with a focus ring) and Tab out.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-after").await?.click().await?;
    let june_12 = date(page, "Wednesday, June 12, 2024").await?;
    page.measure("focus", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    june_12
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    june_12.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// A click on the unavailable date, which it ignores.
#[browser_test]
pub async fn unavailable_date(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let june_20 = date(page, "Thursday, June 20, 2024").await?;
    assert_that!(june_20.attr("data-unavailable").await?).is_equal_to(Some("true".to_owned()));
    page.measure("ignored click", june_20.click()).await?;
    assert_that!(value(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Disabling the calendar, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let calendar = page.element("#perf-calendar").await?;
    page.measure("disable", page.control("disable")).await?;
    calendar
        .wait_for_attr("data-disabled", Some("true"))
        .await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    page.measure("ignored click", june_14.click()).await?;
    assert_that!(value(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    calendar.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// Making the calendar read-only, a click it ignores, and making it editable.
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let grid = page.element("#perf-calendar [role=grid]").await?;
    page.measure("make read-only", page.control("make-read-only"))
        .await?;
    grid.wait_for_attr("aria-readonly", Some("true")).await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    page.measure("ignored click", june_14.click()).await?;
    assert_that!(value(page).await?).is_equal_to(0.0);
    page.measure("make editable", page.control("make-editable"))
        .await?;
    grid.wait_for_attr("aria-readonly", None).await?;
    Ok(())
}

/// Marking the calendar (with a selected date) invalid and valid again.
#[browser_test]
pub async fn invalid(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let calendar = page.element("#perf-calendar").await?;
    page.control("set-value").await?;
    let june_18 = date(page, "Tuesday, June 18, 2024").await?;
    june_18.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("invalidate", page.control("invalidate"))
        .await?;
    calendar.wait_for_attr("data-invalid", Some("true")).await?;
    june_18.wait_for_attr("data-invalid", Some("true")).await?;
    page.measure("validate", page.control("validate")).await?;
    calendar.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// Setting a minimum and a maximum (the dates outside disabled) and clearing both.
#[browser_test]
pub async fn limits(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let june_5 = date(page, "Wednesday, June 5, 2024").await?;
    let june_28 = date(page, "Friday, June 28, 2024").await?;
    page.measure("set min", page.control("set-min")).await?;
    june_5.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("set max", page.control("set-max")).await?;
    june_28.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("clear limits", page.control("clear-limits"))
        .await?;
    june_5.wait_for_attr("data-disabled", None).await?;
    june_28.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// The app setting the calendar's value and clearing it.
#[browser_test]
pub async fn controlled_value(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let june_18 = date(page, "Tuesday, June 18, 2024").await?;
    page.measure("set", page.control("set-value")).await?;
    june_18.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("clear", page.control("clear-value")).await?;
    june_18.wait_for_attr("data-selected", None).await?;
    Ok(())
}

/// The app moving the focused date to another month (the calendar follows) and back.
#[browser_test]
pub async fn controlled_focused_date(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let heading = page.element(HEADING).await?;
    page.measure("focus october", page.control("focus-october"))
        .await?;
    heading.wait_for_inner_text("October 2024").await?;
    page.measure("focus june", page.control("focus-june"))
        .await?;
    heading.wait_for_inner_text("June 2024").await?;
    Ok(())
}

/// Changing the visible duration: a week, back to a month, twelve months and one again.
#[browser_test]
pub async fn visible_duration(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let heading = page.element(HEADING).await?;
    page.measure("show a week", page.control("show-a-week"))
        .await?;
    page.wait_for_count("#perf-calendar [role=gridcell]", 7)
        .await?;
    page.measure("show a month", page.control("show-months"))
        .await?;
    heading.wait_for_inner_text("June 2024").await?;
    page.wait_for_count("#perf-calendar [role=grid]", 1).await?;
    page.measure("show twelve months", page.control("show-twelve-months"))
        .await?;
    page.wait_for_count("#perf-calendar [role=grid]", 12)
        .await?;
    page.measure("show one month", page.control("show-one-month"))
        .await?;
    page.wait_for_count("#perf-calendar [role=grid]", 1).await?;
    heading.wait_for_inner_text("June 2024").await?;
    Ok(())
}

/// Paging by a single month instead of the visible twelve, and back.
#[browser_test]
pub async fn page_behavior(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    show_twelve_months(page).await?;
    let heading = page.element(HEADING).await?;
    page.measure("page single", page.control("page-single"))
        .await?;
    page.element("#perf-calendar button[aria-label=Next]")
        .await?
        .click()
        .await?;
    heading.wait_for_inner_text("February 2024").await?;
    page.measure("page visible", page.control("page-visible"))
        .await?;
    page.element("#perf-calendar button[aria-label=Next]")
        .await?
        .click()
        .await?;
    heading.wait_for_inner_text("February 2025").await?;
    Ok(())
}

/// Starting the weeks on Monday and on the locale's first day (Sunday) again.
#[browser_test]
pub async fn first_day_of_week(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let first = "#perf-calendar thead th:first-child";
    page.measure("start on monday", page.control("start-on-monday"))
        .await?;
    page.element(first).await?.wait_for_inner_text("M").await?;
    page.measure(
        "start on the locale's day",
        page.control("start-on-locale-day"),
    )
    .await?;
    page.element(first).await?.wait_for_inner_text("S").await?;
    Ok(())
}

/// Changing the selection alignment (which applies when the visible range is aligned next) and
/// back.
#[browser_test]
pub async fn selection_alignment(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("align start", page.control("align-start"))
        .await?;
    page.measure("align center", page.control("align-center"))
        .await?;
    page.element(HEADING)
        .await?
        .wait_for_inner_text("June 2024")
        .await?;
    Ok(())
}

/// Showing six week rows in every month and the month's own number again.
#[browser_test]
pub async fn weeks_in_month(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    // June 2024 has six weeks: show October (five) first.
    page.control("focus-october").await?;
    page.element(HEADING)
        .await?
        .wait_for_inner_text("October 2024")
        .await?;
    let rows = "#perf-calendar tbody tr";
    page.wait_for_count(rows, 5).await?;
    page.measure("six weeks", page.control("six-weeks")).await?;
    page.wait_for_count(rows, 6).await?;
    page.measure("weeks of the month", page.control("weeks-of-month"))
        .await?;
    page.wait_for_count(rows, 5).await?;
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
