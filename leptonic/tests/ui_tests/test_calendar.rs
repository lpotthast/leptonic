// Upstream: react-aria-components/test/Calendar.test.js @ 99e6102368
// Upstream: react-aria-components/test/RangeCalendar.test.tsx @ 99e6102368
// Upstream: react-aria/test/calendar/useCalendar.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/RangeCalendar.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/Calendar.ssr.test.js @ 99e6102368
//! Behavior of the calendar hooks (through the calendar atoms): ARIA structure and labels,
//! selection by press and keyboard, keyboard navigation (month, week and day views, pages and
//! years), the previous/next buttons, min/max, unavailable dates, disabled, read-only and
//! invalid calendars, several months, the first day of the week, and range selection by
//! presses, keyboard and dragging.
//! Spec: react-aria-components `Calendar.test.js`, `RangeCalendar.test.tsx`; react-aria
//! `useCalendar.test.js`.
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
use browser_test::thirtyfour::prelude::*;
use rootcause::{Report, bail};

use crate::pages::{ElementActions, Page, PageActions, SyntheticEvent, xpath};

/// The button of the date `label` ("Wednesday, June 5, 2019") in the calendar `name`: its label
/// is the date, possibly with additions ("Today, ", " selected", ", First available date", a
/// selected range's description before it). Other dates' labels mention it only within that
/// description ("... to Monday, June 10, 2019, ...").
async fn date(page: &Page<'_>, name: &str, label: &str) -> Result<WebElement, Report> {
    let expression = format!(
        "//section[@id='test-calendar-{name}']//*[@role='gridcell']/*[@role='button']\
         [@aria-label='{label}' or starts-with(@aria-label, '{label} ') \
         or starts-with(@aria-label, '{label},') or contains(@aria-label, ', {label}')]"
    );
    page.element(xpath(expression)).await
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

/// The label of the first grid of the calendar `name`.
async fn grid_label(page: &Page<'_>, name: &str) -> Result<Option<String>, Report> {
    Ok(grids(page, name)
        .await?
        .swap_remove(0)
        .attr("aria-label")
        .await?)
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

/// Wait until the focus is on the date labelled `label`.
async fn expect_focus(page: &Page<'_>, name: &str, label: &str) -> Result<(), Report> {
    let button = date(page, name, label).await?;
    page.wait_for_focus(&button).await?;
    Ok(())
}

/// Presses `keys` one after the other, each on the element focused at that moment (the focus
/// moves from date to date).
async fn press_keys(page: &Page<'_>, keys: &[Key]) -> Result<(), Report> {
    for key in keys {
        page.send_keys(key.clone()).await?;
    }
    Ok(())
}

/// The calendar is labelled with its label and month, so is its grid; dates are buttons in grid
/// cells, labelled with the full date (and "selected"); only the focused date is tabbable; the
/// weekday header is hidden from assistive technology; dates of other months show, disabled.
pub async fn structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let calendar = page
        .element("#test-calendar-basic [role=application]")
        .await?;
    assert_that!(calendar.attr("aria-label").await?)
        .get_some()
        .is_equal_to("basic, June 2019");
    assert_that!(grid_label(page, "basic").await?)
        .get_some()
        .is_equal_to("basic, June 2019");
    assert_that!(heading(page, "basic").await?.inner_text().await?).is_equal_to("June 2019");

    let header = page.element("#test-calendar-basic thead").await?;
    assert_that!(header.attr("aria-hidden").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(page.inner_texts("#test-calendar-basic thead th").await?)
        .contains_exactly(["S", "M", "T", "W", "T", "F", "S"]);

    let selected = date(page, "basic", "Wednesday, June 5, 2019").await?;
    assert_that!(selected.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Wednesday, June 5, 2019 selected");
    assert_that!(selected.attr("tabindex").await?)
        .get_some()
        .is_equal_to("0");
    assert_that!(selected.attr("data-selected").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(selected.inner_text().await?).is_equal_to("5");
    let selected_cell = cell(&selected).await?;
    assert_that!(selected_cell.attr("aria-selected").await?)
        .get_some()
        .is_equal_to("true");

    let other = date(page, "basic", "Thursday, June 6, 2019").await?;
    assert_that!(other.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Thursday, June 6, 2019");
    assert_that!(other.attr("tabindex").await?)
        .get_some()
        .is_equal_to("-1");
    assert_that!(cell(&other).await?.attr("aria-selected").await?).is_none();

    // June 2019 starts on a Saturday: the first row starts with May 26.
    let outside = date(page, "basic", "Sunday, May 26, 2019").await?;
    assert_that!(outside.attr("data-outside-month").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(outside.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(outside.attr("tabindex").await?).is_none();
    assert_that!(page.count("#test-calendar-basic tbody tr").await?).is_equal_to(6);

    let previous = button(page, "basic", "Previous").await?;
    assert_that!(previous.is_enabled().await?).is_true();
    Ok(())
}

/// Pressing a date selects and focuses it.
pub async fn selection_by_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let june17 = date(page, "basic", "Monday, June 17, 2019").await?;
    june17.click().await?;
    wait_for_value(page, "basic", "2019-06-17").await?;
    june17.wait_for_attr("data-selected", Some("true")).await?;
    june17
        .wait_for_attr("aria-label", Some("Monday, June 17, 2019 selected"))
        .await?;
    page.wait_for_focus(&june17).await?;
    let june5 = date(page, "basic", "Wednesday, June 5, 2019").await?;
    assert_that!(june5.attr("data-selected").await?).is_none();

    // Restore the selection for the next checks.
    june5.click().await?;
    wait_for_value(page, "basic", "2019-06-05").await?;
    Ok(())
}

/// Arrows move by a day and a week, Page Up/Down by a month (with Shift: a year), Home/End to
/// the month's ends; leaving the month pages; Enter selects.
pub async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    enter(page, "basic").await?;
    expect_focus(page, "basic", "Wednesday, June 5, 2019").await?;

    press_keys(page, &[Key::Right]).await?;
    expect_focus(page, "basic", "Thursday, June 6, 2019").await?;
    press_keys(page, &[Key::Left, Key::Left]).await?;
    expect_focus(page, "basic", "Tuesday, June 4, 2019").await?;
    press_keys(page, &[Key::Down]).await?;
    expect_focus(page, "basic", "Tuesday, June 11, 2019").await?;
    press_keys(page, &[Key::Up, Key::Up]).await?;
    expect_focus(page, "basic", "Tuesday, May 28, 2019").await?;
    wait_for_grid_label(page, "basic", "basic, May 2019").await?;
    heading(page, "basic")
        .await?
        .wait_for_inner_text("May 2019")
        .await?;

    press_keys(page, &[Key::PageDown]).await?;
    expect_focus(page, "basic", "Friday, June 28, 2019").await?;
    wait_for_grid_label(page, "basic", "basic, June 2019").await?;
    page.send_keys(Key::Shift + Key::PageDown).await?;
    expect_focus(page, "basic", "Sunday, June 28, 2020").await?;
    wait_for_grid_label(page, "basic", "basic, June 2020").await?;
    page.send_keys(Key::Shift + Key::PageUp).await?;
    expect_focus(page, "basic", "Friday, June 28, 2019").await?;
    press_keys(page, &[Key::PageUp]).await?;
    expect_focus(page, "basic", "Tuesday, May 28, 2019").await?;

    press_keys(page, &[Key::Home]).await?;
    expect_focus(page, "basic", "Wednesday, May 1, 2019").await?;
    press_keys(page, &[Key::End]).await?;
    expect_focus(page, "basic", "Friday, May 31, 2019").await?;

    press_keys(page, &[Key::Enter]).await?;
    wait_for_value(page, "basic", "2019-05-31").await?;

    // From the sixth row of June to July, which has five: the focus stays in the grid.
    press_keys(page, &[Key::PageDown, Key::End]).await?;
    expect_focus(page, "basic", "Sunday, June 30, 2019").await?;
    press_keys(page, &[Key::PageDown]).await?;
    expect_focus(page, "basic", "Tuesday, July 30, 2019").await?;
    press_keys(page, &[Key::PageUp, Key::PageUp]).await?;
    expect_focus(page, "basic", "Thursday, May 30, 2019").await?;
    press_keys(page, &[Key::End]).await?;
    press_keys(page, &[Key::Right, Key::Space]).await?;
    expect_focus(page, "basic", "Saturday, June 1, 2019").await?;
    wait_for_value(page, "basic", "2019-06-01").await?;
    Ok(())
}

/// The buttons page by a month; the focused date moves along.
pub async fn previous_next_buttons(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
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

/// Dates outside min/max are disabled, the first and last available dates say so, and the
/// buttons can't page past them.
pub async fn min_max(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let june9 = date(page, "min-max", "Sunday, June 9, 2019").await?;
    assert_that!(june9.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(june9.attr("data-disabled").await?)
        .get_some()
        .is_equal_to("true");
    let june10 = date(page, "min-max", "Monday, June 10, 2019").await?;
    assert_that!(june10.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Monday, June 10, 2019, First available date");
    let june20 = date(page, "min-max", "Thursday, June 20, 2019").await?;
    assert_that!(june20.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Thursday, June 20, 2019, Last available date");
    assert_that!(
        button(page, "min-max", "Previous")
            .await?
            .is_enabled()
            .await?
    )
    .is_false();
    assert_that!(button(page, "min-max", "Next").await?.is_enabled().await?).is_false();

    june9.click().await?;
    let june21 = date(page, "min-max", "Friday, June 21, 2019").await?;
    june21.click().await?;
    value(page, "min-max")
        .await?
        .inner_text_stays("2019-06-15")
        .await?;

    // The keyboard stops at the limits.
    enter(page, "min-max").await?;
    expect_focus(page, "min-max", "Saturday, June 15, 2019").await?;
    press_keys(page, &[Key::Down]).await?;
    expect_focus(page, "min-max", "Thursday, June 20, 2019").await?;
    press_keys(page, &[Key::PageUp]).await?;
    expect_focus(page, "min-max", "Monday, June 10, 2019").await?;
    Ok(())
}

/// Unavailable dates are marked and can't be selected, neither by press nor by keyboard.
pub async fn unavailable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let june8 = date(page, "unavailable", "Saturday, June 8, 2019").await?;
    assert_that!(june8.attr("data-unavailable").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(june8.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    june8.click().await?;
    value(page, "unavailable")
        .await?
        .inner_text_stays("2019-06-05")
        .await?;

    // Still focusable with the keyboard, but Enter doesn't select it.
    enter(page, "unavailable").await?;
    press_keys(page, &[Key::Right, Key::Right, Key::Right]).await?;
    expect_focus(page, "unavailable", "Saturday, June 8, 2019").await?;
    press_keys(page, &[Key::Enter]).await?;
    press_keys(page, &[Key::Right, Key::Right, Key::Enter]).await?;
    wait_for_value(page, "unavailable", "2019-06-10").await?;
    Ok(())
}

/// A disabled calendar: the grid says so, no date is tabbable or selectable.
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let grid = grids(page, "disabled").await?.swap_remove(0);
    assert_that!(grid.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    let calendar = page
        .element("#test-calendar-disabled [role=application]")
        .await?;
    assert_that!(calendar.attr("data-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(
        page.count("#test-calendar-disabled [role=button][tabindex]")
            .await?
    )
    .is_equal_to(0);
    let june10 = date(page, "disabled", "Monday, June 10, 2019").await?;
    assert_that!(june10.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    june10.click().await?;
    value(page, "disabled")
        .await?
        .inner_text_stays("2019-06-05")
        .await?;
    assert_that!(button(page, "disabled", "Next").await?.is_enabled().await?).is_false();
    Ok(())
}

/// A read-only calendar: the grid says so; dates can be navigated with the keyboard, but
/// neither presses nor Enter select.
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let grid = grids(page, "read-only").await?.swap_remove(0);
    assert_that!(grid.attr("aria-readonly").await?)
        .get_some()
        .is_equal_to("true");
    date(page, "read-only", "Monday, June 10, 2019")
        .await?
        .click()
        .await?;
    enter(page, "read-only").await?;
    expect_focus(page, "read-only", "Wednesday, June 5, 2019").await?;
    press_keys(page, &[Key::Right, Key::Enter]).await?;
    expect_focus(page, "read-only", "Thursday, June 6, 2019").await?;
    value(page, "read-only")
        .await?
        .inner_text_stays("2019-06-05")
        .await?;
    Ok(())
}

/// An invalid calendar: its selected date is marked invalid and described by the error message.
pub async fn invalid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let calendar = page
        .element("#test-calendar-invalid [role=application]")
        .await?;
    assert_that!(calendar.attr("data-invalid").await?)
        .get_some()
        .is_equal_to("true");
    let june5 = date(page, "invalid", "Wednesday, June 5, 2019").await?;
    assert_that!(june5.attr("aria-invalid").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(june5.attr("data-invalid").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(cell(&june5).await?.attr("aria-invalid").await?)
        .get_some()
        .is_equal_to("true");
    let error = page
        .element(xpath(
            "//section[@id='test-calendar-invalid']//div[normalize-space(text())='Invalid date']",
        ))
        .await?;
    let error_id = error.id().await?;
    assert_that!(error_id).is_some();
    assert_that!(june5.attr("aria-describedby").await?).is_equal_to(error_id);
    let june6 = date(page, "invalid", "Thursday, June 6, 2019").await?;
    assert_that!(june6.attr("aria-invalid").await?).is_none();
    Ok(())
}

/// Two months: a grid per month, the calendar labelled with both; paging moves both.
pub async fn two_months(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let calendar = page
        .element("#test-calendar-two-months [role=application]")
        .await?;
    assert_that!(calendar.attr("aria-label").await?)
        .get_some()
        .is_equal_to("two-months, June 2019 to July 2019");
    let grids = grids(page, "two-months").await?;
    assert_that!(grids).has_length(2);
    assert_that!(grids[0].attr("aria-label").await?)
        .get_some()
        .is_equal_to("two-months, June 2019");
    assert_that!(grids[1].attr("aria-label").await?)
        .get_some()
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

/// A week view (`useCalendar.test.js`, "visibleDuration: 1 week"): arrows page by a week when
/// leaving it, Home/End go to its ends.
pub async fn week_view(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    assert_that!(grid_label(page, "week").await?)
        .get_some()
        .is_equal_to("week, June 2, 2019 to June 8, 2019");
    assert_that!(page.count("#test-calendar-week tbody tr").await?).is_equal_to(1);

    enter(page, "week").await?;
    expect_focus(page, "week", "Wednesday, June 5, 2019").await?;
    press_keys(page, &[Key::Right, Key::Right, Key::Right, Key::Right]).await?;
    expect_focus(page, "week", "Sunday, June 9, 2019").await?;
    wait_for_grid_label(page, "week", "week, June 9, 2019 to June 15, 2019").await?;
    press_keys(page, &[Key::Down]).await?;
    expect_focus(page, "week", "Sunday, June 16, 2019").await?;
    press_keys(page, &[Key::End]).await?;
    expect_focus(page, "week", "Saturday, June 22, 2019").await?;
    press_keys(page, &[Key::Home]).await?;
    expect_focus(page, "week", "Sunday, June 16, 2019").await?;
    page.send_keys(Key::Shift + Key::PageDown).await?;
    expect_focus(page, "week", "Tuesday, July 16, 2019").await?;
    Ok(())
}

/// A view of three days (`useCalendar.test.js`, "visibleDuration: 3 days"): centered on the
/// selected date, paging by three days.
pub async fn day_view(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    assert_that!(grid_label(page, "days").await?)
        .get_some()
        .is_equal_to("days, June 4, 2019 to June 6, 2019");
    assert_that!(page.inner_texts("#test-calendar-days th").await?)
        .contains_exactly(["T", "W", "T"]);

    enter(page, "days").await?;
    press_keys(page, &[Key::Left, Key::Left]).await?;
    expect_focus(page, "days", "Monday, June 3, 2019").await?;
    wait_for_grid_label(page, "days", "days, June 1, 2019 to June 3, 2019").await?;
    press_keys(page, &[Key::Down]).await?;
    // A row is the three visible days.
    expect_focus(page, "days", "Thursday, June 6, 2019").await?;
    wait_for_grid_label(page, "days", "days, June 4, 2019 to June 6, 2019").await?;
    Ok(())
}

/// The first day of the week (`useCalendar.test.js`, "firstDayOfWeek").
pub async fn first_day_of_week(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let header = page.element("#test-calendar-monday th").await?;
    assert_that!(header.inner_text().await?).is_equal_to("M");
    let first = page
        .element("#test-calendar-monday [role=gridcell] > [role=button]")
        .await?;
    assert_that!(first.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Monday, May 27, 2019");
    Ok(())
}

/// A range by two presses: the first starts it (and highlights while hovering), the second
/// finishes it; the ends are marked; the ends are labelled with the range.
pub async fn range_by_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let june5 = date(page, "range", "Wednesday, June 5, 2019").await?;
    assert_that!(june5.attr("data-selection-start").await?)
        .get_some()
        .is_equal_to("true");
    let june10 = date(page, "range", "Monday, June 10, 2019").await?;
    assert_that!(june10.attr("data-selection-end").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(june5.attr("aria-label").await?)
        .get_some()
        .starts_with("Selected Range: ");
    let grid = grids(page, "range").await?.swap_remove(0);
    assert_that!(grid.attr("aria-multiselectable").await?)
        .get_some()
        .is_equal_to("true");

    let june12 = date(page, "range", "Wednesday, June 12, 2019").await?;
    june12.click().await?;
    june12.wait_for_attr("data-selected", Some("true")).await?;
    value(page, "range")
        .await?
        .inner_text_stays("2019-06-05 - 2019-06-10")
        .await?;
    // The focused cell says how to go on.
    assert_that!(june12.referenced_text("aria-describedby").await?)
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

/// A range with the keyboard: Enter starts it (moving on by a day), Enter finishes it, Escape
/// cancels a started range.
pub async fn range_by_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    // The pointer away from the dates: while a range is started, hovering a date highlights it
    // (and moves the focus there), also when the layout moves a date under a resting pointer.
    let heading = page.element("h1").await?;
    heading.hover().await?;
    // The focus starts on the selected range's start.
    enter(page, "range").await?;
    expect_focus(page, "range", "Wednesday, June 5, 2019").await?;
    press_keys(page, &[Key::Left, Key::Enter]).await?;
    expect_focus(page, "range", "Wednesday, June 5, 2019").await?;
    press_keys(page, &[Key::Right, Key::Enter]).await?;
    wait_for_value(page, "range", "2019-06-04 - 2019-06-06").await?;

    press_keys(page, &[Key::Down, Key::Enter, Key::Right]).await?;
    expect_focus(page, "range", "Saturday, June 15, 2019").await?;
    let june15 = date(page, "range", "Saturday, June 15, 2019").await?;
    june15.wait_for_attr("data-selected", Some("true")).await?;
    press_keys(page, &[Key::Escape]).await?;
    june15.wait_for_attr("data-selected", None).await?;
    value(page, "range")
        .await?
        .inner_text_stays("2019-06-04 - 2019-06-06")
        .await?;

    press_keys(page, &[Key::Enter, Key::Right, Key::Enter]).await?;
    wait_for_value(page, "range", "2019-06-15 - 2019-06-17").await?;
    Ok(())
}

/// A range by dragging from one date to another.
pub async fn range_by_dragging(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let june3 = date(page, "range", "Monday, June 3, 2019").await?;
    let june6 = date(page, "range", "Thursday, June 6, 2019").await?;
    page.driver
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
    page.driver
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

/// Without non-contiguous ranges, a range can't span unavailable dates: after starting one, the
/// dates beyond the next unavailable date are disabled.
pub async fn range_unavailable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let june11 = date(page, "range-unavailable", "Tuesday, June 11, 2019").await?;
    june11.click().await?;
    let june17 = date(page, "range-unavailable", "Monday, June 17, 2019").await?;
    june17.wait_for_attr("aria-disabled", Some("true")).await?;
    let june14 = date(page, "range-unavailable", "Friday, June 14, 2019").await?;
    assert_that!(june14.attr("aria-disabled").await?).is_none();
    june17.click().await?;
    june14.click().await?;
    wait_for_value(page, "range-unavailable", "2019-06-11 - 2019-06-14").await?;
    june17.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}

/// A calendar labelled by another element as well ("should support aria props on the
/// Calendar"): calendar and grid list it, every referenced id exists.
pub async fn labelled_by_another_element(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let calendar = page
        .element("#test-calendar-labelled [role=application]")
        .await?;
    let grid = grids(page, "labelled").await?.swap_remove(0);
    for element in [&calendar, &grid] {
        let ids = element.attr("aria-labelledby").await?.unwrap_or_default();
        assert_that!(ids.split_whitespace().collect::<Vec<_>>())
            .contains("test-calendar-labelled-label");
        for id in ids.split_whitespace() {
            assert_that!(page.count(format!("[id='{id}']")).await?).is_equal_to(1);
        }
    }
    assert_that!(grid.attr("aria-label").await?)
        .get_some()
        .is_equal_to("labelled, June 2019");
    Ok(())
}

/// A started range is committed when the pointer is released outside the dates (the default
/// commit behavior `Select`).
pub async fn range_committed_by_an_outside_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    date(page, "range", "Tuesday, June 11, 2019")
        .await?
        .click()
        .await?;
    let june13 = date(page, "range", "Thursday, June 13, 2019").await?;
    june13.hover().await?;
    june13.wait_for_attr("data-selected", Some("true")).await?;
    page.element("#test-calendar-range-after")
        .await?
        .click()
        .await?;
    wait_for_value(page, "range", "2019-06-11 - 2019-06-13").await?;
    Ok(())
}

/// In a right-to-left locale, the left arrow moves to the next day.
pub async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
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
    let expression = format!(
        "//section[@id='test-calendar-{name}']//*[@role='gridcell']/*[@role='button']\
         [not(@data-outside-month)][normalize-space(.)='{day}']"
    );
    page.element(xpath(expression)).await
}

/// Moving the focused date from outside ("should not become focused just by setting the focused
/// date") changes the tabbable date but leaves the browser's focus where it is.
pub async fn setting_the_focused_date_keeps_the_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let set = page.element("#test-calendar-focus-set").await?;
    set.click().await?;
    let june20 = date(page, "focus", "Thursday, June 20, 2019").await?;
    june20.wait_for_attr("tabindex", Some("0")).await?;
    page.focus_stays(&set).await?;
    Ok(())
}

/// A controlled range cleared from outside shows no selection.
pub async fn controlled_range_cleared(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    page.element("#test-calendar-range-clear")
        .await?
        .click()
        .await?;
    wait_for_value(page, "range", "none").await?;
    let june5 = date(page, "range", "Wednesday, June 5, 2019").await?;
    june5.wait_for_attr("data-selected", None).await?;
    assert_that!(june5.attr("data-selection-start").await?).is_none();
    assert_that!(page.count("#test-calendar-range [data-selected]").await?).is_equal_to(0);
    Ok(())
}

/// Unavailable dates may depend on the anchor of a range being selected ("should allow changing
/// the unavailable dates based on the anchor date"): here, dates more than a week away.
pub async fn unavailable_dates_depending_on_the_anchor(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let june18 = date(page, "range-week", "Tuesday, June 18, 2019").await?;
    assert_that!(june18.attr("data-unavailable").await?).is_none();
    date(page, "range-week", "Monday, June 10, 2019")
        .await?
        .click()
        .await?;
    june18
        .wait_for_attr("data-unavailable", Some("true"))
        .await?;
    let june17 = date(page, "range-week", "Monday, June 17, 2019").await?;
    assert_that!(june17.attr("data-unavailable").await?).is_none();
    june17.click().await?;
    wait_for_value(page, "range-week", "2019-06-10 - 2019-06-17").await?;
    june18.wait_for_attr("data-unavailable", None).await?;
    Ok(())
}

/// Dispatches a touch pointer event (`pointerdown`, `pointerup`, `pointerenter`,
/// `pointercancel`) at the center of `element`.
async fn touch(element: &WebElement, kind: &str) -> Result<(), Report> {
    let rect = element.client_rect().await?;
    element
        .dispatch(
            SyntheticEvent::pointer(kind)
                .with("bubbles", kind != "pointerenter")
                .with("pointerType", "touch")
                .with("pointerId", 1)
                .with("isPrimary", true)
                .with("button", 0)
                .with("buttons", u8::from(kind == "pointerdown"))
                .with("width", 1)
                .with("height", 1)
                .with("clientX", rect.left + rect.width / 2.0)
                .with("clientY", rect.top + rect.height / 2.0),
        )
        .await?;
    Ok(())
}

/// A quick tap: pressed and released before the drag delay.
async fn touch_tap(element: &WebElement) -> Result<(), Report> {
    touch(element, "pointerdown").await?;
    touch(element, "pointerup").await?;
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

/// Two quick taps select a range: the first starts it (a tap is released before the touch drag
/// delay, so it selects on release), the second finishes it.
pub async fn range_by_touch_taps(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let june11 = date(page, "range-touch", "Tuesday, June 11, 2019").await?;
    touch_tap(&june11).await?;
    wait_for_selected_days(page, "range-touch", &["11"]).await?;
    // Past the touch drag delay (200 ms): still only started.
    page.settle().await?;
    assert_that!(|| selected_days(page, "range-touch"))
        .consistently_ok()
        .for_at_least(Duration::from_millis(400))
        .matches(eq(["11"]))
        .await;
    value(page, "range-touch")
        .await?
        .inner_text_stays("2019-06-05 - 2019-06-10")
        .await?;

    let june13 = date(page, "range-touch", "Thursday, June 13, 2019").await?;
    touch_tap(&june13).await?;
    wait_for_value(page, "range-touch", "2019-06-11 - 2019-06-13").await?;
    Ok(())
}

/// "selects by dragging with touch": after the delay the pressed date starts the range, dates
/// the finger enters extend it, releasing finishes it.
pub async fn range_by_touch_dragging(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let june17 = date(page, "range-touch", "Monday, June 17, 2019").await?;
    touch(&june17, "pointerdown").await?;
    // The delay tells dragging from scrolling: nothing changes at first.
    assert_that!(selected_days(page, "range-touch").await?)
        .contains_exactly(["5", "6", "7", "8", "9", "10"]);
    wait_for_selected_days(page, "range-touch", &["17"]).await?;
    let june18 = date(page, "range-touch", "Tuesday, June 18, 2019").await?;
    touch(&june18, "pointerenter").await?;
    wait_for_selected_days(page, "range-touch", &["17", "18"]).await?;
    let june23 = date(page, "range-touch", "Sunday, June 23, 2019").await?;
    touch(&june23, "pointerenter").await?;
    wait_for_selected_days(
        page,
        "range-touch",
        &["17", "18", "19", "20", "21", "22", "23"],
    )
    .await?;
    value(page, "range-touch")
        .await?
        .inner_text_stays("2019-06-05 - 2019-06-10")
        .await?;
    touch(&june23, "pointerup").await?;
    wait_for_value(page, "range-touch", "2019-06-17 - 2019-06-23").await?;
    Ok(())
}

/// "selection isn't prematurely finalized when touching a day cell to scroll through the
/// calendar": a touch cancelled by scrolling doesn't finish the range being selected.
pub async fn range_kept_when_a_touch_scrolls(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    date(page, "range-touch", "Sunday, June 23, 2019")
        .await?
        .click()
        .await?;
    wait_for_selected_days(page, "range-touch", &["23"]).await?;
    let june10 = date(page, "range-touch", "Monday, June 10, 2019").await?;
    touch(&june10, "pointerdown").await?;
    touch(&june10, "pointercancel").await?;
    // Past the touch drag delay (200 ms), after which a pressed date starts dragging.
    let range_value = value(page, "range-touch").await?;
    page.settle().await?;
    assert_that!(|| range_value.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(400))
        .matches(eq("2019-06-05 - 2019-06-10"))
        .await;
    date(page, "range-touch", "Tuesday, June 25, 2019")
        .await?
        .click()
        .await?;
    wait_for_value(page, "range-touch", "2019-06-23 - 2019-06-25").await?;
    Ok(())
}

/// The browser's today is another date than the server's: the tabbable date is the one marked as
/// today, and keyboard focus goes there.
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
        page.driver
            .cdp()
            .send_raw(
                "Emulation.setTimezoneOverride",
                serde_json::json!({ "timezoneId": zone }),
            )
            .await?;
    }
    page.goto_path("/atoms/calendar").await?;

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

/// `pageBehavior: single` pages by one month, week or day of the visible duration.
pub async fn page_behavior_single(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
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

/// A two-week view (`useCalendar.test.js`, "visibleDuration: 2 weeks"): two rows, labelled with
/// its dates.
pub async fn two_weeks(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    assert_that!(grid_label(page, "two-weeks").await?)
        .get_some()
        .is_equal_to("two-weeks, June 2, 2019 to June 15, 2019");
    assert_that!(page.count("#test-calendar-two-weeks tbody tr").await?).is_equal_to(2);
    enter(page, "two-weeks").await?;
    expect_focus(page, "two-weeks", "Wednesday, June 5, 2019").await?;
    press_keys(page, &[Key::Down, Key::Down]).await?;
    expect_focus(page, "two-weeks", "Wednesday, June 19, 2019").await?;
    wait_for_grid_label(
        page,
        "two-weeks",
        "two-weeks, June 16, 2019 to June 29, 2019",
    )
    .await?;
    Ok(())
}

/// RAC "should support weeksInMonth prop": April 2026 has five week rows, six are shown.
pub async fn weeks_in_month(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    assert_that!(page.count("#test-calendar-six-weeks tbody tr").await?).is_equal_to(6);
    Ok(())
}

/// RAC "should support repeat keydown events when holding an arrow key".
pub async fn held_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let march3 = date(page, "held", "Tuesday, March 3, 2020").await?;
    march3.click().await?;
    page.wait_for_focus(&march3).await?;
    page.hold_key("ArrowRight", 1).await?;
    expect_focus(page, "held", "Thursday, March 5, 2020").await?;
    Ok(())
}

/// RAC "should handle changing the visible duration": a week view becomes a month view around
/// the focused date.
pub async fn changing_the_visible_duration(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let heading = heading(page, "duration").await?;
    assert_that!(heading.inner_text().await?).is_equal_to("April 5, 2026 to April 11, 2026");
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
    select
        .element(xpath(format!("option[normalize-space()='{text}']")))
        .await
}

/// RAC "should support month and year dropdowns": the pickers list the months and 20 years
/// around the focused date's and move it; the year picker follows.
pub async fn month_and_year_pickers(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    let month = page
        .element("#test-calendar-pickers select[aria-label=month]")
        .await?;
    let year = page
        .element("#test-calendar-pickers select[aria-label=year]")
        .await?;
    wait_for_grid_label(page, "pickers", "Appointment date, April 2026").await?;
    assert_that!(month.value().await?)
        .get_some()
        .is_equal_to("4");
    assert_that!(month.inner_texts("option").await?).contains_exactly([
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ]);
    option(&month, "Jun").await?.click().await?;
    wait_for_grid_label(page, "pickers", "Appointment date, June 2026").await?;
    assert_that!(month.value().await?)
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

/// "announces when the current month changes", "announces when the selected date range
/// changes".
pub async fn announcements(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    button(page, "range-touch", "Next").await?.click().await?;
    wait_for_announcement(page, "July 2019").await?;
    button(page, "range-touch", "Previous")
        .await?
        .click()
        .await?;
    wait_for_announcement(page, "June 2019").await?;
    date(page, "range-touch", "Monday, June 17, 2019")
        .await?
        .click()
        .await?;
    date(page, "range-touch", "Monday, June 10, 2019")
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

/// The commit behaviors of a range being selected, when the pointer is released on the calendar
/// outside its dates (its heading) and when the focus leaves it (Tab): `Select` finishes it at the
/// hovered date, `Clear` clears the value, `Reset` keeps the value.
pub async fn commit_behaviors(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/calendar").await?;
    start_range(
        page,
        "commit-select",
        "Tuesday, November 25, 2025",
        "Thursday, November 20, 2025",
    )
    .await?;
    click_heading(page, "commit-select").await?;
    wait_for_value(page, "commit-select", "2025-11-20 - 2025-11-25").await?;
    start_range(
        page,
        "commit-select",
        "Thursday, November 27, 2025",
        "Saturday, November 22, 2025",
    )
    .await?;
    page.send_keys(Key::Tab).await?;
    wait_for_value(page, "commit-select", "2025-11-22 - 2025-11-27").await?;

    start_range(
        page,
        "commit-clear",
        "Tuesday, November 25, 2025",
        "Thursday, November 20, 2025",
    )
    .await?;
    click_heading(page, "commit-clear").await?;
    wait_for_value(page, "commit-clear", "none").await?;
    page.wait_for_count("#test-calendar-commit-clear [data-selected]", 0)
        .await?;
    start_range(
        page,
        "commit-clear",
        "Tuesday, November 25, 2025",
        "Thursday, November 20, 2025",
    )
    .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_count("#test-calendar-commit-clear [data-selected]", 0)
        .await?;
    value(page, "commit-clear")
        .await?
        .inner_text_stays("none")
        .await?;

    let november25 = date(page, "commit-reset", "Tuesday, November 25, 2025").await?;
    let november13 = date(page, "commit-reset", "Thursday, November 13, 2025").await?;
    start_range(
        page,
        "commit-reset",
        "Tuesday, November 25, 2025",
        "Thursday, November 20, 2025",
    )
    .await?;
    assert_that!(november13.attr("data-selected").await?).is_none();
    click_heading(page, "commit-reset").await?;
    november25.wait_for_attr("data-selected", None).await?;
    november13
        .wait_for_attr("data-selected", Some("true"))
        .await?;
    value(page, "commit-reset")
        .await?
        .inner_text_stays("2025-11-13 - 2025-11-15")
        .await?;
    start_range(
        page,
        "commit-reset",
        "Tuesday, November 25, 2025",
        "Thursday, November 20, 2025",
    )
    .await?;
    page.send_keys(Key::Tab).await?;
    november13
        .wait_for_attr("data-selected", Some("true"))
        .await?;
    value(page, "commit-reset")
        .await?
        .inner_text_stays("2025-11-13 - 2025-11-15")
        .await?;
    Ok(())
}
