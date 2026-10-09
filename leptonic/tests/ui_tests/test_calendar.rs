// Upstream: react-aria-components/test/Calendar.test.js @ 99e6102368
// Upstream: react-aria-components/test/RangeCalendar.test.tsx @ 99e6102368
// Upstream: react-aria/test/calendar/useCalendar.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/RangeCalendar.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/CalendarBase.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/Calendar.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/Calendar.ssr.test.js @ 99e6102368
//! Behavior of the calendar hooks (through the calendar atoms): ARIA structure and labels,
//! selection by press and keyboard, keyboard navigation (month, week and day views, pages and
//! years), the previous/next buttons, min/max, unavailable dates, disabled, read-only and
//! invalid calendars, several months, the first day of the week, and range selection by
//! presses, keyboard and dragging.
//! Spec: react-aria-components `Calendar.test.js`, `RangeCalendar.test.tsx`; react-aria
//! `useCalendar.test.js`; react-spectrum `CalendarBase.test.js`, `Calendar.test.js`,
//! `RangeCalendar.test.js` (their cases of both calendars per case where the behavior is the
//! range calendar's own, else on the calendar), react-stately `useCalendarState.test.ts` (native
//! tests in `use_calendar_state.rs`).
//!
//! Range selection by touch (react-spectrum `RangeCalendar.test.js`, "touch"): quick taps start
//! and finish a range, dragging after the press delay selects one, and a touch that turns into
//! a scroll doesn't finish a range being selected.
//!
//! A calendar without a value or focused date shows today (react-spectrum `Calendar.ssr.test.js`
//! renders it on the server). The server's today may be another date than the browser's (its
//! time zone): the page hydrates with the server's date, then the calendar moves to the
//! browser's today, so the tabbable date is the one marked as today.
//!
//! The views and paging of calendars, their pickers and announcements: `pageBehavior: single`
//! (`useCalendar.test.js`, "pagination"), a two-week view, a fixed number of week rows, held arrow
//! keys, a changing visible duration, month and year pickers (RAC `Calendar.test.js`,
//! `RangeCalendar.test.tsx`), the live announcements and the commit behaviors of a range being
//! selected (react-spectrum `RangeCalendar.test.js`, "announcing", "pointer events").
use std::time::Duration;

use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::{Report, bail};

use crate::pages::{
    ElementActions, Page, PointerKind, PointerType, StopwatchEnd, SyntheticEvent, css,
};

const PATH: &str = "/atoms/calendar";

/// The button of the date `label` ("Wednesday, June 5, 2019") in the calendar `name`: its label
/// is the date, possibly with additions ("Today, ", " selected", ", First available date", a
/// selected range's description before it). Other dates' labels mention it only within that
/// description ("... to Monday, June 10, 2019, ...").
async fn date(page: &Page<'_>, name: &str, label: &str) -> Result<WebElement, Report> {
    // Adjacent month grids may include the same date; only its own month is interactive.
    let button =
        format!("#test-calendar-{name} [role=gridcell] > [role=button]:not([data-outside-month])");
    page.element(format!(
        "{button}[aria-label='{label}'], {button}[aria-label^='{label} '], \
         {button}[aria-label^='{label},'], {button}[aria-label*=', {label}']"
    ))
    .await
}

/// The cell (`td`) around a date's button.
async fn cell(button: &WebElement) -> Result<WebElement, Report> {
    Ok(button.parent().await?)
}

/// The month grids of the calendar `name`.
async fn grids(page: &Page<'_>, name: &str) -> Result<Vec<WebElement>, Report> {
    page.elements(format!("#test-calendar-{name} [role=grid]"))
        .await
}

/// The first grid of the calendar `name`.
async fn first_grid(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.first_element(format!("#test-calendar-{name} [role=grid]"))
        .await
}

/// Waits until the first grid of the calendar `name` is labelled `expected`.
async fn wait_for_grid_label(page: &Page<'_>, name: &str, expected: &str) -> Result<(), Report> {
    let grid = grids(page, name).await?.swap_remove(0);
    grid.wait_for_attr("aria-label", Some(expected)).await?;
    Ok(())
}

/// The heading of the calendar `name` (its visible month or range).
async fn heading(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-calendar-{name} h2")).await
}

/// The button labelled `label` ("Previous", "Next") of the calendar `name`.
async fn button(page: &Page<'_>, name: &str, label: &str) -> Result<WebElement, Report> {
    page.element(format!(
        "#test-calendar-{name} button[aria-label='{label}']"
    ))
    .await
}

/// The fixture's output of the value of the calendar `name` (`none` without one).
async fn value(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-calendar-{name}-value")).await
}

/// Waits until the value of the calendar `name` is `expected`.
async fn wait_for_value(page: &Page<'_>, name: &str, expected: &str) -> Result<(), Report> {
    value(page, name).await?.wait_for_inner_text(expected).await
}

/// Tab into the calendar `name`'s grid from the button before it, past the previous and next
/// buttons (each a tab stop while enabled), to the grid's tabbable date.
async fn enter(page: &Page<'_>, name: &str) -> Result<(), Report> {
    let date = page
        .element(format!(
            "#test-calendar-{name} [role=grid] [role=button][tabindex='0']"
        ))
        .await?;
    page.element(format!("#test-calendar-{name}-before"))
        .await?
        .click()
        .await?;
    for _ in 0..3 {
        page.send_keys(Key::Tab).await?;
        if page.focused_element().await? == date {
            break;
        }
    }
    page.wait_for_focus(&date).await
}

/// Presses `keys` one after the other, each on the element focused at that moment: the focus
/// moves from date to date in an effect after the key (across months, once the grid re-rendered),
/// so the page settles after each key before the next goes to the focused element.
async fn press_keys(page: &Page<'_>, keys: &[Key]) -> Result<(), Report> {
    for key in keys {
        page.send_keys(key.clone()).await?;
        page.settle().await?;
    }
    Ok(())
}

/// The calendar and its grid are labelled with the label and month, and its dates are grid-cell
/// buttons labelled with the full date, only the selected one tabbable. The weekday header is
/// hidden from assistive technology and the other months' dates show disabled.
#[browser_test]
pub async fn structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    let calendar = page
        .element("#test-calendar-basic [role=application]")
        .await?;
    assert_that!(calendar)
        .has_attribute("aria-label")
        .await
        .is_equal_to("basic, June 2019");
    assert_that!(first_grid(page, "basic").await?)
        .has_attribute("aria-label")
        .await
        .is_equal_to("basic, June 2019");
    assert_that!(heading(page, "basic").await?)
        .inner_text()
        .await
        .is_equal_to("June 2019");

    let header = page.element("#test-calendar-basic thead").await?;
    assert_that!(header)
        .has_attribute("aria-hidden")
        .await
        .is_equal_to("true");
    assert_that!(page.inner_texts("#test-calendar-basic thead th").await?)
        .contains_exactly(["S", "M", "T", "W", "T", "F", "S"]);

    let selected = date(page, "basic", "Wednesday, June 5, 2019").await?;
    assert_that!(selected)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Wednesday, June 5, 2019 selected");
    assert_that!(selected)
        .has_attribute("tabindex")
        .await
        .is_equal_to("0");
    assert_that!(selected)
        .has_attribute("data-selected")
        .await
        .is_equal_to("true");
    assert_that!(selected).inner_text().await.is_equal_to("5");
    let selected_cell = cell(&selected).await?;
    assert_that!(selected_cell)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");

    let other = date(page, "basic", "Thursday, June 6, 2019").await?;
    assert_that!(other)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Thursday, June 6, 2019");
    assert_that!(other)
        .has_attribute("tabindex")
        .await
        .is_equal_to("-1");
    assert_that!(cell(&other).await?)
        .attribute("aria-selected")
        .await
        .is_none();

    // June 2019 starts on a Saturday: the first row starts with May 26.
    let outside = page
        .element("#test-calendar-basic [role=button][aria-label='Sunday, May 26, 2019']")
        .await?;
    assert_that!(outside)
        .has_attribute("data-outside-month")
        .await
        .is_equal_to("true");
    assert_that!(outside)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    assert_that!(outside).attribute("tabindex").await.is_none();
    assert_that!(page.count("#test-calendar-basic tbody tr").await?).is_equal_to(6);

    let previous = button(page, "basic", "Previous").await?;
    assert_that!(previous).enabled().await.is_true();
    Ok(())
}

/// Pressing a date selects and focuses it ("should support selected state").
#[browser_test]
pub async fn selection_by_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    let june17 = date(page, "basic", "Monday, June 17, 2019").await?;
    june17.click().await?;
    wait_for_value(page, "basic", "2019-06-17").await?;
    june17.wait_for_attr("data-selected", Some("true")).await?;
    june17
        .wait_for_attr("aria-label", Some("Monday, June 17, 2019 selected"))
        .await?;
    page.wait_for_focus(&june17).await?;
    let june5 = date(page, "basic", "Wednesday, June 5, 2019").await?;
    assert_that!(june5)
        .attribute("data-selected")
        .await
        .is_none();
    Ok(())
}

/// The arrow keys move the focus by a day or a week, and moving before the month's first date
/// pages to the previous month.
#[browser_test]
pub async fn keyboard_arrows(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    enter(page, "basic").await?;
    page.wait_for_focus(&date(page, "basic", "Wednesday, June 5, 2019").await?)
        .await?;
    press_keys(page, &[Key::Right]).await?;
    page.wait_for_focus(&date(page, "basic", "Thursday, June 6, 2019").await?)
        .await?;
    press_keys(page, &[Key::Left, Key::Left]).await?;
    page.wait_for_focus(&date(page, "basic", "Tuesday, June 4, 2019").await?)
        .await?;
    press_keys(page, &[Key::Down]).await?;
    page.wait_for_focus(&date(page, "basic", "Tuesday, June 11, 2019").await?)
        .await?;
    press_keys(page, &[Key::Up, Key::Up]).await?;
    page.wait_for_focus(&date(page, "basic", "Tuesday, May 28, 2019").await?)
        .await?;
    wait_for_grid_label(page, "basic", "basic, May 2019").await?;
    heading(page, "basic")
        .await?
        .wait_for_inner_text("May 2019")
        .await?;
    Ok(())
}

/// Page Down/Up move by a month, with Shift by a year.
#[browser_test]
pub async fn keyboard_pages(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    enter(page, "basic").await?;
    page.wait_for_focus(&date(page, "basic", "Wednesday, June 5, 2019").await?)
        .await?;
    press_keys(page, &[Key::PageDown]).await?;
    page.wait_for_focus(&date(page, "basic", "Friday, July 5, 2019").await?)
        .await?;
    wait_for_grid_label(page, "basic", "basic, July 2019").await?;
    page.send_keys(Key::Shift + Key::PageDown).await?;
    page.wait_for_focus(&date(page, "basic", "Sunday, July 5, 2020").await?)
        .await?;
    wait_for_grid_label(page, "basic", "basic, July 2020").await?;
    page.send_keys(Key::Shift + Key::PageUp).await?;
    page.wait_for_focus(&date(page, "basic", "Friday, July 5, 2019").await?)
        .await?;
    press_keys(page, &[Key::PageUp]).await?;
    page.wait_for_focus(&date(page, "basic", "Wednesday, June 5, 2019").await?)
        .await?;
    wait_for_grid_label(page, "basic", "basic, June 2019").await?;
    Ok(())
}

/// Home/End move to the month's ends.
#[browser_test]
pub async fn keyboard_home_end(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    enter(page, "basic").await?;
    page.wait_for_focus(&date(page, "basic", "Wednesday, June 5, 2019").await?)
        .await?;
    press_keys(page, &[Key::Home]).await?;
    page.wait_for_focus(&date(page, "basic", "Saturday, June 1, 2019").await?)
        .await?;
    press_keys(page, &[Key::End]).await?;
    page.wait_for_focus(&date(page, "basic", "Sunday, June 30, 2019").await?)
        .await?;
    Ok(())
}

/// Enter selects the focused date.
#[browser_test]
pub async fn keyboard_enter_selects(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    enter(page, "basic").await?;
    page.wait_for_focus(&date(page, "basic", "Wednesday, June 5, 2019").await?)
        .await?;
    press_keys(page, &[Key::Right]).await?;
    page.wait_for_focus(&date(page, "basic", "Thursday, June 6, 2019").await?)
        .await?;
    press_keys(page, &[Key::Enter]).await?;
    wait_for_value(page, "basic", "2019-06-06").await?;
    Ok(())
}

/// Paging from the sixth row of June to July, which has five, keeps the focus in the grid (on the
/// same day); Space selects.
#[browser_test]
pub async fn keyboard_paging_from_a_sixth_row(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    enter(page, "basic").await?;
    page.wait_for_focus(&date(page, "basic", "Wednesday, June 5, 2019").await?)
        .await?;
    press_keys(page, &[Key::End]).await?;
    page.wait_for_focus(&date(page, "basic", "Sunday, June 30, 2019").await?)
        .await?;
    press_keys(page, &[Key::PageDown]).await?;
    page.wait_for_focus(&date(page, "basic", "Tuesday, July 30, 2019").await?)
        .await?;
    press_keys(page, &[Key::PageUp, Key::PageUp]).await?;
    page.wait_for_focus(&date(page, "basic", "Thursday, May 30, 2019").await?)
        .await?;
    press_keys(page, &[Key::End]).await?;
    page.wait_for_focus(&date(page, "basic", "Friday, May 31, 2019").await?)
        .await?;
    press_keys(page, &[Key::Right, Key::Space]).await?;
    page.wait_for_focus(&date(page, "basic", "Saturday, June 1, 2019").await?)
        .await?;
    wait_for_value(page, "basic", "2019-06-01").await?;
    Ok(())
}

/// The Previous and Next buttons page by a month and show a focus ring when focused by the
/// keyboard.
#[browser_test]
pub async fn previous_next_buttons(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    // Keyboard focus shows on the buttons.
    page.element("#test-calendar-basic-before")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    let previous = button(page, "basic", "Previous").await?;
    page.wait_for_focus(&previous).await?;
    previous
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    let next = button(page, "basic", "Next").await?;
    next.click().await?;
    wait_for_grid_label(page, "basic", "basic, July 2019").await?;
    heading(page, "basic")
        .await?
        .wait_for_inner_text("July 2019")
        .await?;
    let previous = button(page, "basic", "Previous").await?;
    previous.click().await?;
    previous.click().await?;
    wait_for_grid_label(page, "basic", "basic, May 2019").await?;
    next.click().await?;
    wait_for_grid_label(page, "basic", "basic, June 2019").await?;
    Ok(())
}

/// Dates outside min/max are disabled and can't be selected, the keyboard stops at the limits, the
/// first and last available dates say so in their labels, and the buttons can't page past them.
#[browser_test]
pub async fn min_max(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["min-max"]).await?;
    let june9 = date(page, "min-max", "Sunday, June 9, 2019").await?;
    assert_that!(june9)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    assert_that!(june9)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    let june10 = date(page, "min-max", "Monday, June 10, 2019").await?;
    assert_that!(june10)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Monday, June 10, 2019, First available date");
    let june20 = date(page, "min-max", "Thursday, June 20, 2019").await?;
    assert_that!(june20)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Thursday, June 20, 2019, Last available date");
    assert_that!(button(page, "min-max", "Previous").await?)
        .enabled()
        .await
        .is_false();
    assert_that!(button(page, "min-max", "Next").await?)
        .enabled()
        .await
        .is_false();

    june9.click().await?;
    let june21 = date(page, "min-max", "Friday, June 21, 2019").await?;
    june21.click().await?;
    value(page, "min-max")
        .await?
        .inner_text_stays("2019-06-15", std::time::Duration::from_millis(100))
        .await?;

    // The keyboard stops at the limits.
    enter(page, "min-max").await?;
    page.wait_for_focus(&date(page, "min-max", "Saturday, June 15, 2019").await?)
        .await?;
    press_keys(page, &[Key::Down]).await?;
    page.wait_for_focus(&date(page, "min-max", "Thursday, June 20, 2019").await?)
        .await?;
    press_keys(page, &[Key::PageUp]).await?;
    page.wait_for_focus(&date(page, "min-max", "Monday, June 10, 2019").await?)
        .await?;
    Ok(())
}

/// Unavailable dates are marked and can't be selected, neither by press nor by keyboard ("should
/// support unavailable state", "should not modify selection when trying to select an unavailable
/// date by keyboard").
#[browser_test]
pub async fn unavailable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["unavailable"]).await?;
    let june8 = date(page, "unavailable", "Saturday, June 8, 2019").await?;
    assert_that!(june8)
        .has_attribute("data-unavailable")
        .await
        .is_equal_to("true");
    assert_that!(june8)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    june8.click().await?;
    value(page, "unavailable")
        .await?
        .inner_text_stays("2019-06-05", std::time::Duration::from_millis(100))
        .await?;

    // Still focusable with the keyboard, but Enter doesn't select it.
    enter(page, "unavailable").await?;
    press_keys(page, &[Key::Right, Key::Right, Key::Right]).await?;
    page.wait_for_focus(&date(page, "unavailable", "Saturday, June 8, 2019").await?)
        .await?;
    press_keys(page, &[Key::Enter]).await?;
    value(page, "unavailable")
        .await?
        .inner_text_stays("2019-06-05", std::time::Duration::from_millis(100))
        .await?;
    press_keys(page, &[Key::Right, Key::Right, Key::Enter]).await?;
    wait_for_value(page, "unavailable", "2019-06-10").await?;
    Ok(())
}

/// A disabled calendar marks its grid and dates disabled, and no date is tabbable or selectable
/// ("should support disabled state").
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["disabled"]).await?;
    let grid = grids(page, "disabled").await?.swap_remove(0);
    assert_that!(grid)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    let calendar = page
        .element("#test-calendar-disabled [role=application]")
        .await?;
    assert_that!(calendar)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    assert_that!(
        page.count("#test-calendar-disabled [role=button][tabindex]")
            .await?
    )
    .is_equal_to(0);
    let june10 = date(page, "disabled", "Monday, June 10, 2019").await?;
    assert_that!(june10)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    june10.click().await?;
    value(page, "disabled")
        .await?
        .inner_text_stays("2019-06-05", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(button(page, "disabled", "Next").await?)
        .enabled()
        .await
        .is_false();
    Ok(())
}

/// A read-only calendar's grid is `aria-readonly`: its dates can be navigated with the keyboard,
/// but neither presses nor Enter select one.
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["read-only"]).await?;
    let grid = grids(page, "read-only").await?.swap_remove(0);
    assert_that!(grid)
        .has_attribute("aria-readonly")
        .await
        .is_equal_to("true");
    date(page, "read-only", "Monday, June 10, 2019")
        .await?
        .click()
        .await?;
    enter(page, "read-only").await?;
    page.wait_for_focus(&date(page, "read-only", "Wednesday, June 5, 2019").await?)
        .await?;
    press_keys(page, &[Key::Right, Key::Enter]).await?;
    page.wait_for_focus(&date(page, "read-only", "Thursday, June 6, 2019").await?)
        .await?;
    value(page, "read-only")
        .await?
        .inner_text_stays("2019-06-05", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// An invalid calendar marks its selected date invalid and describes it by the error message
/// ("should support invalid state").
#[browser_test]
pub async fn invalid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["invalid"]).await?;
    let calendar = page
        .element("#test-calendar-invalid [role=application]")
        .await?;
    assert_that!(calendar)
        .has_attribute("data-invalid")
        .await
        .is_equal_to("true");
    let june5 = date(page, "invalid", "Wednesday, June 5, 2019").await?;
    assert_that!(june5)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    assert_that!(june5)
        .has_attribute("data-invalid")
        .await
        .is_equal_to("true");
    assert_that!(cell(&june5).await?)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    let error = page
        .element(css("#test-calendar-invalid div").text("Invalid date"))
        .await?;
    let error_id = assert_that!(error)
        .has_attribute("id")
        .await
        .actual()
        .clone();
    assert_that!(june5)
        .has_attribute("aria-describedby")
        .await
        .is_equal_to(error_id);
    let june6 = date(page, "invalid", "Thursday, June 6, 2019").await?;
    assert_that!(june6)
        .attribute("aria-invalid")
        .await
        .is_none();
    Ok(())
}

/// A two-month calendar has a grid per month and is labelled with both; dates of either month can
/// be selected, and Next pages both ("should support multi-month calendars").
#[browser_test]
pub async fn two_months(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["two-months"]).await?;
    let calendar = page
        .element("#test-calendar-two-months [role=application]")
        .await?;
    assert_that!(calendar)
        .has_attribute("aria-label")
        .await
        .is_equal_to("two-months, June 2019 to July 2019");
    let grids = grids(page, "two-months").await?;
    assert_that!(grids).has_length(2);
    assert_that!(grids[0])
        .has_attribute("aria-label")
        .await
        .is_equal_to("two-months, June 2019");
    assert_that!(grids[1])
        .has_attribute("aria-label")
        .await
        .is_equal_to("two-months, July 2019");

    // A date of the second month.
    date(page, "two-months", "Monday, July 15, 2019")
        .await?
        .click()
        .await?;
    wait_for_value(page, "two-months", "2019-07-15").await?;

    button(page, "two-months", "Next").await?.click().await?;
    grids[0]
        .wait_for_attr("aria-label", Some("two-months, August 2019"))
        .await?;
    grids[1]
        .wait_for_attr("aria-label", Some("two-months, September 2019"))
        .await?;
    Ok(())
}

/// A week view shows one row; arrows past its ends page by a week, Home/End go to its ends and
/// Shift+Page Down moves by a month ("should support week view"; useCalendar.test.js
/// "visibleDuration: 1 week").
#[browser_test]
pub async fn week_view(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["week"]).await?;
    assert_that!(first_grid(page, "week").await?)
        .has_attribute("aria-label")
        .await
        .is_equal_to("week, June 2, 2019 to June 8, 2019");
    assert_that!(page.count("#test-calendar-week tbody tr").await?).is_equal_to(1);

    enter(page, "week").await?;
    page.wait_for_focus(&date(page, "week", "Wednesday, June 5, 2019").await?)
        .await?;
    press_keys(page, &[Key::Right, Key::Right, Key::Right, Key::Right]).await?;
    page.wait_for_focus(&date(page, "week", "Sunday, June 9, 2019").await?)
        .await?;
    wait_for_grid_label(page, "week", "week, June 9, 2019 to June 15, 2019").await?;
    press_keys(page, &[Key::Down]).await?;
    page.wait_for_focus(&date(page, "week", "Sunday, June 16, 2019").await?)
        .await?;
    press_keys(page, &[Key::End]).await?;
    page.wait_for_focus(&date(page, "week", "Saturday, June 22, 2019").await?)
        .await?;
    press_keys(page, &[Key::Home]).await?;
    page.wait_for_focus(&date(page, "week", "Sunday, June 16, 2019").await?)
        .await?;
    page.send_keys(Key::Shift + Key::PageDown).await?;
    page.wait_for_focus(&date(page, "week", "Tuesday, July 16, 2019").await?)
        .await?;
    Ok(())
}

/// A three-day view is centered on the selected date, and moving the focus out of it pages by three
/// days ("should support day view"; useCalendar.test.js "visibleDuration: 3 days").
#[browser_test]
pub async fn day_view(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["days"]).await?;
    assert_that!(first_grid(page, "days").await?)
        .has_attribute("aria-label")
        .await
        .is_equal_to("days, June 4, 2019 to June 6, 2019");
    assert_that!(page.inner_texts("#test-calendar-days th").await?)
        .contains_exactly(["T", "W", "T"]);

    enter(page, "days").await?;
    press_keys(page, &[Key::Left, Key::Left]).await?;
    page.wait_for_focus(&date(page, "days", "Monday, June 3, 2019").await?)
        .await?;
    wait_for_grid_label(page, "days", "days, June 1, 2019 to June 3, 2019").await?;
    press_keys(page, &[Key::Down]).await?;
    // A row is the three visible days.
    page.wait_for_focus(&date(page, "days", "Thursday, June 6, 2019").await?)
        .await?;
    wait_for_grid_label(page, "days", "days, June 4, 2019 to June 6, 2019").await?;
    Ok(())
}

/// With Monday as the first day of the week, the weekday header and the first row start on a
/// Monday (useCalendar.test.js "should use firstDayOfWeek $Name").
#[browser_test]
pub async fn first_day_of_week(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["monday"]).await?;
    let header = page.first_element("#test-calendar-monday th").await?;
    assert_that!(header).inner_text().await.is_equal_to("M");
    let first = page
        .first_element("#test-calendar-monday [role=gridcell] > [role=button]")
        .await?;
    assert_that!(first)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Monday, May 27, 2019");
    Ok(())
}

/// Two presses select a range: the first starts it, prompting to finish, and hovering highlights
/// the dates up to the pointer; the second finishes it and marks its ends ("should support
/// selected range states", "$Name adds a range selection prompt to the focused cell").
#[browser_test]
pub async fn range_by_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range"]).await?;
    let june5 = date(page, "range", "Wednesday, June 5, 2019").await?;
    assert_that!(june5)
        .has_attribute("data-selection-start")
        .await
        .is_equal_to("true");
    let june10 = date(page, "range", "Monday, June 10, 2019").await?;
    assert_that!(june10)
        .has_attribute("data-selection-end")
        .await
        .is_equal_to("true");
    assert_that!(june5)
        .has_attribute("aria-label")
        .await
        .starts_with("Selected Range: ");
    let grid = grids(page, "range").await?.swap_remove(0);
    assert_that!(grid)
        .has_attribute("aria-multiselectable")
        .await
        .is_equal_to("true");

    let june12 = date(page, "range", "Wednesday, June 12, 2019").await?;
    june12.click().await?;
    june12.wait_for_attr("data-selected", Some("true")).await?;
    value(page, "range")
        .await?
        .inner_text_stays(
            "2019-06-05 - 2019-06-10",
            std::time::Duration::from_millis(100),
        )
        .await?;
    // The focused cell says how to go on.
    assert_that!(june12)
        .accessible_description()
        .await
        .is_equal_to("Click to finish selecting date range");

    let june14 = date(page, "range", "Friday, June 14, 2019").await?;
    june14.hover().await?;
    let june13 = date(page, "range", "Thursday, June 13, 2019").await?;
    june13.wait_for_attr("data-selected", Some("true")).await?;
    june14.click().await?;
    wait_for_value(page, "range", "2019-06-12 - 2019-06-14").await?;
    june12
        .wait_for_attr("data-selection-start", Some("true"))
        .await?;
    june14
        .wait_for_attr("data-selection-end", Some("true"))
        .await?;
    june5.wait_for_attr("data-selected", None).await?;
    Ok(())
}

/// Enter on a date starts a range and Enter on another finishes it, while Escape cancels a started
/// range ("$Name can select a range with the keyboard (controlled)", "$Name cancels the selection
/// when the escape key is pressed").
#[browser_test]
pub async fn range_by_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range"]).await?;
    // The pointer away from the dates: while a range is started, hovering a date highlights it
    // (and moves the focus there), also when the layout moves a date under a resting pointer.
    let heading = page.element("h1").await?;
    heading.hover().await?;
    // The focus starts on the selected range's start.
    enter(page, "range").await?;
    page.wait_for_focus(&date(page, "range", "Wednesday, June 5, 2019").await?)
        .await?;
    press_keys(page, &[Key::Left, Key::Enter]).await?;
    page.wait_for_focus(&date(page, "range", "Wednesday, June 5, 2019").await?)
        .await?;
    press_keys(page, &[Key::Right, Key::Enter]).await?;
    wait_for_value(page, "range", "2019-06-04 - 2019-06-06").await?;

    press_keys(page, &[Key::Down, Key::Enter, Key::Right]).await?;
    page.wait_for_focus(&date(page, "range", "Saturday, June 15, 2019").await?)
        .await?;
    let june15 = date(page, "range", "Saturday, June 15, 2019").await?;
    june15.wait_for_attr("data-selected", Some("true")).await?;
    press_keys(page, &[Key::Escape]).await?;
    june15.wait_for_attr("data-selected", None).await?;
    value(page, "range")
        .await?
        .inner_text_stays(
            "2019-06-04 - 2019-06-06",
            std::time::Duration::from_millis(100),
        )
        .await?;

    press_keys(page, &[Key::Enter, Key::Right, Key::Enter]).await?;
    wait_for_value(page, "range", "2019-06-15 - 2019-06-17").await?;
    Ok(())
}

/// Dragging from one date to another selects that range, and dragging an end of the range moves it
/// ("selects by dragging with the mouse", "allows dragging the end of the highlighted range to
/// modify it").
#[browser_test]
pub async fn range_by_dragging(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range"]).await?;
    let june3 = date(page, "range", "Monday, June 3, 2019").await?;
    let june6 = date(page, "range", "Thursday, June 6, 2019").await?;
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_center(&june3)
        .click_and_hold()
        .move_to_element_center(&june6)
        .release()
        .perform()
        .await?;
    wait_for_value(page, "range", "2019-06-03 - 2019-06-06").await?;

    // Dragging an end of the range moves it.
    let june8 = date(page, "range", "Saturday, June 8, 2019").await?;
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_center(&june6)
        .click_and_hold()
        .move_to_element_center(&june8)
        .release()
        .perform()
        .await?;
    wait_for_value(page, "range", "2019-06-03 - 2019-06-08").await?;
    Ok(())
}

/// Without non-contiguous ranges, a started range can't span an unavailable date: the dates beyond
/// it are disabled until the range is finished ("disables dates not reachable from start date if
/// isDateUnavailable is provided").
#[browser_test]
pub async fn range_unavailable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-unavailable"]).await?;
    let june11 = date(page, "range-unavailable", "Tuesday, June 11, 2019").await?;
    june11.click().await?;
    let june17 = date(page, "range-unavailable", "Monday, June 17, 2019").await?;
    june17.wait_for_attr("aria-disabled", Some("true")).await?;
    let june14 = date(page, "range-unavailable", "Friday, June 14, 2019").await?;
    assert_that!(june14)
        .attribute("aria-disabled")
        .await
        .is_none();
    june17.click().await?;
    june14.click().await?;
    wait_for_value(page, "range-unavailable", "2019-06-11 - 2019-06-14").await?;
    june17.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}

/// A calendar labelled by another element lists it in its and its grid's `aria-labelledby`, and
/// every referenced id exists ("should support aria props on the Calendar").
#[browser_test]
pub async fn labelled_by_another_element(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["labelled"]).await?;
    let calendar = page
        .element("#test-calendar-labelled [role=application]")
        .await?;
    let grid = grids(page, "labelled").await?.swap_remove(0);
    for element in [&calendar, &grid] {
        let ids = assert_that!(element)
            .has_attribute("aria-labelledby")
            .await
            .map_owned(|ids| {
                ids.split_whitespace()
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
            })
            .contains("test-calendar-labelled-label")
            .actual()
            .clone();
        for id in ids {
            assert_that!(page.count(format!("[id='{id}']")).await?).is_equal_to(1);
        }
    }
    assert_that!(grid)
        .has_attribute("aria-label")
        .await
        .is_equal_to("labelled, June 2019");
    Ok(())
}

/// In a right-to-left locale, the left arrow moves to the next day and the right arrow to the
/// previous one.
#[browser_test]
pub async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["rtl"]).await?;
    enter(page, "rtl").await?;
    page.wait_for_focus(&day_of_month(page, "rtl", 5).await?)
        .await?;
    press_keys(page, &[Key::Left]).await?;
    page.wait_for_focus(&day_of_month(page, "rtl", 6).await?)
        .await?;
    press_keys(page, &[Key::Right, Key::Right]).await?;
    page.wait_for_focus(&day_of_month(page, "rtl", 4).await?)
        .await?;
    Ok(())
}

/// The button of the `day` of the visible month in the calendar `name` (by its number, for
/// calendars labelled in other languages).
async fn day_of_month(page: &Page<'_>, name: &str, day: u8) -> Result<WebElement, Report> {
    page.element(
        css(format!(
            "#test-calendar-{name} [role=gridcell] > [role=button]:not([data-outside-month])"
        ))
        .text(day.to_string()),
    )
    .await
}

/// Setting the focused date from outside makes that date tabbable but leaves the browser's focus
/// where it is ("should not become focused just by setting the focused date").
#[browser_test]
pub async fn setting_the_focused_date_keeps_the_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["focus"]).await?;
    let set = page.element("#test-calendar-focus-set").await?;
    set.click().await?;
    let june20 = date(page, "focus", "Thursday, June 20, 2019").await?;
    june20.wait_for_attr("tabindex", Some("0")).await?;
    page.focus_stays(&set, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Clearing a controlled range from outside removes the selection from every date.
#[browser_test]
pub async fn controlled_range_cleared(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range"]).await?;
    page.element("#test-calendar-range-clear")
        .await?
        .click()
        .await?;
    wait_for_value(page, "range", "none").await?;
    let june5 = date(page, "range", "Wednesday, June 5, 2019").await?;
    june5.wait_for_attr("data-selected", None).await?;
    assert_that!(june5)
        .attribute("data-selection-start")
        .await
        .is_none();
    assert_that!(page.count("#test-calendar-range [data-selected]").await?).is_equal_to(0);
    Ok(())
}

/// Unavailable dates can depend on the anchor of a range being selected: here, dates more than a
/// week after it, available again once the range is finished ("should allow changing the
/// unavailable dates based on the anchor date").
#[browser_test]
pub async fn unavailable_dates_depending_on_the_anchor(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-week"]).await?;
    let june18 = date(page, "range-week", "Tuesday, June 18, 2019").await?;
    assert_that!(june18)
        .attribute("data-unavailable")
        .await
        .is_none();
    date(page, "range-week", "Monday, June 10, 2019")
        .await?
        .click()
        .await?;
    june18
        .wait_for_attr("data-unavailable", Some("true"))
        .await?;
    let june17 = date(page, "range-week", "Monday, June 17, 2019").await?;
    assert_that!(june17)
        .attribute("data-unavailable")
        .await
        .is_none();
    june17.click().await?;
    wait_for_value(page, "range-week", "2019-06-10 - 2019-06-17").await?;
    june18.wait_for_attr("data-unavailable", None).await?;
    Ok(())
}

/// Dispatches a touch pointer event (`pointerdown`, `pointerup`, `pointerenter`,
/// `pointercancel`) at the center of `element`.
async fn touch(element: &WebElement, kind: PointerKind) -> Result<(), Report> {
    let rect = element.client_rect().await?;
    element
        .dispatch(
            SyntheticEvent::pointer(kind)
                .pointer_type(PointerType::Touch)
                .at(rect.left + rect.width / 2.0, rect.top + rect.height / 2.0),
        )
        .await?;
    Ok(())
}

/// A quick tap: pressed and released before the drag delay.
async fn touch_tap(element: &WebElement) -> Result<(), Report> {
    touch(element, PointerKind::Down).await?;
    touch(element, PointerKind::Up).await?;
    Ok(())
}

/// The numbers of the selected dates of the calendar `name`, in document order.
async fn selected_days(page: &Page<'_>, name: &str) -> Result<Vec<String>, Report> {
    page.inner_texts(format!(
        "#test-calendar-{name} [role=button][data-selected]"
    ))
    .await
}

/// Waits until the selected days of the calendar `name` are `expected`.
async fn wait_for_selected_days(
    page: &Page<'_>,
    name: &str,
    expected: &[&str],
) -> Result<(), Report> {
    assert_that!(|| selected_days(page, name))
        .with_subject_name(format!("the selected days of {name}"))
        .eventually_ok()
        .matches(eq(expected))
        .await;
    Ok(())
}

/// Two quick taps select a range: the first only starts it, also once the touch drag delay passed,
/// and the second finishes it.
#[browser_test]
pub async fn range_by_touch_taps(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range"]).await?;
    let june11 = date(page, "range", "Tuesday, June 11, 2019").await?;
    touch_tap(&june11).await?;
    wait_for_selected_days(page, "range", &["11"]).await?;
    // Past the touch drag delay (200 ms): still only started.
    page.settle().await?;
    assert_that!(|| selected_days(page, "range"))
        .consistently_ok()
        .for_at_least(Duration::from_millis(400))
        .matches(eq(["11"]))
        .await;
    value(page, "range")
        .await?
        .inner_text_stays(
            "2019-06-05 - 2019-06-10",
            std::time::Duration::from_millis(100),
        )
        .await?;

    let june13 = date(page, "range", "Thursday, June 13, 2019").await?;
    touch_tap(&june13).await?;
    wait_for_value(page, "range", "2019-06-11 - 2019-06-13").await?;
    Ok(())
}

/// After the touch drag delay, the touched date starts a range, the dates the finger enters extend
/// it and lifting the finger finishes it ("selects by dragging with touch").
#[browser_test]
pub async fn range_by_touch_dragging(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range"]).await?;
    let june17 = date(page, "range", "Monday, June 17, 2019").await?;
    assert_that!(selected_days(page, "range").await?)
        .contains_exactly(["5", "6", "7", "8", "9", "10"]);
    // The delay tells dragging from scrolling: nothing changes at first (timed in the page).
    let stopwatch = page
        .start_stopwatch(
            &june17,
            PointerKind::Down,
            StopwatchEnd::AttributeChanges {
                element: &page.element("#test-calendar-range").await?,
                name: "data-selected",
            },
        )
        .await?;
    touch(&june17, PointerKind::Down).await?;
    wait_for_selected_days(page, "range", &["17"]).await?;
    assert_that!(stopwatch.finish().await?)
        .with_detail_message("from the pointerdown to the first selection change (delay: 200 ms)")
        .is_greater_or_equal_to(Duration::from_millis(195));
    let june18 = date(page, "range", "Tuesday, June 18, 2019").await?;
    touch(&june18, PointerKind::Enter).await?;
    wait_for_selected_days(page, "range", &["17", "18"]).await?;
    let june23 = date(page, "range", "Sunday, June 23, 2019").await?;
    touch(&june23, PointerKind::Enter).await?;
    wait_for_selected_days(page, "range", &["17", "18", "19", "20", "21", "22", "23"]).await?;
    value(page, "range")
        .await?
        .inner_text_stays(
            "2019-06-05 - 2019-06-10",
            std::time::Duration::from_millis(100),
        )
        .await?;
    touch(&june23, PointerKind::Up).await?;
    wait_for_value(page, "range", "2019-06-17 - 2019-06-23").await?;
    Ok(())
}

/// A touch on a date that turns into a scroll doesn't finish the range being selected ("selection
/// isn't prematurely finalized when touching a day cell to scroll through the calendar").
#[browser_test]
pub async fn range_kept_when_a_touch_scrolls(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range"]).await?;
    date(page, "range", "Sunday, June 23, 2019")
        .await?
        .click()
        .await?;
    wait_for_selected_days(page, "range", &["23"]).await?;
    let june10 = date(page, "range", "Monday, June 10, 2019").await?;
    touch(&june10, PointerKind::Down).await?;
    touch(&june10, PointerKind::Cancel).await?;
    // Past the touch drag delay (200 ms), after which a pressed date starts dragging.
    let range_value = value(page, "range").await?;
    page.settle().await?;
    assert_that!(|| range_value.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(400))
        .matches(eq("2019-06-05 - 2019-06-10"))
        .await;
    date(page, "range", "Tuesday, June 25, 2019")
        .await?
        .click()
        .await?;
    wait_for_value(page, "range", "2019-06-23 - 2019-06-25").await?;
    Ok(())
}

/// When the browser's today is another date than the server's, the browser's today is marked as
/// today and is the only tabbable date, and keyboard focus goes there.
#[browser_test]
pub async fn today_in_the_browsers_time_zone(page: &Page<'_>) -> Result<(), Report> {
    {
        // A browser time zone in which today is another date than on the server (this
        // machine): 14 hours ahead or 12 behind UTC; one of them always differs.
        let server_today = jiff::Zoned::now().date();
        let mut browser_zone = None;
        for zone in ["Pacific/Kiritimati", "Etc/GMT+12"] {
            if jiff::Zoned::now().in_tz(zone)?.date() != server_today {
                browser_zone = Some(zone);
                break;
            }
        }
        let Some(zone) = browser_zone else {
            bail!("no time zone with another date than {server_today}");
        };
        page.low_level()
            .driver()
            .cdp()
            .send_raw(
                "Emulation.setTimezoneOverride",
                serde_json::json!({ "timezoneId": zone }),
            )
            .await?;
    }
    page.goto_sections(PATH, &["today"]).await?;

    let today = page
        .element("#test-calendar-today [role=gridcell] > [role=button][tabindex='0'][aria-label^='Today, ']")
        .await?;
    assert_that!(
        page.count("#test-calendar-today [role=button][tabindex='0']")
            .await?
    )
    .is_equal_to(1);
    enter(page, "today").await?;
    page.wait_for_focus(&today).await?;
    Ok(())
}

/// The labels of the grids of the calendar `name`.
async fn grid_labels(page: &Page<'_>, name: &str) -> Result<Vec<String>, Report> {
    let mut labels = Vec::new();
    for grid in grids(page, name).await? {
        labels.push(grid.attr("aria-label").await?.unwrap_or_default());
    }
    Ok(labels)
}

/// Waits until the grids of the calendar `name` are labelled `expected`.
async fn wait_for_grid_labels(
    page: &Page<'_>,
    name: &str,
    expected: &[&str],
) -> Result<(), Report> {
    assert_that!(|| grid_labels(page, name))
        .with_subject_name(format!("the grid labels of {name}"))
        .eventually_ok()
        .matches(eq(expected))
        .await;
    Ok(())
}

/// With `pageBehavior: single`, the buttons page by one month, week or day instead of the whole
/// visible duration (useCalendar.test.js "should use pageBehavior single $Name").
#[browser_test]
pub async fn page_behavior_single(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["single-page", "weeks-single", "days-single"])
        .await?;
    wait_for_grid_labels(
        page,
        "single-page",
        &["single-page, January 2019", "single-page, February 2019"],
    )
    .await?;
    button(page, "single-page", "Next").await?.click().await?;
    wait_for_grid_labels(
        page,
        "single-page",
        &["single-page, February 2019", "single-page, March 2019"],
    )
    .await?;
    let previous = button(page, "single-page", "Previous").await?;
    previous.click().await?;
    previous.click().await?;
    wait_for_grid_labels(
        page,
        "single-page",
        &["single-page, December 2018", "single-page, January 2019"],
    )
    .await?;

    wait_for_grid_label(
        page,
        "weeks-single",
        "weeks-single, December 23, 2018 to January 12, 2019",
    )
    .await?;
    button(page, "weeks-single", "Next").await?.click().await?;
    wait_for_grid_label(
        page,
        "weeks-single",
        "weeks-single, December 30, 2018 to January 19, 2019",
    )
    .await?;
    let previous = button(page, "weeks-single", "Previous").await?;
    previous.click().await?;
    previous.click().await?;
    wait_for_grid_label(
        page,
        "weeks-single",
        "weeks-single, December 16, 2018 to January 5, 2019",
    )
    .await?;

    wait_for_grid_label(
        page,
        "days-single",
        "days-single, December 30, 2018 to January 3, 2019",
    )
    .await?;
    button(page, "days-single", "Next").await?.click().await?;
    wait_for_grid_label(
        page,
        "days-single",
        "days-single, December 31, 2018 to January 4, 2019",
    )
    .await?;
    Ok(())
}

/// A two-week view shows two rows labelled with its dates, and moving the focus past its end pages
/// by two weeks (useCalendar.test.js "visibleDuration: 2 weeks").
#[browser_test]
pub async fn two_weeks(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["two-weeks"]).await?;
    assert_that!(first_grid(page, "two-weeks").await?)
        .has_attribute("aria-label")
        .await
        .is_equal_to("two-weeks, June 2, 2019 to June 15, 2019");
    assert_that!(page.count("#test-calendar-two-weeks tbody tr").await?).is_equal_to(2);
    enter(page, "two-weeks").await?;
    page.wait_for_focus(&date(page, "two-weeks", "Wednesday, June 5, 2019").await?)
        .await?;
    press_keys(page, &[Key::Down, Key::Down]).await?;
    page.wait_for_focus(&date(page, "two-weeks", "Wednesday, June 19, 2019").await?)
        .await?;
    wait_for_grid_label(
        page,
        "two-weeks",
        "two-weeks, June 16, 2019 to June 29, 2019",
    )
    .await?;
    Ok(())
}

/// With six weeks per month, April 2026, which needs five rows, shows six ("should support
/// weeksInMonth prop").
#[browser_test]
pub async fn weeks_in_month(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["six-weeks"]).await?;
    assert_that!(page.count("#test-calendar-six-weeks tbody tr").await?).is_equal_to(6);
    Ok(())
}

/// Holding the right arrow key moves the focus on by a day for every repeated keydown ("should
/// support repeat keydown events when holding an arrow key").
#[browser_test]
pub async fn held_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["reset"]).await?;
    let march3 = date(page, "reset", "Tuesday, March 3, 2020").await?;
    march3.click().await?;
    page.wait_for_focus(&march3).await?;
    page.hold_key("ArrowRight", 1).await?;
    page.wait_for_focus(&date(page, "reset", "Thursday, March 5, 2020").await?)
        .await?;
    Ok(())
}

/// Switching a week view to a month view shows the month of the focused date ("should handle
/// changing the visible duration").
#[browser_test]
pub async fn changing_the_visible_duration(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["duration"]).await?;
    let heading = heading(page, "duration").await?;
    assert_that!(heading)
        .inner_text()
        .await
        .is_equal_to("April 5, 2026 to April 11, 2026");
    page.element("#test-calendar-duration-month")
        .await?
        .click()
        .await?;
    heading.wait_for_inner_text("April 2026").await?;
    wait_for_grid_label(page, "duration", "duration, April 2026").await?;
    assert_that!(page.count("#test-calendar-duration tbody tr").await?).is_equal_to(5);
    Ok(())
}

/// The option of `select` with the text `text`.
async fn option(select: &WebElement, text: &str) -> Result<WebElement, Report> {
    select.element(css("option").text(text)).await
}

/// The month and year pickers list the months and 20 years around the focused date and move the
/// calendar to the chosen one, the year list following ("should support month and year dropdowns").
#[browser_test]
pub async fn month_and_year_pickers(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["pickers"]).await?;
    let month = page
        .element("#test-calendar-pickers select[aria-label=month]")
        .await?;
    let year = page
        .element("#test-calendar-pickers select[aria-label=year]")
        .await?;
    wait_for_grid_label(page, "pickers", "Appointment date, April 2026").await?;
    assert_that!(month)
        .property("value")
        .await
        .get_some()
        .is_equal_to("4");
    assert_that!(month.inner_texts("option").await?).contains_exactly([
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ]);
    option(&month, "Jun").await?.click().await?;
    wait_for_grid_label(page, "pickers", "Appointment date, June 2026").await?;
    assert_that!(month)
        .property("value")
        .await
        .get_some()
        .is_equal_to("6");

    let years =
        |range: std::ops::Range<i32>| range.map(|year| year.to_string()).collect::<Vec<_>>();
    assert_that!(year.inner_texts("option").await?).is_equal_to(years(2016..2036));
    option(&year, "2030").await?.click().await?;
    wait_for_grid_label(page, "pickers", "Appointment date, June 2030").await?;
    assert_that!(|| async { Ok::<_, Report>(year.inner_texts("option").await?.first().cloned()) })
        .eventually_ok()
        .matches(eq(Some("2020".to_owned())))
        .await;
    assert_that!(year.inner_texts("option").await?).is_equal_to(years(2020..2040));
    Ok(())
}

/// Wait for a polite live announcement.
async fn wait_for_announcement(page: &Page<'_>, text: &str) -> Result<(), Report> {
    assert_that!(|| announcements_now(page))
        .eventually_ok()
        .satisfies(|entries| {
            entries.contains(text);
        })
        .await;
    Ok(())
}

/// The polite live announcements now.
async fn announcements_now(page: &Page<'_>) -> Result<Vec<String>, Report> {
    page.inner_texts("[data-live-announcer] [aria-live=polite] div")
        .await
}

/// Paging announces the new month and finishing a range announces the selected range ("announces
/// when the current month changes", "announces when the selected date range changes").
#[browser_test]
pub async fn announcements(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range"]).await?;
    button(page, "range", "Next").await?.click().await?;
    wait_for_announcement(page, "July 2019").await?;
    button(page, "range", "Previous").await?.click().await?;
    wait_for_announcement(page, "June 2019").await?;
    date(page, "range", "Monday, June 17, 2019")
        .await?
        .click()
        .await?;
    date(page, "range", "Monday, June 10, 2019")
        .await?
        .click()
        .await?;
    wait_for_announcement(
        page,
        "Selected Range: Monday, June 10, 2019 to Monday, June 17, 2019",
    )
    .await?;
    Ok(())
}

/// Starts a range at `start` and hovers `end` in the calendar `name`.
async fn start_range(page: &Page<'_>, name: &str, start: &str, end: &str) -> Result<(), Report> {
    date(page, name, start).await?.click().await?;
    let end = date(page, name, end).await?;
    end.hover().await?;
    end.wait_for_attr("data-selected", Some("true")).await?;
    Ok(())
}

/// Clicks the heading of the calendar `name`.
async fn click_heading(page: &Page<'_>, name: &str) -> Result<(), Report> {
    page.element(format!("#test-calendar-{name} h2"))
        .await?
        .click()
        .await?;
    Ok(())
}

/// Starts the range November 25 to 20, 2025 (hovered) in the calendar `commit-<behavior>`
/// (value November 13 to 15, 2025).
async fn start_commit_range(page: &Page<'_>, behavior: &str) -> Result<(), Report> {
    start_range(
        page,
        &format!("commit-{behavior}"),
        "Tuesday, November 25, 2025",
        "Thursday, November 20, 2025",
    )
    .await
}

/// With `Select`, pressing the calendar's heading finishes a started range at the hovered date
/// ("should select the last hovered date when commitBehavior is 'select'").
#[browser_test]
pub async fn commit_select_on_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["commit-select"]).await?;
    start_commit_range(page, "select").await?;
    click_heading(page, "commit-select").await?;
    wait_for_value(page, "commit-select", "2025-11-20 - 2025-11-25").await?;
    Ok(())
}

/// With `Select`, the focus leaving the calendar finishes a started range at the hovered date
/// ("should select the hovered range when commitBehavior is 'select' and calendar blurs").
#[browser_test]
pub async fn commit_select_on_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["commit-select"]).await?;
    start_commit_range(page, "select").await?;
    page.send_keys(Key::Tab).await?;
    wait_for_value(page, "commit-select", "2025-11-20 - 2025-11-25").await?;
    Ok(())
}

/// With `Clear`, pressing the calendar's heading during a started range clears the value ("should
/// clear the selection when commitBehavior is 'clear'").
#[browser_test]
pub async fn commit_clear_on_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["commit-clear"]).await?;
    start_commit_range(page, "clear").await?;
    click_heading(page, "commit-clear").await?;
    wait_for_value(page, "commit-clear", "none").await?;
    page.wait_for_count("#test-calendar-commit-clear [data-selected]", 0)
        .await?;
    Ok(())
}

/// With `Clear`, the focus leaving the calendar during a started range clears the value ("should
/// clear the selection when commitBehavior is 'clear' and calendar blurs").
#[browser_test]
pub async fn commit_clear_on_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["commit-clear"]).await?;
    start_commit_range(page, "clear").await?;
    page.send_keys(Key::Tab).await?;
    wait_for_value(page, "commit-clear", "none").await?;
    page.wait_for_count("#test-calendar-commit-clear [data-selected]", 0)
        .await?;
    Ok(())
}

/// With `Reset`, pressing the calendar's heading during a started range keeps the value and shows
/// its range again ("should reset to the initial range when commitBehavior is 'reset'").
#[browser_test]
pub async fn commit_reset_on_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["commit-reset"]).await?;
    let november25 = date(page, "commit-reset", "Tuesday, November 25, 2025").await?;
    let november13 = date(page, "commit-reset", "Thursday, November 13, 2025").await?;
    start_commit_range(page, "reset").await?;
    assert_that!(november13)
        .attribute("data-selected")
        .await
        .is_none();
    click_heading(page, "commit-reset").await?;
    november25.wait_for_attr("data-selected", None).await?;
    november13
        .wait_for_attr("data-selected", Some("true"))
        .await?;
    value(page, "commit-reset")
        .await?
        .inner_text_stays(
            "2025-11-13 - 2025-11-15",
            std::time::Duration::from_millis(100),
        )
        .await?;
    Ok(())
}

/// With `Reset`, the focus leaving the calendar during a started range keeps the value and shows
/// its range again ("should reset to the initial range when commitBehavior is 'reset' and calendar
/// blurs").
#[browser_test]
pub async fn commit_reset_on_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["commit-reset"]).await?;
    let november25 = date(page, "commit-reset", "Tuesday, November 25, 2025").await?;
    let november13 = date(page, "commit-reset", "Thursday, November 13, 2025").await?;
    start_commit_range(page, "reset").await?;
    assert_that!(november13)
        .attribute("data-selected")
        .await
        .is_none();
    page.send_keys(Key::Tab).await?;
    november25.wait_for_attr("data-selected", None).await?;
    november13
        .wait_for_attr("data-selected", Some("true"))
        .await?;
    value(page, "commit-reset")
        .await?
        .inner_text_stays(
            "2025-11-13 - 2025-11-15",
            std::time::Duration::from_millis(100),
        )
        .await?;
    Ok(())
}

/// Loads the calendar `name` (the focus on its selected date, from the button before it), presses
/// `key` `count` times (with Shift: `shift`), and waits until the focus is on the date `focused`
/// and the first grid is labelled `grid_label`.
async fn key_moves(
    page: &Page<'_>,
    name: &str,
    key: Key,
    count: usize,
    shift: bool,
    focused: &str,
    grid_label: &str,
) -> Result<(), Report> {
    page.goto_sections(PATH, &[name]).await?;
    enter(page, name).await?;
    for _ in 0..count {
        if shift {
            page.send_keys(Key::Shift + key.clone()).await?;
        } else {
            page.send_keys(key.clone()).await?;
        }
        page.settle().await?;
    }
    page.wait_for_focus(&date(page, name, focused).await?)
        .await?;
    wait_for_grid_label(page, name, grid_label).await
}

/// In a three-day view, the left and right arrows move by a day, paging past the visible days
/// (useCalendar.test.js "visibleDuration: 3 days", "should move the focused date by one day with
/// the left/right arrows").
#[browser_test]
pub async fn day_view_left_right_arrows(page: &Page<'_>) -> Result<(), Report> {
    let june = "days, June 4, 2019 to June 6, 2019";
    key_moves(
        page,
        "days",
        Key::Left,
        1,
        false,
        "Tuesday, June 4, 2019",
        june,
    )
    .await?;
    let before = "days, June 1, 2019 to June 3, 2019";
    key_moves(
        page,
        "days",
        Key::Left,
        2,
        false,
        "Monday, June 3, 2019",
        before,
    )
    .await?;
    key_moves(
        page,
        "days",
        Key::Right,
        1,
        false,
        "Thursday, June 6, 2019",
        june,
    )
    .await?;
    let after = "days, June 7, 2019 to June 9, 2019";
    key_moves(
        page,
        "days",
        Key::Right,
        2,
        false,
        "Friday, June 7, 2019",
        after,
    )
    .await?;
    Ok(())
}

/// In a three-day view, the up and down arrows move by a row of three days (useCalendar.test.js
/// "visibleDuration: 3 days", "should move the focused date by one row with the up/down arrows").
#[browser_test]
pub async fn day_view_up_down_arrows(page: &Page<'_>) -> Result<(), Report> {
    let before = "days, June 1, 2019 to June 3, 2019";
    key_moves(
        page,
        "days",
        Key::Up,
        1,
        false,
        "Sunday, June 2, 2019",
        before,
    )
    .await?;
    let after = "days, June 7, 2019 to June 9, 2019";
    key_moves(
        page,
        "days",
        Key::Down,
        1,
        false,
        "Saturday, June 8, 2019",
        after,
    )
    .await?;
    Ok(())
}

/// In a three-day view, Page Up/Down move by a row of three days (useCalendar.test.js
/// "visibleDuration: 3 days", "should move the focused date by one row with the page up/page down
/// arrows").
#[browser_test]
pub async fn day_view_page_keys(page: &Page<'_>) -> Result<(), Report> {
    let before = "days, June 1, 2019 to June 3, 2019";
    key_moves(
        page,
        "days",
        Key::PageUp,
        1,
        false,
        "Sunday, June 2, 2019",
        before,
    )
    .await?;
    let after = "days, June 7, 2019 to June 9, 2019";
    key_moves(
        page,
        "days",
        Key::PageDown,
        1,
        false,
        "Saturday, June 8, 2019",
        after,
    )
    .await?;
    Ok(())
}

/// In a three-day view, Shift+Page Up/Down move by a row of three days as well
/// (useCalendar.test.js "visibleDuration: 3 days", "should move the focused date by one row with
/// the shift + page up/page down arrows").
#[browser_test]
pub async fn day_view_shift_page_keys(page: &Page<'_>) -> Result<(), Report> {
    let before = "days, June 1, 2019 to June 3, 2019";
    key_moves(
        page,
        "days",
        Key::PageUp,
        1,
        true,
        "Sunday, June 2, 2019",
        before,
    )
    .await?;
    let after = "days, June 7, 2019 to June 9, 2019";
    key_moves(
        page,
        "days",
        Key::PageDown,
        1,
        true,
        "Saturday, June 8, 2019",
        after,
    )
    .await?;
    Ok(())
}

/// In a three-day view, Home and End move to the first and last visible day (useCalendar.test.js
/// "visibleDuration: 3 days", "should move the focused date to the start/end of the visible range
/// with the home/end keys").
#[browser_test]
pub async fn day_view_home_end(page: &Page<'_>) -> Result<(), Report> {
    let june = "days, June 4, 2019 to June 6, 2019";
    key_moves(
        page,
        "days",
        Key::Home,
        1,
        false,
        "Tuesday, June 4, 2019",
        june,
    )
    .await?;
    key_moves(
        page,
        "days",
        Key::End,
        1,
        false,
        "Thursday, June 6, 2019",
        june,
    )
    .await?;
    Ok(())
}

/// In a week view, the left and right arrows move by a day, paging by a week past its ends
/// (useCalendar.test.js "visibleDuration: 1 week", "should move the focused date by one day with
/// the left/right arrows").
#[browser_test]
pub async fn week_view_left_right_arrows(page: &Page<'_>) -> Result<(), Report> {
    let june = "week, June 2, 2019 to June 8, 2019";
    key_moves(
        page,
        "week",
        Key::Left,
        1,
        false,
        "Tuesday, June 4, 2019",
        june,
    )
    .await?;
    let before = "week, May 26, 2019 to June 1, 2019";
    key_moves(
        page,
        "week",
        Key::Left,
        4,
        false,
        "Saturday, June 1, 2019",
        before,
    )
    .await?;
    key_moves(
        page,
        "week",
        Key::Right,
        1,
        false,
        "Thursday, June 6, 2019",
        june,
    )
    .await?;
    let after = "week, June 9, 2019 to June 15, 2019";
    key_moves(
        page,
        "week",
        Key::Right,
        4,
        false,
        "Sunday, June 9, 2019",
        after,
    )
    .await?;
    Ok(())
}

/// In a week view, the up and down arrows move by a week (useCalendar.test.js "visibleDuration: 1
/// week", "should move the focused date by one week with the up/down arrows").
#[browser_test]
pub async fn week_view_up_down_arrows(page: &Page<'_>) -> Result<(), Report> {
    let before = "week, May 26, 2019 to June 1, 2019";
    key_moves(
        page,
        "week",
        Key::Up,
        1,
        false,
        "Wednesday, May 29, 2019",
        before,
    )
    .await?;
    let after = "week, June 9, 2019 to June 15, 2019";
    key_moves(
        page,
        "week",
        Key::Down,
        1,
        false,
        "Wednesday, June 12, 2019",
        after,
    )
    .await?;
    Ok(())
}

/// In a week view, Page Up/Down move by a week (useCalendar.test.js "visibleDuration: 1 week",
/// "should move the focused date by one week with the page up/page down arrows").
#[browser_test]
pub async fn week_view_page_keys(page: &Page<'_>) -> Result<(), Report> {
    let before = "week, May 26, 2019 to June 1, 2019";
    key_moves(
        page,
        "week",
        Key::PageUp,
        1,
        false,
        "Wednesday, May 29, 2019",
        before,
    )
    .await?;
    let after = "week, June 9, 2019 to June 15, 2019";
    key_moves(
        page,
        "week",
        Key::PageDown,
        1,
        false,
        "Wednesday, June 12, 2019",
        after,
    )
    .await?;
    Ok(())
}

/// In a week view, Shift+Page Up/Down move by a month (useCalendar.test.js "visibleDuration: 1
/// week", "should move the focused date by one month with the shift + page up/page down arrows").
#[browser_test]
pub async fn week_view_shift_page_keys(page: &Page<'_>) -> Result<(), Report> {
    let before = "week, May 5, 2019 to May 11, 2019";
    key_moves(
        page,
        "week",
        Key::PageUp,
        1,
        true,
        "Sunday, May 5, 2019",
        before,
    )
    .await?;
    let after = "week, June 30, 2019 to July 6, 2019";
    key_moves(
        page,
        "week",
        Key::PageDown,
        1,
        true,
        "Friday, July 5, 2019",
        after,
    )
    .await?;
    Ok(())
}

/// In a week view, Home and End move to the week's first and last day (useCalendar.test.js
/// "visibleDuration: 1 week", "should move the focused date to the start/end of the week with the
/// home/end keys").
#[browser_test]
pub async fn week_view_home_end(page: &Page<'_>) -> Result<(), Report> {
    let june = "week, June 2, 2019 to June 8, 2019";
    key_moves(
        page,
        "week",
        Key::Home,
        1,
        false,
        "Sunday, June 2, 2019",
        june,
    )
    .await?;
    key_moves(
        page,
        "week",
        Key::End,
        1,
        false,
        "Saturday, June 8, 2019",
        june,
    )
    .await?;
    Ok(())
}

/// In a two-week view, the left and right arrows move by a day, paging by two weeks past its ends
/// (useCalendar.test.js "visibleDuration: 2 weeks", "should move the focused date by one day with
/// the left/right arrows").
#[browser_test]
pub async fn two_weeks_left_right_arrows(page: &Page<'_>) -> Result<(), Report> {
    let name = "two-weeks";
    let june = "two-weeks, June 2, 2019 to June 15, 2019";
    key_moves(
        page,
        name,
        Key::Left,
        1,
        false,
        "Tuesday, June 4, 2019",
        june,
    )
    .await?;
    let before = "two-weeks, May 19, 2019 to June 1, 2019";
    key_moves(
        page,
        name,
        Key::Left,
        4,
        false,
        "Saturday, June 1, 2019",
        before,
    )
    .await?;
    key_moves(
        page,
        name,
        Key::Right,
        1,
        false,
        "Thursday, June 6, 2019",
        june,
    )
    .await?;
    key_moves(
        page,
        name,
        Key::Right,
        4,
        false,
        "Sunday, June 9, 2019",
        june,
    )
    .await?;
    let after = "two-weeks, June 16, 2019 to June 29, 2019";
    key_moves(
        page,
        name,
        Key::Right,
        11,
        false,
        "Sunday, June 16, 2019",
        after,
    )
    .await?;
    Ok(())
}

/// In a two-week view, the up and down arrows move by a week (useCalendar.test.js
/// "visibleDuration: 2 weeks", "should move the focused date by one week with the up/down
/// arrows").
#[browser_test]
pub async fn two_weeks_up_down_arrows(page: &Page<'_>) -> Result<(), Report> {
    let name = "two-weeks";
    let before = "two-weeks, May 19, 2019 to June 1, 2019";
    key_moves(
        page,
        name,
        Key::Up,
        1,
        false,
        "Wednesday, May 29, 2019",
        before,
    )
    .await?;
    let june = "two-weeks, June 2, 2019 to June 15, 2019";
    key_moves(
        page,
        name,
        Key::Down,
        1,
        false,
        "Wednesday, June 12, 2019",
        june,
    )
    .await?;
    let after = "two-weeks, June 16, 2019 to June 29, 2019";
    key_moves(
        page,
        name,
        Key::Down,
        2,
        false,
        "Wednesday, June 19, 2019",
        after,
    )
    .await?;
    Ok(())
}

/// In a two-week view, Page Up/Down move by a week (useCalendar.test.js "visibleDuration: 2
/// weeks", "should move the focused date by one week with the page up/page down arrows").
#[browser_test]
pub async fn two_weeks_page_keys(page: &Page<'_>) -> Result<(), Report> {
    let name = "two-weeks";
    let before = "two-weeks, May 19, 2019 to June 1, 2019";
    key_moves(
        page,
        name,
        Key::PageUp,
        1,
        false,
        "Wednesday, May 29, 2019",
        before,
    )
    .await?;
    let june = "two-weeks, June 2, 2019 to June 15, 2019";
    key_moves(
        page,
        name,
        Key::PageDown,
        1,
        false,
        "Wednesday, June 12, 2019",
        june,
    )
    .await?;
    Ok(())
}

/// In a two-week view, Shift+Page Up/Down move by a month (useCalendar.test.js "visibleDuration: 2
/// weeks", "should move the focused date by one month with the shift + page up/page down
/// arrows").
#[browser_test]
pub async fn two_weeks_shift_page_keys(page: &Page<'_>) -> Result<(), Report> {
    let name = "two-weeks";
    let before = "two-weeks, April 28, 2019 to May 11, 2019";
    key_moves(
        page,
        name,
        Key::PageUp,
        1,
        true,
        "Sunday, May 5, 2019",
        before,
    )
    .await?;
    let after = "two-weeks, June 30, 2019 to July 13, 2019";
    key_moves(
        page,
        name,
        Key::PageDown,
        1,
        true,
        "Friday, July 5, 2019",
        after,
    )
    .await?;
    Ok(())
}

/// In a two-week view, Home and End move to the first and last day of the week (useCalendar.test.js
/// "visibleDuration: 2 weeks", "should move the focused date to the start/end of the visible range
/// with the home/end keys").
#[browser_test]
pub async fn two_weeks_home_end(page: &Page<'_>) -> Result<(), Report> {
    let name = "two-weeks";
    let june = "two-weeks, June 2, 2019 to June 15, 2019";
    key_moves(
        page,
        name,
        Key::Home,
        1,
        false,
        "Sunday, June 2, 2019",
        june,
    )
    .await?;
    key_moves(
        page,
        name,
        Key::End,
        1,
        false,
        "Saturday, June 8, 2019",
        june,
    )
    .await?;
    Ok(())
}

/// Loads the calendar `name`, presses its `button` ("Previous", "Next") `count` times and waits
/// until its first grid is labelled `expected`.
async fn pages_to(
    page: &Page<'_>,
    name: &str,
    button_label: &str,
    count: usize,
    expected: &str,
) -> Result<(), Report> {
    page.goto_sections(PATH, &[name]).await?;
    let button = button(page, name, button_label).await?;
    for _ in 0..count {
        button.click().await?;
    }
    wait_for_grid_label(page, name, expected).await
}

/// The buttons of a two-month calendar page by two months (useCalendar.test.js "should use visible
/// as default value $Name").
#[browser_test]
pub async fn pagination_by_the_visible_months(page: &Page<'_>) -> Result<(), Report> {
    let name = "two-months";
    pages_to(page, name, "Next", 1, "two-months, August 2019").await?;
    pages_to(page, name, "Next", 2, "two-months, October 2019").await?;
    pages_to(page, name, "Previous", 1, "two-months, April 2019").await?;
    pages_to(page, name, "Previous", 2, "two-months, February 2019").await?;
    Ok(())
}

/// The buttons of a three-week view page by three weeks (useCalendar.test.js "should use visible as
/// default $Name", weeks).
#[browser_test]
pub async fn pagination_by_the_visible_weeks(page: &Page<'_>) -> Result<(), Report> {
    let name = "weeks-visible";
    page.goto_sections(PATH, &[name]).await?;
    wait_for_grid_label(
        page,
        name,
        "weeks-visible, December 23, 2018 to January 12, 2019",
    )
    .await?;
    let label = |range: &str| format!("weeks-visible, {range}");
    pages_to(
        page,
        name,
        "Next",
        1,
        &label("January 13, 2019 to February 2, 2019"),
    )
    .await?;
    pages_to(
        page,
        name,
        "Next",
        2,
        &label("February 3, 2019 to February 23, 2019"),
    )
    .await?;
    pages_to(
        page,
        name,
        "Previous",
        1,
        &label("December 2, 2018 to December 22, 2018"),
    )
    .await?;
    pages_to(
        page,
        name,
        "Previous",
        2,
        &label("November 11, 2018 to December 1, 2018"),
    )
    .await?;
    Ok(())
}

/// The buttons of a five-day view page by five days (useCalendar.test.js "should use visible as
/// default $Name", days).
#[browser_test]
pub async fn pagination_by_the_visible_days(page: &Page<'_>) -> Result<(), Report> {
    let name = "days-visible";
    page.goto_sections(PATH, &[name]).await?;
    wait_for_grid_label(
        page,
        name,
        "days-visible, December 30, 2018 to January 3, 2019",
    )
    .await?;
    let label = |range: &str| format!("days-visible, {range}");
    pages_to(
        page,
        name,
        "Next",
        1,
        &label("January 4, 2019 to January 8, 2019"),
    )
    .await?;
    pages_to(
        page,
        name,
        "Next",
        2,
        &label("January 9, 2019 to January 13, 2019"),
    )
    .await?;
    pages_to(
        page,
        name,
        "Previous",
        1,
        &label("December 25, 2018 to December 29, 2018"),
    )
    .await?;
    pages_to(
        page,
        name,
        "Previous",
        2,
        &label("December 20, 2018 to December 24, 2018"),
    )
    .await?;
    Ok(())
}

/// The first date button of the calendar `name` (the first row's first cell).
async fn first_date_button(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.first_element(format!(
        "#test-calendar-{name} [role=gridcell] > [role=button]"
    ))
    .await
}

/// With Saturday as the first day of the week, the header starts with Saturday and January 1,
/// 2024 is the third cell (useCalendar.test.js "should use firstDayOfWeek Saturday";
/// CalendarBase.test.js "should override start of week with firstDayOfWeek="sat" (en-US)").
#[browser_test]
pub async fn first_day_of_week_saturday(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["saturday"]).await?;
    assert_that!(page.inner_texts("#test-calendar-saturday th").await?)
        .contains_exactly(["S", "S", "M", "T", "W", "T", "F"]);
    assert_that!(first_date_button(page, "saturday").await?)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Saturday, December 30, 2023");
    let third = page
        .elements("#test-calendar-saturday [role=gridcell] > [role=button]")
        .await?
        .swap_remove(2);
    assert_that!(third).inner_text().await.is_equal_to("1");
    Ok(())
}

/// French calendars start weeks on Monday and label their dates in French; a first day of the week
/// overrides the locale's (useCalendar.test.js "should use firstDayOfWeek default (fr-FR)",
/// "Sunday (fr-FR)", "Saturday (fr-FR)"; CalendarBase.test.js "should override start of week with
/// firstDayOfWeek="mon" (fr-FR)", "="sat" (fr-FR)").
#[browser_test]
pub async fn first_day_of_week_in_french(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["fr", "fr-sunday", "fr-saturday"])
        .await?;
    assert_that!(page.inner_texts("#test-calendar-fr th").await?)
        .contains_exactly(["L", "M", "M", "J", "V", "S", "D"]);
    assert_that!(first_date_button(page, "fr").await?)
        .has_attribute("aria-label")
        .await
        .is_equal_to("lundi 1 janvier 2024 sélectionné");
    assert_that!(first_date_button(page, "fr-sunday").await?)
        .has_attribute("aria-label")
        .await
        .is_equal_to("dimanche 31 décembre 2023");
    assert_that!(page.inner_texts("#test-calendar-fr-saturday th").await?)
        .contains_exactly(["S", "D", "L", "M", "M", "J", "V"]);
    assert_that!(first_date_button(page, "fr-saturday").await?)
        .has_attribute("aria-label")
        .await
        .is_equal_to("samedi 30 décembre 2023");
    Ok(())
}

/// With Thursday as the first day of the week, January 2025 needs six rows, the last ending with
/// February 5 (CalendarBase.test.js "should render enough weeks to display all days in month for
/// firstDayOfWeek").
#[browser_test]
pub async fn first_day_of_week_needing_six_rows(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["thursday"]).await?;
    assert_that!(page.inner_texts("#test-calendar-thursday th").await?)
        .contains_exactly(["T", "F", "S", "S", "M", "T", "W"]);
    assert_that!(page.count("#test-calendar-thursday tbody tr").await?).is_equal_to(6);
    let cells = page
        .elements("#test-calendar-thursday [role=gridcell]")
        .await?;
    assert_that!(&cells).has_length(42);
    assert_that!(cells[35])
        .inner_text()
        .await
        .is_equal_to("30".to_owned());
    assert_that!(cells[36])
        .inner_text()
        .await
        .is_equal_to("31".to_owned());
    assert_that!(cells[41])
        .inner_text()
        .await
        .is_equal_to("5".to_owned());
    assert_that!(cells[41])
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    Ok(())
}

/// The first day of the week follows the locale: Sunday in the United States, Monday in Germany
/// (CalendarBase.test.js "should change the week start day based on the locale").
#[browser_test]
pub async fn first_day_of_week_of_the_locale(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic", "de"]).await?;
    let first_header = |name: &str| format!("#test-calendar-{name} th");
    assert_that!(page.first_element(first_header("basic")).await?)
        .inner_text()
        .await
        .is_equal_to("S");
    assert_that!(page.first_element(first_header("de")).await?)
        .inner_text()
        .await
        .is_equal_to("M");
    Ok(())
}

/// Without a label, the calendar and its grid are named by the visible month, and the calendar
/// has an id (CalendarBase.test.js "should be labeled by month heading by default").
#[browser_test]
pub async fn labelled_by_the_month_by_default(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["unlabelled"]).await?;
    let calendar = page
        .element("#test-calendar-unlabelled [role=application]")
        .await?;
    assert_that!(calendar)
        .has_attribute("aria-label")
        .await
        .is_equal_to("June 2019");
    assert_that!(calendar).has_attribute("id").await;
    assert_that!(first_grid(page, "unlabelled").await?)
        .has_attribute("aria-label")
        .await
        .is_equal_to("June 2019");
    Ok(())
}

/// Labelled by another element only, the calendar and its grid are named by the month and that
/// element, each labelling itself first (CalendarBase.test.js "should support labeling with
/// aria-labelledby").
#[browser_test]
pub async fn labelled_only_by_another_element(page: &Page<'_>) -> Result<(), Report> {
    let name = "labelledby-only";
    page.goto_sections(PATH, &[name]).await?;
    let label = "test-calendar-labelledby-only-label";
    let calendar = page
        .element("#test-calendar-labelledby-only [role=application]")
        .await?;
    let grid = first_grid(page, name).await?;
    for element in [&calendar, &grid] {
        assert_that!(element)
            .has_attribute("aria-label")
            .await
            .is_equal_to("June 2019");
        let id = element.id().await?.unwrap_or_default();
        assert_that!(element)
            .has_attribute("aria-labelledby")
            .await
            .derive_owned(|ids| ids.split_whitespace().collect::<Vec<_>>())
            .contains_exactly([id.as_str(), label]);
    }
    assert_that!(calendar)
        .accessible_name()
        .await
        .is_equal_to("June 2019 Booking");
    Ok(())
}

/// A calendar given an id keeps it, labelled by its label, the month and another element
/// (CalendarBase.test.js "should support labeling with a custom id").
#[browser_test]
pub async fn custom_id(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["custom-id"]).await?;
    let calendar = page
        .element("#test-calendar-custom-id [role=application]")
        .await?;
    assert_that!(calendar)
        .attribute("id")
        .await
        .is_equal_to(Some("hi".to_owned()));
    assert_that!(calendar)
        .has_attribute("aria-label")
        .await
        .is_equal_to("custom-id, June 2019");
    assert_that!(calendar)
        .has_attribute("aria-labelledby")
        .await
        .derive_owned(|ids| ids.split_whitespace().collect::<Vec<_>>())
        .contains_exactly(["hi", "test-calendar-custom-id-label"]);
    Ok(())
}

/// A calendar showing three months is labelled with all of them and each grid with its month
/// (CalendarBase.test.js "should support labeling with multiple visible months").
#[browser_test]
pub async fn labelled_with_several_months(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["three-months"]).await?;
    assert_that!(
        page.element("#test-calendar-three-months [role=application]")
            .await?
    )
    .has_attribute("aria-label")
    .await
    .is_equal_to("three-months, May 2019 to July 2019");
    assert_that!(grid_labels(page, "three-months").await?).contains_exactly([
        "three-months, May 2019",
        "three-months, June 2019",
        "three-months, July 2019",
    ]);
    Ok(())
}

/// A calendar's `aria-describedby` and `aria-details` refer to the given elements (RAC
/// `Calendar.test.js` "should support aria props on the Calendar").
#[browser_test]
pub async fn described_and_detailed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["described"]).await?;
    let calendar = page
        .element("#test-calendar-described [role=application]")
        .await?;
    assert_that!(calendar)
        .has_attribute("aria-describedby")
        .await
        .is_equal_to("test-calendar-described-description");
    assert_that!(calendar)
        .has_attribute("aria-details")
        .await
        .is_equal_to("test-calendar-described-details");
    Ok(())
}

/// Every part renders its default class (RAC `Calendar.test.js` "should render with default
/// classes").
#[browser_test]
pub async fn default_classes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    for (selector, expected) in [
        ("[role=application]", "leptonic-Calendar"),
        ("[role=grid]", "leptonic-CalendarGrid"),
        ("thead", "leptonic-CalendarGridHeader"),
        ("tbody", "leptonic-CalendarGridBody"),
        ("th", "leptonic-CalendarHeaderCell"),
        ("[role=gridcell]", "leptonic-CalendarCell"),
        (
            "[role=gridcell] > [role=button]",
            "leptonic-CalendarCellButton",
        ),
    ] {
        let part = page
            .first_element(format!("#test-calendar-basic {selector}"))
            .await?;
        assert_that!(part)
            .with_detail_message(selector)
            .has_attribute("class")
            .await
            .is_equal_to(expected);
    }
    Ok(())
}

/// Classes and DOM attributes given to the calendar, its grid and its cells end up on their
/// elements, next to the default classes (RAC `Calendar.test.js` "should render with custom
/// classes", "should support DOM props").
#[browser_test]
pub async fn custom_classes_and_attributes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["classes"]).await?;
    let calendar = page
        .element("#test-calendar-classes [role=application]")
        .await?;
    assert_that!(calendar)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-Calendar calendar");
    assert_that!(calendar)
        .has_attribute("data-foo")
        .await
        .is_equal_to("bar");
    let grid = first_grid(page, "classes").await?;
    assert_that!(grid)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-CalendarGrid grid");
    assert_that!(grid)
        .has_attribute("data-bar")
        .await
        .is_equal_to("baz");
    let cell = date(page, "classes", "Thursday, June 6, 2019").await?;
    assert_that!(cell)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-CalendarCellButton cell");
    assert_that!(cell)
        .has_attribute("data-baz")
        .await
        .is_equal_to("foo");
    Ok(())
}

/// Hovering a date marks it hovered, until the pointer leaves (RAC `Calendar.test.js` "should
/// support hover").
#[browser_test]
pub async fn cell_hover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    let june13 = date(page, "basic", "Thursday, June 13, 2019").await?;
    assert_that!(june13)
        .attribute("data-hovered")
        .await
        .is_none();
    june13.hover().await?;
    june13.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("h1").await?.hover().await?;
    june13.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// A date focused by the keyboard shows a focus ring, which goes with the focus (RAC
/// `Calendar.test.js` "should support focus ring").
#[browser_test]
pub async fn cell_focus_ring(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    let june5 = date(page, "basic", "Wednesday, June 5, 2019").await?;
    assert_that!(june5)
        .attribute("data-focus-visible")
        .await
        .is_none();
    enter(page, "basic").await?;
    june5
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.send_keys(Key::Tab).await?;
    june5.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// A date is pressed while the pointer is down on it (RAC `Calendar.test.js` "should support press
/// state").
#[browser_test]
pub async fn cell_press_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    let june13 = date(page, "basic", "Thursday, June 13, 2019").await?;
    assert_that!(june13)
        .attribute("data-pressed")
        .await
        .is_none();
    let held = june13.press_and_hold().await?;
    june13.wait_for_attr("data-pressed", Some("true")).await?;
    held.release().await?;
    june13.wait_for_attr("data-pressed", None).await?;
    Ok(())
}

/// The grid's weekday names can be short ("Sun") instead of narrow ("S") (RAC `Calendar.test.js`
/// "should support weekdayStyle").
#[browser_test]
pub async fn weekday_style(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["weekday-short"]).await?;
    assert_that!(page.inner_texts("#test-calendar-weekday-short th").await?)
        .contains_exactly(["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]);
    Ok(())
}

/// Setting the value to none through the calendar's state clears the selection (RAC
/// `Calendar.test.js` "should support setting "null" for method setValue").
#[browser_test]
pub async fn clearing_the_value_through_the_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["reset"]).await?;
    let march10 = date(page, "reset", "Tuesday, March 10, 2020").await?;
    march10.click().await?;
    march10.wait_for_attr("data-selected", Some("true")).await?;
    page.element("#test-calendar-reset-reset")
        .await?
        .click()
        .await?;
    march10.wait_for_attr("data-selected", None).await?;
    wait_for_value(page, "reset", "none").await?;
    Ok(())
}

/// The number of dates of the calendar `name` that aren't disabled.
async fn enabled_cells(page: &Page<'_>, name: &str) -> Result<usize, Report> {
    page.count(format!(
        "#test-calendar-{name} [role=gridcell]:not([aria-disabled=true])"
    ))
    .await
}

/// A calendar without a value shows the current month: its heading names it, today is marked
/// and tabbable, and every date of the month is enabled and labelled (CalendarBase.test.js "v3
/// Calendar shows the current month by default").
#[browser_test]
pub async fn shows_the_current_month_by_default(page: &Page<'_>) -> Result<(), Report> {
    shows_the_current_month(page, "today").await
}

/// A range calendar without a value shows the current month (CalendarBase.test.js "v3
/// RangeCalendar shows the current month by default").
#[browser_test]
pub async fn range_shows_the_current_month_by_default(page: &Page<'_>) -> Result<(), Report> {
    shows_the_current_month(page, "range-today").await
}

async fn shows_the_current_month(page: &Page<'_>, name: &str) -> Result<(), Report> {
    page.goto_sections(PATH, &[name]).await?;
    let today = jiff::Zoned::now().date();
    heading(page, name)
        .await?
        .wait_for_inner_text(&today.strftime("%B %Y").to_string())
        .await?;
    let label = format!("Today, {}", today.strftime("%A, %B %-d, %Y"));
    let today_button = page
        .element(format!(
            "#test-calendar-{name} [role=gridcell] > [role=button][aria-label='{label}']"
        ))
        .await?;
    today_button.wait_for_attr("tabindex", Some("0")).await?;
    assert_that!(enabled_cells(page, name).await?)
        .is_equal_to(usize::try_from(today.days_in_month())?);
    assert_that!(first_grid(page, name).await?)
        .attribute("tabindex")
        .await
        .is_none();
    Ok(())
}

/// Dates outside min and max are disabled: only the eleven days from June 10 to 20 are enabled
/// (CalendarBase.test.js "should set aria-disabled on cells outside the valid date range").
/// Pressing a date outside them selects nothing, pressing the first or last available date
/// selects it (Calendar.test.js "does not select a date on click if outside the valid date
/// range").
#[browser_test]
pub async fn press_outside_the_limits(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["min-max"]).await?;
    assert_that!(enabled_cells(page, "min-max").await?).is_equal_to(11);
    date(page, "min-max", "Sunday, June 9, 2019")
        .await?
        .click()
        .await?;
    date(page, "min-max", "Friday, June 21, 2019")
        .await?
        .click()
        .await?;
    value(page, "min-max")
        .await?
        .inner_text_stays("2019-06-15", Duration::from_millis(100))
        .await?;
    date(
        page,
        "min-max",
        "Monday, June 10, 2019, First available date",
    )
    .await?
    .click()
    .await?;
    wait_for_value(page, "min-max", "2019-06-10").await?;
    date(
        page,
        "min-max",
        "Thursday, June 20, 2019, Last available date",
    )
    .await?
    .click()
    .await?;
    wait_for_value(page, "min-max", "2019-06-20").await?;
    Ok(())
}

/// The previous button is disabled when the previous month is before min, the next one when the
/// next month is after max (CalendarBase.test.js "should disable the previous button if outside
/// valid date range", "should disable the next button if outside valid date range").
#[browser_test]
pub async fn limits_disable_the_buttons(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["limits-previous", "limits-next"])
        .await?;
    assert_that!(button(page, "limits-previous", "Previous").await?)
        .enabled()
        .await
        .is_false();
    assert_that!(button(page, "limits-previous", "Next").await?)
        .enabled()
        .await
        .is_true();
    assert_that!(button(page, "limits-next", "Previous").await?)
        .enabled()
        .await
        .is_true();
    assert_that!(button(page, "limits-next", "Next").await?)
        .enabled()
        .await
        .is_false();
    Ok(())
}

/// A previous or next button disabled by paging while it has the focus hands the focus to the
/// focused date, a button still enabled keeps it (CalendarBase.test.js "should move focus when the
/// previous or next buttons become disabled").
async fn buttons_disabled_while_focused(page: &Page<'_>, name: &str) -> Result<(), Report> {
    page.goto_sections(PATH, &[name]).await?;
    let previous = button(page, name, "Previous").await?;
    let next = button(page, name, "Next").await?;
    previous.click().await?;
    previous.wait_for_prop("disabled", "true").await?;
    page.wait_for_focus(&date(page, name, "Sunday, February 10, 2019").await?)
        .await?;
    next.click().await?;
    previous.wait_for_prop("disabled", "false").await?;
    page.wait_for_focus(&next).await?;
    next.click().await?;
    next.wait_for_prop("disabled", "true").await?;
    page.wait_for_focus(&date(page, name, "Wednesday, April 10, 2019").await?)
        .await?;
    Ok(())
}

/// A calendar's previous or next button disabled while focused moves the focus to the grid
/// (CalendarBase.test.js "v3 Calendar should move focus when the previous or next buttons become
/// disabled").
#[browser_test]
pub async fn buttons_disabled_while_focused_move_the_focus(page: &Page<'_>) -> Result<(), Report> {
    buttons_disabled_while_focused(page, "buttons-focus").await
}

/// A range calendar's previous or next button disabled while focused moves the focus to the grid
/// (CalendarBase.test.js "v3 RangeCalendar should move focus when the previous or next buttons
/// become disabled").
#[browser_test]
pub async fn range_buttons_disabled_while_focused_move_the_focus(
    page: &Page<'_>,
) -> Result<(), Report> {
    buttons_disabled_while_focused(page, "range-buttons-focus").await
}

/// With min June 2 and max June 8, no key moves the focus beyond them (CalendarBase.test.js
/// "should not move the focused date outside the valid range").
#[browser_test]
pub async fn keys_stop_at_the_limits(page: &Page<'_>) -> Result<(), Report> {
    let name = "week-limits";
    let june = "week-limits, June 2019";
    for (key, shift, focused) in [
        (Key::Up, false, "Sunday, June 2, 2019"),
        (Key::Down, false, "Saturday, June 8, 2019"),
        (Key::Home, false, "Sunday, June 2, 2019"),
        (Key::End, false, "Saturday, June 8, 2019"),
        (Key::PageUp, false, "Sunday, June 2, 2019"),
        (Key::PageDown, false, "Saturday, June 8, 2019"),
        (Key::PageUp, true, "Sunday, June 2, 2019"),
        (Key::PageDown, true, "Saturday, June 8, 2019"),
    ] {
        key_moves(page, name, key, 1, shift, focused, june).await?;
    }
    // From an end, the arrows don't leave the limits.
    key_moves(
        page,
        name,
        Key::Home,
        1,
        false,
        "Sunday, June 2, 2019",
        june,
    )
    .await?;
    press_keys(page, &[Key::Left]).await?;
    page.focus_stays(
        &date(page, name, "Sunday, June 2, 2019").await?,
        Duration::from_millis(100),
    )
    .await?;
    key_moves(
        page,
        name,
        Key::End,
        1,
        false,
        "Saturday, June 8, 2019",
        june,
    )
    .await?;
    press_keys(page, &[Key::Right]).await?;
    page.focus_stays(
        &date(page, name, "Saturday, June 8, 2019").await?,
        Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// At the last month jiff supports (December 9999), the next button is disabled and the month
/// ends on the 31st (CalendarBase.test.js "should handle maximum dates in a calendar system").
#[browser_test]
pub async fn maximum_date(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["max-date"]).await?;
    let cells = page
        .inner_texts(
            "#test-calendar-max-date [role=gridcell] > [role=button]:not([data-outside-month])",
        )
        .await?;
    assert_that!(cells.last().cloned()).is_equal_to(Some("31".to_owned()));
    assert_that!(button(page, "max-date", "Previous").await?)
        .enabled()
        .await
        .is_true();
    assert_that!(button(page, "max-date", "Next").await?)
        .enabled()
        .await
        .is_false();
    Ok(())
}

/// Dates before Christ name their era: the calendar's label and heading ("February 5 BC") and the
/// selected date's label (CalendarBase.test.js "should show era for BC dates"; Calendar.test.js
/// "should show era for BC dates").
#[browser_test]
pub async fn era_of_dates_before_christ(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["bc"]).await?;
    assert_that!(page.element("#test-calendar-bc [role=application]").await?)
        .has_attribute("aria-label")
        .await
        .is_equal_to("bc, February 5 BC");
    assert_that!(heading(page, "bc").await?)
        .inner_text()
        .await
        .is_equal_to("February 5 BC");
    date(page, "bc", "Saturday, February 3, 5 BC selected").await?;
    Ok(())
}

/// A range from before Christ to after names both eras in its description (RangeCalendar.test.js
/// "should show era for BC dates").
#[browser_test]
pub async fn range_era_of_dates_before_christ(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-bc"]).await?;
    let start = page
        .element("#test-calendar-range-bc [role=button][data-selection-start]")
        .await?;
    assert_that!(start)
        .has_attribute("aria-label")
        .await
        .is_equal_to(
            "Selected Range: Thursday, December 14, 1 BC to Monday, January 22, 1 AD, \
             Thursday, December 14, 1 BC selected",
        );
    Ok(())
}

/// Selecting a date before Christ and paging announce them with their era (Calendar.test.js
/// "includes era in BC dates").
#[browser_test]
pub async fn announcements_of_dates_before_christ(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["bc"]).await?;
    day_of_month(page, "bc", 17).await?.click().await?;
    wait_for_announcement(page, "Selected Date: Saturday, February 17, 5 BC").await?;
    button(page, "bc", "Next").await?.click().await?;
    wait_for_announcement(page, "March 5 BC").await?;
    Ok(())
}

/// Selecting a range before Christ and paging announce them with their era (RangeCalendar.test.js
/// "includes era in BC dates"; the range's ends are formatted apart: ICU4X has no range format).
#[browser_test]
pub async fn range_announcements_of_dates_before_christ(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-bc-days"]).await?;
    day_of_month(page, "range-bc-days", 17)
        .await?
        .click()
        .await?;
    day_of_month(page, "range-bc-days", 23)
        .await?
        .click()
        .await?;
    wait_for_announcement(
        page,
        "Selected Range: Saturday, February 17, 5 BC to Friday, February 23, 5 BC",
    )
    .await?;
    button(page, "range-bc-days", "Next").await?.click().await?;
    wait_for_announcement(page, "March 5 BC").await?;
    Ok(())
}

/// Paging announces the new month and selecting a date announces it (Calendar.test.js "announces
/// when the current month changes", "announces when the selected date changes").
#[browser_test]
pub async fn calendar_announcements(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    date(page, "basic", "Monday, June 17, 2019")
        .await?
        .click()
        .await?;
    wait_for_announcement(page, "Selected Date: Monday, June 17, 2019").await?;
    button(page, "basic", "Next").await?.click().await?;
    wait_for_announcement(page, "July 2019").await?;
    Ok(())
}

/// The labels of the grids of a three-month calendar `name` after loading it.
async fn three_month_grids(page: &Page<'_>, name: &str) -> Result<Vec<String>, Report> {
    page.goto_sections(PATH, &[name]).await?;
    grid_labels(page, name).await
}

/// The selected date sits in the first, second or third of three months as aligned at the start,
/// center or end (RAC `Calendar.test.js`, Calendar.test.js "should align the initial value $name").
#[browser_test]
pub async fn alignment_of_the_initial_value(page: &Page<'_>) -> Result<(), Report> {
    for (name, months) in [
        ("align-start", ["February 2020", "March 2020", "April 2020"]),
        (
            "align-center",
            ["January 2020", "February 2020", "March 2020"],
        ),
        (
            "align-end",
            ["December 2019", "January 2020", "February 2020"],
        ),
    ] {
        let expected: Vec<String> = months
            .iter()
            .map(|month| format!("{name}, {month}"))
            .collect();
        assert_that!(three_month_grids(page, name).await?).is_equal_to(expected);
    }
    Ok(())
}

/// The selected range sits in the first, second or third of three months as aligned at the start,
/// center or end (RAC `RangeCalendar.test.tsx`, RangeCalendar.test.js "should align the initial
/// value $name").
#[browser_test]
pub async fn range_alignment_of_the_initial_value(page: &Page<'_>) -> Result<(), Report> {
    for (name, months) in [
        (
            "range-align-start",
            ["February 2020", "March 2020", "April 2020"],
        ),
        (
            "range-align-center",
            ["January 2020", "February 2020", "March 2020"],
        ),
        (
            "range-align-end",
            ["December 2019", "January 2020", "February 2020"],
        ),
    ] {
        let expected: Vec<String> = months
            .iter()
            .map(|month| format!("{name}, {month}"))
            .collect();
        assert_that!(three_month_grids(page, name).await?).is_equal_to(expected);
    }
    Ok(())
}

/// The index of the grid of the calendar `name` holding `element`.
async fn grid_index(page: &Page<'_>, name: &str, element: &WebElement) -> Result<usize, Report> {
    for (index, grid) in grids(page, name).await?.iter().enumerate() {
        let inside = grid
            .elements("[role=button][data-selected]")
            .await?
            .contains(element);
        if inside {
            return Ok(index);
        }
    }
    bail!("the element is in no grid of {name}")
}

/// The grid indices of the selected dates of the calendar `name`.
async fn selected_grid_indices(page: &Page<'_>, name: &str) -> Result<Vec<usize>, Report> {
    let mut indices = Vec::new();
    for selected in page
        .elements(format!(
            "#test-calendar-{name} [role=button][data-selected]"
        ))
        .await?
    {
        indices.push(grid_index(page, name, &selected).await?);
    }
    Ok(indices)
}

/// Three months center the selected date: it is in the second (Calendar.test.js "should center the
/// selected date if multiple months are visible"); a min just before it moves it to the first
/// ("should constrain the visible region depending on the minValue").
#[browser_test]
pub async fn several_months_place_the_selected_date(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["three-months", "min-visible"])
        .await?;
    assert_that!(selected_grid_indices(page, "three-months").await?).contains_exactly([1]);
    assert_that!(selected_grid_indices(page, "min-visible").await?).contains_exactly([0]);
    Ok(())
}

/// Three months center the selected range (RangeCalendar.test.js "should center the selected range
/// if multiple months are visible"); a min just before it moves it to the first month ("should
/// constrain the visible region depending on the minValue"), as does a range too long to center
/// ("should start align the selected range if it would go out of view when centered").
#[browser_test]
pub async fn several_months_place_the_selected_range(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(
        PATH,
        &["range-align-center", "range-min-visible", "range-wide"],
    )
    .await?;
    assert_that!(selected_grid_indices(page, "range-align-center").await?)
        .contains_exactly([1, 1, 1, 1, 1, 1, 1, 1]);
    assert_that!(selected_grid_indices(page, "range-min-visible").await?)
        .contains_exactly([0, 0, 0, 0, 0, 0, 0, 0]);
    let first = page
        .element("#test-calendar-range-wide [role=button][data-selection-start]")
        .await?;
    assert_that!(grid_index(page, "range-wide", &first).await?).is_equal_to(0);
    Ok(())
}

/// The buttons of a three-month calendar page by three months (CalendarBase.test.js "should change
/// the month when previous or next buttons are clicked and multiple months are visible").
async fn three_months_paging(page: &Page<'_>, name: &str) -> Result<(), Report> {
    page.goto_sections(PATH, &[name]).await?;
    let months = |months: [&str; 3]| -> Vec<String> {
        months
            .iter()
            .map(|month| format!("{name}, {month}"))
            .collect()
    };
    let initial = months(["May 2019", "June 2019", "July 2019"]);
    let initial: Vec<&str> = initial.iter().map(String::as_str).collect();
    wait_for_grid_labels(page, name, &initial).await?;
    button(page, name, "Next").await?.click().await?;
    let later = months(["August 2019", "September 2019", "October 2019"]);
    let later: Vec<&str> = later.iter().map(String::as_str).collect();
    wait_for_grid_labels(page, name, &later).await?;
    button(page, name, "Previous").await?.click().await?;
    wait_for_grid_labels(page, name, &initial).await?;
    Ok(())
}

/// A three-month calendar pages by three months (CalendarBase.test.js "v3 Calendar should change
/// the month when previous or next buttons are clicked and multiple months are visible").
#[browser_test]
pub async fn three_months_paging_by_the_buttons(page: &Page<'_>) -> Result<(), Report> {
    three_months_paging(page, "three-months").await
}

/// A three-month range calendar pages by three months (CalendarBase.test.js "v3 RangeCalendar
/// should change the month when previous or next buttons are clicked and multiple months are
/// visible").
#[browser_test]
pub async fn range_three_months_paging_by_the_buttons(page: &Page<'_>) -> Result<(), Report> {
    three_months_paging(page, "range-three-months").await
}

/// Moving the focus past the last visible month with the keyboard pages by three months, and back
/// again (CalendarBase.test.js "should change the month when keyboard navigating and multiple
/// months are visible").
#[browser_test]
pub async fn three_months_paging_by_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    let name = "three-months";
    page.goto_sections(PATH, &[name]).await?;
    enter(page, name).await?;
    // From June 5 to June 30, July 30 and July 31.
    press_keys(page, &[Key::End, Key::PageDown, Key::End]).await?;
    page.wait_for_focus(&date(page, name, "Wednesday, July 31, 2019").await?)
        .await?;
    press_keys(page, &[Key::Right]).await?;
    wait_for_grid_labels(
        page,
        name,
        &[
            "three-months, August 2019",
            "three-months, September 2019",
            "three-months, October 2019",
        ],
    )
    .await?;
    press_keys(page, &[Key::Left]).await?;
    wait_for_grid_labels(
        page,
        name,
        &[
            "three-months, May 2019",
            "three-months, June 2019",
            "three-months, July 2019",
        ],
    )
    .await?;
    Ok(())
}

/// In a three-month calendar, Home/End move to the focused month's ends and Page Up/Down by a
/// month, without paging (CalendarBase.test.js "should move the focused date to the start or end of
/// the month with the home/end keys when multiple months are visible", "should move the focused
/// date by one month with the page up/page down keys when multiple months are visible").
#[browser_test]
pub async fn three_months_home_end_and_page_keys(page: &Page<'_>) -> Result<(), Report> {
    let name = "three-months";
    let may = "three-months, May 2019";
    key_moves(
        page,
        name,
        Key::Home,
        1,
        false,
        "Saturday, June 1, 2019",
        may,
    )
    .await?;
    key_moves(page, name, Key::End, 1, false, "Sunday, June 30, 2019", may).await?;
    key_moves(
        page,
        name,
        Key::PageUp,
        1,
        false,
        "Sunday, May 5, 2019",
        may,
    )
    .await?;
    key_moves(
        page,
        name,
        Key::PageDown,
        1,
        false,
        "Friday, July 5, 2019",
        may,
    )
    .await?;
    Ok(())
}

/// Dates in unavailable intervals are disabled and can't be selected, the others can
/// (CalendarBase.test.js "v3 Calendar should set aria-disabled on cells for which isDateUnavailable
/// returns true").
#[browser_test]
pub async fn unavailable_intervals(page: &Page<'_>) -> Result<(), Report> {
    unavailable_intervals_of(page, "intervals").await
}

/// A range calendar's dates in unavailable intervals are disabled and can't be selected
/// (CalendarBase.test.js "v3 RangeCalendar should set aria-disabled on cells for which
/// isDateUnavailable returns true").
#[browser_test]
pub async fn range_unavailable_intervals(page: &Page<'_>) -> Result<(), Report> {
    unavailable_intervals_of(page, "range-intervals").await
}

async fn unavailable_intervals_of(page: &Page<'_>, name: &str) -> Result<(), Report> {
    page.goto_sections(PATH, &[name]).await?;
    for day in [6, 7, 8, 9, 10, 22, 23, 24, 25, 26] {
        assert_that!(day_of_month(page, name, day).await?)
            .has_attribute("aria-disabled")
            .await
            .with_detail_message(format!("December {day}"))
            .is_equal_to("true");
    }
    assert_that!(enabled_cells(page, name).await?).is_equal_to(21);
    let december22 = day_of_month(page, name, 22).await?;
    december22.click().await?;
    cell(&december22)
        .await?
        .attr_stays("aria-selected", None, Duration::from_millis(100))
        .await?;
    let december12 = day_of_month(page, name, 12).await?;
    december12.click().await?;
    cell(&december12)
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    Ok(())
}

/// A default focused date is focused on mounting (auto focus), the arrows move it and the
/// calendar reports the new one (CalendarBase.test.js "should support defaultFocusedValue").
#[browser_test]
pub async fn default_focused_value(page: &Page<'_>) -> Result<(), Report> {
    let name = "focus-default";
    page.goto_sections(PATH, &[name]).await?;
    wait_for_grid_label(page, name, "focus-default, June 2019").await?;
    page.wait_for_focus(&date(page, name, "Wednesday, June 5, 2019").await?)
        .await?;
    press_keys(page, &[Key::Right]).await?;
    page.wait_for_focus(&date(page, name, "Thursday, June 6, 2019").await?)
        .await?;
    page.element("#test-calendar-focus-default-focus-change")
        .await?
        .wait_for_inner_text("2019-06-06")
        .await?;
    Ok(())
}

/// A controlled focused date stays where the app keeps it: the arrows only report the date they
/// would move to (CalendarBase.test.js "should support controlled focusedValue").
#[browser_test]
pub async fn controlled_focused_value(page: &Page<'_>) -> Result<(), Report> {
    let name = "focus-controlled";
    page.goto_sections(PATH, &[name]).await?;
    let june5 = date(page, name, "Wednesday, June 5, 2019").await?;
    page.wait_for_focus(&june5).await?;
    page.send_keys(Key::Right).await?;
    page.element("#test-calendar-focus-controlled-focus-change")
        .await?
        .wait_for_inner_text("2019-06-06")
        .await?;
    page.focus_stays(&june5, Duration::from_millis(100)).await?;
    Ok(())
}

/// A default focused date before min is moved to min (CalendarBase.test.js "should constrain
/// defaultFocusedValue").
#[browser_test]
pub async fn default_focused_value_constrained(page: &Page<'_>) -> Result<(), Report> {
    let name = "focus-constrained";
    page.goto_sections(PATH, &[name]).await?;
    wait_for_grid_label(page, name, "focus-constrained, July 2019").await?;
    page.wait_for_focus(&date(page, name, "Friday, July 5, 2019").await?)
        .await?;
    Ok(())
}

/// A controlled focused date before min shows and focuses min (CalendarBase.test.js "should
/// constrain focusedValue").
#[browser_test]
pub async fn controlled_focused_value_constrained(page: &Page<'_>) -> Result<(), Report> {
    let name = "focus-controlled-constrained";
    page.goto_sections(PATH, &[name]).await?;
    wait_for_grid_label(page, name, "focus-controlled-constrained, July 2019").await?;
    page.wait_for_focus(&date(page, name, "Friday, July 5, 2019").await?)
        .await?;
    Ok(())
}

/// An auto-focused calendar without a value focuses today (CalendarBase.test.js "should focus
/// today if autoFocus is set and there is no selected value").
#[browser_test]
pub async fn auto_focus_today(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["auto-today"]).await?;
    let today = page
        .element("#test-calendar-auto-today [role=gridcell] > [role=button][aria-label^='Today, ']")
        .await?;
    page.wait_for_focus(&today).await?;
    Ok(())
}

/// An auto-focused calendar focuses its selected date (Calendar.test.js "should focus the selected
/// date if autoFocus is set").
#[browser_test]
pub async fn auto_focus_the_selected_date(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["auto-selected"]).await?;
    let selected = date(page, "auto-selected", "Sunday, February 3, 2019 selected").await?;
    page.wait_for_focus(&selected).await?;
    assert_that!(cell(&selected).await?)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    Ok(())
}

/// An auto-focused range calendar focuses the first selected date (RangeCalendar.test.js "should
/// focus the first selected date if autoFocus is set").
#[browser_test]
pub async fn range_auto_focus_the_first_selected_date(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-auto"]).await?;
    let start = page
        .element("#test-calendar-range-auto [role=button][data-selection-start]")
        .await?;
    page.wait_for_focus(&start).await?;
    assert_that!(start).inner_text().await.is_equal_to("3");
    Ok(())
}

/// A controlled calendar keeps its value: Enter and Space on other dates and presses only report
/// them (Calendar.test.js "selects a date on keyDown Enter/Space (controlled)", "selects a date on
/// click (controlled)").
#[browser_test]
pub async fn controlled_selection(page: &Page<'_>) -> Result<(), Report> {
    let name = "controlled";
    page.goto_sections(PATH, &[name]).await?;
    let june5 = date(page, name, "Wednesday, June 5, 2019").await?;
    page.wait_for_focus(&june5).await?;
    let changes = page.element("#test-calendar-controlled-changes").await?;
    press_keys(page, &[Key::Left, Key::Enter]).await?;
    changes.wait_for_inner_text("2019-06-04").await?;
    press_keys(page, &[Key::Left, Key::Space]).await?;
    changes
        .wait_for_inner_text("2019-06-04, 2019-06-03")
        .await?;
    date(page, name, "Monday, June 17, 2019")
        .await?
        .click()
        .await?;
    changes
        .wait_for_inner_text("2019-06-04, 2019-06-03, 2019-06-17")
        .await?;
    assert_that!(june5)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Wednesday, June 5, 2019 selected");
    Ok(())
}

/// A read-only calendar selects nothing with Enter or Space (Calendar.test.js "does not select a
/// date on keyDown Enter/Space if isReadOnly").
#[browser_test]
pub async fn read_only_keyboard_selection(page: &Page<'_>) -> Result<(), Report> {
    let name = "controlled-read-only";
    page.goto_sections(PATH, &[name]).await?;
    page.wait_for_focus(&date(page, name, "Wednesday, June 5, 2019").await?)
        .await?;
    press_keys(page, &[Key::Left, Key::Enter, Key::Left, Key::Space]).await?;
    page.wait_for_focus(&date(page, name, "Monday, June 3, 2019").await?)
        .await?;
    page.element("#test-calendar-controlled-read-only-changes")
        .await?
        .inner_text_stays("none", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A valid selected date isn't invalid and isn't described by the error message (Calendar.test.js
/// "does not show error message without isInvalid").
#[browser_test]
pub async fn valid_selection_has_no_error(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    let june5 = date(page, "basic", "Wednesday, June 5, 2019").await?;
    assert_that!(june5)
        .attribute("aria-invalid")
        .await
        .is_none();
    assert_that!(june5)
        .attribute("aria-describedby")
        .await
        .is_none();
    assert_that!(cell(&june5).await?)
        .attribute("aria-invalid")
        .await
        .is_none();
    Ok(())
}

/// A selected date that is unavailable makes the calendar invalid: the date is marked invalid and
/// described by the error message (Calendar.test.js "automatically marks selection as invalid
/// using isDateUnavailable").
#[browser_test]
pub async fn unavailable_selection_is_invalid(page: &Page<'_>) -> Result<(), Report> {
    let name = "invalid-unavailable";
    page.goto_sections(PATH, &[name]).await?;
    let march5 = date(page, name, "Saturday, March 5, 2022 selected").await?;
    assert_that!(march5)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    assert_that!(cell(&march5).await?)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    assert_that!(march5)
        .accessible_description()
        .await
        .is_equal_to("Invalid date");
    Ok(())
}

/// The selected range's dates are labelled as selected, its ends with the range's description
/// (RangeCalendar.test.js "should render a calendar with a defaultValue", "should render a calendar
/// with a value").
#[browser_test]
pub async fn range_labels(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range"]).await?;
    let description = "Selected Range: Wednesday, June 5, 2019 to Monday, June 10, 2019";
    let mut labels = Vec::new();
    for button in page
        .elements("#test-calendar-range [role=button][data-selected]")
        .await?
    {
        labels.push(button.attr("aria-label").await?.unwrap_or_default());
    }
    assert_that!(labels).is_equal_to(vec![
        format!("{description}, Wednesday, June 5, 2019 selected"),
        "Thursday, June 6, 2019 selected".to_owned(),
        "Friday, June 7, 2019 selected".to_owned(),
        "Saturday, June 8, 2019 selected".to_owned(),
        "Sunday, June 9, 2019 selected".to_owned(),
        format!("{description}, Monday, June 10, 2019 selected"),
    ]);
    assert_that!(
        page.count("#test-calendar-range [role=gridcell][aria-selected=true]")
            .await?
    )
    .is_equal_to(6);
    assert_that!(enabled_cells(page, "range").await?).is_equal_to(30);
    Ok(())
}

/// A range across months shows its selected dates in either month, its ends labelled with the
/// range; the buttons page there and back and keep the focus (RangeCalendar.test.js "should show
/// selected dates across multiple months").
#[browser_test]
pub async fn range_across_months(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-across-months";
    page.goto_sections(PATH, &[name]).await?;
    let description = "Selected Range: Thursday, June 20, 2019 to Wednesday, July 10, 2019";
    wait_for_selected_days(
        page,
        name,
        &[
            "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30",
        ],
    )
    .await?;
    date(
        page,
        name,
        &format!("{description}, Thursday, June 20, 2019 selected"),
    )
    .await?;
    let next = button(page, name, "Next").await?;
    next.click().await?;
    wait_for_selected_days(
        page,
        name,
        &["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
    )
    .await?;
    date(
        page,
        name,
        &format!("{description}, Wednesday, July 10, 2019 selected"),
    )
    .await?;
    page.wait_for_focus(&next).await?;
    let previous = button(page, name, "Previous").await?;
    previous.click().await?;
    wait_for_selected_days(
        page,
        name,
        &[
            "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30",
        ],
    )
    .await?;
    page.wait_for_focus(&previous).await?;
    Ok(())
}

/// The focused date says how to start a range ("Click to start selecting date range"); once
/// Enter started one, how to finish it (RangeCalendar.test.js "$Name adds a range selection prompt
/// to the focused cell").
#[browser_test]
pub async fn range_selection_prompts(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-today"]).await?;
    enter(page, "range-today").await?;
    let today = page.focused_element().await?;
    assert_that!(today)
        .accessible_description()
        .await
        .is_equal_to("Click to start selecting date range");
    press_keys(page, &[Key::Enter]).await?;
    cell(&today)
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    // Completing the key press advances focus to the next available day.
    let focused = page.focused_element().await?;
    assert_that!(|| focused.accessible_description())
        .eventually_ok()
        .matches(eq("Click to finish selecting date range"))
        .await;
    Ok(())
}

/// A controlled range calendar keeps its range: a range selected with the keyboard is only
/// reported (RangeCalendar.test.js "$Name can select a range with the keyboard (controlled)").
#[browser_test]
pub async fn range_keyboard_selection_controlled(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-controlled";
    page.goto_sections(PATH, &[name]).await?;
    page.element("h1").await?.hover().await?;
    enter(page, name).await?;
    press_keys(page, &[Key::Left, Key::Enter]).await?;
    // Starting a range moves the focus on by a day.
    wait_for_selected_days(page, name, &["4", "5"]).await?;
    press_keys(page, &[Key::Right, Key::Right, Key::Right, Key::Right]).await?;
    wait_for_selected_days(page, name, &["4", "5", "6", "7", "8", "9"]).await?;
    press_keys(page, &[Key::Space]).await?;
    page.element("#test-calendar-range-controlled-changes")
        .await?
        .wait_for_inner_text("2019-06-04 - 2019-06-09")
        .await?;
    wait_for_selected_days(page, name, &["5", "6", "7", "8", "9", "10"]).await?;
    Ok(())
}

/// A controlled range calendar keeps its range: a range selected by presses is only reported
/// (RangeCalendar.test.js "$Name selects a range with the mouse (controlled)").
#[browser_test]
pub async fn range_press_selection_controlled(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-controlled";
    page.goto_sections(PATH, &[name]).await?;
    day_of_month(page, name, 17).await?.click().await?;
    wait_for_selected_days(page, name, &["17"]).await?;
    day_of_month(page, name, 10).await?.hover().await?;
    wait_for_selected_days(
        page,
        name,
        &["10", "11", "12", "13", "14", "15", "16", "17"],
    )
    .await?;
    day_of_month(page, name, 7).await?.click().await?;
    page.element("#test-calendar-range-controlled-changes")
        .await?
        .wait_for_inner_text("2019-06-07 - 2019-06-17")
        .await?;
    wait_for_selected_days(page, name, &["5", "6", "7", "8", "9", "10"]).await?;
    Ok(())
}

/// A read-only range calendar starts no range with Enter (RangeCalendar.test.js "does not enter
/// selection mode with the keyboard if isReadOnly").
#[browser_test]
pub async fn range_read_only_keyboard(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-today-read-only";
    page.goto_sections(PATH, &[name]).await?;
    let today = page
        .element("#test-calendar-range-today-read-only [role=button][aria-label^='Today, ']")
        .await?;
    page.wait_for_focus(&today).await?;
    press_keys(page, &[Key::Enter]).await?;
    cell(&today)
        .await?
        .attr_stays("aria-selected", None, Duration::from_millis(100))
        .await?;
    page.focus_stays(&today, Duration::from_millis(100)).await?;
    Ok(())
}

/// A read-only range calendar starts no range by presses, neither on a date nor on the range's
/// end, and keeps its range (RangeCalendar.test.js "does not enter selection mode with the mouse if
/// isReadOnly", "does not enter selection mode with the mouse on range end if isReadOnly").
#[browser_test]
pub async fn range_read_only_pointer(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-read-only";
    page.goto_sections(PATH, &[name]).await?;
    let all = [
        "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20",
    ];
    day_of_month(page, name, 20).await?.click().await?;
    day_of_month(page, name, 15).await?.click().await?;
    day_of_month(page, name, 25).await?.click().await?;
    page.settle().await?;
    assert_that!(|| selected_days(page, name))
        .consistently_ok()
        .matches(eq(all))
        .await;
    Ok(())
}

/// A disabled range calendar has only disabled dates and selects nothing (RangeCalendar.test.js
/// "$Name does not select a date on click if isDisabled"; CalendarBase.test.js "v3 RangeCalendar
/// should set aria-disabled when isDisabled").
#[browser_test]
pub async fn range_disabled(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-disabled";
    page.goto_sections(PATH, &[name]).await?;
    assert_that!(enabled_cells(page, name).await?).is_equal_to(0);
    assert_that!(first_grid(page, name).await?)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    for label in ["Previous", "Next"] {
        assert_that!(button(page, name, label).await?)
            .enabled()
            .await
            .is_false();
    }
    day_of_month(page, name, 17).await?.click().await?;
    page.count_stays(
        "#test-calendar-range-disabled [data-selected]",
        0,
        Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// Presses outside min and max start no range; a range from the first to the last available date
/// is selected (RangeCalendar.test.js "$Name does not select a date on click if outside the valid
/// date range").
#[browser_test]
pub async fn range_press_outside_the_limits(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-min-max";
    page.goto_sections(PATH, &[name]).await?;
    date(page, name, "Sunday, February 3, 2019")
        .await?
        .click()
        .await?;
    date(page, name, "Sunday, February 17, 2019")
        .await?
        .click()
        .await?;
    page.settle().await?;
    assert_that!(|| selected_days(page, name))
        .consistently_ok()
        .matches(eq(["8", "9", "10", "11", "12", "13", "14", "15"]))
        .await;
    date(
        page,
        name,
        "Tuesday, February 5, 2019, First available date",
    )
    .await?
    .click()
    .await?;
    wait_for_selected_days(page, name, &["5"]).await?;
    date(page, name, "Friday, February 15, 2019, Last available date")
        .await?
        .click()
        .await?;
    wait_for_value(page, name, "2019-02-05 - 2019-02-15").await?;
    Ok(())
}

/// Escape cancels a range started by a press and hovered, showing the range before again
/// (RangeCalendar.test.js "$Name cancels the selection when the escape key is pressed").
#[browser_test]
pub async fn range_escape_cancels_a_pressed_range(page: &Page<'_>) -> Result<(), Report> {
    let name = "range";
    page.goto_sections(PATH, &[name]).await?;
    day_of_month(page, name, 17).await?.click().await?;
    wait_for_selected_days(page, name, &["17"]).await?;
    day_of_month(page, name, 10).await?.hover().await?;
    wait_for_selected_days(
        page,
        name,
        &["10", "11", "12", "13", "14", "15", "16", "17"],
    )
    .await?;
    page.send_keys(Key::Escape).await?;
    wait_for_selected_days(page, name, &["5", "6", "7", "8", "9", "10"]).await?;
    value(page, name)
        .await?
        .inner_text_stays("2019-06-05 - 2019-06-10", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Pressing the start of the range and dragging moves the start, keeping the end; the range is
/// committed on release (RangeCalendar.test.js "allows dragging the start of the highlighted range
/// to modify it").
#[browser_test]
pub async fn range_dragging_the_start(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-mid";
    page.goto_sections(PATH, &[name]).await?;
    let all = [
        "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20",
    ];
    let held = day_of_month(page, name, 10).await?.press_and_hold().await?;
    // Pressing an end of the range keeps it.
    page.settle().await?;
    assert_that!(selected_days(page, name).await?).is_equal_to(all.map(str::to_owned).to_vec());
    held.move_to(&day_of_month(page, name, 11).await?).await?;
    wait_for_selected_days(page, name, &all[1..]).await?;
    held.move_to(&day_of_month(page, name, 8).await?).await?;
    wait_for_selected_days(
        page,
        name,
        &[
            "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20",
        ],
    )
    .await?;
    value(page, name)
        .await?
        .inner_text_stays("2019-06-10 - 2019-06-20", Duration::from_millis(100))
        .await?;
    held.release().await?;
    wait_for_value(page, name, "2019-06-08 - 2019-06-20").await?;
    Ok(())
}

/// A drag released outside the calendar commits the range dragged (RangeCalendar.test.js
/// "releasing drag outside calendar commits it").
#[browser_test]
pub async fn range_drag_released_outside(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-mid";
    page.goto_sections(PATH, &[name]).await?;
    let held = day_of_month(page, name, 22).await?.press_and_hold().await?;
    wait_for_selected_days(page, name, &["22"]).await?;
    held.move_to(&day_of_month(page, name, 25).await?).await?;
    wait_for_selected_days(page, name, &["22", "23", "24", "25"]).await?;
    held.move_to(&page.element("#test-calendar-range-mid-after").await?)
        .await?;
    held.release().await?;
    wait_for_value(page, name, "2019-06-22 - 2019-06-25").await?;
    Ok(())
}

/// Pressing outside the calendar commits a started range up to the hovered date, which it
/// highlighted before (the default commit behavior `Select`; RangeCalendar.test.js "clicking
/// outside calendar commits selection").
#[browser_test]
pub async fn range_committed_by_an_outside_press(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-mid";
    page.goto_sections(PATH, &[name]).await?;
    day_of_month(page, name, 22).await?.click().await?;
    wait_for_selected_days(page, name, &["22"]).await?;
    day_of_month(page, name, 25).await?.hover().await?;
    wait_for_selected_days(page, name, &["22", "23", "24", "25"]).await?;
    value(page, name)
        .await?
        .inner_text_stays("2019-06-10 - 2019-06-20", Duration::from_millis(100))
        .await?;
    page.element("h1").await?.click().await?;
    wait_for_value(page, name, "2019-06-22 - 2019-06-25").await?;
    wait_for_selected_days(page, name, &["22", "23", "24", "25"]).await?;
    Ok(())
}

/// Paging during a started range doesn't finish it: the range follows the focus into the next
/// month and is finished there (RangeCalendar.test.js "clicking on next/previous buttons does not
/// commit selection").
#[browser_test]
pub async fn range_paging_does_not_commit(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-mid";
    page.goto_sections(PATH, &[name]).await?;
    day_of_month(page, name, 22).await?.click().await?;
    day_of_month(page, name, 25).await?.hover().await?;
    wait_for_selected_days(page, name, &["22", "23", "24", "25"]).await?;
    button(page, name, "Next").await?.click().await?;
    wait_for_grid_label(page, name, "range-mid, July 2019").await?;
    let changes = page.element("#test-calendar-range-mid-changes").await?;
    changes
        .inner_text_stays("none", Duration::from_millis(100))
        .await?;
    let july10 = day_of_month(page, name, 10).await?;
    july10.hover().await?;
    wait_for_selected_days(
        page,
        name,
        &["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
    )
    .await?;
    july10.click().await?;
    changes
        .wait_for_inner_text("2019-06-22 - 2019-07-10")
        .await?;
    Ok(())
}

/// Pressing the start, the end or a date inside the range (without dragging) starts a new range
/// there; hovering and pressing another date finishes it (RangeCalendar.test.js "clicking on the
/// start of the highlighted range starts a new selection", "clicking on the end of the highlighted
/// range starts a new selection", "mouse down in the middle of the highlighted range starts a new
/// selection").
async fn range_restarted_by_a_press(
    page: &Page<'_>,
    pressed: u8,
    hovered: u8,
    expected_days: &[&str],
    expected_value: &str,
) -> Result<(), Report> {
    let name = "range-mid";
    page.goto_sections(PATH, &[name]).await?;
    day_of_month(page, name, pressed).await?.click().await?;
    let pressed_text = pressed.to_string();
    wait_for_selected_days(page, name, &[pressed_text.as_str()]).await?;
    let hovered = day_of_month(page, name, hovered).await?;
    hovered.hover().await?;
    wait_for_selected_days(page, name, expected_days).await?;
    hovered.click().await?;
    wait_for_value(page, name, expected_value).await?;
    Ok(())
}

/// Pressing the range's start starts a new range there (RangeCalendar.test.js "clicking on the
/// start of the highlighted range starts a new selection").
#[browser_test]
pub async fn range_press_on_the_start_starts_a_new_range(page: &Page<'_>) -> Result<(), Report> {
    range_restarted_by_a_press(page, 10, 12, &["10", "11", "12"], "2019-06-10 - 2019-06-12").await
}

/// Pressing the range's end starts a new range there (RangeCalendar.test.js "clicking on the end of
/// the highlighted range starts a new selection").
#[browser_test]
pub async fn range_press_on_the_end_starts_a_new_range(page: &Page<'_>) -> Result<(), Report> {
    range_restarted_by_a_press(page, 20, 18, &["18", "19", "20"], "2019-06-18 - 2019-06-20").await
}

/// Pressing a date inside the range starts a new range there (RangeCalendar.test.js "mouse down in
/// the middle of the highlighted range starts a new selection").
#[browser_test]
pub async fn range_press_in_the_middle_starts_a_new_range(page: &Page<'_>) -> Result<(), Report> {
    range_restarted_by_a_press(page, 15, 17, &["15", "16", "17"], "2019-06-15 - 2019-06-17").await
}

/// An invalid range's end can't be dragged by touch, and a range can still be selected by presses
/// (RangeCalendar.test.js "does not allow dragging the end of an invalid range").
#[browser_test]
pub async fn range_invalid_end_not_draggable_by_touch(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-invalid-drag";
    page.goto_sections(PATH, &[name]).await?;
    let all = [
        "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20",
    ];
    let june20 = day_of_month(page, name, 20).await?;
    touch(&june20, PointerKind::Down).await?;
    touch(&day_of_month(page, name, 21).await?, PointerKind::Enter).await?;
    touch(&day_of_month(page, name, 19).await?, PointerKind::Enter).await?;
    let june19 = day_of_month(page, name, 19).await?;
    touch(&june19, PointerKind::Up).await?;
    page.settle().await?;
    assert_that!(|| selected_days(page, name))
        .consistently_ok()
        .matches(eq(all))
        .await;
    page.element("#test-calendar-range-invalid-drag-changes")
        .await?
        .inner_text_stays("none", Duration::from_millis(100))
        .await?;
    day_of_month(page, name, 15).await?.click().await?;
    wait_for_selected_days(page, name, &["15"]).await?;
    day_of_month(page, name, 20).await?.click().await?;
    wait_for_value(page, name, "2019-06-15 - 2019-06-20").await?;
    Ok(())
}

/// A touch on `target` that turns into a scroll (pointer down, then cancel) doesn't finish the
/// range started at June 23 (RangeCalendar.test.js "selection isn't prematurely finalized when
/// touching ... to scroll through the calendar").
async fn range_kept_when_a_touch_on_scrolls(page: &Page<'_>, target: &str) -> Result<(), Report> {
    let name = "range";
    page.goto_sections(PATH, &[name]).await?;
    day_of_month(page, name, 23).await?.click().await?;
    wait_for_selected_days(page, name, &["23"]).await?;
    let target = page.first_element(target).await?;
    touch(&target, PointerKind::Down).await?;
    touch(&target, PointerKind::Cancel).await?;
    day_of_month(page, name, 25).await?.click().await?;
    wait_for_value(page, name, "2019-06-23 - 2019-06-25").await?;
    Ok(())
}

/// A touch on a disabled date (May 31) that scrolls doesn't finish the range being selected
/// (RangeCalendar.test.js "selection isn't prematurely finalized when touching a disabled day cell
/// to scroll through the calendar").
#[browser_test]
pub async fn range_kept_when_a_touch_on_a_disabled_date_scrolls(
    page: &Page<'_>,
) -> Result<(), Report> {
    range_kept_when_a_touch_on_scrolls(
        page,
        "#test-calendar-range [role=button][aria-label='Friday, May 31, 2019']",
    )
    .await
}

/// A touch on a weekday header that scrolls doesn't finish the range being selected
/// (RangeCalendar.test.js "selection isn't prematurely finalized when touching a weekday header to
/// scroll through the calendar").
#[browser_test]
pub async fn range_kept_when_a_touch_on_a_weekday_scrolls(page: &Page<'_>) -> Result<(), Report> {
    range_kept_when_a_touch_on_scrolls(page, "#test-calendar-range th").await
}

/// A touch on the heading that scrolls doesn't finish the range being selected
/// (RangeCalendar.test.js "selection isn't prematurely finalized when touching the header to scroll
/// through the calendar").
#[browser_test]
pub async fn range_kept_when_a_touch_on_the_heading_scrolls(page: &Page<'_>) -> Result<(), Report> {
    range_kept_when_a_touch_on_scrolls(page, "#test-calendar-range h2").await
}

/// Waits until the date `label` of the calendar `name` is disabled (out of the tab order), or
/// enabled (focusable).
async fn wait_for_disabled(
    page: &Page<'_>,
    name: &str,
    label: &str,
    disabled: bool,
) -> Result<(), Report> {
    let button = date(page, name, label).await?;
    if disabled {
        button.wait_for_attr("aria-disabled", Some("true")).await?;
        button.wait_for_attr("tabindex", None).await?;
    } else {
        button.wait_for_attr("aria-disabled", None).await?;
        button.wait_for_attr("tabindex", Some("-1")).await?;
    }
    Ok(())
}

/// A started range can't cross an unavailable interval: the dates beyond it and the buttons are
/// disabled until the range is finished, also when started on a selected date
/// (RangeCalendar.test.js "disables dates not reachable from start date if isDateUnavailable is
/// provided").
#[browser_test]
pub async fn range_unreachable_dates_and_buttons(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-intervals";
    page.goto_sections(PATH, &[name]).await?;
    let (before, after) = ("Sunday, December 5, 2021", "Monday, December 27, 2021");
    wait_for_disabled(page, name, before, false).await?;
    wait_for_disabled(page, name, after, false).await?;
    let previous = button(page, name, "Previous").await?;
    let next = button(page, name, "Next").await?;
    date(page, name, "Sunday, December 12, 2021")
        .await?
        .click()
        .await?;
    wait_for_disabled(page, name, before, true).await?;
    wait_for_disabled(page, name, after, true).await?;
    previous.wait_for_prop("disabled", "true").await?;
    next.wait_for_prop("disabled", "true").await?;
    date(page, name, "Tuesday, December 14, 2021")
        .await?
        .click()
        .await?;
    wait_for_value(page, name, "2021-12-12 - 2021-12-14").await?;
    wait_for_disabled(page, name, before, false).await?;
    wait_for_disabled(page, name, after, false).await?;
    previous.wait_for_prop("disabled", "false").await?;
    next.wait_for_prop("disabled", "false").await?;
    // Starting on a selected date limits the range the same way.
    day_of_month(page, name, 12).await?.click().await?;
    wait_for_disabled(page, name, before, true).await?;
    wait_for_disabled(page, name, after, true).await?;
    previous.wait_for_prop("disabled", "true").await?;
    next.wait_for_prop("disabled", "true").await?;
    Ok(())
}

/// A range started where the last day of the previous month can't be reached disables the previous
/// button (RangeCalendar.test.js "disables the previous button if the last day of the previous
/// month is unavailable").
#[browser_test]
pub async fn range_previous_button_disabled_by_unavailable_dates(
    page: &Page<'_>,
) -> Result<(), Report> {
    let name = "range-previous-unavailable";
    page.goto_sections(PATH, &[name]).await?;
    date(page, name, "Wednesday, May 4, 2022")
        .await?
        .click()
        .await?;
    button(page, name, "Previous")
        .await?
        .wait_for_prop("disabled", "true")
        .await?;
    assert_that!(button(page, name, "Next").await?)
        .enabled()
        .await
        .is_true();
    Ok(())
}

/// A range started where the first day of the next month can't be reached disables the next button
/// (RangeCalendar.test.js "disables the next button if the first day of the next month is
/// unavailable").
#[browser_test]
pub async fn range_next_button_disabled_by_unavailable_dates(
    page: &Page<'_>,
) -> Result<(), Report> {
    let name = "range-next-unavailable";
    page.goto_sections(PATH, &[name]).await?;
    date(page, name, "Thursday, April 28, 2022")
        .await?
        .click()
        .await?;
    button(page, name, "Next")
        .await?
        .wait_for_prop("disabled", "true")
        .await?;
    assert_that!(button(page, name, "Previous").await?)
        .enabled()
        .await
        .is_true();
    Ok(())
}

/// Paging during a started range shows which dates of the new month are reachable: the last one
/// before the unavailable dates says so and has the focus's tab stop (RangeCalendar.test.js
/// "updates the unavailable dates when navigating").
#[browser_test]
pub async fn range_unavailable_dates_after_paging(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-navigation-unavailable";
    page.goto_sections(PATH, &[name]).await?;
    date(page, name, "Thursday, April 28, 2022")
        .await?
        .click()
        .await?;
    let next = button(page, name, "Next").await?;
    assert_that!(next).enabled().await.is_true();
    next.click().await?;
    wait_for_grid_label(page, name, "range-navigation-unavailable, May 2022").await?;
    let may1 = date(
        page,
        name,
        "Sunday, May 1, 2022 selected, Last available date",
    )
    .await?;
    assert_that!(may1)
        .attribute("aria-disabled")
        .await
        .is_none();
    may1.wait_for_attr("tabindex", Some("0")).await?;
    assert_that!(date(page, name, "Monday, May 2, 2022").await?)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    Ok(())
}

/// Enter on the last available date before an unavailable interval starts a range going backwards:
/// the focus moves to the day before (RangeCalendar.test.js "advances selection backwards when
/// starting a selection at the end of an available range").
#[browser_test]
pub async fn range_started_at_the_end_of_available_dates(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-intervals";
    page.goto_sections(PATH, &[name]).await?;
    enter(page, name).await?;
    // From December 15 to the 21st.
    press_keys(page, &[Key::Down, Key::Left]).await?;
    page.wait_for_focus(&date(page, name, "Tuesday, December 21, 2021").await?)
        .await?;
    press_keys(page, &[Key::Enter]).await?;
    let december20 = date(page, name, "Monday, December 20, 2021").await?;
    page.wait_for_focus(&december20).await?;
    cell(&december20)
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    Ok(())
}

/// With non-contiguous ranges, unavailable dates stay disabled but a started range can span them,
/// and an unavailable date within it isn't selected (RangeCalendar.test.js "does not disable dates
/// not reachable from start date if allowsNonContiguousRanges is provider").
#[browser_test]
pub async fn range_non_contiguous(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-non-contiguous";
    page.goto_sections(PATH, &[name]).await?;
    let december5 = date(page, name, "Sunday, December 5, 2021").await?;
    assert_that!(december5)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    assert_that!(date(page, name, "Monday, December 6, 2021").await?)
        .attribute("aria-disabled")
        .await
        .is_none();
    let december7 = date(page, name, "Tuesday, December 7, 2021").await?;
    december7.click().await?;
    cell(&december7)
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    assert_that!(date(page, name, "Monday, December 13, 2021").await?)
        .attribute("aria-disabled")
        .await
        .is_none();
    let december14 = date(page, name, "Tuesday, December 14, 2021").await?;
    assert_that!(december14)
        .attribute("aria-disabled")
        .await
        .is_none();
    december14.click().await?;
    wait_for_value(page, name, "2021-12-07 - 2021-12-14").await?;
    assert_that!(cell(&december5).await?)
        .attribute("aria-selected")
        .await
        .is_none();
    Ok(())
}

/// Leaving the calendar with the focus on an unavailable date finishes the started range at the
/// nearest available date before it (RangeCalendar.test.js "selects the nearest available date
/// when blurring the calendar").
#[browser_test]
pub async fn range_blur_selects_the_nearest_available_date(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-blur-nearest";
    page.goto_sections(PATH, &[name]).await?;
    let march9 = date(page, name, "Wednesday, March 9, 2022").await?;
    march9.click().await?;
    cell(&march9)
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    press_keys(page, &[Key::PageDown]).await?;
    let april9 = date(page, name, "Saturday, April 9, 2022").await?;
    page.wait_for_focus(&april9).await?;
    assert_that!(april9)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    page.blur_focused().await?;
    wait_for_value(page, name, "2022-03-09 - 2022-04-08").await?;
    Ok(())
}

/// An invalid range marks its dates invalid and describes them by the error message; the focused
/// date adds how to start a range (RangeCalendar.test.js "should support invalid state", "should
/// support a custom errorMessage").
#[browser_test]
pub async fn range_invalid(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-invalid";
    page.goto_sections(PATH, &[name]).await?;
    let march11 = date(page, name, "Friday, March 11, 2022 selected").await?;
    assert_that!(march11)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    assert_that!(cell(&march11).await?)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    assert_that!(march11)
        .accessible_description()
        .await
        .is_equal_to("Invalid date");
    enter(page, name).await?;
    press_keys(page, &[Key::Right]).await?;
    page.wait_for_focus(&march11).await?;
    assert_that!(|| march11.accessible_description())
        .eventually_ok()
        .matches(eq("Invalid date Click to start selecting date range"))
        .await;
    Ok(())
}

/// A valid range isn't invalid and isn't described by the error message; focused, a date says how
/// to start a range (RangeCalendar.test.js "does not show error message without isInvalid").
#[browser_test]
pub async fn range_valid_has_no_error(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-valid";
    page.goto_sections(PATH, &[name]).await?;
    let march11 = date(page, name, "Friday, March 11, 2022 selected").await?;
    assert_that!(march11)
        .attribute("aria-invalid")
        .await
        .is_none();
    assert_that!(march11)
        .attribute("aria-describedby")
        .await
        .is_none();
    enter(page, name).await?;
    press_keys(page, &[Key::Right]).await?;
    page.wait_for_focus(&march11).await?;
    assert_that!(|| march11.accessible_description())
        .eventually_ok()
        .matches(eq("Click to start selecting date range"))
        .await;
    Ok(())
}

/// A range spanning unavailable dates (non-contiguous) is invalid: its dates are marked invalid and
/// described by the error message (RangeCalendar.test.js "automatically marks selection as invalid
/// using isDateUnavailable").
#[browser_test]
pub async fn range_unavailable_selection_is_invalid(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-blur-nearest";
    page.goto_sections(PATH, &[name]).await?;
    let march4 = date(page, name, "Friday, March 4, 2022 selected").await?;
    assert_that!(march4)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    assert_that!(cell(&march4).await?)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    assert_that!(march4)
        .accessible_description()
        .await
        .is_equal_to("Invalid date");
    Ok(())
}

/// The first press of a range marks the date as both start and end; the second makes it the start
/// and the other date the end (RAC `RangeCalendar.test.tsx` "should support selected range
/// states").
#[browser_test]
pub async fn range_start_and_end_states(page: &Page<'_>) -> Result<(), Report> {
    let name = "range-today";
    page.goto_sections(PATH, &[name]).await?;
    let first = day_of_month(page, name, 7).await?;
    let middle = day_of_month(page, name, 8).await?;
    let last = day_of_month(page, name, 10).await?;
    assert_that!(first)
        .attribute("data-selection-start")
        .await
        .is_none();
    first.click().await?;
    first
        .wait_for_attr("data-selection-start", Some("true"))
        .await?;
    first
        .wait_for_attr("data-selection-end", Some("true"))
        .await?;
    page.element("h1").await?.hover().await?;
    last.click().await?;
    last.wait_for_attr("data-selection-end", Some("true"))
        .await?;
    first.wait_for_attr("data-selection-end", None).await?;
    assert_that!(first)
        .has_attribute("data-selection-start")
        .await
        .is_equal_to("true");
    assert_that!(middle)
        .attribute("data-selection-start")
        .await
        .is_none();
    assert_that!(middle)
        .attribute("data-selection-end")
        .await
        .is_none();
    Ok(())
}

/// With the commit behavior `Clear` and no range before, pressing outside the dates during a
/// started range leaves no date selected and reports nothing (RangeCalendar.test.js "should clear
/// the selection when commitBehavior is "clear" no default selected range").
#[browser_test]
pub async fn commit_clear_without_a_range(page: &Page<'_>) -> Result<(), Report> {
    let name = "commit-clear-empty";
    page.goto_sections(PATH, &[name]).await?;
    start_range(
        page,
        name,
        "Tuesday, November 25, 2025",
        "Thursday, November 20, 2025",
    )
    .await?;
    click_heading(page, name).await?;
    page.wait_for_count("#test-calendar-commit-clear-empty [data-selected]", 0)
        .await?;
    page.element("#test-calendar-commit-clear-empty-changes")
        .await?
        .inner_text_stays("none", Duration::from_millis(100))
        .await?;
    Ok(())
}
