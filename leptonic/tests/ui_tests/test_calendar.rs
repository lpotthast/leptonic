// Upstream: react-aria-components/test/Calendar.test.js @ 99e6102368
// Upstream: react-aria-components/test/RangeCalendar.test.tsx @ 99e6102368
// Upstream: react-aria/test/calendar/useCalendar.test.js @ 99e6102368
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
    assert_that!(value(page, "min-max").await?).is_equal_to("2019-06-15".to_owned());

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
    assert_that!(value(page, "unavailable").await?).is_equal_to("2019-06-05".to_owned());

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
    assert_that!(value(page, "disabled").await?).is_equal_to("2019-06-05".to_owned());
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
    assert_that!(value(page, "read-only").await?).is_equal_to("2019-06-05".to_owned());
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
