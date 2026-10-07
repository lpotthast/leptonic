// Upstream: react-aria-components/test/Calendar.test.js @ 99e6102368
// Upstream: react-aria-components/test/RangeCalendar.test.tsx @ 99e6102368
// Upstream: react-aria/test/calendar/useCalendar.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/RangeCalendar.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/Calendar.ssr.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Behavior of the calendar hooks (through the calendar atoms): ARIA structure and labels,
/// selection by press and keyboard, keyboard navigation (month, week and day views, pages and
/// years), the previous/next buttons, min/max, unavailable dates, disabled, read-only and
/// invalid calendars, several months, the first day of the week, and range selection by
/// presses, keyboard and dragging.
/// Spec: react-aria-components `Calendar.test.js`, `RangeCalendar.test.tsx`; react-aria
/// `useCalendar.test.js`.
pub struct CalendarTests {}

#[async_trait]
impl BrowserTest<str> for CalendarTests {
    fn name(&self) -> Cow<'_, str> {
        "calendar_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/calendar").await?;

        structure(&page).await?;
        selection_by_press(&page).await?;
        keyboard_navigation(&page).await?;
        previous_next_buttons(&page).await?;
        min_max(&page).await?;
        unavailable(&page).await?;
        disabled(&page).await?;
        read_only(&page).await?;
        invalid(&page).await?;
        two_months(&page).await?;
        week_view(&page).await?;
        day_view(&page).await?;
        first_day_of_week(&page).await?;
        labelled_by_another_element(&page).await?;
        right_to_left(&page).await?;
        setting_the_focused_date_keeps_the_focus(&page).await?;
        range_by_press(&page).await?;
        range_by_keyboard(&page).await?;
        range_by_dragging(&page).await?;
        range_committed_by_an_outside_press(&page).await?;
        controlled_range_cleared(&page).await?;
        unavailable_dates_depending_on_the_anchor(&page).await?;
        range_unavailable(&page).await?;

        page.expect_no_page_errors().await
    }
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

/// The button of the date `label` ("Wednesday, June 5, 2019") in the calendar `name`: its label
/// is the date, possibly with additions ("Today, ", " selected", ", First available date", a
/// selected range's description before it). Other dates' labels mention it only within that
/// description ("... to Monday, June 10, 2019, ...").
async fn date(page: &Page<'_>, name: &str, label: &str) -> Result<WebElement, Report> {
    let xpath = format!(
        "//section[@id='test-calendar-{name}']//*[@role='gridcell']/*[@role='button']\
         [@aria-label='{label}' or starts-with(@aria-label, '{label} ') \
         or starts-with(@aria-label, '{label},') or contains(@aria-label, ', {label}')]"
    );
    Ok(page.driver.find(By::XPath(xpath)).await?)
}

/// The cell (`td`) around a date's button.
async fn cell(button: &WebElement) -> Result<WebElement, Report> {
    Ok(button.find(By::XPath("..")).await?)
}

async fn grids(page: &Page<'_>, name: &str) -> Result<Vec<WebElement>, Report> {
    Ok(page
        .driver
        .find_all(By::Css(format!("#test-calendar-{name} [role=grid]")))
        .await?)
}

async fn grid_label(page: &Page<'_>, name: &str) -> Result<Option<String>, Report> {
    attr(&grids(page, name).await?.swap_remove(0), "aria-label").await
}

async fn wait_for_grid_label(page: &Page<'_>, name: &str, expected: &str) -> Result<(), Report> {
    let grid = grids(page, name).await?.swap_remove(0);
    page.wait_for_attr(&grid, "aria-label", Some(expected))
        .await
}

async fn heading(page: &Page<'_>, name: &str) -> Result<String, Report> {
    Ok(page
        .css(&format!("#test-calendar-{name} h2"))
        .await?
        .text()
        .await?)
}

async fn button(page: &Page<'_>, name: &str, label: &str) -> Result<WebElement, Report> {
    page.css(&format!(
        "#test-calendar-{name} button[aria-label='{label}']"
    ))
    .await
}

async fn wait_for_value(page: &Page<'_>, name: &str, expected: &str) -> Result<(), Report> {
    page.wait_for_text(&format!("test-calendar-{name}-value"), expected)
        .await
}

async fn value(page: &Page<'_>, name: &str) -> Result<String, Report> {
    page.read_text_of(&format!("test-calendar-{name}-value"))
        .await
}

/// The value of the calendar `name` stays `expected` (checked again after the effects of an
/// interaction had time to run).
async fn expect_value_unchanged(page: &Page<'_>, name: &str, expected: &str) -> Result<(), Report> {
    assert_that!(value(page, name).await?).is_equal_to(expected.to_owned());
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(value(page, name).await?).is_equal_to(expected.to_owned());
    Ok(())
}

/// Tab into the calendar `name`'s grid from the button before it (past the enabled previous and
/// next buttons).
async fn enter(page: &Page<'_>, name: &str) -> Result<(), Report> {
    page.click_element_with_id(&format!("test-calendar-{name}-before"))
        .await?;
    for _ in 0..3 {
        page.press_tab().await?;
        let active = page.driver.active_element().await?;
        if attr(&active, "role").await?.as_deref() == Some("button") {
            return Ok(());
        }
    }
    Ok(())
}

/// Wait until the focus is on the date labelled `label`.
async fn expect_focus(page: &Page<'_>, name: &str, label: &str) -> Result<(), Report> {
    let button = date(page, name, label).await?;
    page.wait_for_focus_on(&button, label).await
}

async fn press_keys(page: &Page<'_>, keys: &[Key]) -> Result<(), Report> {
    for key in keys {
        page.send_keys_to_active(key.clone()).await?;
    }
    Ok(())
}

/// The calendar is labelled with its label and month, so is its grid; dates are buttons in grid
/// cells, labelled with the full date (and "selected"); only the focused date is tabbable; the
/// weekday header is hidden from assistive technology; dates of other months show, disabled.
async fn structure(page: &Page<'_>) -> Result<(), Report> {
    let calendar = page.css("#test-calendar-basic [role=application]").await?;
    assert_that!(attr(&calendar, "aria-label").await?)
        .is_equal_to(Some("basic, June 2019".to_owned()));
    assert_that!(grid_label(page, "basic").await?).is_equal_to(Some("basic, June 2019".to_owned()));
    assert_that!(heading(page, "basic").await?).is_equal_to("June 2019".to_owned());

    let header = page.css("#test-calendar-basic thead").await?;
    assert_that!(attr(&header, "aria-hidden").await?).is_equal_to(Some("true".to_owned()));
    let mut names = Vec::new();
    for th in header.find_all(By::Css("th")).await? {
        names.push(th.text().await?);
    }
    assert_that!(names).is_equal_to(
        ["S", "M", "T", "W", "T", "F", "S"]
            .map(str::to_owned)
            .to_vec(),
    );

    let selected = date(page, "basic", "Wednesday, June 5, 2019").await?;
    assert_that!(attr(&selected, "aria-label").await?)
        .is_equal_to(Some("Wednesday, June 5, 2019 selected".to_owned()));
    assert_that!(attr(&selected, "tabindex").await?).is_equal_to(Some("0".to_owned()));
    assert_that!(attr(&selected, "data-selected").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(selected.text().await?).is_equal_to("5".to_owned());
    let selected_cell = cell(&selected).await?;
    assert_that!(attr(&selected_cell, "aria-selected").await?).is_equal_to(Some("true".to_owned()));

    let other = date(page, "basic", "Thursday, June 6, 2019").await?;
    assert_that!(attr(&other, "aria-label").await?)
        .is_equal_to(Some("Thursday, June 6, 2019".to_owned()));
    assert_that!(attr(&other, "tabindex").await?).is_equal_to(Some("-1".to_owned()));
    assert_that!(attr(&cell(&other).await?, "aria-selected").await?).is_none();

    // June 2019 starts on a Saturday: the first row starts with May 26.
    let outside = date(page, "basic", "Sunday, May 26, 2019").await?;
    assert_that!(attr(&outside, "data-outside-month").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&outside, "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&outside, "tabindex").await?).is_none();
    let rows = page
        .driver
        .find_all(By::Css("#test-calendar-basic tbody tr"))
        .await?;
    assert_that!(rows.len()).is_equal_to(6);

    let previous = button(page, "basic", "Previous").await?;
    assert_that!(attr(&previous, "disabled").await?).is_none();
    Ok(())
}

/// Pressing a date selects and focuses it.
async fn selection_by_press(page: &Page<'_>) -> Result<(), Report> {
    let june17 = date(page, "basic", "Monday, June 17, 2019").await?;
    june17.click().await?;
    wait_for_value(page, "basic", "2019-06-17").await?;
    page.wait_for_attr(&june17, "data-selected", Some("true"))
        .await?;
    page.wait_for_attr(
        &june17,
        "aria-label",
        Some("Monday, June 17, 2019 selected"),
    )
    .await?;
    page.wait_for_focus_on(&june17, "June 17").await?;
    let june5 = date(page, "basic", "Wednesday, June 5, 2019").await?;
    assert_that!(attr(&june5, "data-selected").await?).is_none();

    // Restore the selection for the next checks.
    june5.click().await?;
    wait_for_value(page, "basic", "2019-06-05").await
}

/// Arrows move by a day and a week, Page Up/Down by a month (with Shift: a year), Home/End to
/// the month's ends; leaving the month pages; Enter selects.
async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
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
    assert_that!(heading(page, "basic").await?).is_equal_to("May 2019".to_owned());

    press_keys(page, &[Key::PageDown]).await?;
    expect_focus(page, "basic", "Friday, June 28, 2019").await?;
    wait_for_grid_label(page, "basic", "basic, June 2019").await?;
    page.send_keys_to_active(Key::Shift + Key::PageDown).await?;
    expect_focus(page, "basic", "Sunday, June 28, 2020").await?;
    wait_for_grid_label(page, "basic", "basic, June 2020").await?;
    page.send_keys_to_active(Key::Shift + Key::PageUp).await?;
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
    wait_for_value(page, "basic", "2019-06-01").await
}

/// The buttons page by a month; the focused date moves along.
async fn previous_next_buttons(page: &Page<'_>) -> Result<(), Report> {
    // Keyboard focus shows on the buttons.
    page.click_element_with_id("test-calendar-basic-before")
        .await?;
    page.press_tab().await?;
    let previous = button(page, "basic", "Previous").await?;
    page.wait_for_focus_on(&previous, "the previous button")
        .await?;
    page.wait_for_attr(&previous, "data-focus-visible", Some("true"))
        .await?;
    let next = button(page, "basic", "Next").await?;
    next.click().await?;
    wait_for_grid_label(page, "basic", "basic, July 2019").await?;
    assert_that!(heading(page, "basic").await?).is_equal_to("July 2019".to_owned());
    let previous = button(page, "basic", "Previous").await?;
    previous.click().await?;
    previous.click().await?;
    wait_for_grid_label(page, "basic", "basic, May 2019").await?;
    next.click().await?;
    wait_for_grid_label(page, "basic", "basic, June 2019").await
}

/// Dates outside min/max are disabled, the first and last available dates say so, and the
/// buttons can't page past them.
async fn min_max(page: &Page<'_>) -> Result<(), Report> {
    let june9 = date(page, "min-max", "Sunday, June 9, 2019").await?;
    assert_that!(attr(&june9, "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&june9, "data-disabled").await?).is_equal_to(Some("true".to_owned()));
    let june10 = date(page, "min-max", "Monday, June 10, 2019").await?;
    assert_that!(attr(&june10, "aria-label").await?).is_equal_to(Some(
        "Monday, June 10, 2019, First available date".to_owned(),
    ));
    let june20 = date(page, "min-max", "Thursday, June 20, 2019").await?;
    assert_that!(attr(&june20, "aria-label").await?).is_equal_to(Some(
        "Thursday, June 20, 2019, Last available date".to_owned(),
    ));
    assert_that!(attr(&button(page, "min-max", "Previous").await?, "disabled").await?).is_some();
    assert_that!(attr(&button(page, "min-max", "Next").await?, "disabled").await?).is_some();

    june9.click().await?;
    let june21 = date(page, "min-max", "Friday, June 21, 2019").await?;
    june21.click().await?;
    expect_value_unchanged(page, "min-max", "2019-06-15").await?;

    // The keyboard stops at the limits.
    enter(page, "min-max").await?;
    expect_focus(page, "min-max", "Saturday, June 15, 2019").await?;
    press_keys(page, &[Key::Down]).await?;
    expect_focus(page, "min-max", "Thursday, June 20, 2019").await?;
    press_keys(page, &[Key::PageUp]).await?;
    expect_focus(page, "min-max", "Monday, June 10, 2019").await
}

/// Unavailable dates are marked and can't be selected, neither by press nor by keyboard.
async fn unavailable(page: &Page<'_>) -> Result<(), Report> {
    let june8 = date(page, "unavailable", "Saturday, June 8, 2019").await?;
    assert_that!(attr(&june8, "data-unavailable").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&june8, "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    june8.click().await?;
    expect_value_unchanged(page, "unavailable", "2019-06-05").await?;

    // Still focusable with the keyboard, but Enter doesn't select it.
    enter(page, "unavailable").await?;
    press_keys(page, &[Key::Right, Key::Right, Key::Right]).await?;
    expect_focus(page, "unavailable", "Saturday, June 8, 2019").await?;
    press_keys(page, &[Key::Enter]).await?;
    press_keys(page, &[Key::Right, Key::Right, Key::Enter]).await?;
    wait_for_value(page, "unavailable", "2019-06-10").await
}

/// A disabled calendar: the grid says so, no date is tabbable or selectable.
async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    let grid = grids(page, "disabled").await?.swap_remove(0);
    assert_that!(attr(&grid, "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    let calendar = page
        .css("#test-calendar-disabled [role=application]")
        .await?;
    assert_that!(attr(&calendar, "data-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(
        page.count_matching("#test-calendar-disabled [role=button][tabindex]")
            .await?
    )
    .is_equal_to(0);
    let june10 = date(page, "disabled", "Monday, June 10, 2019").await?;
    assert_that!(attr(&june10, "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    june10.click().await?;
    expect_value_unchanged(page, "disabled", "2019-06-05").await?;
    assert_that!(attr(&button(page, "disabled", "Next").await?, "disabled").await?).is_some();
    Ok(())
}

/// A read-only calendar: the grid says so; dates can be navigated with the keyboard, but
/// neither presses nor Enter select.
async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    let grid = grids(page, "read-only").await?.swap_remove(0);
    assert_that!(attr(&grid, "aria-readonly").await?).is_equal_to(Some("true".to_owned()));
    date(page, "read-only", "Monday, June 10, 2019")
        .await?
        .click()
        .await?;
    enter(page, "read-only").await?;
    expect_focus(page, "read-only", "Wednesday, June 5, 2019").await?;
    press_keys(page, &[Key::Right, Key::Enter]).await?;
    expect_focus(page, "read-only", "Thursday, June 6, 2019").await?;
    expect_value_unchanged(page, "read-only", "2019-06-05").await?;
    Ok(())
}

/// An invalid calendar: its selected date is marked invalid and described by the error message.
async fn invalid(page: &Page<'_>) -> Result<(), Report> {
    let calendar = page
        .css("#test-calendar-invalid [role=application]")
        .await?;
    assert_that!(attr(&calendar, "data-invalid").await?).is_equal_to(Some("true".to_owned()));
    let june5 = date(page, "invalid", "Wednesday, June 5, 2019").await?;
    assert_that!(attr(&june5, "aria-invalid").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&june5, "data-invalid").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&cell(&june5).await?, "aria-invalid").await?)
        .is_equal_to(Some("true".to_owned()));
    let error = page
        .driver
        .find(By::XPath(
            "//section[@id='test-calendar-invalid']//div[normalize-space(text())='Invalid date']",
        ))
        .await?;
    let error_id = attr(&error, "id").await?;
    assert_that!(error_id.is_some()).is_true();
    assert_that!(attr(&june5, "aria-describedby").await?).is_equal_to(error_id);
    let june6 = date(page, "invalid", "Thursday, June 6, 2019").await?;
    assert_that!(attr(&june6, "aria-invalid").await?).is_none();
    Ok(())
}

/// Two months: a grid per month, the calendar labelled with both; paging moves both.
async fn two_months(page: &Page<'_>) -> Result<(), Report> {
    let calendar = page
        .css("#test-calendar-two-months [role=application]")
        .await?;
    assert_that!(attr(&calendar, "aria-label").await?)
        .is_equal_to(Some("two-months, June 2019 to July 2019".to_owned()));
    let grids = grids(page, "two-months").await?;
    assert_that!(grids.len()).is_equal_to(2);
    assert_that!(attr(&grids[0], "aria-label").await?)
        .is_equal_to(Some("two-months, June 2019".to_owned()));
    assert_that!(attr(&grids[1], "aria-label").await?)
        .is_equal_to(Some("two-months, July 2019".to_owned()));

    // A date of the second month.
    date(page, "two-months", "Monday, July 15, 2019")
        .await?
        .click()
        .await?;
    wait_for_value(page, "two-months", "2019-07-15").await?;

    button(page, "two-months", "Next").await?.click().await?;
    page.wait_for_attr(&grids[0], "aria-label", Some("two-months, August 2019"))
        .await?;
    page.wait_for_attr(&grids[1], "aria-label", Some("two-months, September 2019"))
        .await
}

/// A week view (`useCalendar.test.js`, "visibleDuration: 1 week"): arrows page by a week when
/// leaving it, Home/End go to its ends.
async fn week_view(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(grid_label(page, "week").await?)
        .is_equal_to(Some("week, June 2, 2019 to June 8, 2019".to_owned()));
    let rows = page
        .driver
        .find_all(By::Css("#test-calendar-week tbody tr"))
        .await?;
    assert_that!(rows.len()).is_equal_to(1);

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
    page.send_keys_to_active(Key::Shift + Key::PageDown).await?;
    expect_focus(page, "week", "Tuesday, July 16, 2019").await
}

/// A view of three days (`useCalendar.test.js`, "visibleDuration: 3 days"): centered on the
/// selected date, paging by three days.
async fn day_view(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(grid_label(page, "days").await?)
        .is_equal_to(Some("days, June 4, 2019 to June 6, 2019".to_owned()));
    let mut names = Vec::new();
    for th in page
        .driver
        .find_all(By::Css("#test-calendar-days th"))
        .await?
    {
        names.push(th.text().await?);
    }
    assert_that!(names).is_equal_to(["T", "W", "T"].map(str::to_owned).to_vec());

    enter(page, "days").await?;
    press_keys(page, &[Key::Left, Key::Left]).await?;
    expect_focus(page, "days", "Monday, June 3, 2019").await?;
    wait_for_grid_label(page, "days", "days, June 1, 2019 to June 3, 2019").await?;
    press_keys(page, &[Key::Down]).await?;
    // A row is the three visible days.
    expect_focus(page, "days", "Thursday, June 6, 2019").await?;
    wait_for_grid_label(page, "days", "days, June 4, 2019 to June 6, 2019").await
}

/// The first day of the week (`useCalendar.test.js`, "firstDayOfWeek").
async fn first_day_of_week(page: &Page<'_>) -> Result<(), Report> {
    let header = page.css("#test-calendar-monday th").await?;
    assert_that!(header.text().await?).is_equal_to("M".to_owned());
    let first = page
        .css("#test-calendar-monday [role=gridcell] > [role=button]")
        .await?;
    assert_that!(attr(&first, "aria-label").await?)
        .is_equal_to(Some("Monday, May 27, 2019".to_owned()));
    Ok(())
}

/// A range by two presses: the first starts it (and highlights while hovering), the second
/// finishes it; the ends are marked; the ends are labelled with the range.
async fn range_by_press(page: &Page<'_>) -> Result<(), Report> {
    let june5 = date(page, "range", "Wednesday, June 5, 2019").await?;
    assert_that!(attr(&june5, "data-selection-start").await?).is_equal_to(Some("true".to_owned()));
    let june10 = date(page, "range", "Monday, June 10, 2019").await?;
    assert_that!(attr(&june10, "data-selection-end").await?).is_equal_to(Some("true".to_owned()));
    let label = attr(&june5, "aria-label").await?.unwrap_or_default();
    assert_that!(label.as_str()).starts_with("Selected Range: ");
    let grid = grids(page, "range").await?.swap_remove(0);
    assert_that!(attr(&grid, "aria-multiselectable").await?).is_equal_to(Some("true".to_owned()));

    let june12 = date(page, "range", "Wednesday, June 12, 2019").await?;
    june12.click().await?;
    page.wait_for_attr(&june12, "data-selected", Some("true"))
        .await?;
    assert_that!(value(page, "range").await?).is_equal_to("2019-06-05 - 2019-06-10".to_owned());
    // The focused cell says how to go on.
    let description = attr(&june12, "aria-describedby").await?.unwrap_or_default();
    assert_that!(
        page.element(&description)
            .await?
            .prop("textContent")
            .await?
    )
    .is_equal_to(Some("Click to finish selecting date range".to_owned()));

    let june14 = date(page, "range", "Friday, June 14, 2019").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&june14)
        .perform()
        .await?;
    let june13 = date(page, "range", "Thursday, June 13, 2019").await?;
    page.wait_for_attr(&june13, "data-selected", Some("true"))
        .await?;
    june14.click().await?;
    wait_for_value(page, "range", "2019-06-12 - 2019-06-14").await?;
    page.wait_for_attr(&june12, "data-selection-start", Some("true"))
        .await?;
    page.wait_for_attr(&june14, "data-selection-end", Some("true"))
        .await?;
    page.wait_for_attr(&june5, "data-selected", None).await
}

/// A range with the keyboard: Enter starts it (moving on by a day), Enter finishes it, Escape
/// cancels a started range.
async fn range_by_keyboard(page: &Page<'_>) -> Result<(), Report> {
    // The pointer away from the dates: while a range is started, hovering a date highlights it
    // (and moves the focus there), also when the layout moves a date under a resting pointer.
    let heading = page.driver.find(By::Css("h1")).await?;
    page.driver
        .action_chain()
        .move_to_element_center(&heading)
        .perform()
        .await?;
    // The focus returns to the last focused date.
    enter(page, "range").await?;
    expect_focus(page, "range", "Friday, June 14, 2019").await?;
    press_keys(page, &[Key::Enter]).await?;
    expect_focus(page, "range", "Saturday, June 15, 2019").await?;
    press_keys(page, &[Key::Right, Key::Enter]).await?;
    wait_for_value(page, "range", "2019-06-14 - 2019-06-16").await?;

    press_keys(page, &[Key::Down, Key::Enter, Key::Right]).await?;
    expect_focus(page, "range", "Tuesday, June 25, 2019").await?;
    let june25 = date(page, "range", "Tuesday, June 25, 2019").await?;
    page.wait_for_attr(&june25, "data-selected", Some("true"))
        .await?;
    press_keys(page, &[Key::Escape]).await?;
    page.wait_for_attr(&june25, "data-selected", None).await?;
    assert_that!(value(page, "range").await?).is_equal_to("2019-06-14 - 2019-06-16".to_owned());

    press_keys(page, &[Key::Enter, Key::Right, Key::Enter]).await?;
    wait_for_value(page, "range", "2019-06-25 - 2019-06-27").await
}

/// A range by dragging from one date to another.
async fn range_by_dragging(page: &Page<'_>) -> Result<(), Report> {
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
    wait_for_value(page, "range", "2019-06-03 - 2019-06-08").await
}

/// Without non-contiguous ranges, a range can't span unavailable dates: after starting one, the
/// dates beyond the next unavailable date are disabled.
async fn range_unavailable(page: &Page<'_>) -> Result<(), Report> {
    let june11 = date(page, "range-unavailable", "Tuesday, June 11, 2019").await?;
    june11.click().await?;
    let june17 = date(page, "range-unavailable", "Monday, June 17, 2019").await?;
    page.wait_for_attr(&june17, "aria-disabled", Some("true"))
        .await?;
    let june14 = date(page, "range-unavailable", "Friday, June 14, 2019").await?;
    assert_that!(attr(&june14, "aria-disabled").await?).is_none();
    june17.click().await?;
    june14.click().await?;
    wait_for_value(page, "range-unavailable", "2019-06-11 - 2019-06-14").await?;
    page.wait_for_attr(&june17, "aria-disabled", None).await
}

/// A calendar labelled by another element as well ("should support aria props on the
/// Calendar"): calendar and grid list it, every referenced id exists.
async fn labelled_by_another_element(page: &Page<'_>) -> Result<(), Report> {
    let calendar = page
        .css("#test-calendar-labelled [role=application]")
        .await?;
    let grid = grids(page, "labelled").await?.swap_remove(0);
    for element in [&calendar, &grid] {
        let ids = attr(element, "aria-labelledby").await?.unwrap_or_default();
        assert_that!(
            ids.split_whitespace()
                .any(|id| id == "test-calendar-labelled-label")
        )
        .is_true();
        for id in ids.split_whitespace() {
            assert_that!(page.count_matching(&format!("[id='{id}']")).await?).is_equal_to(1);
        }
    }
    assert_that!(attr(&grid, "aria-label").await?)
        .is_equal_to(Some("labelled, June 2019".to_owned()));
    Ok(())
}

/// A started range is committed when the pointer is released outside the dates (the default
/// commit behavior `Select`).
async fn range_committed_by_an_outside_press(page: &Page<'_>) -> Result<(), Report> {
    date(page, "range", "Tuesday, June 11, 2019")
        .await?
        .click()
        .await?;
    let june13 = date(page, "range", "Thursday, June 13, 2019").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&june13)
        .perform()
        .await?;
    page.wait_for_attr(&june13, "data-selected", Some("true"))
        .await?;
    page.click_element_with_id("test-calendar-range-after")
        .await?;
    wait_for_value(page, "range", "2019-06-11 - 2019-06-13").await
}

/// In a right-to-left locale, the left arrow moves to the next day.
async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    enter(page, "rtl").await?;
    page.wait_for_active_text("5").await?;
    press_keys(page, &[Key::Left]).await?;
    page.wait_for_active_text("6").await?;
    press_keys(page, &[Key::Right, Key::Right]).await?;
    page.wait_for_active_text("4").await
}

/// Moving the focused date from outside ("should not become focused just by setting the focused
/// date") changes the tabbable date but leaves the browser's focus where it is.
async fn setting_the_focused_date_keeps_the_focus(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-calendar-focus-set")
        .await?;
    let june20 = date(page, "focus", "Thursday, June 20, 2019").await?;
    page.wait_for_attr(&june20, "tabindex", Some("0")).await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(page.active_element_id().await?)
        .is_equal_to(Some("test-calendar-focus-set".to_owned()));
    Ok(())
}

/// A controlled range cleared from outside shows no selection.
async fn controlled_range_cleared(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-calendar-range-clear")
        .await?;
    wait_for_value(page, "range", "none").await?;
    let june11 = date(page, "range", "Tuesday, June 11, 2019").await?;
    page.wait_for_attr(&june11, "data-selected", None).await?;
    assert_that!(attr(&june11, "data-selection-start").await?).is_none();
    assert_that!(
        page.count_matching("#test-calendar-range [data-selected]")
            .await?
    )
    .is_equal_to(0);
    Ok(())
}

/// Unavailable dates may depend on the anchor of a range being selected ("should allow changing
/// the unavailable dates based on the anchor date"): here, dates more than a week away.
async fn unavailable_dates_depending_on_the_anchor(page: &Page<'_>) -> Result<(), Report> {
    let june18 = date(page, "range-week", "Tuesday, June 18, 2019").await?;
    assert_that!(attr(&june18, "data-unavailable").await?).is_none();
    date(page, "range-week", "Monday, June 10, 2019")
        .await?
        .click()
        .await?;
    page.wait_for_attr(&june18, "data-unavailable", Some("true"))
        .await?;
    let june17 = date(page, "range-week", "Monday, June 17, 2019").await?;
    assert_that!(attr(&june17, "data-unavailable").await?).is_none();
    june17.click().await?;
    wait_for_value(page, "range-week", "2019-06-10 - 2019-06-17").await?;
    page.wait_for_attr(&june18, "data-unavailable", None).await
}

/// Range selection by touch (react-spectrum `RangeCalendar.test.js`, "touch"): quick taps start
/// and finish a range, dragging after the press delay selects one, and a touch that turns into
/// a scroll doesn't finish a range being selected.
pub struct RangeCalendarTouchTests {}

#[async_trait]
impl BrowserTest<str> for RangeCalendarTouchTests {
    fn name(&self) -> Cow<'_, str> {
        "calendar_range_touch_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/calendar").await?;

        range_by_touch_taps(&page).await?;
        range_by_touch_dragging(&page).await?;
        range_kept_when_a_touch_scrolls(&page).await?;

        page.expect_no_page_errors().await
    }
}

/// Dispatches a touch pointer event (`pointerdown`, `pointerup`, `pointerenter`,
/// `pointercancel`) at the center of `element`.
async fn touch(page: &Page<'_>, element: &WebElement, kind: &str) -> Result<(), Report> {
    page.driver
        .execute(
            "const [element, kind] = arguments;
             const rect = element.getBoundingClientRect();
             element.dispatchEvent(new PointerEvent(kind, {
                 bubbles: kind !== 'pointerenter', cancelable: true, composed: true,
                 pointerType: 'touch', pointerId: 1, isPrimary: true, button: 0,
                 buttons: kind === 'pointerdown' ? 1 : 0, width: 1, height: 1,
                 clientX: rect.x + rect.width / 2, clientY: rect.y + rect.height / 2,
             }));",
            vec![element.to_json()?, serde_json::Value::from(kind)],
        )
        .await?;
    Ok(())
}

/// A quick tap: pressed and released before the drag delay.
async fn touch_tap(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    touch(page, element, "pointerdown").await?;
    touch(page, element, "pointerup").await
}

/// The labels of the selected dates of the calendar `name`, in document order.
async fn selected_days(page: &Page<'_>, name: &str) -> Result<Vec<String>, Report> {
    let mut days = Vec::new();
    for button in page
        .driver
        .find_all(By::Css(format!("#test-calendar-{name} [role=button][data-selected]")))
        .await?
    {
        days.push(button.text().await?);
    }
    Ok(days)
}

async fn wait_for_selected_days(page: &Page<'_>, name: &str, expected: &[&str]) -> Result<(), Report> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let days = selected_days(page, name).await?;
        if days == expected {
            return Ok(());
        }
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!("expected the selected days {expected:?} of {name}, got {days:?}");
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

/// Two quick taps select a range: the first starts it (a tap is released before the touch drag
/// delay, so it selects on release), the second finishes it.
async fn range_by_touch_taps(page: &Page<'_>) -> Result<(), Report> {
    let june11 = date(page, "range-touch", "Tuesday, June 11, 2019").await?;
    touch_tap(page, &june11).await?;
    wait_for_selected_days(page, "range-touch", &["11"]).await?;
    // Past the drag delay: still only started.
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    assert_that!(selected_days(page, "range-touch").await?).is_equal_to(vec!["11".to_owned()]);
    assert_that!(value(page, "range-touch").await?).is_equal_to("2019-06-05 - 2019-06-10".to_owned());

    let june13 = date(page, "range-touch", "Thursday, June 13, 2019").await?;
    touch_tap(page, &june13).await?;
    wait_for_value(page, "range-touch", "2019-06-11 - 2019-06-13").await
}

/// "selects by dragging with touch": after the delay the pressed date starts the range, dates
/// the finger enters extend it, releasing finishes it.
async fn range_by_touch_dragging(page: &Page<'_>) -> Result<(), Report> {
    let june17 = date(page, "range-touch", "Monday, June 17, 2019").await?;
    touch(page, &june17, "pointerdown").await?;
    // The delay tells dragging from scrolling: nothing changes at first.
    assert_that!(selected_days(page, "range-touch").await?)
        .is_equal_to(["11", "12", "13"].map(str::to_owned).to_vec());
    wait_for_selected_days(page, "range-touch", &["17"]).await?;
    let june18 = date(page, "range-touch", "Tuesday, June 18, 2019").await?;
    touch(page, &june18, "pointerenter").await?;
    wait_for_selected_days(page, "range-touch", &["17", "18"]).await?;
    let june23 = date(page, "range-touch", "Sunday, June 23, 2019").await?;
    touch(page, &june23, "pointerenter").await?;
    wait_for_selected_days(page, "range-touch", &["17", "18", "19", "20", "21", "22", "23"]).await?;
    assert_that!(value(page, "range-touch").await?).is_equal_to("2019-06-11 - 2019-06-13".to_owned());
    touch(page, &june23, "pointerup").await?;
    wait_for_value(page, "range-touch", "2019-06-17 - 2019-06-23").await
}

/// "selection isn't prematurely finalized when touching a day cell to scroll through the
/// calendar": a touch cancelled by scrolling doesn't finish the range being selected.
async fn range_kept_when_a_touch_scrolls(page: &Page<'_>) -> Result<(), Report> {
    date(page, "range-touch", "Sunday, June 23, 2019")
        .await?
        .click()
        .await?;
    wait_for_selected_days(page, "range-touch", &["23"]).await?;
    let june10 = date(page, "range-touch", "Monday, June 10, 2019").await?;
    touch(page, &june10, "pointerdown").await?;
    touch(page, &june10, "pointercancel").await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    assert_that!(value(page, "range-touch").await?).is_equal_to("2019-06-17 - 2019-06-23".to_owned());
    date(page, "range-touch", "Tuesday, June 25, 2019")
        .await?
        .click()
        .await?;
    wait_for_value(page, "range-touch", "2019-06-23 - 2019-06-25").await
}

/// A calendar without a value or focused date shows today (react-spectrum `Calendar.ssr.test.js`
/// renders it on the server). The server's today may be another date than the browser's (its
/// time zone): the page hydrates with the server's date, then the calendar moves to the
/// browser's today, so the tabbable date is the one marked as today.
pub struct CalendarTodayTests {}

#[async_trait]
impl BrowserTest<str> for CalendarTodayTests {
    fn name(&self) -> Cow<'_, str> {
        "calendar_today_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
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
            leptos_browser_test::bail!("no time zone with another date than {server_today}");
        };
        driver
            .cdp()
            .send_raw(
                "Emulation.setTimezoneOverride",
                serde_json::json!({ "timezoneId": zone }),
            )
            .await?;
        page.goto_path("/atoms/calendar").await?;

        page.wait_for_selector(
            "#test-calendar-today [role=gridcell] > [role=button][tabindex='0'][aria-label^='Today, ']",
        )
        .await?;
        assert_that!(
            page.count_matching("#test-calendar-today [role=button][tabindex='0']")
                .await?
        )
        .is_equal_to(1);
        // Keyboard focus goes there.
        enter(&page, "today").await?;
        let active = page.driver.active_element().await?;
        assert_that!(attr(&active, "aria-label").await?.unwrap_or_default().as_str())
            .starts_with("Today, ");

        page.expect_no_page_errors().await
    }
}

/// The views and paging of calendars, their pickers and announcements: `pageBehavior: single`
/// (`useCalendar.test.js`, "pagination"), a two-week view, a fixed number of week rows, held arrow
/// keys, a changing visible duration, month and year pickers (RAC `Calendar.test.js`,
/// `RangeCalendar.test.tsx`), the live announcements and the commit behaviors of a range being
/// selected (react-spectrum `RangeCalendar.test.js`, "announcing", "pointer events").
pub struct CalendarViewTests {}

#[async_trait]
impl BrowserTest<str> for CalendarViewTests {
    fn name(&self) -> Cow<'_, str> {
        "calendar_view_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/calendar").await?;

        page_behavior_single(&page).await?;
        two_weeks(&page).await?;
        weeks_in_month(&page).await?;
        held_arrow_keys(&page).await?;
        changing_the_visible_duration(&page).await?;
        month_and_year_pickers(&page).await?;
        announcements(&page).await?;
        commit_behaviors(&page).await?;

        page.expect_no_page_errors().await
    }
}

async fn grid_labels(page: &Page<'_>, name: &str) -> Result<Vec<String>, Report> {
    let mut labels = Vec::new();
    for grid in grids(page, name).await? {
        labels.push(attr(&grid, "aria-label").await?.unwrap_or_default());
    }
    Ok(labels)
}

async fn wait_for_grid_labels(page: &Page<'_>, name: &str, expected: &[&str]) -> Result<(), Report> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let labels = grid_labels(page, name).await?;
        if labels == expected {
            return Ok(());
        }
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!("expected the grids of {name} {expected:?}, got {labels:?}");
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

/// `pageBehavior: single` pages by one month, week or day of the visible duration.
async fn page_behavior_single(page: &Page<'_>) -> Result<(), Report> {
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
    .await
}

/// A two-week view (`useCalendar.test.js`, "visibleDuration: 2 weeks"): two rows, labelled with
/// its dates.
async fn two_weeks(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(grid_label(page, "two-weeks").await?)
        .is_equal_to(Some("two-weeks, June 2, 2019 to June 15, 2019".to_owned()));
    assert_that!(
        page.count_matching("#test-calendar-two-weeks tbody tr")
            .await?
    )
    .is_equal_to(2);
    enter(page, "two-weeks").await?;
    expect_focus(page, "two-weeks", "Wednesday, June 5, 2019").await?;
    press_keys(page, &[Key::Down, Key::Down]).await?;
    expect_focus(page, "two-weeks", "Wednesday, June 19, 2019").await?;
    wait_for_grid_label(page, "two-weeks", "two-weeks, June 16, 2019 to June 29, 2019").await
}

/// RAC "should support weeksInMonth prop": April 2026 has five week rows, six are shown.
async fn weeks_in_month(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(
        page.count_matching("#test-calendar-six-weeks tbody tr")
            .await?
    )
    .is_equal_to(6);
    Ok(())
}

/// Dispatches a keydown (`repeat`: as when the key is held) and, for the last, a keyup to the
/// focused element.
async fn hold_key(page: &Page<'_>, key: &str, repeats: usize) -> Result<(), Report> {
    page.driver
        .execute(
            "const [key, repeats] = arguments;
             const target = document.activeElement;
             const fire = (type, repeat) => target.dispatchEvent(new KeyboardEvent(type, {
                 key, repeat, bubbles: true, cancelable: true, composed: true,
             }));
             fire('keydown', false);
             for (let i = 0; i < repeats; i++) fire('keydown', true);
             fire('keyup', false);",
            vec![serde_json::Value::from(key), serde_json::Value::from(repeats)],
        )
        .await?;
    Ok(())
}

/// RAC "should support repeat keydown events when holding an arrow key".
async fn held_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    let march3 = date(page, "held", "Tuesday, March 3, 2020").await?;
    march3.click().await?;
    page.wait_for_focus_on(&march3, "March 3").await?;
    hold_key(page, "ArrowRight", 1).await?;
    expect_focus(page, "held", "Thursday, March 5, 2020").await
}

/// RAC "should handle changing the visible duration": a week view becomes a month view around
/// the focused date.
async fn changing_the_visible_duration(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(heading(page, "duration").await?)
        .is_equal_to("April 5, 2026 to April 11, 2026".to_owned());
    page.click_element_with_id("test-calendar-duration-month")
        .await?;
    page.wait_for_selector_text("#test-calendar-duration h2", "April 2026")
        .await?;
    wait_for_grid_label(page, "duration", "duration, April 2026").await?;
    assert_that!(
        page.count_matching("#test-calendar-duration tbody tr")
            .await?
    )
    .is_equal_to(5);
    Ok(())
}

async fn option_texts(select: &WebElement) -> Result<Vec<String>, Report> {
    let mut texts = Vec::new();
    for option in select.find_all(By::Css("option")).await? {
        texts.push(option.text().await?);
    }
    Ok(texts)
}

/// RAC "should support month and year dropdowns": the pickers list the months and 20 years
/// around the focused date's and move it; the year picker follows.
async fn month_and_year_pickers(page: &Page<'_>) -> Result<(), Report> {
    let month = page
        .css("#test-calendar-pickers select[aria-label=month]")
        .await?;
    let year = page
        .css("#test-calendar-pickers select[aria-label=year]")
        .await?;
    wait_for_grid_label(page, "pickers", "Appointment date, April 2026").await?;
    assert_that!(month.prop("value").await?).is_equal_to(Some("4".to_owned()));
    assert_that!(option_texts(&month).await?).is_equal_to(
        [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ]
        .map(str::to_owned)
        .to_vec(),
    );
    month
        .find(By::XPath("option[normalize-space()='Jun']"))
        .await?
        .click()
        .await?;
    wait_for_grid_label(page, "pickers", "Appointment date, June 2026").await?;
    assert_that!(month.prop("value").await?).is_equal_to(Some("6".to_owned()));

    assert_that!(option_texts(&year).await?)
        .is_equal_to((2016..2036).map(|year| year.to_string()).collect::<Vec<_>>());
    year.find(By::XPath("option[normalize-space()='2030']"))
        .await?
        .click()
        .await?;
    wait_for_grid_label(page, "pickers", "Appointment date, June 2030").await?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while option_texts(&year).await?.first().map(String::as_str) != Some("2020") {
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!("the year picker didn't follow the focused year");
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    assert_that!(option_texts(&year).await?)
        .is_equal_to((2020..2040).map(|year| year.to_string()).collect::<Vec<_>>());
    Ok(())
}

/// Wait for a polite live announcement.
async fn wait_for_announcement(page: &Page<'_>, text: &str) -> Result<(), Report> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let mut texts = Vec::new();
        for entry in page
            .driver
            .find_all(By::Css("[data-live-announcer] [aria-live=polite] div"))
            .await?
        {
            texts.push(entry.prop("textContent").await?.unwrap_or_default());
        }
        if texts.iter().any(|entry| entry == text) {
            return Ok(());
        }
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!("no announcement {text:?}, got {texts:?}");
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

/// "announces when the current month changes", "announces when the selected date range
/// changes".
async fn announcements(page: &Page<'_>) -> Result<(), Report> {
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
    .await
}

/// Starts a range at `start` and hovers `end` in the calendar `name`.
async fn start_range(page: &Page<'_>, name: &str, start: &str, end: &str) -> Result<(), Report> {
    date(page, name, start).await?.click().await?;
    let end = date(page, name, end).await?;
    page.driver
        .action_chain()
        .move_to_element_center(&end)
        .perform()
        .await?;
    page.wait_for_attr(&end, "data-selected", Some("true")).await
}

/// The commit behaviors of a range being selected, when the pointer is released on the calendar
/// outside its dates (its heading) and when the focus leaves it (Tab): `Select` finishes it at the
/// hovered date, `Clear` clears the value, `Reset` keeps the value.
async fn commit_behaviors(page: &Page<'_>) -> Result<(), Report> {
    let click_heading = |name: &'static str| async move {
        page.css(&format!("#test-calendar-{name} h2"))
            .await?
            .click()
            .await?;
        Ok::<(), Report>(())
    };

    start_range(page, "commit-select", "Tuesday, November 25, 2025", "Thursday, November 20, 2025")
        .await?;
    click_heading("commit-select").await?;
    wait_for_value(page, "commit-select", "2025-11-20 - 2025-11-25").await?;
    start_range(page, "commit-select", "Thursday, November 27, 2025", "Saturday, November 22, 2025")
        .await?;
    page.press_tab().await?;
    wait_for_value(page, "commit-select", "2025-11-22 - 2025-11-27").await?;

    start_range(page, "commit-clear", "Tuesday, November 25, 2025", "Thursday, November 20, 2025")
        .await?;
    click_heading("commit-clear").await?;
    wait_for_value(page, "commit-clear", "none").await?;
    page.wait_for_count("#test-calendar-commit-clear [data-selected]", 0)
        .await?;
    start_range(page, "commit-clear", "Tuesday, November 25, 2025", "Thursday, November 20, 2025")
        .await?;
    page.press_tab().await?;
    page.wait_for_count("#test-calendar-commit-clear [data-selected]", 0)
        .await?;
    expect_value_unchanged(page, "commit-clear", "none").await?;

    let november25 = date(page, "commit-reset", "Tuesday, November 25, 2025").await?;
    let november13 = date(page, "commit-reset", "Thursday, November 13, 2025").await?;
    start_range(page, "commit-reset", "Tuesday, November 25, 2025", "Thursday, November 20, 2025")
        .await?;
    assert_that!(attr(&november13, "data-selected").await?).is_none();
    click_heading("commit-reset").await?;
    page.wait_for_attr(&november25, "data-selected", None).await?;
    page.wait_for_attr(&november13, "data-selected", Some("true"))
        .await?;
    expect_value_unchanged(page, "commit-reset", "2025-11-13 - 2025-11-15").await?;
    start_range(page, "commit-reset", "Tuesday, November 25, 2025", "Thursday, November 20, 2025")
        .await?;
    page.press_tab().await?;
    page.wait_for_attr(&november13, "data-selected", Some("true"))
        .await?;
    expect_value_unchanged(page, "commit-reset", "2025-11-13 - 2025-11-15").await
}
