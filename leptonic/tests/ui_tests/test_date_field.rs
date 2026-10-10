// Upstream: react-aria-components/test/DateField.test.js @ 99e6102368
// Upstream: react-aria-components/test/TimeField.test.js @ 99e6102368
// Upstream: react-aria-components/test/DatePicker.test.js @ 99e6102368
// Upstream: react-aria-components/test/DateRangePicker.test.js @ 99e6102368
// Upstream: react-aria-components/test/HiddenDateInput.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/datepicker/DateField.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/datepicker/TimeField.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/datepicker/DatePicker.test.js @ 99e6102368
//! Behavior of the date and time field atoms: structure and labels, typing that moves on when a
//! segment is full, arrows between and within segments, Backspace, an invalid date committed
//! constrained when the field is left, form reset, min validation, disabled and read-only
//! fields, dates with times, zoned values, a 12-hour time field, and a date picker (opening by
//! press and Alt+ArrowDown, selecting in its calendar, Escape). Segment editing per segment
//! (arrows, Page Up/Down, Home/End, typing, Backspace, Arabic digits, eras), daylight saving time,
//! spin button values, focus on pressing the field, labels and descriptions, states, native and
//! ARIA validation (min/max, `validate`, server errors, custom messages, reset).
//! Spec: react-aria-components `DateField.test.js`, `TimeField.test.js`, `DatePicker.test.js`,
//! `HiddenDateInput.test.js`; react-spectrum `DateField.test.js`, `TimeField.test.js` and the
//! "editing" and "focus management" parts of `DatePicker.test.js`.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, Platform, css};

const PATH: &str = "/atoms/date-field";

/// The segment of `kind` (`month`, `day`, `hour`, `literal`, ...) of the field in
/// `#test-df-<section>`.
async fn segment(page: &Page<'_>, section: &str, kind: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-df-{section} [data-type='{kind}']"))
        .await
}

/// The fixture's output of the value of the field in `#test-df-<section>` (`none` without one).
async fn value(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-df-{section}-value")).await
}

/// Waits until the value of the field in `#test-df-<section>` is `expected`.
async fn wait_for_value(page: &Page<'_>, section: &str, expected: &str) -> Result<(), Report> {
    value(page, section)
        .await?
        .wait_for_inner_text(expected)
        .await
}

/// Types June 15, 2024 into the basic field, tabbing in from the button before it; the focus
/// stays on the year.
async fn type_basic_date(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-df-basic-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&segment(page, "basic", "month").await?)
        .await?;
    page.type_text("6").await?;
    page.wait_for_focus(&segment(page, "basic", "day").await?)
        .await?;
    page.type_text("15").await?;
    page.wait_for_focus(&segment(page, "basic", "year").await?)
        .await?;
    page.type_text("2024").await?;
    wait_for_value(page, "basic", "2024-06-15").await?;
    page.wait_for_focus(&segment(page, "basic", "year").await?)
        .await?;
    Ok(())
}

/// The id of the label with the text `text` in the section `#test-df-<section>`.
async fn label_id(page: &Page<'_>, section: &str, text: &str) -> Result<String, Report> {
    let label = page
        .element(css(format!("#test-df-{section} span")).text(text))
        .await?;
    Ok(label.id().await?.unwrap_or_default())
}

/// The ids of an id reference list (`aria-labelledby`, ...).
fn ids(list: &str) -> Vec<&str> {
    list.split_whitespace().collect()
}

/// The field is a group named by its label whose segments are spin buttons named by kind and
/// field, showing placeholders; literals are hidden, only the first segment has the description,
/// and a hidden input carries the field's name ("provides slots").
#[browser_test]
pub async fn structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    let group = page.element("#test-df-basic [role=group]").await?;
    let label_id = label_id(page, "basic", "Birthday").await?;
    assert_that!(group)
        .accessible_name()
        .await
        .is_equal_to("Birthday");

    let month = segment(page, "basic", "month").await?;
    assert_that!(month)
        .has_attribute("role")
        .await
        .is_equal_to("spinbutton");
    assert_that!(month)
        .has_attribute("aria-label")
        .await
        .is_equal_to("month, ");
    // Named by its `aria-label` (the self-reference) and the field's label.
    let month_id = month.id().await?.unwrap_or_default();
    assert_that!(month)
        .has_attribute("aria-labelledby")
        .await
        .derive_owned(|labelledby| ids(labelledby))
        .contains_exactly([month_id.as_str(), label_id.as_str()]);
    assert_that!(month).inner_text().await.is_equal_to("mm");
    assert_that!(month)
        .has_attribute("data-placeholder")
        .await
        .is_equal_to("true");
    assert_that!(month)
        .has_attribute("contenteditable")
        .await
        .is_equal_to("true");
    assert_that!(month)
        .has_attribute("inputmode")
        .await
        .is_equal_to("numeric");
    assert_that!(segment(page, "basic", "day").await?)
        .inner_text()
        .await
        .is_equal_to("dd");
    assert_that!(segment(page, "basic", "year").await?)
        .inner_text()
        .await
        .is_equal_to("yyyy");
    for literal in page.elements("#test-df-basic [data-type=literal]").await? {
        assert_that!(literal)
            .has_attribute("aria-hidden")
            .await
            .is_equal_to("true");
    }

    // The description: on the first segment, not on the others.
    assert_that!(month)
        .accessible_description()
        .await
        .is_equal_to("Your birthday");
    let day = segment(page, "basic", "day").await?;
    assert_that!(day)
        .attribute("aria-describedby")
        .await
        .is_none();

    let input = page
        .element("#test-df-basic input[type=text][name=birthday]")
        .await?;
    assert_that!(input)
        .property("value")
        .await
        .some()
        .is_empty();
    Ok(())
}

/// On iOS the segments are text boxes without spin button values (VoiceOver can't focus spin
/// buttons there), also when the server rendered them as spin buttons (useDateSegment: "Spin
/// buttons can't be focused with VoiceOver on iOS").
#[browser_test]
pub async fn segments_are_textboxes_on_ios(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::IPhone).await?;
    page.goto_sections(PATH, &["basic"]).await?;
    let month = segment(page, "basic", "month").await?;
    month.wait_for_attr("role", Some("textbox")).await?;
    assert_that!(month)
        .attribute("aria-valuenow")
        .await
        .is_none();
    assert_that!(month)
        .attribute("aria-valuetext")
        .await
        .is_none();
    assert_that!(month)
        .attribute("aria-valuemin")
        .await
        .is_none();
    assert_that!(month)
        .attribute("aria-valuemax")
        .await
        .is_none();
    Ok(())
}

/// Typing fills a segment and moves on once no further digit fits; the date then goes to the
/// hidden input and the group's description ("Selected Date: June 15, 2024").
#[browser_test]
pub async fn typing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    page.element("#test-df-basic-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&segment(page, "basic", "month").await?)
        .await?;
    page.type_text("6").await?;
    page.wait_for_focus(&segment(page, "basic", "day").await?)
        .await?;
    page.type_text("1").await?;
    // 1 may still become 10 to 19.
    page.wait_for_focus(&segment(page, "basic", "day").await?)
        .await?;
    page.type_text("5").await?;
    page.wait_for_focus(&segment(page, "basic", "year").await?)
        .await?;
    page.type_text("2024").await?;
    // The last segment keeps the focus (no era shows for the years before 1000 typed on the
    // way: it would take the focus, then go away with it).
    page.wait_for_focus(&segment(page, "basic", "year").await?)
        .await?;
    wait_for_value(page, "basic", "2024-06-15").await?;
    let input = page
        .element("#test-df-basic input[type=text][name=birthday]")
        .await?;
    assert_that!(input)
        .property("value")
        .await
        .some()
        .is_equal_to("2024-06-15");

    // The group describes the value.
    let group = page.element("#test-df-basic [role=group]").await?;
    assert_that!(group)
        .accessible_description()
        .await
        .contains("Selected Date: June 15, 2024");
    Ok(())
}

/// Arrows step a segment and move between them; Backspace deletes a digit, and on an empty
/// segment moves to the previous one ("should focus previous segment when backspacing on an empty
/// date segment").
#[browser_test]
pub async fn arrows_and_backspace(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    type_basic_date(page).await?;
    page.send_keys(Key::Up).await?;
    wait_for_value(page, "basic", "2025-06-15").await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&segment(page, "basic", "day").await?)
        .await?;
    page.send_keys(Key::Down).await?;
    wait_for_value(page, "basic", "2025-06-14").await?;
    page.send_keys(Key::Backspace).await?;
    segment(page, "basic", "day")
        .await?
        .wait_for_inner_text("1")
        .await?;
    wait_for_value(page, "basic", "2025-06-01").await?;
    page.send_keys(Key::Backspace).await?;
    segment(page, "basic", "day")
        .await?
        .wait_for_inner_text("dd")
        .await?;
    // Incomplete: the value stays until the field has one again.
    value(page, "basic")
        .await?
        .inner_text_stays("2025-06-01", std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Backspace).await?;
    page.wait_for_focus(&segment(page, "basic", "month").await?)
        .await?;
    page.send_keys(Key::Right).await?;
    page.type_text("5").await?;
    wait_for_value(page, "basic", "2025-06-05").await?;
    Ok(())
}

/// February 30 stays as typed while the field is edited, and is committed as February 29 (of
/// the leap year 2024) when the field is left.
#[browser_test]
pub async fn invalid_date_committed_when_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    type_basic_date(page).await?;
    segment(page, "basic", "month").await?.click().await?;
    page.wait_for_focus(&segment(page, "basic", "month").await?)
        .await?;
    page.type_text("2").await?;
    wait_for_value(page, "basic", "2024-02-15").await?;
    page.wait_for_focus(&segment(page, "basic", "day").await?)
        .await?;
    page.type_text("30").await?;
    segment(page, "basic", "day")
        .await?
        .wait_for_inner_text("30")
        .await?;
    // "3" made a valid date (February 3), "30" does not.
    value(page, "basic")
        .await?
        .inner_text_stays("2024-02-03", std::time::Duration::from_millis(100))
        .await?;
    page.element("#test-df-basic-before").await?.click().await?;
    wait_for_value(page, "basic", "2024-02-29").await?;
    segment(page, "basic", "day")
        .await?
        .wait_for_inner_text("29")
        .await?;
    Ok(())
}

/// Resetting the form restores the initial value.
#[browser_test]
pub async fn form_reset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    type_basic_date(page).await?;
    page.element("#test-df-reset").await?.click().await?;
    wait_for_value(page, "basic", "none").await?;
    segment(page, "basic", "month")
        .await?
        .wait_for_inner_text("mm")
        .await?;
    Ok(())
}

/// A value before the minimum is invalid with its message right away, and valid again once
/// ArrowUp moves it past the minimum.
#[browser_test]
pub async fn min_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["min"]).await?;
    let month = segment(page, "min", "month").await?;
    month.wait_for_attr("aria-invalid", Some("true")).await?;
    let section = page.element("#test-df-min").await?;
    assert_that!(section)
        .inner_text()
        .await
        .contains("Value must be 5/1/2024 or later.");
    month.click().await?;
    page.send_keys(Key::Up).await?;
    wait_for_value(page, "min", "2024-05-30").await?;
    month.wait_for_attr("aria-invalid", None).await?;
    assert_that!(|| section.inner_text())
        .eventually_ok()
        .satisfies(|text| {
            text.does_not_contain("or later");
        })
        .await;
    Ok(())
}

/// A disabled field's segments are disabled, neither tabbable nor editable; a read-only field's
/// segments don't change on ArrowUp ("should support disabled state", "should support readonly
/// state").
#[browser_test]
pub async fn disabled_and_read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["disabled", "read-only"]).await?;
    let disabled = segment(page, "disabled", "month").await?;
    assert_that!(disabled).attribute("tabindex").await.is_none();
    assert_that!(disabled)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    assert_that!(disabled)
        .attribute("contenteditable")
        .await
        .is_none();
    let group = page.element("#test-df-disabled [role=group]").await?;
    assert_that!(group)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");

    let read_only = segment(page, "read-only", "month").await?;
    assert_that!(read_only)
        .has_attribute("aria-readonly")
        .await
        .is_equal_to("true");
    read_only.click().await?;
    page.send_keys(Key::Up).await?;
    read_only
        .inner_text_stays("6", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// In a field with a 24-hour time, typing moves on from the date to the hour and minute and fills
/// the whole date and time.
#[browser_test]
pub async fn date_and_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["date-time"]).await?;
    segment(page, "date-time", "month").await?.click().await?;
    page.type_text("6152024").await?;
    page.wait_for_focus(&segment(page, "date-time", "hour").await?)
        .await?;
    page.type_text("14").await?;
    page.wait_for_focus(&segment(page, "date-time", "minute").await?)
        .await?;
    page.type_text("30").await?;
    wait_for_value(page, "date-time", "2024-06-15T14:30:00").await?;
    Ok(())
}

/// A zoned value shows its time zone (a read-only textbox); stepping the hour keeps the zone, also
/// in the form value, ISO 8601 with the zone ("supports form values").
#[browser_test]
pub async fn zoned(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["zoned"]).await?;
    let input = page.element("#test-df-zoned input[type=text]").await?;
    assert_that!(input)
        .property("value")
        .await
        .some()
        .is_equal_to("2024-06-05T09:30:00-04:00[America/New_York]");
    let zone = segment(page, "zoned", "timeZoneName").await?;
    assert_that!(zone).inner_text().await.is_equal_to("EDT");
    assert_that!(zone)
        .has_attribute("role")
        .await
        .is_equal_to("textbox");
    assert_that!(segment(page, "zoned", "dayPeriod").await?)
        .inner_text()
        .await
        .is_equal_to("AM");
    segment(page, "zoned", "hour").await?.click().await?;
    page.send_keys(Key::Up).await?;
    wait_for_value(page, "zoned", "2024-06-05T10:30:00-04:00[America/New_York]").await?;
    input
        .wait_for_prop("value", "2024-06-05T10:30:00-04:00[America/New_York]")
        .await?;
    assert_that!(zone).inner_text().await.is_equal_to("EDT");
    Ok(())
}

/// In a 12-hour time field, the hour moves on after a digit that can't start a two-digit hour, "p"
/// picks PM, and the form gets the 24-hour time ("should support form value").
#[browser_test]
pub async fn time_field(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["time"]).await?;
    assert_that!(segment(page, "time", "hour").await?)
        .inner_text()
        .await
        .is_equal_to("––");
    segment(page, "time", "hour").await?.click().await?;
    page.type_text("9").await?;
    page.wait_for_focus(&segment(page, "time", "minute").await?)
        .await?;
    page.type_text("30").await?;
    page.wait_for_focus(&segment(page, "time", "dayPeriod").await?)
        .await?;
    page.type_text("p").await?;
    wait_for_value(page, "time", "21:30:00").await?;
    // The form gets the time ("should support form value").
    let input = page.element("#test-df-time input[name=alarm]").await?;
    assert_that!(input)
        .property("value")
        .await
        .some()
        .is_equal_to("21:30:00");
    Ok(())
}

/// The date picker's calendar button (`#test-df-picker`).
async fn picker_button(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-df-picker button[aria-haspopup=dialog]")
        .await
}

/// The date button of the open picker's calendar whose label starts with `label`.
async fn calendar_date(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(format!(
        "[role=dialog] [role=button][aria-label^='{label}']"
    ))
    .await
}

/// Opens the date picker by its button and selects June 15, 2024 in its calendar.
async fn pick_june_15(page: &Page<'_>) -> Result<(), Report> {
    picker_button(page).await?.click().await?;
    calendar_date(page, "Saturday, June 15, 2024")
        .await?
        .click()
        .await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    wait_for_value(page, "picker", "2024-06-15").await?;
    Ok(())
}

/// A date picker is a group named by its label, holding the field (without a role of its own, its
/// segments labelled by the picker's label) and a collapsed "Calendar" button opening a dialog.
#[browser_test]
pub async fn date_picker_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["picker"]).await?;
    let group = page.element("#test-df-picker [role=group]").await?;
    assert_that!(group)
        .accessible_name()
        .await
        .is_equal_to("Event");
    page.element("#test-df-picker [role=presentation]").await?;
    let label_id = label_id(page, "picker", "Event").await?;
    let month = segment(page, "picker", "month").await?;
    let month_id = month.id().await?.unwrap_or_default();
    assert_that!(month)
        .has_attribute("aria-labelledby")
        .await
        .derive_owned(|labelledby| ids(labelledby))
        .contains_exactly([month_id.as_str(), label_id.as_str()]);
    let button = picker_button(page).await?;
    assert_that!(button)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Calendar");
    assert_that!(button)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("false");
    Ok(())
}

/// Opened by its button, the calendar shows the placeholder's month with its date focused;
/// selecting a date closes it and fills the field ("should support close on select = true").
#[browser_test]
pub async fn date_picker_selects_in_its_calendar(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["picker"]).await?;
    let button = picker_button(page).await?;
    button.click().await?;
    page.element("[role=dialog] [role=grid]").await?;
    button.wait_for_attr("aria-expanded", Some("true")).await?;
    let june1 = calendar_date(page, "Saturday, June 1, 2024").await?;
    page.wait_for_focus(&june1).await?;
    calendar_date(page, "Saturday, June 15, 2024")
        .await?
        .click()
        .await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    button.wait_for_attr("aria-expanded", Some("false")).await?;
    wait_for_value(page, "picker", "2024-06-15").await?;
    segment(page, "picker", "day")
        .await?
        .wait_for_inner_text("15")
        .await?;
    Ok(())
}

/// Alt+ArrowDown in the field opens the calendar with the value's date focused; ArrowRight and
/// Enter then select the next day and close it.
#[browser_test]
pub async fn date_picker_opens_by_alt_arrow_down(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["picker"]).await?;
    pick_june_15(page).await?;
    segment(page, "picker", "month").await?.click().await?;
    page.send_keys(Key::Alt + Key::Down).await?;
    page.element("[role=dialog] [role=grid]").await?;
    let june15 = calendar_date(page, "Saturday, June 15, 2024").await?;
    page.wait_for_focus(&june15).await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    wait_for_value(page, "picker", "2024-06-16").await?;
    Ok(())
}

/// Escape closes the calendar without changing the value, even after focus moved to another
/// date.
#[browser_test]
pub async fn date_picker_escape_keeps_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["picker"]).await?;
    pick_june_15(page).await?;
    picker_button(page).await?.click().await?;
    let june15 = calendar_date(page, "Saturday, June 15, 2024").await?;
    page.wait_for_focus(&june15).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&calendar_date(page, "Sunday, June 16, 2024").await?)
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    value(page, "picker")
        .await?
        .inner_text_stays("2024-06-15", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Editing a segment of the picker's field changes the value.
#[browser_test]
pub async fn date_picker_field_edits_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["picker"]).await?;
    pick_june_15(page).await?;
    segment(page, "picker", "day").await?.click().await?;
    page.send_keys(Key::Up).await?;
    wait_for_value(page, "picker", "2024-06-16").await?;
    Ok(())
}

/// A partial edit (month 7) replaced by a date selected in the calendar doesn't come back when
/// the value is cleared: the month shows its placeholder again.
#[browser_test]
pub async fn date_picker_edit_does_not_come_back(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["picker"]).await?;
    let month = segment(page, "picker", "month").await?;
    month.wait_for_inner_text("mm").await?;
    month.click().await?;
    page.type_text("7").await?;
    month.wait_for_inner_text("7").await?;
    picker_button(page).await?.click().await?;
    calendar_date(page, "Wednesday, June 12, 2024")
        .await?
        .click()
        .await?;
    wait_for_value(page, "picker", "2024-06-12").await?;
    page.element("#test-df-picker-clear").await?.click().await?;
    wait_for_value(page, "picker", "none").await?;
    segment(page, "picker", "month")
        .await?
        .wait_for_inner_text("mm")
        .await?;
    Ok(())
}

/// The segments of one field of the range picker (`0`: start, `1`: end).
async fn range_segment(page: &Page<'_>, field: usize, kind: &str) -> Result<WebElement, Report> {
    let mut segments = page
        .elements(format!("#test-df-range [data-type='{kind}']"))
        .await?;
    Ok(segments.swap_remove(field))
}

/// A range selected in the calendar fills the start and end fields (named Start Date and End
/// Date), and arrows move across both fields on to the button. An end before the start shows an
/// error once the picker is left.
#[browser_test]
pub async fn date_range_picker(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range"]).await?;
    let start_month = range_segment(page, 0, "month").await?;
    assert_that!(start_month)
        .has_attribute("aria-label")
        .await
        .starts_with("month, Start Date");
    let end_month = range_segment(page, 1, "month").await?;
    assert_that!(end_month)
        .has_attribute("aria-label")
        .await
        .starts_with("month, End Date");

    page.element("#test-df-range button[aria-haspopup=dialog]")
        .await?
        .click()
        .await?;
    page.element("[role=dialog] [role=grid]").await?;
    page.element("[role=dialog] [role=button][aria-label*='Monday, June 10, 2024']")
        .await?
        .click()
        .await?;
    page.element("[role=dialog] [role=button][aria-label*='Friday, June 14, 2024']")
        .await?
        .click()
        .await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    wait_for_value(page, "range", "2024-06-10 - 2024-06-14").await?;
    assert_that!(range_segment(page, 1, "day").await?)
        .inner_text()
        .await
        .is_equal_to("14");

    // From the start's year to the end's month.
    let start_year = range_segment(page, 0, "year").await?;
    start_year.click().await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&end_month).await?;
    // Arrows go on to the button (react-aria's `useDatePickerGroup`); the segments' own moving on
    // (a full year typed) stays in the fields.
    let end_year = range_segment(page, 1, "year").await?;
    end_year.click().await?;
    page.send_keys(Key::Right).await?;
    let button = page
        .element("#test-df-range button[aria-haspopup=dialog]")
        .await?;
    page.wait_for_focus(&button).await?;
    end_year.click().await?;
    page.type_text("2024").await?;
    page.wait_for_focus(&end_year).await?;

    // An end before the start is invalid, shown when the picker is left (native validation).
    end_month.click().await?;
    page.send_keys(Key::Down).await?;
    wait_for_value(page, "range", "2024-06-10 - 2024-05-14").await?;
    page.element("#test-df-range-after").await?.click().await?;
    let section = page.element("#test-df-range").await?;
    assert_that!(|| section.inner_text())
        .eventually_ok()
        .satisfies(|text| {
            text.contains("Start date must be before end date.");
        })
        .await;
    Ok(())
}

/// The field and the picker's group show `data-focus-within` while a segment has focus and
/// `data-focus-visible` only with keyboard focus; the hovered group shows `data-hovered` ("should
/// support focus visible state").
#[browser_test]
pub async fn group_states(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic", "picker"]).await?;
    let input = page
        .element("#test-df-basic :has(> [data-type=month])")
        .await?;
    assert_that!(input)
        .attribute("data-focus-within")
        .await
        .is_none();
    page.element("#test-df-basic-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&segment(page, "basic", "month").await?)
        .await?;
    input
        .wait_for_attr("data-focus-within", Some("true"))
        .await?;
    input
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;

    let group = page.element("#test-df-picker [role=group]").await?;
    segment(page, "picker", "day").await?.click().await?;
    group
        .wait_for_attr("data-focus-within", Some("true"))
        .await?;
    group
        .attr_stays(
            "data-focus-visible",
            None,
            std::time::Duration::from_millis(100),
        )
        .await?;
    group.wait_for_attr("data-hovered", Some("true")).await?;
    input.wait_for_attr("data-focus-within", None).await?;
    Ok(())
}

// Editing from values set by the cases (react-spectrum's `DatePicker.test.js`, "editing", on
// date fields: the picker's segments are a date field's).

/// Sets the field of `section` to `value` (ISO 8601, as the fixture shows it) through the
/// fixture's `#test-df-<section>-set`, and waits until it has it.
async fn set(page: &Page<'_>, section: &str, value: &str) -> Result<(), Report> {
    // Each editing case starts a new focus session, like upstream's fresh render. Setting the
    // controlled value alone intentionally leaves an active segment's typed-digit buffer intact.
    page.blur_focused().await?;
    page.element(format!("#test-df-{section}-set"))
        .await?
        .virtual_input(value)
        .await?;
    wait_for_value(page, section, value).await
}

/// From `start`, `up` in the `kind` segment of `section` changes the value to `incremented`, and,
/// from `start` again, `down` to `decremented` (react-spectrum's `testArrows`).
async fn arrows(
    page: &Page<'_>,
    section: &str,
    kind: &str,
    start: &str,
    (up, incremented): (Key, &str),
    (down, decremented): (Key, &str),
) -> Result<(), Report> {
    set(page, section, start).await?;
    let segment = segment(page, section, kind).await?;
    segment.focus().await?;
    page.wait_for_focus(&segment).await?;
    page.send_keys(up).await?;
    wait_for_value(page, section, incremented).await?;
    set(page, section, start).await?;
    segment.focus().await?;
    page.send_keys(down).await?;
    wait_for_value(page, section, decremented).await?;
    Ok(())
}

/// From `start`, typing `keys` into the `kind` segment of `section` changes the value to
/// `expected`; the focus then is on the `focus` segment (react-spectrum's `testInput`).
async fn typed(
    page: &Page<'_>,
    section: &str,
    kind: &str,
    start: &str,
    keys: &str,
    expected: &str,
    focus: &str,
) -> Result<(), Report> {
    set(page, section, start).await?;
    let segment = segment(page, section, kind).await?;
    segment.focus().await?;
    page.wait_for_focus(&segment).await?;
    page.type_text(keys).await?;
    wait_for_value(page, section, expected).await?;
    page.wait_for_focus(&self::segment(page, section, focus).await?)
        .await?;
    Ok(())
}

/// From `start`, Backspace in the `kind` segment of `section` changes the value to `expected`
/// (`None`: the segment is empty and the value stays; react-spectrum's `testBackspace`).
async fn backspace(
    page: &Page<'_>,
    section: &str,
    kind: &str,
    start: &str,
    expected: Option<&str>,
) -> Result<(), Report> {
    set(page, section, start).await?;
    let segment = segment(page, section, kind).await?;
    segment.focus().await?;
    page.wait_for_focus(&segment).await?;
    page.send_keys(Key::Backspace).await?;
    if let Some(expected) = expected {
        wait_for_value(page, section, expected).await?;
    } else {
        segment
            .wait_for_attr("aria-valuetext", Some("Empty"))
            .await?;
        value(page, section)
            .await?
            .inner_text_stays(start, std::time::Duration::from_millis(100))
            .await?;
    }
    Ok(())
}

/// ArrowUp and ArrowDown step the month ("should support using the arrow keys to increment and
/// decrement the month").
#[browser_test]
pub async fn month_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    arrows(
        page,
        "edit-date",
        "month",
        "2019-02-03",
        (Key::Up, "2019-03-03"),
        (Key::Down, "2019-01-03"),
    )
    .await
}

/// The month wraps from December to January and back ("should wrap around when incrementing and
/// decrementing the month").
#[browser_test]
pub async fn month_wraps(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    arrows(
        page,
        "edit-date",
        "month",
        "2019-12-03",
        (Key::Up, "2019-01-03"),
        (Key::Down, "2019-11-03"),
    )
    .await?;
    arrows(
        page,
        "edit-date",
        "month",
        "2019-01-03",
        (Key::Up, "2019-02-03"),
        (Key::Down, "2019-12-03"),
    )
    .await
}

/// PageUp and PageDown step the month by 2, wrapping ("should support using the page up and down
/// keys to increment and decrement the month by 2").
#[browser_test]
pub async fn month_page_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    arrows(
        page,
        "edit-date",
        "month",
        "2019-01-03",
        (Key::PageUp, "2019-03-03"),
        (Key::PageDown, "2019-11-03"),
    )
    .await?;
    arrows(
        page,
        "edit-date",
        "month",
        "2019-02-03",
        (Key::PageUp, "2019-04-03"),
        (Key::PageDown, "2019-12-03"),
    )
    .await
}

/// End and Home jump to December and January ("should support using the home and end keys to
/// jump to the min and max month").
#[browser_test]
pub async fn month_home_end(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    arrows(
        page,
        "edit-date",
        "month",
        "2019-06-03",
        (Key::End, "2019-12-03"),
        (Key::Home, "2019-01-03"),
    )
    .await
}

/// ArrowUp and ArrowDown step the day ("should support using the arrow keys to increment and
/// decrement the day").
#[browser_test]
pub async fn day_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    arrows(
        page,
        "edit-date",
        "day",
        "2019-02-03",
        (Key::Up, "2019-02-04"),
        (Key::Down, "2019-02-02"),
    )
    .await
}

/// The day wraps within its month ("should wrap around when incrementing and decrementing the
/// day").
#[browser_test]
pub async fn day_wraps(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    arrows(
        page,
        "edit-date",
        "day",
        "2019-08-31",
        (Key::Up, "2019-08-01"),
        (Key::Down, "2019-08-30"),
    )
    .await?;
    arrows(
        page,
        "edit-date",
        "day",
        "2019-08-01",
        (Key::Up, "2019-08-02"),
        (Key::Down, "2019-08-31"),
    )
    .await
}

/// PageUp and PageDown step the day by 7, wrapping within the month ("should support using the
/// page up and down keys to increment and decrement the day by 7").
#[browser_test]
pub async fn day_page_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    arrows(
        page,
        "edit-date",
        "day",
        "2019-02-03",
        (Key::PageUp, "2019-02-10"),
        (Key::PageDown, "2019-02-27"),
    )
    .await
}

/// End and Home jump to the month's last and first day ("should support using the home and end
/// keys to jump to the min and max day").
#[browser_test]
pub async fn day_home_end(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    arrows(
        page,
        "edit-date",
        "day",
        "2019-08-05",
        (Key::End, "2019-08-31"),
        (Key::Home, "2019-08-01"),
    )
    .await
}

/// ArrowUp and ArrowDown step the year ("should support using the arrow keys to increment and
/// decrement the year").
#[browser_test]
pub async fn year_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    arrows(
        page,
        "edit-date",
        "year",
        "2019-02-03",
        (Key::Up, "2020-02-03"),
        (Key::Down, "2018-02-03"),
    )
    .await
}

/// PageUp and PageDown move the year to the next or previous multiple of 5 ("should support
/// using the page up and down keys to increment and decrement the year to the nearest 5").
#[browser_test]
pub async fn year_page_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    arrows(
        page,
        "edit-date",
        "year",
        "2019-02-03",
        (Key::PageUp, "2020-02-03"),
        (Key::PageDown, "2015-02-03"),
    )
    .await
}

/// ArrowUp and ArrowDown step the hour ("should support using the arrow keys to increment and
/// decrement the hour").
#[browser_test]
pub async fn hour_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-12h"]).await?;
    arrows(
        page,
        "edit-12h",
        "hour",
        "2019-02-03T08:00:00",
        (Key::Up, "2019-02-03T09:00:00"),
        (Key::Down, "2019-02-03T07:00:00"),
    )
    .await
}

/// In 12-hour time the hour wraps within its half of the day: 11\u{202f}AM to 12\u{202f}AM, 11\u{202f}PM to 12\u{202f}PM
/// ("should wrap around when incrementing and decrementing the hour in 12 hour time").
#[browser_test]
pub async fn hour_wraps_in_12_hour_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-12h"]).await?;
    for (start, incremented, decremented) in [
        (
            "2019-02-03T11:00:00",
            "2019-02-03T00:00:00",
            "2019-02-03T10:00:00",
        ),
        (
            "2019-02-03T00:00:00",
            "2019-02-03T01:00:00",
            "2019-02-03T11:00:00",
        ),
        (
            "2019-02-03T23:00:00",
            "2019-02-03T12:00:00",
            "2019-02-03T22:00:00",
        ),
        (
            "2019-02-03T12:00:00",
            "2019-02-03T13:00:00",
            "2019-02-03T23:00:00",
        ),
    ] {
        arrows(
            page,
            "edit-12h",
            "hour",
            start,
            (Key::Up, incremented),
            (Key::Down, decremented),
        )
        .await?;
    }
    Ok(())
}

/// In 24-hour time the hour wraps from 23 to 0 ("should wrap around when incrementing and
/// decrementing the hour in 24 hour time").
#[browser_test]
pub async fn hour_wraps_in_24_hour_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-24h"]).await?;
    arrows(
        page,
        "edit-24h",
        "hour",
        "2019-02-03T23:00:00",
        (Key::Up, "2019-02-03T00:00:00"),
        (Key::Down, "2019-02-03T22:00:00"),
    )
    .await?;
    arrows(
        page,
        "edit-24h",
        "hour",
        "2019-02-03T00:00:00",
        (Key::Up, "2019-02-03T01:00:00"),
        (Key::Down, "2019-02-03T23:00:00"),
    )
    .await
}

/// PageUp and PageDown step the hour by 2 ("should support using the page up and down keys to
/// increment and decrement the hour by 2").
#[browser_test]
pub async fn hour_page_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-12h"]).await?;
    arrows(
        page,
        "edit-12h",
        "hour",
        "2019-02-03T08:00:00",
        (Key::PageUp, "2019-02-03T10:00:00"),
        (Key::PageDown, "2019-02-03T06:00:00"),
    )
    .await
}

/// In 12-hour time End and Home jump to 11 and 12 of the value's half of the day ("should support
/// using the home and end keys to jump to the min and max hour in 12 hour time").
#[browser_test]
pub async fn hour_home_end_in_12_hour_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-12h"]).await?;
    arrows(
        page,
        "edit-12h",
        "hour",
        "2019-02-03T08:00:00",
        (Key::End, "2019-02-03T11:00:00"),
        (Key::Home, "2019-02-03T00:00:00"),
    )
    .await?;
    arrows(
        page,
        "edit-12h",
        "hour",
        "2019-02-03T16:00:00",
        (Key::End, "2019-02-03T23:00:00"),
        (Key::Home, "2019-02-03T12:00:00"),
    )
    .await
}

/// In 24-hour time End and Home jump to 23 and 0 ("should support using the home and end keys to
/// jump to the min and max hour in 24 hour time").
#[browser_test]
pub async fn hour_home_end_in_24_hour_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-24h"]).await?;
    arrows(
        page,
        "edit-24h",
        "hour",
        "2019-02-03T08:00:00",
        (Key::End, "2019-02-03T23:00:00"),
        (Key::Home, "2019-02-03T00:00:00"),
    )
    .await
}

/// The minute steps, wraps, moves to the next or previous quarter hour with PageUp/PageDown, and
/// jumps to 59 and 0 with End/Home ("should support using the arrow keys to increment and
/// decrement the minute", "should wrap around when incrementing and decrementing the minute",
/// "... the page up and down keys ... to the nearest 15", "... the home and end keys to jump to
/// the min and max minute").
#[browser_test]
pub async fn minute_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-24h"]).await?;
    for (start, up, down) in [
        (
            "2019-02-03T08:05:00",
            (Key::Up, "2019-02-03T08:06:00"),
            (Key::Down, "2019-02-03T08:04:00"),
        ),
        (
            "2019-02-03T08:59:00",
            (Key::Up, "2019-02-03T08:00:00"),
            (Key::Down, "2019-02-03T08:58:00"),
        ),
        (
            "2019-02-03T08:00:00",
            (Key::Up, "2019-02-03T08:01:00"),
            (Key::Down, "2019-02-03T08:59:00"),
        ),
        (
            "2019-02-03T08:22:00",
            (Key::PageUp, "2019-02-03T08:30:00"),
            (Key::PageDown, "2019-02-03T08:15:00"),
        ),
        (
            "2019-02-03T08:22:00",
            (Key::End, "2019-02-03T08:59:00"),
            (Key::Home, "2019-02-03T08:00:00"),
        ),
    ] {
        arrows(page, "edit-24h", "minute", start, up, down).await?;
    }
    Ok(())
}

/// The second steps, wraps, moves to the next or previous quarter minute with PageUp/PageDown,
/// and jumps to 59 and 0 with End/Home ("should support using the arrow keys to increment and
/// decrement the second", "should wrap around when incrementing and decrementing the second", "...
/// the page up and down keys ... to the nearest 15", "... the home and end keys to jump to the
/// min and max second").
#[browser_test]
pub async fn second_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-24h"]).await?;
    for (start, up, down) in [
        (
            "2019-02-03T08:05:10",
            (Key::Up, "2019-02-03T08:05:11"),
            (Key::Down, "2019-02-03T08:05:09"),
        ),
        (
            "2019-02-03T08:05:59",
            (Key::Up, "2019-02-03T08:05:00"),
            (Key::Down, "2019-02-03T08:05:58"),
        ),
        (
            "2019-02-03T08:05:00",
            (Key::Up, "2019-02-03T08:05:01"),
            (Key::Down, "2019-02-03T08:05:59"),
        ),
        (
            "2019-02-03T08:05:22",
            (Key::PageUp, "2019-02-03T08:05:30"),
            (Key::PageDown, "2019-02-03T08:05:15"),
        ),
        (
            "2019-02-03T08:05:22",
            (Key::End, "2019-02-03T08:05:59"),
            (Key::Home, "2019-02-03T08:05:00"),
        ),
    ] {
        arrows(page, "edit-24h", "second", start, up, down).await?;
    }
    Ok(())
}

/// ArrowUp and ArrowDown toggle AM and PM ("should support using the arrow keys to increment and
/// decrement the day period").
#[browser_test]
pub async fn day_period_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-12h"]).await?;
    arrows(
        page,
        "edit-12h",
        "dayPeriod",
        "2019-02-03T08:00:00",
        (Key::Up, "2019-02-03T20:00:00"),
        (Key::Down, "2019-02-03T20:00:00"),
    )
    .await?;
    arrows(
        page,
        "edit-12h",
        "dayPeriod",
        "2019-02-03T20:00:00",
        (Key::Up, "2019-02-03T08:00:00"),
        (Key::Down, "2019-02-03T08:00:00"),
    )
    .await
}

/// An era segment shows for a year before Christ and goes when the era is stepped back to AD,
/// handing the focus to the segment before it ("should show and hide the era field as needed",
/// "should focus the previous segment when the era is removed").
#[browser_test]
pub async fn era_shows_and_hides(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    page.wait_for_count("#test-df-edit-date [data-type=era]", 0)
        .await?;
    let year = segment(page, "edit-date", "year").await?;
    year.focus().await?;
    page.wait_for_focus(&year).await?;
    page.type_text("1").await?;
    wait_for_value(page, "edit-date", "0001-02-03").await?;
    page.send_keys(Key::Down).await?;
    wait_for_value(page, "edit-date", "0000-02-03").await?;
    let era = segment(page, "edit-date", "era").await?;
    era.wait_for_inner_text("BC").await?;
    era.focus().await?;
    page.wait_for_focus(&era).await?;
    page.send_keys(Key::Down).await?;
    wait_for_value(page, "edit-date", "0001-02-03").await?;
    page.wait_for_count("#test-df-edit-date [data-type=era]", 0)
        .await?;
    page.wait_for_focus(&segment(page, "edit-date", "year").await?)
        .await?;
    Ok(())
}

/// A date before Christ shows its era, also in the field's description ("should include era for
/// BC dates").
#[browser_test]
pub async fn era_of_dates_before_christ(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    set(page, "edit-date", "-000004-02-03").await?;
    segment(page, "edit-date", "era")
        .await?
        .wait_for_inner_text("BC")
        .await?;
    let group = page.element("#test-df-edit-date [role=group]").await?;
    assert_that!(|| group.accessible_description())
        .eventually_ok()
        .matches(assertr::matchers::eq(
            "Selected Date: February 3, 5 BC Help text".to_owned(),
        ))
        .await;
    Ok(())
}

/// Arrows may make an invalid date (February 30): it shows, the value stays, and leaving the
/// field constrains it ("should allow entering invalid dates, and constrain on blur", arrow
/// keys).
#[browser_test]
pub async fn arrows_constrain_an_invalid_date_on_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    set(page, "edit-date", "2026-02-28").await?;
    let day = segment(page, "edit-date", "day").await?;
    day.focus().await?;
    page.wait_for_focus(&day).await?;
    page.send_keys(Key::Up).await?;
    page.send_keys(Key::Up).await?;
    day.wait_for_inner_text("30").await?;
    value(page, "edit-date")
        .await?
        .inner_text_stays("2026-02-28", std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    day.wait_for_inner_text("28").await?;
    assert_that!(value(page, "edit-date").await?)
        .inner_text()
        .await
        .is_equal_to("2026-02-28");
    Ok(())
}

/// Typing into the month: a digit that may start a two-digit month waits, others move on; "0"
/// alone changes nothing until the field is left ("should support typing into the month
/// segment").
#[browser_test]
pub async fn typing_into_the_month(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    let start = "2019-02-03";
    typed(
        page,
        "edit-date",
        "month",
        start,
        "1",
        "2019-01-03",
        "month",
    )
    .await?;
    typed(page, "edit-date", "month", start, "01", "2019-01-03", "day").await?;
    typed(page, "edit-date", "month", start, "12", "2019-12-03", "day").await?;
    typed(page, "edit-date", "month", start, "4", "2019-04-03", "day").await?;
    typed_zero(page, "edit-date", "month", start, "0", "1").await
}

/// From `start`, typing `keys` of zeros into the `kind` segment of `section` shows "0" and
/// changes nothing; leaving the field shows `after` (react-spectrum's `testIgnored`).
async fn typed_zero(
    page: &Page<'_>,
    section: &str,
    kind: &str,
    start: &str,
    keys: &str,
    after: &str,
) -> Result<(), Report> {
    set(page, section, start).await?;
    let segment = segment(page, section, kind).await?;
    segment.focus().await?;
    page.wait_for_focus(&segment).await?;
    page.type_text(keys).await?;
    segment.wait_for_inner_text("0").await?;
    value(page, section)
        .await?
        .inner_text_stays(start, std::time::Duration::from_millis(100))
        .await?;
    page.blur_focused().await?;
    segment.wait_for_inner_text(after).await?;
    Ok(())
}

/// Typing into the day, as into the month ("should support typing into the day segment").
#[browser_test]
pub async fn typing_into_the_day(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    let start = "2019-02-03";
    typed(page, "edit-date", "day", start, "1", "2019-02-01", "day").await?;
    typed(page, "edit-date", "day", start, "01", "2019-02-01", "year").await?;
    typed(page, "edit-date", "day", start, "12", "2019-02-12", "year").await?;
    typed(page, "edit-date", "day", start, "4", "2019-02-04", "year").await?;
    typed_zero(page, "edit-date", "day", start, "00", "1").await
}

/// Four digits fill the year; the focus moves on to a following time, else stays ("should
/// support typing into the year segment").
#[browser_test]
pub async fn typing_into_the_year(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date", "edit-12h"]).await?;
    typed(
        page,
        "edit-date",
        "year",
        "2019-02-03",
        "1993",
        "1993-02-03",
        "year",
    )
    .await?;
    typed(
        page,
        "edit-12h",
        "year",
        "2019-02-03T08:00:00",
        "1993",
        "1993-02-03T08:00:00",
        "hour",
    )
    .await?;
    typed_zero(page, "edit-date", "year", "2019-02-03", "0", "1").await
}

/// In 12-hour time typed hours stay in the value's half of the day and 12 is its first hour
/// ("should support typing into the hour segment in 12 hour time").
#[browser_test]
pub async fn typing_into_the_hour_in_12_hour_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-12h"]).await?;
    for (start, cases) in [
        (
            "2019-02-03T08:00:00",
            [
                ("1", "01:00", "hour"),
                ("01", "01:00", "minute"),
                ("11", "11:00", "minute"),
                ("12", "00:00", "minute"),
                ("4", "04:00", "minute"),
            ],
        ),
        (
            "2019-02-03T20:00:00",
            [
                ("1", "13:00", "hour"),
                ("01", "13:00", "minute"),
                ("11", "23:00", "minute"),
                ("12", "12:00", "minute"),
                ("4", "16:00", "minute"),
            ],
        ),
    ] {
        for (keys, time, focus) in cases {
            let expected = format!("2019-02-03T{time}:00");
            typed(page, "edit-12h", "hour", start, keys, &expected, focus).await?;
        }
        typed_zero(page, "edit-12h", "hour", start, "0", "12").await?;
    }
    Ok(())
}

/// In 24-hour time 0 is an hour of its own ("should support typing into the hour segment in 24
/// hour time").
#[browser_test]
pub async fn typing_into_the_hour_in_24_hour_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-24h"]).await?;
    let start = "2019-02-03T08:00:00";
    for (keys, time, focus) in [
        ("0", "00:00", "hour"),
        ("00", "00:00", "minute"),
        ("1", "01:00", "hour"),
        ("01", "01:00", "minute"),
        ("11", "11:00", "minute"),
        ("23", "23:00", "minute"),
    ] {
        let expected = format!("2019-02-03T{time}:00");
        typed(page, "edit-24h", "hour", start, keys, &expected, focus).await?;
    }
    Ok(())
}

/// Typing into the minute: digits up to 5 wait for a second digit, others move on ("should
/// support typing into the minute segment").
#[browser_test]
pub async fn typing_into_the_minute(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-24h"]).await?;
    let start = "2019-02-03T08:08:00";
    for (keys, minute, focus) in [
        ("0", "00", "minute"),
        ("00", "00", "second"),
        ("1", "01", "minute"),
        ("01", "01", "second"),
        ("2", "02", "minute"),
        ("02", "02", "second"),
        ("5", "05", "minute"),
        ("6", "06", "second"),
        ("59", "59", "second"),
    ] {
        let expected = format!("2019-02-03T08:{minute}:00");
        typed(page, "edit-24h", "minute", start, keys, &expected, focus).await?;
    }
    Ok(())
}

/// Typing into the second, as into the minute; the last segment keeps the focus ("should support
/// typing into the second segment").
#[browser_test]
pub async fn typing_into_the_second(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-12h"]).await?;
    let start = "2019-02-03T08:05:08";
    for (keys, second, focus) in [
        ("0", "00", "second"),
        ("00", "00", "dayPeriod"),
        ("1", "01", "second"),
        ("01", "01", "dayPeriod"),
        ("2", "02", "second"),
        ("5", "05", "second"),
        ("6", "06", "dayPeriod"),
        ("59", "59", "dayPeriod"),
    ] {
        let expected = format!("2019-02-03T08:05:{second}");
        typed(page, "edit-12h", "second", start, keys, &expected, focus).await?;
    }
    Ok(())
}

/// "p" and "a" choose PM and AM ("should support typing into the day period segment").
#[browser_test]
pub async fn typing_into_the_day_period(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-12h"]).await?;
    typed(
        page,
        "edit-12h",
        "dayPeriod",
        "2019-02-03T08:00:00",
        "p",
        "2019-02-03T20:00:00",
        "dayPeriod",
    )
    .await?;
    typed(
        page,
        "edit-12h",
        "dayPeriod",
        "2019-02-03T20:00:00",
        "a",
        "2019-02-03T08:00:00",
        "dayPeriod",
    )
    .await
}

/// Arabic-Indic digits typed into an English field fill the year ("should support entering
/// arabic digits").
#[browser_test]
pub async fn typing_arabic_digits(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    typed(
        page,
        "edit-date",
        "year",
        "2019-02-03",
        "٢٠٢٤",
        "2024-02-03",
        "year",
    )
    .await
}

/// A typed local time the time zone skips (2:45 on the night the clocks go forward) shows as typed
/// and is committed as the time it becomes (3:45) once the field is left ("should allow entering
/// invalid times, and constrain on blur").
#[browser_test]
pub async fn typing_a_skipped_time_constrains_on_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["nonexistent"]).await?;
    let month = segment(page, "nonexistent", "month").await?;
    month.focus().await?;
    page.wait_for_focus(&month).await?;
    page.type_text("3").await?;
    page.type_text("8").await?;
    page.type_text("2026").await?;
    page.type_text("02").await?;
    page.type_text("45").await?;
    let hour = segment(page, "nonexistent", "hour").await?;
    segment(page, "nonexistent", "minute")
        .await?
        .wait_for_inner_text("45")
        .await?;
    assert_that!(hour).inner_text().await.is_equal_to("2");
    value(page, "nonexistent")
        .await?
        .inner_text_stays("none", std::time::Duration::from_millis(100))
        .await?;
    page.element("#test-df-nonexistent-after")
        .await?
        .click()
        .await?;
    wait_for_value(
        page,
        "nonexistent",
        "2026-03-08T03:45:00-07:00[America/Los_Angeles]",
    )
    .await?;
    hour.wait_for_inner_text("3").await?;
    Ok(())
}

/// Backspace deletes the month's last digit; one digit left empties it ("should support backspace
/// in the month segment").
#[browser_test]
pub async fn backspace_in_the_month(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    backspace(page, "edit-date", "month", "2019-02-03", None).await?;
    backspace(page, "edit-date", "month", "2019-06-03", None).await?;
    backspace(page, "edit-date", "month", "2019-12-03", Some("2019-01-03")).await
}

/// Backspace in the day ("should support backspace in the day segment").
#[browser_test]
pub async fn backspace_in_the_day(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    backspace(page, "edit-date", "day", "2019-02-03", None).await?;
    backspace(page, "edit-date", "day", "2019-02-20", Some("2019-02-02")).await
}

/// Backspace in the year ("should support backspace in the year segment").
#[browser_test]
pub async fn backspace_in_the_year(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    backspace(page, "edit-date", "year", "2019-02-03", Some("0201-02-03")).await?;
    backspace(page, "edit-date", "year", "0002-02-03", None).await
}

/// In 12-hour time Backspace keeps the hour in its half of the day ("should support backspace in
/// the hour segment in 12 hour time").
#[browser_test]
pub async fn backspace_in_the_hour_in_12_hour_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-12h"]).await?;
    backspace(page, "edit-12h", "hour", "2019-02-03T08:00:00", None).await?;
    backspace(
        page,
        "edit-12h",
        "hour",
        "2019-02-03T11:00:00",
        Some("2019-02-03T01:00:00"),
    )
    .await?;
    backspace(page, "edit-12h", "hour", "2019-02-03T16:00:00", None).await?;
    backspace(
        page,
        "edit-12h",
        "hour",
        "2019-02-03T23:00:00",
        Some("2019-02-03T13:00:00"),
    )
    .await
}

/// Backspace in a 24-hour hour ("should support backspace in the hour segment in 24 hour time").
#[browser_test]
pub async fn backspace_in_the_hour_in_24_hour_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-24h"]).await?;
    backspace(page, "edit-24h", "hour", "2019-02-03T08:00:00", None).await?;
    for (start, expected) in [
        ("2019-02-03T11:00:00", "2019-02-03T01:00:00"),
        ("2019-02-03T16:00:00", "2019-02-03T01:00:00"),
        ("2019-02-03T23:00:00", "2019-02-03T02:00:00"),
    ] {
        backspace(page, "edit-24h", "hour", start, Some(expected)).await?;
    }
    Ok(())
}

/// Backspace empties the day period, keeping the value ("should support backspace in the am/pm
/// field").
#[browser_test]
pub async fn backspace_in_the_day_period(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-12h"]).await?;
    for start in ["2019-02-03T08:00:00", "2019-02-03T16:00:00"] {
        backspace(page, "edit-12h", "dayPeriod", start, None).await?;
        segment(page, "edit-12h", "dayPeriod")
            .await?
            .wait_for_attr("data-placeholder", Some("true"))
            .await?;
    }
    Ok(())
}

/// Backspace in the minute and the second ("should support backspace in the minute segment",
/// "should support second in the minute segment").
#[browser_test]
pub async fn backspace_in_the_minute_and_second(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-24h"]).await?;
    backspace(page, "edit-24h", "minute", "2019-02-03T05:08:00", None).await?;
    backspace(
        page,
        "edit-24h",
        "minute",
        "2019-02-03T05:25:00",
        Some("2019-02-03T05:02:00"),
    )
    .await?;
    backspace(
        page,
        "edit-24h",
        "minute",
        "2019-02-03T05:59:00",
        Some("2019-02-03T05:05:00"),
    )
    .await?;
    backspace(page, "edit-24h", "second", "2019-02-03T05:05:08", None).await?;
    backspace(
        page,
        "edit-24h",
        "second",
        "2019-02-03T05:05:25",
        Some("2019-02-03T05:05:02"),
    )
    .await?;
    backspace(
        page,
        "edit-24h",
        "second",
        "2019-02-03T05:05:59",
        Some("2019-02-03T05:05:05"),
    )
    .await
}

/// In Arabic, Backspace deletes the last Arabic-Indic digit ("should support backspace with
/// arabic digits").
#[browser_test]
pub async fn backspace_with_arabic_digits(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["arabic"]).await?;
    let year = segment(page, "arabic", "year").await?;
    assert_that!(year).inner_text().await.is_equal_to("٢٠١٩");
    year.focus().await?;
    page.wait_for_focus(&year).await?;
    page.send_keys(Key::Backspace).await?;
    wait_for_value(page, "arabic", "0201-02-03").await?;
    year.wait_for_inner_text("٢٠١").await?;
    Ok(())
}

/// Clearing every segment with Backspace (moving back over the empty ones) empties the value
/// once the last segment is cleared ("should trigger onChange with null when all segments are
/// cleared").
#[browser_test]
pub async fn clearing_every_segment_empties_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    let year = segment(page, "edit-date", "year").await?;
    year.focus().await?;
    page.wait_for_focus(&year).await?;
    for _ in 0..5 {
        page.send_keys(Key::Backspace).await?;
    }
    year.wait_for_inner_text("yyyy").await?;
    let day = segment(page, "edit-date", "day").await?;
    page.wait_for_focus(&day).await?;
    wait_for_value(page, "edit-date", "0002-02-03").await?;
    page.send_keys(Key::Backspace).await?;
    page.send_keys(Key::Backspace).await?;
    day.wait_for_inner_text("dd").await?;
    page.wait_for_focus(&segment(page, "edit-date", "month").await?)
        .await?;
    value(page, "edit-date")
        .await?
        .inner_text_stays("0002-02-03", std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Backspace).await?;
    page.send_keys(Key::Backspace).await?;
    wait_for_value(page, "edit-date", "none").await?;
    segment(page, "edit-date", "month")
        .await?
        .wait_for_inner_text("mm")
        .await?;
    Ok(())
}

/// The segments are spin buttons with their value, limits and a text value: the month's name, the
/// hour with its day period, "Empty" when empty (react-aria's `useDateSegment`, via
/// "should keep dayPeriod the same when hour segment that has a value >= 12 is cleared").
#[browser_test]
pub async fn spin_button_values(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-12h"]).await?;
    set(page, "edit-12h", "2019-02-03T08:05:10").await?;
    for (kind, now, text, min, max) in [
        ("month", "2", "2 – February", "1", "12"),
        ("day", "3", "3", "1", "31"),
        ("year", "2019", "2019", "1", "9999"),
        ("hour", "8", "8\u{202f}AM", "1", "12"),
        ("minute", "5", "05", "0", "59"),
        ("second", "10", "10", "0", "59"),
        ("dayPeriod", "0", "AM", "0", "1"),
    ] {
        let segment = segment(page, "edit-12h", kind).await?;
        assert_that!(segment)
            .has_attribute("aria-valuenow")
            .await
            .with_detail_message(kind)
            .is_equal_to(now);
        assert_that!(segment)
            .has_attribute("aria-valuetext")
            .await
            .with_detail_message(kind)
            .is_equal_to(text);
        assert_that!(segment)
            .has_attribute("aria-valuemin")
            .await
            .with_detail_message(kind)
            .is_equal_to(min);
        assert_that!(segment)
            .has_attribute("aria-valuemax")
            .await
            .with_detail_message(kind)
            .is_equal_to(max);
    }
    Ok(())
}

/// Clearing an afternoon hour keeps PM ("should keep dayPeriod the same when hour segment that has
/// a value >= 12 is cleared").
#[browser_test]
pub async fn clearing_the_hour_keeps_the_day_period(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["time"]).await?;
    set(page, "time", "20:24:00").await?;
    let hour = segment(page, "time", "hour").await?;
    let day_period = segment(page, "time", "dayPeriod").await?;
    hour.focus().await?;
    page.wait_for_focus(&hour).await?;
    assert_that!(hour)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("8\u{202f}PM");
    assert_that!(day_period)
        .has_attribute("aria-label")
        .await
        .is_equal_to("AM/PM, ");
    assert_that!(day_period)
        .inner_text()
        .await
        .is_equal_to("PM");
    page.send_keys(Key::Backspace).await?;
    hour.wait_for_attr("aria-valuetext", Some("Empty")).await?;
    day_period
        .inner_text_stays("PM", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

// Time zones and daylight saving time.

/// The hour of a zoned value steps through the repeated hour of the night the clocks go back
/// (1:45 PDT, 1:45 PST, 2:45 PST) and back, 12\u{202f}AM showing as "12" ("should support cycling through
/// DST fall back transitions").
#[browser_test]
pub async fn hour_through_the_fall_back(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["dst"]).await?;
    let hour = segment(page, "dst", "hour").await?;
    hour.focus().await?;
    page.wait_for_focus(&hour).await?;
    for (key, expected) in [
        (Key::Up, "2021-11-07T01:45:00-07:00[America/Los_Angeles]"),
        (Key::Up, "2021-11-07T01:45:00-08:00[America/Los_Angeles]"),
        (Key::Up, "2021-11-07T02:45:00-08:00[America/Los_Angeles]"),
        (Key::Down, "2021-11-07T01:45:00-08:00[America/Los_Angeles]"),
        (Key::Down, "2021-11-07T01:45:00-07:00[America/Los_Angeles]"),
        (Key::Down, "2021-11-07T00:45:00-07:00[America/Los_Angeles]"),
    ] {
        page.send_keys(key).await?;
        wait_for_value(page, "dst", expected).await?;
    }
    hour.wait_for_inner_text("12").await?;
    page.send_keys(Key::Down).await?;
    wait_for_value(
        page,
        "dst",
        "2021-11-07T11:45:00-08:00[America/Los_Angeles]",
    )
    .await?;
    Ok(())
}

/// The hour steps through the repeated hour also while the minute is empty ("should support
/// cycling through DST fall back transitions even if the minutes are undefined").
#[browser_test]
pub async fn hour_through_the_fall_back_without_minutes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["dst"]).await?;
    let minute = segment(page, "dst", "minute").await?;
    minute.focus().await?;
    page.wait_for_focus(&minute).await?;
    page.send_keys(Key::Backspace).await?;
    minute.wait_for_attr("aria-valuetext", Some("04")).await?;
    page.send_keys(Key::Backspace).await?;
    minute
        .wait_for_attr("aria-valuetext", Some("Empty"))
        .await?;
    page.send_keys(Key::Left).await?;
    let hour = segment(page, "dst", "hour").await?;
    page.wait_for_focus(&hour).await?;
    for (key, text) in [
        (Key::Up, "1"),
        (Key::Up, "1"),
        (Key::Up, "2"),
        (Key::Down, "1"),
        (Key::Down, "1"),
        (Key::Down, "12"),
        (Key::Down, "11"),
    ] {
        page.send_keys(key).await?;
        hour.wait_for_inner_text(text).await?;
    }
    Ok(())
}

/// An empty zoned time field steps its hour from the placeholder through the repeated hour
/// ("should support cycling through DST fall back transitions with ZonedDateTime placeholder").
#[browser_test]
pub async fn time_field_through_the_fall_back_from_the_placeholder(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_sections(PATH, &["dst-time"]).await?;
    let hour = segment(page, "dst-time", "hour").await?;
    assert_that!(segment(page, "dst-time", "minute").await?)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Empty");
    assert_that!(hour)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Empty");
    hour.focus().await?;
    page.wait_for_focus(&hour).await?;
    for (key, text) in [
        (Key::Up, "1"),
        (Key::Up, "1"),
        (Key::Up, "2"),
        (Key::Down, "1"),
        (Key::Down, "1"),
        (Key::Down, "12"),
    ] {
        page.send_keys(key).await?;
        hour.wait_for_inner_text(text).await?;
    }
    Ok(())
}

/// A zoned time field steps its value's hour through the repeated hour ("should support cycling
/// through DST fall back transitions with ZonedDateTime defaultValue").
#[browser_test]
pub async fn time_field_through_the_fall_back(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["dst-time"]).await?;
    set(
        page,
        "dst-time",
        "2021-11-07T01:45:00-07:00[America/Los_Angeles]",
    )
    .await?;
    let hour = segment(page, "dst-time", "hour").await?;
    hour.focus().await?;
    page.wait_for_focus(&hour).await?;
    for (key, expected) in [
        (Key::Up, "2021-11-07T01:45:00-08:00[America/Los_Angeles]"),
        (Key::Up, "2021-11-07T02:45:00-08:00[America/Los_Angeles]"),
        (Key::Down, "2021-11-07T01:45:00-08:00[America/Los_Angeles]"),
        (Key::Down, "2021-11-07T01:45:00-07:00[America/Los_Angeles]"),
        (Key::Down, "2021-11-07T00:45:00-07:00[America/Los_Angeles]"),
    ] {
        page.send_keys(key).await?;
        wait_for_value(page, "dst-time", expected).await?;
    }
    Ok(())
}

/// A zoned time field shows its value's time zone, a read-only textbox, which stays while the
/// minute is cleared and set again ("should have a timeZone when set by defaultValue", "should
/// keep timeZone from defaultValue when minute segment cleared", "... cleared then set").
#[browser_test]
pub async fn time_field_keeps_its_time_zone(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["dst-time"]).await?;
    set(
        page,
        "dst-time",
        "2023-07-01T01:05:00-07:00[America/Los_Angeles]",
    )
    .await?;
    let zone = segment(page, "dst-time", "timeZoneName").await?;
    assert_that!(zone)
        .has_attribute("role")
        .await
        .is_equal_to("textbox");
    assert_that!(zone)
        .has_attribute("aria-label")
        .await
        .is_equal_to("time zone, ");
    assert_that!(zone).inner_text().await.is_equal_to("PDT");
    let minute = segment(page, "dst-time", "minute").await?;
    minute.focus().await?;
    page.wait_for_focus(&minute).await?;
    assert_that!(minute)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("05");
    page.send_keys(Key::Backspace).await?;
    minute
        .wait_for_attr("aria-valuetext", Some("Empty"))
        .await?;
    page.wait_for_focus(&minute).await?;
    zone.inner_text_stays("PDT", std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Up).await?;
    // An empty segment starts at this fixture's explicit 01:45 placeholder.
    minute.wait_for_attr("aria-valuetext", Some("45")).await?;
    assert_that!(zone).inner_text().await.is_equal_to("PDT");
    Ok(())
}

/// A time field of dates and times is bounded by times of day on any day: 8:00 is before 9:00 on
/// June 5 and on any other date, 10:00 is within (react-stately's `useTimeFieldState`,
/// `convertValue(minValue, day)`).
#[browser_test]
pub async fn time_bounds_on_the_values_day(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["time-bounds"]).await?;
    let section = page.element("#test-df-time-bounds").await?;
    let hour = segment(page, "time-bounds", "hour").await?;
    hour.wait_for_attr("aria-invalid", Some("true")).await?;
    assert_that!(section)
        .inner_text()
        .await
        .contains("Value must be 9:00\u{202f}AM or later.");
    set(page, "time-bounds", "2030-01-01T10:00:00").await?;
    hour.wait_for_attr("aria-invalid", None).await?;
    set(page, "time-bounds", "2030-01-01T17:30:00").await?;
    hour.wait_for_attr("aria-invalid", Some("true")).await?;
    assert_that!(|| section.inner_text())
        .eventually_ok()
        .satisfies(|text| {
            text.contains("Value must be 5:00\u{202f}PM or earlier.");
        })
        .await;
    set(page, "time-bounds", "2030-01-01T08:00:00").await?;
    assert_that!(|| section.inner_text())
        .eventually_ok()
        .satisfies(|text| {
            text.contains("Value must be 9:00\u{202f}AM or later.");
        })
        .await;
    Ok(())
}

/// A time field of dates and times bounded by dates and times: 8:00 is valid on June 6, after the
/// minimum's day, and invalid on June 5, where stepping the hour to 9:00 makes it valid; 17:30 on
/// June 7 is after the maximum (react-stately's `useTimeFieldState`: `convertValue` keeps a bound
/// with a day).
#[browser_test]
pub async fn time_bounds_with_dates(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["time-absolute-bounds"]).await?;
    let section = page.element("#test-df-time-absolute-bounds").await?;
    let hour = segment(page, "time-absolute-bounds", "hour").await?;
    hour.attr_stays("aria-invalid", None, std::time::Duration::from_millis(100))
        .await?;
    set(page, "time-absolute-bounds", "2024-06-05T08:00:00").await?;
    hour.wait_for_attr("aria-invalid", Some("true")).await?;
    assert_that!(|| section.inner_text())
        .eventually_ok()
        .satisfies(|text| {
            text.contains("Value must be 9:00\u{202f}AM or later.");
        })
        .await;
    hour.focus().await?;
    page.wait_for_focus(&hour).await?;
    page.send_keys(Key::Up).await?;
    wait_for_value(page, "time-absolute-bounds", "2024-06-05T09:00:00").await?;
    hour.wait_for_attr("aria-invalid", None).await?;
    set(page, "time-absolute-bounds", "2024-06-07T17:30:00").await?;
    hour.wait_for_attr("aria-invalid", Some("true")).await?;
    assert_that!(|| section.inner_text())
        .eventually_ok()
        .satisfies(|text| {
            text.contains("Value must be 5:00\u{202f}PM or earlier.");
        })
        .await;
    Ok(())
}

// Focus.

/// Pressing the field beside its segments focuses the first segment when empty, the first empty
/// one after a filled one, and the last one with a value ("should focus the first segment on mouse
/// down in the field", "should focus the first unfilled segment on mouse down in the field",
/// "should focus the last segment on mouse down in the field with a value").
#[browser_test]
pub async fn pressing_the_field_focuses_a_segment(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["mouse-down"]).await?;
    let group = page.element("#test-df-mouse-down [role=group]").await?;
    let press_beside_the_segments =
        || async { group.press_and_hold_at(130, 0).await?.release().await };
    let month = segment(page, "mouse-down", "month").await?;
    press_beside_the_segments().await?;
    page.wait_for_focus(&month).await?;

    page.send_keys(Key::Up).await?;
    month.wait_for_attr("data-placeholder", None).await?;
    press_beside_the_segments().await?;
    page.wait_for_focus(&segment(page, "mouse-down", "day").await?)
        .await?;

    set(page, "mouse-down", "2020-02-03").await?;
    press_beside_the_segments().await?;
    page.wait_for_focus(&segment(page, "mouse-down", "year").await?)
        .await?;
    Ok(())
}

/// An auto-focused field focuses its first segment ("should support autoFocus").
#[browser_test]
pub async fn auto_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["auto-focus"]).await?;
    page.wait_for_focus(&segment(page, "auto-focus", "month").await?)
        .await?;
    Ok(())
}

/// Focus moving into the field reports a focus change once, moving between its segments none,
/// and leaving it one more ("should focus field and switching segments via tab does not change
/// focus", "should call blur when focus leaves").
#[browser_test]
pub async fn focus_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["focus-events"]).await?;
    let log = page.element("#test-df-focus-events-log").await?;
    page.element("#test-df-focus-events-before")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&segment(page, "focus-events", "month").await?)
        .await?;
    log.wait_for_inner_text("true").await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&segment(page, "focus-events", "day").await?)
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&segment(page, "focus-events", "year").await?)
        .await?;
    log.inner_text_stays("true", std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-df-focus-events-after").await?)
        .await?;
    log.wait_for_inner_text("true,false").await?;
    Ok(())
}

// Labels, descriptions and states.

/// An `aria-label` names the group and, after the segment's name, each segment, which then has no
/// `aria-labelledby` ("should support labeling with aria-label").
#[browser_test]
pub async fn labelled_by_aria_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["aria-label"]).await?;
    let group = page.element("#test-df-aria-label [role=group]").await?;
    assert_that!(group)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Birth date");
    assert_that!(group).has_attribute("id").await;
    for segment in page
        .elements("#test-df-aria-label [role=spinbutton]")
        .await?
    {
        assert_that!(segment)
            .has_attribute("aria-label")
            .await
            .ends_with(" Birth date");
        assert_that!(segment)
            .attribute("aria-labelledby")
            .await
            .is_none();
    }
    Ok(())
}

/// An `aria-labelledby` labels the group, and each segment after itself ("should support labeling
/// with aria-labelledby").
#[browser_test]
pub async fn labelled_by_another_element(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["aria-labelledby"]).await?;
    let group = page
        .element("#test-df-aria-labelledby [role=group]")
        .await?;
    assert_that!(group).attribute("aria-label").await.is_none();
    assert_that!(group)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to("test-df-external-label");
    for segment in page
        .elements("#test-df-aria-labelledby [role=spinbutton]")
        .await?
    {
        let id = segment.id().await?.unwrap_or_default();
        assert_that!(segment)
            .has_attribute("aria-labelledby")
            .await
            .is_equal_to(format!("{id} test-df-external-label"));
    }
    Ok(())
}

/// The value's description comes before the help text; only the first segment is described
/// ("should support help text with a value").
#[browser_test]
pub async fn help_text_with_a_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["edit-date"]).await?;
    set(page, "edit-date", "2020-02-03").await?;
    let group = page.element("#test-df-edit-date [role=group]").await?;
    assert_that!(|| group.accessible_description())
        .eventually_ok()
        .matches(assertr::matchers::eq(
            "Selected Date: February 3, 2020 Help text".to_owned(),
        ))
        .await;
    let describedby = group.attr("aria-describedby").await?;
    let segments = page
        .elements("#test-df-edit-date [role=spinbutton]")
        .await?;
    assert_that!(segments[0])
        .attribute("aria-describedby")
        .await
        .is_equal_to(describedby);
    for segment in &segments[1..] {
        assert_that!(segment)
            .attribute("aria-describedby")
            .await
            .is_none();
    }
    Ok(())
}

/// An invalid field's error message describes the group and every segment, after the value's
/// description once there is a value ("should support error message", "should support error
/// message with a value").
#[browser_test]
pub async fn error_message(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["error"]).await?;
    let group = page.element("#test-df-error [role=group]").await?;
    assert_that!(group)
        .accessible_description()
        .await
        .is_equal_to("Error message");
    let check_segments = || async {
        let describedby = group.attr("aria-describedby").await?;
        for segment in page.elements("#test-df-error [role=spinbutton]").await? {
            assert_that!(segment)
                .attribute("aria-describedby")
                .await
                .is_equal_to(describedby.clone());
        }
        Ok::<(), Report>(())
    };
    check_segments().await?;
    set(page, "error", "2020-02-03").await?;
    assert_that!(|| group.accessible_description())
        .eventually_ok()
        .matches(assertr::matchers::eq(
            "Selected Date: February 3, 2020 Error message".to_owned(),
        ))
        .await;
    check_segments().await
}

/// A valid field shows no error message and describes nothing ("should not display error message
/// if not invalid").
#[browser_test]
pub async fn no_error_message_while_valid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["no-error"]).await?;
    let group = page.element("#test-df-no-error [role=group]").await?;
    assert_that!(group)
        .attribute("aria-describedby")
        .await
        .is_none();
    for segment in page.elements("#test-df-no-error [role=spinbutton]").await? {
        assert_that!(segment)
            .attribute("aria-describedby")
            .await
            .is_none();
    }
    assert_that!(page.count("#test-df-no-error .leptonic-FieldError").await?).is_equal_to(0);
    Ok(())
}

/// A typed unavailable date makes the field invalid with its message ("Shows as invalid if an
/// unavailable date is given").
#[browser_test]
pub async fn unavailable_date(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["unavailable"]).await?;
    let month = segment(page, "unavailable", "month").await?;
    month.focus().await?;
    page.wait_for_focus(&month).await?;
    page.type_text("01011980").await?;
    page.element(css("#test-df-unavailable .leptonic-FieldError").text("Date unavailable."))
        .await?;
    month.wait_for_attr("aria-invalid", Some("true")).await?;
    Ok(())
}

/// The group and the segments show hover: `data-hovered` while the pointer is over them ("should
/// support hover state").
#[browser_test]
pub async fn hover_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    let group = page.element("#test-df-basic [role=group]").await?;
    let month = segment(page, "basic", "month").await?;
    assert_that!(group)
        .attribute("data-hovered")
        .await
        .is_none();
    group.hover().await?;
    group.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("#test-df-basic-before").await?.hover().await?;
    group.wait_for_attr("data-hovered", None).await?;
    month.hover().await?;
    month.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("#test-df-basic-before").await?.hover().await?;
    month.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// A disabled field marks its group, segments and literals disabled, none read-only ("should
/// support disabled state").
#[browser_test]
pub async fn disabled_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["disabled"]).await?;
    let group = page.element("#test-df-disabled [role=group]").await?;
    assert_that!(group)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    for segment in page
        .elements("#test-df-disabled .leptonic-DateSegment")
        .await?
    {
        assert_that!(segment)
            .has_attribute("data-disabled")
            .await
            .is_equal_to("true");
        assert_that!(segment)
            .attribute("data-readonly")
            .await
            .is_none();
    }
    Ok(())
}

/// A read-only field marks its group, segments and literals read-only, not disabled; with both,
/// both ("should support readonly state", "should support readonly with disabled state").
#[browser_test]
pub async fn read_only_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["read-only", "read-only-disabled"])
        .await?;
    for (section, disabled) in [("read-only", None), ("read-only-disabled", Some("true"))] {
        let group = page
            .element(format!("#test-df-{section} [role=group]"))
            .await?;
        assert_that!(group)
            .attribute("data-readonly")
            .await
            .with_detail_message(section)
            .is_equal_to(Some("true".to_owned()));
        assert_that!(group)
            .attribute("data-disabled")
            .await
            .derive_owned(|value| value.as_deref())
            .with_detail_message(section)
            .is_equal_to(disabled);
        for segment in page
            .elements(format!("#test-df-{section} .leptonic-DateSegment"))
            .await?
        {
            assert_that!(segment)
                .attribute("data-readonly")
                .await
                .with_detail_message(section)
                .is_equal_to(Some("true".to_owned()));
            assert_that!(segment)
                .attribute("data-disabled")
                .await
                .derive_owned(|value| value.as_deref())
                .with_detail_message(section)
                .is_equal_to(disabled);
        }
    }
    Ok(())
}

/// A required field marks itself `data-required` and its segments `aria-required` ("should support
/// required state").
#[browser_test]
pub async fn required_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic", "v-required"]).await?;
    assert_that!(page.element("#test-df-basic .leptonic-DateField").await?)
        .attribute("data-required")
        .await
        .is_none();
    let field = page
        .element("#test-df-v-required .leptonic-DateField")
        .await?;
    assert_that!(field)
        .has_attribute("data-required")
        .await
        .is_equal_to("true");
    for segment in page
        .elements("#test-df-v-required [role=spinbutton]")
        .await?
    {
        assert_that!(segment)
            .has_attribute("aria-required")
            .await
            .is_equal_to("true");
    }
    Ok(())
}

// Validation.

/// Waits until the description of `section`'s group contains `text` (or doesn't).
async fn wait_for_description(
    page: &Page<'_>,
    section: &str,
    text: &str,
    present: bool,
) -> Result<(), Report> {
    let group = page
        .element(format!("#test-df-{section} [role=group]"))
        .await?;
    assert_that!(|| group.accessible_description())
        .eventually_ok()
        .satisfies(|description| {
            if present {
                description.contains(text);
            } else {
                description.does_not_contain(text);
            }
        })
        .await;
    Ok(())
}

/// Checks the form of `section` (`checkValidity`), which fails.
async fn check_invalid_form(page: &Page<'_>, section: &str) -> Result<(), Report> {
    let valid = page
        .element(format!("#test-df-{section}-form"))
        .await?
        .check_validity()
        .await?;
    assert_that!(valid).is_false();
    Ok(())
}

/// A required field shows the browser's message once the form is checked, focusing its first
/// segment; the message stays while a date is entered and goes when the field is left ("supports
/// validation errors", "supports isRequired").
#[browser_test]
pub async fn required_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["v-required"]).await?;
    let input = page
        .element("#test-df-v-required input[type=text][name=date]")
        .await?;
    let group = page.element("#test-df-v-required [role=group]").await?;
    let field = page
        .element("#test-df-v-required .leptonic-DateField")
        .await?;
    assert_that!(input).has_attribute("required").await;
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(group)
        .attribute("aria-describedby")
        .await
        .is_none();
    assert_that!(field)
        .attribute("data-invalid")
        .await
        .is_none();
    let message = assert_that!(input)
        .property("validationMessage")
        .await
        .some()
        .is_not_blank()
        .actual()
        .clone();

    check_invalid_form(page, "v-required").await?;
    wait_for_description(page, "v-required", &message, true).await?;
    field.wait_for_attr("data-invalid", Some("true")).await?;
    page.wait_for_focus(&segment(page, "v-required", "month").await?)
        .await?;
    page.send_keys(Key::Up).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Up).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Up).await?;
    input.wait_for_prop("validationMessage", "").await?;
    page.settle().await?;
    assert_that!(|| group.accessible_description())
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .satisfies(|description| {
            description.contains(&message);
        })
        .await;
    page.send_keys(Key::Tab).await?;
    wait_for_description(page, "v-required", &message, false).await?;
    field.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// Resetting the form clears the shown validation ("clears validation on form reset").
#[browser_test]
pub async fn reset_clears_the_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["v-required"]).await?;
    let group = page.element("#test-df-v-required [role=group]").await?;
    let message = page
        .element("#test-df-v-required input[type=text][name=date]")
        .await?
        .prop("validationMessage")
        .await?
        .unwrap_or_default();
    check_invalid_form(page, "v-required").await?;
    wait_for_description(page, "v-required", &message, true).await?;
    page.element("#test-df-v-required-reset")
        .await?
        .click()
        .await?;
    group.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

/// Focusing and leaving the field without a change shows no validation; once checked, typing a
/// date and leaving clears the message ("only commits on blur if the value changed").
#[browser_test]
pub async fn validation_commits_on_blur_only_after_a_change(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["v-required"]).await?;
    let group = page.element("#test-df-v-required [role=group]").await?;
    let month = segment(page, "v-required", "month").await?;
    month.focus().await?;
    page.wait_for_focus(&month).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.settle().await?;
    group
        .attr_stays(
            "aria-describedby",
            None,
            std::time::Duration::from_millis(100),
        )
        .await?;
    check_invalid_form(page, "v-required").await?;
    let input = page
        .element("#test-df-v-required input[type=text][name=date]")
        .await?;
    let error = input.prop("validationMessage").await?.unwrap_or_default();
    page.wait_for_focus(&month).await?;
    page.type_text("232023").await?;
    input.wait_for_prop("validationMessage", "").await?;
    assert_that!(group).has_attribute("aria-describedby").await;
    page.send_keys(Key::Tab).await?;
    wait_for_description(page, "v-required", &error, false).await?;
    wait_for_description(page, "v-required", "Selected Date:", true).await?;
    Ok(())
}

/// Native min and max: the message shows only once the form is checked, stays until the field is
/// left after a fix, for the minimum and the maximum ("supports minValue and maxValue",
/// validationBehavior=native).
#[browser_test]
pub async fn native_min_and_max(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["v-minmax"]).await?;
    let input = page
        .element("#test-df-v-minmax input[type=text][name=date]")
        .await?;
    let later = "Value must be 2/3/2020 or later.";
    let earlier = "Value must be 2/3/2024 or earlier.";
    assert_that!(input.is_valid().await?).is_false();
    wait_for_description(page, "v-minmax", later, false).await?;

    check_invalid_form(page, "v-minmax").await?;
    wait_for_description(page, "v-minmax", later, true).await?;
    page.wait_for_focus(&segment(page, "v-minmax", "month").await?)
        .await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Up).await?;
    input.wait_for_prop("validationMessage", "").await?;
    wait_for_description(page, "v-minmax", later, true).await?;
    page.send_keys(Key::Tab).await?;
    wait_for_description(page, "v-minmax", later, false).await?;

    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&segment(page, "v-minmax", "year").await?)
        .await?;
    page.type_text("2025").await?;
    assert_that!(|| input.is_valid())
        .eventually_ok()
        .matches(assertr::matchers::eq(false))
        .await;
    wait_for_description(page, "v-minmax", earlier, false).await?;
    page.send_keys(Key::Tab).await?;
    check_invalid_form(page, "v-minmax").await?;
    wait_for_description(page, "v-minmax", earlier, true).await?;
    page.wait_for_focus(&segment(page, "v-minmax", "month").await?)
        .await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Down).await?;
    input.wait_for_prop("validationMessage", "").await?;
    wait_for_description(page, "v-minmax", earlier, true).await?;
    page.send_keys(Key::Tab).await?;
    wait_for_description(page, "v-minmax", earlier, false).await?;
    Ok(())
}

/// A native `validate` error shows once the form is checked and goes when a valid date is
/// committed ("supports validate function", validationBehavior=native).
#[browser_test]
pub async fn native_validate(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["v-validate"]).await?;
    let input = page
        .element("#test-df-v-validate input[type=text][name=date]")
        .await?;
    wait_for_description(page, "v-validate", "Invalid value", false).await?;
    assert_that!(input.is_valid().await?).is_false();
    check_invalid_form(page, "v-validate").await?;
    wait_for_description(page, "v-validate", "Invalid value", true).await?;
    page.wait_for_focus(&segment(page, "v-validate", "month").await?)
        .await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    page.type_text("2024").await?;
    input.wait_for_prop("validationMessage", "").await?;
    wait_for_description(page, "v-validate", "Invalid value", true).await?;
    page.send_keys(Key::Tab).await?;
    wait_for_description(page, "v-validate", "Invalid value", false).await?;
    assert_that!(input.is_valid().await?).is_true();
    Ok(())
}

/// A form's server error shows after submitting and goes once a new date is committed ("supports
/// server validation", validationBehavior=native).
#[browser_test]
pub async fn native_server_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["v-server"]).await?;
    let group = page.element("#test-df-v-server [role=group]").await?;
    let input = page
        .element("#test-df-v-server input[type=text][name=date]")
        .await?;
    assert_that!(group)
        .attribute("aria-describedby")
        .await
        .is_none();
    page.element("#test-df-v-server-submit")
        .await?
        .click()
        .await?;
    wait_for_description(page, "v-server", "Invalid value", true).await?;
    assert_that!(input.is_valid().await?).is_false();
    let year = segment(page, "v-server", "year").await?;
    year.focus().await?;
    page.wait_for_focus(&year).await?;
    page.type_text("2024").await?;
    page.send_keys(Key::Left).await?;
    page.type_text("2").await?;
    page.send_keys(Key::Left).await?;
    page.type_text("2").await?;
    page.element("#test-df-v-server-after")
        .await?
        .click()
        .await?;
    wait_for_description(page, "v-server", "Invalid value", false).await?;
    assert_that!(input.is_valid().await?).is_true();
    Ok(())
}

/// A `FieldError` message chosen by the validation details replaces the browser's ("supports
/// customizing native error messages").
#[browser_test]
pub async fn custom_native_message(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["v-custom"]).await?;
    let group = page.element("#test-df-v-custom [role=group]").await?;
    assert_that!(group)
        .attribute("aria-describedby")
        .await
        .is_none();
    check_invalid_form(page, "v-custom").await?;
    assert_that!(|| group.accessible_description())
        .eventually_ok()
        .matches(assertr::matchers::eq("Please enter a value".to_owned()))
        .await;
    Ok(())
}

/// With ARIA validation, min and max errors show right away and follow the value ("supports
/// minValue and maxValue", validationBehavior=aria).
#[browser_test]
pub async fn aria_min_and_max(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["v-aria-minmax"]).await?;
    let later = "Value must be 2/3/2020 or later.";
    let earlier = "Value must be 2/3/2024 or earlier.";
    wait_for_description(page, "v-aria-minmax", later, true).await?;
    let year = segment(page, "v-aria-minmax", "year").await?;
    year.focus().await?;
    page.wait_for_focus(&year).await?;
    page.send_keys(Key::Up).await?;
    wait_for_description(page, "v-aria-minmax", later, false).await?;
    for _ in 0..5 {
        page.send_keys(Key::Up).await?;
    }
    wait_for_description(page, "v-aria-minmax", earlier, true).await?;
    page.send_keys(Key::Down).await?;
    wait_for_description(page, "v-aria-minmax", earlier, false).await?;
    Ok(())
}

/// With ARIA validation, a `validate` error shows right away and goes with a valid date
/// ("supports validate function", validationBehavior=aria).
#[browser_test]
pub async fn aria_validate(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["v-aria-validate"]).await?;
    wait_for_description(page, "v-aria-validate", "Invalid value", true).await?;
    let year = segment(page, "v-aria-validate", "year").await?;
    year.focus().await?;
    page.wait_for_focus(&year).await?;
    page.type_text("2024").await?;
    wait_for_description(page, "v-aria-validate", "Invalid value", false).await?;
    Ok(())
}

/// With ARIA validation, a form's server error shows right away and goes once a new date is
/// committed ("supports server validation", validationBehavior=aria).
#[browser_test]
pub async fn aria_server_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["v-aria-server"]).await?;
    wait_for_description(page, "v-aria-server", "Invalid value", true).await?;
    let year = segment(page, "v-aria-server", "year").await?;
    year.focus().await?;
    page.wait_for_focus(&year).await?;
    page.type_text("2024").await?;
    page.element("#test-df-v-aria-server-after")
        .await?
        .click()
        .await?;
    wait_for_description(page, "v-aria-server", "Invalid value", false).await?;
    Ok(())
}

/// A time field's native min and max: 8:00 is before 9:00 once the form is checked, fixed with
/// ArrowUp; 6\u{202f}PM is after 5\u{202f}PM, fixed with ArrowDown ("supports minValue and maxValue",
/// `TimeField.test.js`).
#[browser_test]
pub async fn time_field_native_min_and_max(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["time-minmax"]).await?;
    let input = page
        .element("#test-df-time-minmax input[type=text][name=time]")
        .await?;
    let later = "Value must be 9:00\u{202f}AM or later.";
    let earlier = "Value must be 5:00\u{202f}PM or earlier.";
    assert_that!(input.is_valid().await?).is_false();
    wait_for_description(page, "time-minmax", later, false).await?;
    check_invalid_form(page, "time-minmax").await?;
    wait_for_description(page, "time-minmax", later, true).await?;
    let hour = segment(page, "time-minmax", "hour").await?;
    page.wait_for_focus(&hour).await?;
    page.send_keys(Key::Up).await?;
    input.wait_for_prop("validationMessage", "").await?;
    wait_for_description(page, "time-minmax", later, true).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    wait_for_description(page, "time-minmax", later, false).await?;

    hour.focus().await?;
    page.wait_for_focus(&hour).await?;
    page.type_text("6").await?;
    page.wait_for_focus(&segment(page, "time-minmax", "minute").await?)
        .await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Up).await?;
    assert_that!(|| input.is_valid())
        .eventually_ok()
        .matches(assertr::matchers::eq(false))
        .await;
    wait_for_description(page, "time-minmax", earlier, false).await?;
    page.send_keys(Key::Tab).await?;
    check_invalid_form(page, "time-minmax").await?;
    wait_for_description(page, "time-minmax", earlier, true).await?;
    page.wait_for_focus(&hour).await?;
    page.send_keys(Key::Down).await?;
    input.wait_for_prop("validationMessage", "").await?;
    wait_for_description(page, "time-minmax", earlier, true).await?;
    page.blur_focused().await?;
    wait_for_description(page, "time-minmax", earlier, false).await?;
    Ok(())
}

/// A time field describes its value ("Selected Time: 8:30\u{202f}AM") on its first segment, submits it
/// as `08:30:00`, and its form's reset restores it ("should include a selected value
/// description", "supports form reset", `TimeField.test.js`).
#[browser_test]
pub async fn time_field_description_and_reset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["time-reset"]).await?;
    let group = page.element("#test-df-time-reset [role=group]").await?;
    let input = page.element("#test-df-time-reset input[name=time]").await?;
    assert_that!(group)
        .accessible_description()
        .await
        .is_equal_to("Selected Time: 8:30\u{202f}AM");
    assert_that!(input)
        .property("value")
        .await
        .some()
        .is_equal_to("08:30:00");
    let hour = segment(page, "time-reset", "hour").await?;
    let describedby = group.attr("aria-describedby").await?;
    assert_that!(hour)
        .attribute("aria-describedby")
        .await
        .is_equal_to(describedby);
    assert_that!(segment(page, "time-reset", "minute").await?)
        .attribute("aria-describedby")
        .await
        .is_none();
    hour.focus().await?;
    page.wait_for_focus(&hour).await?;
    page.send_keys(Key::Up).await?;
    input.wait_for_prop("value", "09:30:00").await?;
    assert_that!(|| group.accessible_description())
        .eventually_ok()
        .matches(assertr::matchers::eq(
            "Selected Time: 9:30\u{202f}AM".to_owned(),
        ))
        .await;
    page.element("#test-df-time-reset-reset")
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "08:30:00").await?;
    assert_that!(|| group.accessible_description())
        .eventually_ok()
        .matches(assertr::matchers::eq(
            "Selected Time: 8:30\u{202f}AM".to_owned(),
        ))
        .await;
    Ok(())
}

/// The hidden date input's container tells accessibility checkers to ignore its hidden focusable
/// input and keeps the focus out of it ("should always add a data attribute
/// data-a11y-ignore="aria-hidden-focus"", "should always add a data attribute
/// data-react-aria-prevent-focus", `HiddenDateInput.test.js`).
#[browser_test]
pub async fn hidden_date_input_container(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["basic"]).await?;
    let container = page
        .element("#test-df-basic input[type=date]")
        .await?
        .parent()
        .await?;
    assert_that!(container)
        .has_attribute("data-a11y-ignore")
        .await
        .is_equal_to("aria-hidden-focus");
    assert_that!(container)
        .has_attribute("data-leptonic-prevent-focus")
        .await
        .is_equal_to("true");
    Ok(())
}

/// A field removed as a whole while one of its segments has the focus leaves the focus alone (on
/// `<body>`), without errors ("does not try to shift focus when the entire datepicker is
/// unmounted while focused").
#[browser_test]
pub async fn removing_the_focused_field(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["unmount"]).await?;
    page.element("#test-df-unmount-remove")
        .await?
        .click()
        .await?;
    let era = segment(page, "unmount", "era").await?;
    era.focus().await?;
    page.wait_for_focus(&era).await?;
    // The fixture removes the field 500 ms after the click.
    page.wait_for_count("#test-df-unmount [role=group]", 0)
        .await?;
    let body = page.element("body").await?;
    page.wait_for_focus(&body).await?;
    page.focus_stays(&body, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}
