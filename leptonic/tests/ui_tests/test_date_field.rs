// Upstream: react-aria-components/test/DateField.test.js @ 99e6102368
// Upstream: react-aria-components/test/TimeField.test.js @ 99e6102368
// Upstream: react-aria-components/test/DatePicker.test.js @ 99e6102368
// Upstream: react-aria-components/test/DateRangePicker.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Behavior of the date and time field atoms: structure and labels, typing that moves on when a
/// segment is full, arrows between and within segments, Backspace, an invalid date committed
/// constrained when the field is left, form reset, min validation, disabled and read-only
/// fields, dates with times, zoned values, a 12-hour time field, and a date picker (opening by
/// press and Alt+ArrowDown, selecting in its calendar, Escape).
/// Spec: react-aria-components `DateField.test.js`, `TimeField.test.js`, `DatePicker.test.js`.
pub struct DateFieldTests {}

#[async_trait]
impl BrowserTest<str> for DateFieldTests {
    fn name(&self) -> Cow<'_, str> {
        "date_field_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/date-field").await?;

        structure(&page).await?;
        typing(&page).await?;
        arrows_and_backspace(&page).await?;
        invalid_date_committed_when_left(&page).await?;
        form_reset(&page).await?;
        min_validation(&page).await?;
        disabled_and_read_only(&page).await?;
        date_and_time(&page).await?;
        zoned(&page).await?;
        time_field(&page).await?;
        date_picker(&page).await?;
        date_range_picker(&page).await?;
        group_states(&page).await?;

        page.expect_no_page_errors().await
    }
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn segment(page: &Page<'_>, section: &str, kind: &str) -> Result<WebElement, Report> {
    page.css(&format!("#test-df-{section} [data-type='{kind}']"))
        .await
}

async fn wait_for_segment_text(
    page: &Page<'_>,
    section: &str,
    kind: &str,
    expected: &str,
) -> Result<(), Report> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let text = segment(page, section, kind).await?.text().await?;
        if text == expected {
            return Ok(());
        }
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!("expected {section} {kind} {expected:?}, got {text:?}");
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

async fn expect_focus(page: &Page<'_>, section: &str, kind: &str) -> Result<(), Report> {
    let element = segment(page, section, kind).await?;
    page.wait_for_focus_on(&element, &format!("{section} {kind}"))
        .await
}

async fn wait_for_value(page: &Page<'_>, section: &str, expected: &str) -> Result<(), Report> {
    page.wait_for_text(&format!("test-df-{section}-value"), expected)
        .await
}

async fn type_text(page: &Page<'_>, text: &str) -> Result<(), Report> {
    for key in text.chars() {
        page.send_keys_to_active(key.to_string()).await?;
    }
    Ok(())
}

/// A group labelled by its label, segments as spin buttons named by kind and labelled by the
/// field, placeholders, hidden literals, the description on the first segment only, a hidden
/// input with the field's name.
async fn structure(page: &Page<'_>) -> Result<(), Report> {
    let group = page.css("#test-df-basic [role=group]").await?;
    let label_id = page
        .driver
        .find(By::XPath(
            "//section[@id='test-df-basic']//span[normalize-space()='Birthday']",
        ))
        .await?
        .attr("id")
        .await?
        .unwrap_or_default();
    let labelledby = attr(&group, "aria-labelledby").await?.unwrap_or_default();
    assert_that!(labelledby.split_whitespace().any(|id| id == label_id)).is_true();

    let month = segment(page, "basic", "month").await?;
    assert_that!(attr(&month, "role").await?).is_equal_to(Some("spinbutton".to_owned()));
    assert_that!(attr(&month, "aria-label").await?).is_equal_to(Some("month, ".to_owned()));
    let month_labelledby = attr(&month, "aria-labelledby").await?.unwrap_or_default();
    assert_that!(month_labelledby.split_whitespace().any(|id| id == label_id)).is_true();
    assert_that!(month.text().await?).is_equal_to("mm".to_owned());
    assert_that!(attr(&month, "data-placeholder").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&month, "contenteditable").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&month, "inputmode").await?).is_equal_to(Some("numeric".to_owned()));
    assert_that!(segment(page, "basic", "day").await?.text().await?).is_equal_to("dd".to_owned());
    assert_that!(segment(page, "basic", "year").await?.text().await?)
        .is_equal_to("yyyy".to_owned());
    let literal = segment(page, "basic", "literal").await?;
    assert_that!(attr(&literal, "aria-hidden").await?).is_equal_to(Some("true".to_owned()));

    // The description: on the first segment, not on the others.
    assert_that!(attr(&month, "aria-describedby").await?).is_some();
    let day = segment(page, "basic", "day").await?;
    assert_that!(attr(&day, "aria-describedby").await?).is_none();

    let input = page.css("#test-df-basic input[name=birthday]").await?;
    assert_that!(input.prop("value").await?).is_equal_to(Some(String::new()));
    Ok(())
}

/// Typing fills a segment and moves on once no further digit fits.
async fn typing(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-df-basic-before").await?;
    page.press_tab().await?;
    expect_focus(page, "basic", "month").await?;
    type_text(page, "6").await?;
    expect_focus(page, "basic", "day").await?;
    type_text(page, "1").await?;
    // 1 may still become 10 to 19.
    expect_focus(page, "basic", "day").await?;
    type_text(page, "5").await?;
    expect_focus(page, "basic", "year").await?;
    type_text(page, "2024").await?;
    // The last segment keeps the focus (no era shows for the years before 1000 typed on the
    // way: it would take the focus, then go away with it).
    expect_focus(page, "basic", "year").await?;
    wait_for_value(page, "basic", "2024-06-15").await?;
    let input = page.css("#test-df-basic input[name=birthday]").await?;
    assert_that!(input.prop("value").await?).is_equal_to(Some("2024-06-15".to_owned()));

    // The group describes the value.
    let group = page.css("#test-df-basic [role=group]").await?;
    let describedby = attr(&group, "aria-describedby").await?.unwrap_or_default();
    let mut descriptions = Vec::new();
    for id in describedby.split_whitespace() {
        descriptions.push(
            page.element(id)
                .await?
                .prop("textContent")
                .await?
                .unwrap_or_default(),
        );
    }
    assert_that!(descriptions).contains("Selected Date: June 15, 2024".to_owned());
    Ok(())
}

/// Arrows step a segment and move between them; Backspace deletes a digit, and on an empty
/// segment moves to the previous one.
async fn arrows_and_backspace(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys_to_active(Key::Up).await?;
    wait_for_value(page, "basic", "2025-06-15").await?;
    page.send_keys_to_active(Key::Left).await?;
    expect_focus(page, "basic", "day").await?;
    page.send_keys_to_active(Key::Down).await?;
    wait_for_value(page, "basic", "2025-06-14").await?;
    page.send_keys_to_active(Key::Backspace).await?;
    wait_for_segment_text(page, "basic", "day", "1").await?;
    wait_for_value(page, "basic", "2025-06-01").await?;
    page.send_keys_to_active(Key::Backspace).await?;
    wait_for_segment_text(page, "basic", "day", "dd").await?;
    // Incomplete: the value stays until the field has one again.
    assert_that!(page.read_text_of("test-df-basic-value").await?)
        .is_equal_to("2025-06-01".to_owned());
    page.send_keys_to_active(Key::Backspace).await?;
    expect_focus(page, "basic", "month").await?;
    page.send_keys_to_active(Key::Right).await?;
    type_text(page, "5").await?;
    wait_for_value(page, "basic", "2025-06-05").await
}

/// February 30 stays as typed while the field is edited, and is committed as February 28 when
/// the field is left.
async fn invalid_date_committed_when_left(page: &Page<'_>) -> Result<(), Report> {
    segment(page, "basic", "month").await?.click().await?;
    expect_focus(page, "basic", "month").await?;
    type_text(page, "2").await?;
    wait_for_value(page, "basic", "2025-02-05").await?;
    expect_focus(page, "basic", "day").await?;
    type_text(page, "30").await?;
    wait_for_segment_text(page, "basic", "day", "30").await?;
    // "3" made a valid date (February 3), "30" does not.
    assert_that!(page.read_text_of("test-df-basic-value").await?)
        .is_equal_to("2025-02-03".to_owned());
    page.click_element_with_id("test-df-basic-before").await?;
    wait_for_value(page, "basic", "2025-02-28").await?;
    wait_for_segment_text(page, "basic", "day", "28").await
}

/// Resetting the form restores the initial value.
async fn form_reset(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-df-reset").await?;
    wait_for_value(page, "basic", "none").await?;
    wait_for_segment_text(page, "basic", "month", "mm").await
}

/// A value before the minimum is invalid (shown right away with ARIA validation) until fixed.
async fn min_validation(page: &Page<'_>) -> Result<(), Report> {
    let month = segment(page, "min", "month").await?;
    page.wait_for_attr(&month, "aria-invalid", Some("true"))
        .await?;
    let text = page.css("#test-df-min").await?.text().await?;
    assert_that!(text.as_str()).contains("Value must be 5/1/2024 or later.");
    month.click().await?;
    page.send_keys_to_active(Key::Up).await?;
    wait_for_value(page, "min", "2024-05-30").await?;
    page.wait_for_attr(&month, "aria-invalid", None).await?;
    let text = page.css("#test-df-min").await?.text().await?;
    assert_that!(text.contains("or later")).is_false();
    Ok(())
}

/// A disabled field's segments aren't tabbable; a read-only field's don't change.
async fn disabled_and_read_only(page: &Page<'_>) -> Result<(), Report> {
    let disabled = segment(page, "disabled", "month").await?;
    assert_that!(attr(&disabled, "tabindex").await?).is_none();
    assert_that!(attr(&disabled, "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&disabled, "contenteditable").await?).is_none();
    let group = page.css("#test-df-disabled [role=group]").await?;
    assert_that!(attr(&group, "aria-disabled").await?).is_equal_to(Some("true".to_owned()));

    let read_only = segment(page, "read-only", "month").await?;
    assert_that!(attr(&read_only, "aria-readonly").await?).is_equal_to(Some("true".to_owned()));
    read_only.click().await?;
    page.send_keys_to_active(Key::Up).await?;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    assert_that!(read_only.text().await?).is_equal_to("6".to_owned());
    Ok(())
}

/// A date with a time (24 hours): hours and minutes follow the date.
async fn date_and_time(page: &Page<'_>) -> Result<(), Report> {
    segment(page, "date-time", "month").await?.click().await?;
    type_text(page, "6152024").await?;
    expect_focus(page, "date-time", "hour").await?;
    type_text(page, "14").await?;
    expect_focus(page, "date-time", "minute").await?;
    type_text(page, "30").await?;
    wait_for_value(page, "date-time", "2024-06-15T14:30:00").await
}

/// A zoned value shows its time zone (not editable); stepping the hour keeps the zone.
async fn zoned(page: &Page<'_>) -> Result<(), Report> {
    let zone = segment(page, "zoned", "timeZoneName").await?;
    assert_that!(zone.text().await?).is_equal_to("EDT".to_owned());
    assert_that!(attr(&zone, "role").await?).is_equal_to(Some("textbox".to_owned()));
    assert_that!(segment(page, "zoned", "dayPeriod").await?.text().await?)
        .is_equal_to("AM".to_owned());
    segment(page, "zoned", "hour").await?.click().await?;
    page.send_keys_to_active(Key::Up).await?;
    wait_for_value(page, "zoned", "2024-06-05T10:30:00-04:00[America/New_York]").await
}

/// A 12-hour time field: the hour moves on after a digit that can't start a two-digit hour,
/// "p" picks PM.
async fn time_field(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(segment(page, "time", "hour").await?.text().await?).is_equal_to("––".to_owned());
    segment(page, "time", "hour").await?.click().await?;
    type_text(page, "9").await?;
    expect_focus(page, "time", "minute").await?;
    type_text(page, "30").await?;
    expect_focus(page, "time", "dayPeriod").await?;
    type_text(page, "p").await?;
    wait_for_value(page, "time", "21:30:00").await?;
    // The form gets the time ("should support form value").
    let input = page.css("#test-df-time input[name=alarm]").await?;
    assert_that!(input.prop("value").await?).is_equal_to(Some("21:30:00".to_owned()));
    Ok(())
}

/// A date picker: a labelled group with the field and a button opening a dialog with a calendar;
/// selecting a date closes it and fills the field; Alt+ArrowDown opens it from the field; Escape
/// closes it unchanged.
async fn date_picker(page: &Page<'_>) -> Result<(), Report> {
    let group = page.css("#test-df-picker [role=group]").await?;
    assert_that!(attr(&group, "aria-labelledby").await?).is_some();
    // The field inside has no role of its own (the group labels and describes it); its
    // segments are labelled by the picker's label.
    page.css("#test-df-picker [role=presentation]").await?;
    let label_id = page
        .driver
        .find(By::XPath(
            "//section[@id='test-df-picker']//span[normalize-space()='Event']",
        ))
        .await?
        .attr("id")
        .await?
        .unwrap_or_default();
    let month_labelledby = attr(&segment(page, "picker", "month").await?, "aria-labelledby")
        .await?
        .unwrap_or_default();
    assert_that!(month_labelledby.split_whitespace().any(|id| id == label_id)).is_true();
    let button = page
        .css("#test-df-picker button[aria-haspopup=dialog]")
        .await?;
    assert_that!(attr(&button, "aria-label").await?).is_equal_to(Some("Calendar".to_owned()));
    assert_that!(attr(&button, "aria-expanded").await?).is_equal_to(Some("false".to_owned()));

    // Opening: the calendar shows the placeholder's month, its date focused.
    button.click().await?;
    page.wait_for_selector("[role=dialog] [role=grid]").await?;
    page.wait_for_attr(&button, "aria-expanded", Some("true"))
        .await?;
    let june1 = page
        .css("[role=dialog] [role=button][aria-label^='Saturday, June 1, 2024']")
        .await?;
    page.wait_for_focus_on(&june1, "the placeholder date")
        .await?;
    page.css("[role=dialog] [role=button][aria-label^='Saturday, June 15, 2024']")
        .await?
        .click()
        .await?;
    page.wait_for_no_selector("[role=dialog]").await?;
    wait_for_value(page, "picker", "2024-06-15").await?;
    wait_for_segment_text(page, "picker", "day", "15").await?;

    // From the field: Alt+ArrowDown opens on the value, arrows and Enter select.
    segment(page, "picker", "month").await?.click().await?;
    page.send_keys_to_active(Key::Alt + Key::Down).await?;
    page.wait_for_selector("[role=dialog] [role=grid]").await?;
    let june15 = page
        .css("[role=dialog] [role=button][aria-label^='Saturday, June 15, 2024']")
        .await?;
    page.wait_for_focus_on(&june15, "the value's date").await?;
    page.send_keys_to_active(Key::Right).await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_no_selector("[role=dialog]").await?;
    wait_for_value(page, "picker", "2024-06-16").await?;

    // Escape closes without a change.
    button.click().await?;
    page.wait_for_selector("[role=dialog] [role=grid]").await?;
    page.send_keys_to_active(Key::Right).await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=dialog]").await?;
    assert_that!(page.read_text_of("test-df-picker-value").await?)
        .is_equal_to("2024-06-16".to_owned());

    // Typing in the field changes the value.
    segment(page, "picker", "day").await?.click().await?;
    page.send_keys_to_active(Key::Up).await?;
    wait_for_value(page, "picker", "2024-06-17").await?;

    // An edit doesn't come back when the value returns to what it was edited from.
    page.click_element_with_id("test-df-picker-clear").await?;
    wait_for_segment_text(page, "picker", "month", "mm").await?;
    segment(page, "picker", "month").await?.click().await?;
    type_text(page, "7").await?;
    wait_for_segment_text(page, "picker", "month", "7").await?;
    button.click().await?;
    page.wait_for_selector("[role=dialog] [role=grid]").await?;
    page.css("[role=dialog] [role=button][aria-label^='Wednesday, June 12, 2024']")
        .await?
        .click()
        .await?;
    wait_for_value(page, "picker", "2024-06-12").await?;
    page.click_element_with_id("test-df-picker-clear").await?;
    wait_for_segment_text(page, "picker", "month", "mm").await
}

/// The segments of one field of the range picker (`0`: start, `1`: end).
async fn range_segment(page: &Page<'_>, field: usize, kind: &str) -> Result<WebElement, Report> {
    let mut segments = page
        .driver
        .find_all(By::Css(format!("#test-df-range [data-type='{kind}']")))
        .await?;
    Ok(segments.swap_remove(field))
}

/// A date range picker: start and end fields named as such; a range selected in the popover
/// fills both; arrows move across both fields (not to the button); a reversed range is invalid.
async fn date_range_picker(page: &Page<'_>) -> Result<(), Report> {
    let start_month = range_segment(page, 0, "month").await?;
    assert_that!(
        attr(&start_month, "aria-label")
            .await?
            .unwrap_or_default()
            .as_str()
    )
    .starts_with("month, Start Date");
    let end_month = range_segment(page, 1, "month").await?;
    assert_that!(
        attr(&end_month, "aria-label")
            .await?
            .unwrap_or_default()
            .as_str()
    )
    .starts_with("month, End Date");

    page.css("#test-df-range button[aria-haspopup=dialog]")
        .await?
        .click()
        .await?;
    page.wait_for_selector("[role=dialog] [role=grid]").await?;
    page.css("[role=dialog] [role=button][aria-label*='Monday, June 10, 2024']")
        .await?
        .click()
        .await?;
    page.css("[role=dialog] [role=button][aria-label*='Friday, June 14, 2024']")
        .await?
        .click()
        .await?;
    page.wait_for_no_selector("[role=dialog]").await?;
    page.wait_for_text("test-df-range-value", "2024-06-10 - 2024-06-14")
        .await?;
    assert_that!(range_segment(page, 1, "day").await?.text().await?).is_equal_to("14".to_owned());

    // From the start's year to the end's month.
    let start_year = range_segment(page, 0, "year").await?;
    start_year.click().await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_focus_on(&end_month, "the end's month")
        .await?;
    // Arrows go on to the button (react-aria's `useDatePickerGroup`); the segments' own moving on
    // (a full year typed) stays in the fields.
    let end_year = range_segment(page, 1, "year").await?;
    end_year.click().await?;
    page.send_keys_to_active(Key::Right).await?;
    let button = page
        .css("#test-df-range button[aria-haspopup=dialog]")
        .await?;
    page.wait_for_focus_on(&button, "the button").await?;
    end_year.click().await?;
    type_text(page, "2024").await?;
    page.wait_for_focus_on(&end_year, "the end's year").await?;

    // An end before the start is invalid, shown when the picker is left (native validation).
    end_month.click().await?;
    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_text("test-df-range-value", "2024-06-10 - 2024-05-14")
        .await?;
    page.click_element_with_id("test-df-range-after").await?;
    let section = page.css("#test-df-range").await?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !section
        .text()
        .await?
        .contains("Start date must be before end date.")
    {
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!("no range order error shown");
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    Ok(())
}

/// The field and the picker's group show hover and focus like react-aria-components' `Group`:
/// `data-focus-within`, and `data-focus-visible` only with the keyboard.
async fn group_states(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/date-field").await?;
    let input = page
        .driver
        .find(By::XPath(
            "//section[@id='test-df-basic']//*[@data-type='month']/..",
        ))
        .await?;
    assert_that!(attr(&input, "data-focus-within").await?).is_none();
    page.click_element_with_id("test-df-basic-before").await?;
    page.press_tab().await?;
    expect_focus(page, "basic", "month").await?;
    page.wait_for_attr(&input, "data-focus-within", Some("true"))
        .await?;
    page.wait_for_attr(&input, "data-focus-visible", Some("true"))
        .await?;

    let group = page.css("#test-df-picker [role=group]").await?;
    segment(page, "picker", "day").await?.click().await?;
    page.wait_for_attr(&group, "data-focus-within", Some("true"))
        .await?;
    assert_that!(attr(&group, "data-focus-visible").await?).is_none();
    page.wait_for_attr(&group, "data-hovered", Some("true"))
        .await?;
    page.wait_for_attr(&input, "data-focus-within", None).await
}
