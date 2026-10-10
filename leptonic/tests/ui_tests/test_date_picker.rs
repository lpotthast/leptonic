// Upstream: react-aria-components/test/DatePicker.test.js @ 99e6102368
// Upstream: react-aria-components/test/DateRangePicker.test.js @ 99e6102368
// Upstream: react-aria-components/test/DateField.test.js @ 99e6102368
// Upstream: react-aria-components/test/TimeField.test.js @ 99e6102368
// Upstream: react-aria/test/datepicker/useDatePicker.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/datepicker/DatePickerBase.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/datepicker/DatePicker.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/datepicker/DateRangePicker.test.js @ 99e6102368
//! Date pickers, date fields and time fields beyond `date_field_tests`: closing on select or
//! not, the pressed button and open state while open, a disabled picker, a programmatic value
//! in an empty picker, required pickers and time fields with their errors, a range picker's
//! placeholder time, Enter, held keys, deleting a partial field, the selection while another
//! element has the focus, and fields outside en-US (German order, right-to-left segments and
//! the isolated time, the segment styles following the locale). Then the pickers' structure
//! (slots, state attributes, form values, read-only, required and invalid segments, the button
//! controlling the dialog, arrows between segments, contexts cleared in the popover), focus
//! changes, times in the popover (time fields bound to the pickers' state contexts), labelling
//! and descriptions, focus on presses of the field, validation as the user edits, time zones
//! and forms (reset, native and `Aria` validation, server errors).
//! Spec: react-aria-components `DatePicker.test.js`, `DateRangePicker.test.js`,
//! `DateField.test.js`, `TimeField.test.js`; react-aria `useDatePicker.test.tsx`;
//! react-spectrum `DatePicker.test.js`, `DateRangePicker.test.js`, `DatePickerBase.test.js`.
use assertr::{
    matchers::{eq, gt, lt},
    prelude::*,
};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::{Report, bail};

use crate::pages::{ElementActions, EventKind, Page, PointerKind, SyntheticEvent, css};

const PATH: &str = "/atoms/date-picker";

/// The segment of `kind` (`month`, `day`, `hour`, ...) of the field in `#test-dp-<section>`.
async fn segment(page: &Page<'_>, section: &str, kind: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-dp-{section} [data-type='{kind}']"))
        .await
}

/// The kinds of the editable segments of `#test-dp-<section>`, in order.
async fn segment_types(page: &Page<'_>, section: &str) -> Result<Vec<String>, Report> {
    let mut types = Vec::new();
    for segment in page
        .elements(format!(
            "#test-dp-{section} [role=spinbutton], #test-dp-{section} [role=textbox]"
        ))
        .await?
    {
        types.push(segment.attr("data-type").await?.unwrap_or_default());
    }
    Ok(types)
}

/// The fixture's output of the value of `#test-dp-<section>` (`none` without one).
async fn value(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-dp-{section}-value")).await
}

/// Waits until the value of `#test-dp-<section>` is `expected`.
async fn wait_for_value(page: &Page<'_>, section: &str, expected: &str) -> Result<(), Report> {
    value(page, section)
        .await?
        .wait_for_inner_text(expected)
        .await
}

/// The button opening the popover of the picker in `#test-dp-<section>`.
async fn button(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-dp-{section} button[aria-haspopup=dialog]"))
        .await
}

/// The text of the segments of `section`'s first `DateInput` (the isolation marks kept).
async fn input_text(page: &Page<'_>, section: &str) -> Result<String, Report> {
    Ok(page
        .first_element(format!("#test-dp-{section} .leptonic-DateInput"))
        .await?
        .prop("textContent")
        .await?
        .unwrap_or_default())
}

/// While the calendar is open its button is pressed and the picker open; selecting a date closes
/// it, or keeps it open until Escape without close on select ("should support close on select =
/// true", "should support close on select = false", "should apply isPressed state to button when
/// expanded", "should support data-open state").
#[browser_test]
pub async fn close_on_select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["close-true", "close-false"])
        .await?;
    let picker = page
        .element("#test-dp-close-true .leptonic-DatePicker")
        .await?;
    let open_button = button(page, "close-true").await?;
    assert_that!(open_button)
        .attribute("data-pressed")
        .await
        .is_none();
    assert_that!(picker).attribute("data-open").await.is_none();
    open_button.click().await?;
    page.element("[role=dialog] [role=grid]").await?;
    open_button
        .wait_for_attr("data-pressed", Some("true"))
        .await?;
    picker.wait_for_attr("data-open", Some("true")).await?;
    let selected = page
        .element("[role=dialog] [role=gridcell][aria-selected=true] > [role=button]")
        .await?;
    assert_that!(selected)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Sunday, February 3, 2019 selected");
    page.element("[role=dialog] [role=button][aria-label^='Monday, February 4, 2019']")
        .await?
        .click()
        .await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    wait_for_value(page, "close-true", "2019-02-04").await?;
    open_button.wait_for_attr("data-pressed", None).await?;

    button(page, "close-false").await?.click().await?;
    page.element("[role=dialog] [role=grid]").await?;
    page.element("[role=dialog] [role=button][aria-label^='Monday, February 4, 2019']")
        .await?
        .click()
        .await?;
    wait_for_value(page, "close-false", "2019-02-04").await?;
    page.count_stays("[role=dialog]", 1, std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    Ok(())
}

/// A disabled picker disables its button, group, segments and hidden input ("should disable button
/// and date input when DatePicker is disabled").
#[browser_test]
pub async fn disabled_picker(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["disabled"]).await?;
    let open_button = button(page, "disabled").await?;
    assert_that!(open_button).enabled().await.is_false();
    let group = page.element("#test-dp-disabled [role=group]").await?;
    assert_that!(group)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    let segments = group.elements("[role=spinbutton]").await?;
    for segment in segments {
        assert_that!(segment)
            .has_attribute("aria-disabled")
            .await
            .is_equal_to("true");
    }
    let input = page
        .element("#test-dp-disabled input[type=text][name='disabled-date']")
        .await?;
    assert_that!(input).enabled().await.is_false();
    Ok(())
}

/// A value set programmatically on an empty picker shows in its field ("should commit
/// programmatically setValue when field is empty").
#[browser_test]
pub async fn programmatic_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty"]).await?;
    assert_that!(
        page.first_element("#test-dp-empty .leptonic-DateInput")
            .await?
    )
    .text_content()
    .await
    .map_owned(Option::unwrap_or_default)
    .is_equal_to("mm/dd/yyyy");
    page.element("#test-dp-empty-set").await?.click().await?;
    wait_for_value(page, "empty", "2020-02-03").await?;
    assert_that!(|| input_text(page, "empty"))
        .eventually_ok()
        .matches(eq("2/3/2020"))
        .await;
    Ok(())
}

/// Waits until `element`'s description contains `text` (or doesn't).
async fn wait_for_description(
    element: &WebElement,
    text: &str,
    present: bool,
) -> Result<(), Report> {
    assert_that!(|| element.accessible_description())
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

/// A required picker shows the browser's error and focuses its first segment once the form is
/// checked; the error stays while a date is entered and goes once focus leaves ("supports
/// validation errors").
#[browser_test]
pub async fn required_picker(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["required"]).await?;
    let input = page
        .element("#test-dp-required input[type=text][name=date]")
        .await?;
    let group = page.element("#test-dp-required [role=group]").await?;
    let picker = page
        .element("#test-dp-required .leptonic-DatePicker")
        .await?;
    assert_that!(input).has_attribute("required").await;
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(picker)
        .attribute("data-invalid")
        .await
        .is_none();
    // The browser's message for a missing value.
    let message = assert_that!(input)
        .property("validationMessage")
        .await
        .some()
        .is_not_blank()
        .actual()
        .clone();

    assert_that!(
        page.element("#test-dp-required-form")
            .await?
            .check_validity()
            .await?
    )
    .is_false();
    wait_for_description(&group, &message, true).await?;
    picker.wait_for_attr("data-invalid", Some("true")).await?;
    let month = segment(page, "required", "month").await?;
    page.wait_for_focus(&month).await?;

    page.send_keys(Key::Up).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Up).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Up).await?;
    assert_that!(|| input.prop("validationMessage"))
        .eventually_ok()
        .matches(eq(Some(String::new())))
        .await;
    // The error stays while the field has the focus.
    page.settle().await?;
    assert_that!(|| group.accessible_description())
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .satisfies(|description| {
            description.contains(&message);
        })
        .await;

    page.element("#test-dp-required-after")
        .await?
        .click()
        .await?;
    wait_for_description(&group, &message, false).await?;
    picker.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// A required time field shows the browser's error and focuses its hour once the form is checked;
/// the error stays while a time is entered and goes once focus leaves ("supports validation
/// errors").
#[browser_test]
pub async fn required_time_field(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["time-required"]).await?;
    let input = page
        .element("#test-dp-time-required input[name=time]")
        .await?;
    let group = page.element("#test-dp-time-required [role=group]").await?;
    assert_that!(input).has_attribute("required").await;
    assert_that!(input.is_valid().await?).is_false();
    // The browser's message for a missing value.
    let message = assert_that!(input)
        .property("validationMessage")
        .await
        .some()
        .is_not_blank()
        .actual()
        .clone();

    assert_that!(
        page.element("#test-dp-time-required-form")
            .await?
            .check_validity()
            .await?
    )
    .is_false();
    wait_for_description(&group, &message, true).await?;
    let hour = segment(page, "time-required", "hour").await?;
    page.wait_for_focus(&hour).await?;

    page.send_keys(Key::Up).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Up).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Up).await?;
    assert_that!(|| input.prop("validationMessage"))
        .eventually_ok()
        .matches(eq(Some(String::new())))
        .await;
    // The error stays while the field has the focus.
    page.settle().await?;
    assert_that!(|| group.accessible_description())
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .satisfies(|description| {
            description.contains(&message);
        })
        .await;
    page.element("#test-dp-time-required-after")
        .await?
        .click()
        .await?;
    wait_for_description(&group, &message, false).await?;
    Ok(())
}

/// Selecting a range in a range picker with times closes it with the placeholder's time on both
/// dates; without close on select it stays open, and closing it commits the range with the
/// placeholder's time ("should set a placeholder time when closing", "should support close on
/// select = false").
#[browser_test]
pub async fn range_placeholder_times(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-time", "range-open"])
        .await?;
    button(page, "range-time").await?.click().await?;
    page.element("[role=dialog] [role=grid]").await?;
    page.element("[role=dialog] [role=button][aria-label*='Friday, January 6, 2023']")
        .await?
        .click()
        .await?;
    page.element("[role=dialog] [role=button][aria-label*='Wednesday, January 11, 2023']")
        .await?
        .click()
        .await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    wait_for_value(
        page,
        "range-time",
        "2023-01-06T00:00:00 - 2023-01-11T00:00:00",
    )
    .await?;
    assert_that!(
        page.first_element("#test-dp-range-time .leptonic-DateInput")
            .await?
    )
    .text_content()
    .await
    .map_owned(|text| {
        text.unwrap_or_default()
            .replace(['\u{2066}', '\u{2069}'], "")
    })
    .is_equal_to("1/6/2023, 12:00:00\u{202f}AM");

    button(page, "range-open").await?.click().await?;
    page.element("[role=dialog] [role=grid]").await?;
    page.element("[role=dialog] [role=button][aria-label*='Friday, January 13, 2023']")
        .await?
        .click()
        .await?;
    page.element("[role=dialog] [role=button][aria-label*='Monday, January 16, 2023']")
        .await?
        .click()
        .await?;
    // Waits for the times while open.
    page.count_stays("[role=dialog]", 1, std::time::Duration::from_millis(100))
        .await?;
    value(page, "range-open")
        .await?
        .inner_text_stays("none", std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    wait_for_value(
        page,
        "range-open",
        "2023-01-13T10:30:00 - 2023-01-16T10:30:00",
    )
    .await?;
    Ok(())
}

/// Enter in a segment keeps the focus and the value and doesn't submit the form ("should do
/// nothing when pressing enter").
#[browser_test]
pub async fn enter_does_nothing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["keys"]).await?;
    let year = segment(page, "keys", "year").await?;
    year.click().await?;
    page.wait_for_focus(&year).await?;
    page.send_keys(Key::Enter).await?;
    page.focus_stays(&year, std::time::Duration::from_millis(100))
        .await?;
    // A submitted form would have reloaded the page with `?keys=...`.
    let url = page.low_level().driver().current_url().await?;
    let submitted: Vec<String> = url
        .query_pairs()
        .map(|(name, _)| name.into_owned())
        .collect();
    assert_that!(submitted).does_not_contain("keys".to_owned());
    value(page, "keys")
        .await?
        .inner_text_stays("2024-12-31", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Holding ArrowRight moves on across segments, and holding Backspace moves back across empty
/// segments ("should support repeat keydown events when holding an arrow key to navigate
/// segments", "should support repeat keydown events when holding backspace across empty
/// segments").
#[browser_test]
pub async fn held_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["keys", "empty-field"]).await?;
    let month = segment(page, "keys", "month").await?;
    month.click().await?;
    page.wait_for_focus(&month).await?;
    page.hold_key("ArrowRight", 1).await?;
    let year = segment(page, "keys", "year").await?;
    page.wait_for_focus(&year).await?;

    let empty_year = segment(page, "empty-field", "year").await?;
    empty_year.click().await?;
    page.wait_for_focus(&empty_year).await?;
    page.hold_key("Backspace", 1).await?;
    let empty_month = segment(page, "empty-field", "month").await?;
    page.wait_for_focus(&empty_month).await?;
    Ok(())
}

/// Deleting the only filled segment of a partially filled field shows every segment's
/// placeholder again ("should reset to placeholders when deleting a partially filled DateField").
#[browser_test]
pub async fn deleting_a_partial_field(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty-field"]).await?;
    let month = segment(page, "empty-field", "month").await?;
    month.click().await?;
    page.wait_for_focus(&month).await?;
    page.type_text("11").await?;
    month.wait_for_inner_text("11").await?;
    month.click().await?;
    page.wait_for_focus(&month).await?;
    page.send_keys(Key::Backspace).await?;
    page.send_keys(Key::Backspace).await?;
    month.wait_for_inner_text("mm").await?;
    assert_that!(segment(page, "empty-field", "day").await?)
        .inner_text()
        .await
        .is_equal_to("dd");
    assert_that!(segment(page, "empty-field", "year").await?)
        .inner_text()
        .await
        .is_equal_to("yyyy");
    Ok(())
}

/// Fills the hidden date input of `section` as a browser's autofill does: the value, `input`,
/// `change`.
async fn fill_hidden_date_input(page: &Page<'_>, section: &str, value: &str) -> Result<(), Report> {
    let input = page
        .element(format!("#test-dp-{section} input[type=date]"))
        .await?;
    input.virtual_input(value).await?;
    input
        .dispatch(SyntheticEvent::plain(EventKind::Change))
        .await?;
    Ok(())
}

/// A hidden date input (not focusable, hidden from assistive technology, not submitted) passes
/// what the browser autofills on to the field and the picker ("should support autofill").
#[browser_test]
pub async fn autofill(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty-field", "empty"]).await?;
    let input = page
        .element("#test-dp-empty-field input[type=date]")
        .await?;
    assert_that!(input)
        .has_attribute("tabindex")
        .await
        .is_equal_to("-1");
    assert_that!(input).has_attribute("form").await.is_empty();
    let container = input.parent().await?;
    assert_that!(container)
        .has_attribute("aria-hidden")
        .await
        .is_equal_to("true");
    fill_hidden_date_input(page, "empty-field", "2000-05-30").await?;
    assert_that!(|| input_text(page, "empty-field"))
        .eventually_ok()
        .matches(eq("5/30/2000"))
        .await;

    fill_hidden_date_input(page, "empty", "2000-05-30").await?;
    wait_for_value(page, "empty", "2000-05-30").await?;
    Ok(())
}

/// A selection left inside a segment doesn't take the focus from another element ("does not
/// collapse the selection onto a segment while another element is focused").
#[browser_test]
pub async fn selection_while_elsewhere(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["keys"]).await?;
    let before = page.element("#test-dp-keys-before").await?;
    before.click().await?;
    page.wait_for_focus(&before).await?;
    let year = segment(page, "keys", "year").await?;
    page.low_level()
        .eval::<()>(
            "const [segment, before] = arguments;
         document.getSelection().collapse(segment.firstChild, 0);
         before.focus();
         document.dispatchEvent(new Event('selectionchange'));",
            vec![year.to_json()?, before.to_json()?],
        )
        .await?;
    page.focus_stays(&before, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A German date field shows day, month and year with two-digit day and month, names its
/// segments in German ("Tag") and moves on in that order while typing.
#[browser_test]
pub async fn german_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["de"]).await?;
    assert_that!(segment_types(page, "de").await?).contains_exactly(["day", "month", "year"]);
    assert_that!(
        page.first_element("#test-dp-de .leptonic-DateInput")
            .await?
    )
    .text_content()
    .await
    .map_owned(Option::unwrap_or_default)
    .is_equal_to("05.06.2024");
    let day = segment(page, "de", "day").await?;
    // Segment names follow the locale.
    assert_that!(day)
        .has_attribute("aria-label")
        .await
        .starts_with("Tag");
    day.click().await?;
    page.wait_for_focus(&day).await?;
    page.type_text("17").await?;
    let month = segment(page, "de", "month").await?;
    page.wait_for_focus(&month).await?;
    page.type_text("3").await?;
    wait_for_value(page, "de", "2024-03-17").await?;
    Ok(())
}

/// 12-hour time fields show the locale's 12-hour clock as `Intl`'s `hour12: true` does: German
/// "12\u{202f}AM" (not the day period "nachts"), also hour-only, and Japanese "午前0"; arrows cycle the
/// hour within its day period, and typing a day period's letter switches it.
#[browser_test]
pub async fn twelve_hour_clocks(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["de-12h", "de-12h-hour", "ja-12h"])
        .await?;
    for (section, hour, day_period) in [
        ("de-12h", "12", "AM"),
        ("de-12h-hour", "12", "AM"),
        ("ja-12h", "0", "午前"),
    ] {
        let hour_segment = segment(page, section, "hour").await?;
        assert_that!(hour_segment)
            .inner_text()
            .await
            .with_detail_message(format!("the hour in {section}"))
            .is_equal_to(hour);
        let day_period_segment = segment(page, section, "dayPeriod").await?;
        assert_that!(day_period_segment)
            .inner_text()
            .await
            .with_detail_message(format!("the day period in {section}"))
            .is_equal_to(day_period);
    }
    // The hour cycles within the day period of the locale's clock: German 12\u{202f}AM up to 1\u{202f}AM,
    // Japanese 0 down to 11, both still before noon.
    for (section, key, hour, day_period) in [
        ("de-12h", Key::Up, "1", "AM"),
        ("ja-12h", Key::Down, "11", "午前"),
    ] {
        let hour_segment = segment(page, section, "hour").await?;
        hour_segment.focus().await?;
        page.wait_for_focus(&hour_segment).await?;
        page.send_keys(key).await?;
        hour_segment.wait_for_inner_text(hour).await?;
        assert_that!(segment(page, section, "dayPeriod").await?)
            .inner_text()
            .await
            .with_detail_message(format!("the day period in {section}"))
            .is_equal_to(day_period);
    }
    // Typing "p" switches the German day period to PM.
    let day_period = segment(page, "de-12h", "dayPeriod").await?;
    day_period.focus().await?;
    page.wait_for_focus(&day_period).await?;
    page.type_text("p").await?;
    day_period.wait_for_inner_text("PM").await?;
    Ok(())
}

/// In a Hebrew date picker with a time, the time is isolated and the segments are embedded left
/// to right; ArrowLeft and ArrowRight move by position through the segments and to the button
/// ("DatePicker should support arrow keys to move between segments in an RTL locale").
#[browser_test]
pub async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["rtl"]).await?;
    // The time is isolated (LRI ... PDI), so that it reads hour:minute (the hour with or without
    // a leading zero).
    assert_that!(
        page.first_element("#test-dp-rtl .leptonic-DateInput")
            .await?
    )
    .text_content()
    .await
    .map_owned(|text| {
        text.unwrap_or_default()
            .replace("\u{2066}09:", "\u{2066}9:")
    })
    .contains("\u{2066}9:30\u{2069}");
    assert_that!(segment_types(page, "rtl").await?)
        .contains_exactly(["day", "month", "year", "hour", "minute"]);
    let day = segment(page, "rtl", "day").await?;
    assert_that!(day)
        .attribute("style")
        .await
        .map_owned(|style| style.unwrap_or_default().replace(' ', ""))
        .contains("direction:ltr")
        .contains("unicode-bidi:embed");

    // Arrow keys by position ("DatePicker should support arrow keys to move between segments in
    // an RTL locale", react-spectrum `DatePickerBase.test.js`): ArrowLeft walks leftwards through
    // the segments to the button, ArrowRight back.
    let button = page.element("#test-dp-rtl button").await?;
    // Focused directly, as upstream does (a click at the center of a bidi-embedded segment can
    // land on its neighbor).
    day.focus().await?;
    page.wait_for_focus(&day).await?;
    let mut left = active_left(page).await?;
    let mut steps = 0;
    while page.focused_element().await? != button {
        steps += 1;
        if steps > 6 {
            bail!("ArrowLeft didn't reach the button from the day");
        }
        page.send_keys(Key::Left).await?;
        assert_that!(|| active_left(page))
            .eventually_ok()
            .matches(lt(left))
            .await;
        left = active_left(page).await?;
    }
    page.send_keys(Key::Right).await?;
    assert_that!(|| active_left(page))
        .eventually_ok()
        .matches(gt(left))
        .await;
    Ok(())
}

/// The left edge of the focused element.
async fn active_left(page: &Page<'_>) -> Result<f64, Report> {
    Ok(page.focused_element().await?.client_rect().await?.left)
}

/// The `style` attribute of `element` without spaces.
async fn style_of(element: &WebElement) -> Result<String, Report> {
    Ok(element
        .attr("style")
        .await?
        .unwrap_or_default()
        .replace(' ', ""))
}

/// Switching the locale to a right-to-left one embeds the segments left to right (the styles
/// follow the locale).
#[browser_test]
pub async fn switching_to_right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["switch"]).await?;
    let day = segment(page, "switch", "day").await?;
    assert_that!(day)
        .attribute("style")
        .await
        .map_owned(|style| style.unwrap_or_default().replace(' ', ""))
        .does_not_contain("unicode-bidi");
    page.element("#test-dp-switch-he").await?.click().await?;
    assert_that!(|| async { style_of(&segment(page, "switch", "day").await?).await })
        .eventually_ok()
        .satisfies(|style| {
            style
                .contains("unicode-bidi:embed")
                .contains("direction:ltr");
        })
        .await;
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Structure and state (react-aria-components `DatePicker.test.js`, `DateRangePicker.test.js`;
// react-spectrum `DatePickerBase.test.js`).
// ---------------------------------------------------------------------------------------------

/// The first group (`role=group`) in `#test-dp-<section>`: the picker's.
async fn group(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.first_element(format!("#test-dp-{section} [role=group]"))
        .await
}

/// The picker element (`.leptonic-DatePicker`/`.leptonic-DateRangePicker`) in
/// `#test-dp-<section>`.
async fn picker(page: &Page<'_>, section: &str, class: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-dp-{section} .leptonic-{class}"))
        .await
}

/// The editable segments (spin buttons) in `#test-dp-<section>`, in order.
async fn spinbuttons(page: &Page<'_>, section: &str) -> Result<Vec<WebElement>, Report> {
    page.elements(format!("#test-dp-{section} [role=spinbutton]"))
        .await
}

/// The date button of the open calendar whose label starts with `label`.
async fn calendar_date(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(format!(
        "[role=dialog] [role=gridcell] > [role=button][aria-label^='{label}']"
    ))
    .await
}

/// The ids of an id reference list (`aria-labelledby`, ...).
fn ids(list: &str) -> Vec<&str> {
    list.split_whitespace().collect()
}

/// Remove bidi isolation marks from an observed DOM text value.
fn without_bidi_marks(text: String) -> String {
    text.replace(['\u{2066}', '\u{2067}', '\u{2068}', '\u{2069}'], "")
}

/// Fallible text observation for polling a field that can change between samples.
async fn text_without_marks(element: &WebElement) -> Result<String, Report> {
    Ok(without_bidi_marks(
        element.prop("textContent").await?.unwrap_or_default(),
    ))
}

/// Opens the popover of `section`'s picker with its button and waits for its calendar.
async fn open(page: &Page<'_>, section: &str) -> Result<(), Report> {
    button(page, section).await?.click().await?;
    page.element("[role=dialog] [role=grid]").await?;
    Ok(())
}

/// Checks a picker's slots: placeholder segments, the button, the label naming the group, the
/// description describing it, and the dialog named by the label.
async fn check_slots(
    page: &Page<'_>,
    section: &str,
    class: &str,
    label: &str,
) -> Result<(), Report> {
    let group = group(page, section).await?;
    let inputs = group.elements(".leptonic-DateInput").await?;
    for input in &inputs {
        assert_that!(input)
            .text_content()
            .await
            .map_owned(Option::unwrap_or_default)
            .map_owned(without_bidi_marks)
            .is_equal_to("mm/dd/yyyy".to_owned());
    }
    let open_button = button(page, section).await?;
    assert_that!(open_button)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Calendar");
    assert_that!(picker(page, section, class).await?)
        .has_attribute("data-foo")
        .await
        .is_equal_to("bar");
    let labelledby = group.attr("aria-labelledby").await?.unwrap_or_default();
    let label_element = page.element(format!("#{labelledby}")).await?;
    assert_that!(label_element)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-Label");
    assert_that!(label_element)
        .inner_text()
        .await
        .is_equal_to(label);
    assert_that!(group)
        .accessible_description()
        .await
        .is_equal_to("Description");
    for segment in spinbuttons(page, section).await? {
        assert_that!(segment)
            .has_attribute("class")
            .await
            .is_equal_to("leptonic-DateSegment");
        assert_that!(segment)
            .has_attribute("data-placeholder")
            .await
            .is_equal_to("true");
        assert_that!(segment).has_attribute("data-type").await;
    }

    open(page, section).await?;
    let dialog = page.element("[role=dialog]").await?;
    assert_that!(dialog)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-Dialog");
    assert_that!(dialog)
        .has_attribute("aria-labelledby")
        .await
        .derive_owned(|value| ids(value))
        .contains(labelledby.as_str());
    assert_that!(page.element("[role=dialog] [role=grid]").await?)
        .has_attribute("class")
        .await
        .contains("leptonic-CalendarGrid");
    Ok(())
}

/// A date picker shows placeholder segments, a "Calendar" button, a group named by its label and
/// described by its description, and a dialog named by the label ("provides slots").
#[browser_test]
pub async fn slots(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["slots"]).await?;
    check_slots(page, "slots", "DatePicker", "Birth date").await
}

/// A description after the controls registers too; its position must not change hydration.
#[browser_test]
pub async fn slots_description_after(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["slots-description-after"])
        .await?;
    check_slots(page, "slots-description-after", "DatePicker", "Birth date").await
}

/// A date range picker shows placeholder segments in both fields, a "Calendar" button, a group
/// named by its label and described by its description, and a dialog named by the label
/// ("provides slots", `DateRangePicker.test.js`).
#[browser_test]
pub async fn range_slots(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-slots"]).await?;
    check_slots(page, "range-slots", "DateRangePicker", "Trip dates").await
}

/// An attribute set on the picker lands on its outer element only ("should render data-
/// attributes only on the outer element").
#[browser_test]
pub async fn data_attributes_on_the_outer_element(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["slots", "range-slots"]).await?;
    for (section, class) in [("slots", "DatePicker"), ("range-slots", "DateRangePicker")] {
        let marked = page
            .elements(format!("#test-dp-{section} [data-foo]"))
            .await?;
        assert_that!(marked).has_length(1);
        assert_that!(&marked[0])
            .has_attribute("class")
            .await
            .contains(format!("leptonic-{class}"));
    }
    Ok(())
}

/// While the range calendar is open, the range picker's button is pressed and the picker open
/// ("should apply isPressed state to button when expanded", "should support data-open state",
/// `DateRangePicker.test.js`).
#[browser_test]
pub async fn range_pressed_while_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-slots"]).await?;
    let open_button = button(page, "range-slots").await?;
    let range_picker = picker(page, "range-slots", "DateRangePicker").await?;
    assert_that!(open_button)
        .attribute("data-pressed")
        .await
        .is_none();
    assert_that!(range_picker)
        .attribute("data-open")
        .await
        .is_none();
    open(page, "range-slots").await?;
    open_button
        .wait_for_attr("data-pressed", Some("true"))
        .await?;
    range_picker
        .wait_for_attr("data-open", Some("true"))
        .await?;
    Ok(())
}

/// A picker before its minimum and a reversed range are invalid at once with `Aria` validation:
/// `data-invalid` on the picker and its group ("should support render props").
#[browser_test]
pub async fn invalid_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["invalid"]).await?;
    for class in ["DatePicker", "DateRangePicker"] {
        let element = picker(page, "invalid", class).await?;
        element.wait_for_attr("data-invalid", Some("true")).await?;
        assert_that!(element.element(".leptonic-DatePickerGroup").await?)
            .has_attribute("data-invalid")
            .await
            .is_equal_to("true");
    }
    Ok(())
}

/// A required picker marks itself `data-required`, an optional one doesn't ("should support
/// required state", "should support required render prop").
#[browser_test]
pub async fn required_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["form-value", "slots"]).await?;
    for class in ["DatePicker", "DateRangePicker"] {
        assert_that!(picker(page, "form-value", class).await?)
            .has_attribute("data-required")
            .await
            .is_equal_to("true");
    }
    assert_that!(picker(page, "slots", "DatePicker").await?)
        .attribute("data-required")
        .await
        .is_none();
    Ok(())
}

/// The hidden inputs carry the value in ISO 8601 under the picker's name (the range's under the
/// start and end names), for the named form ("should support form value").
#[browser_test]
pub async fn form_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["form-value"]).await?;
    for (name, value) in [
        ("birthday", "2020-02-03"),
        ("start", "2023-01-10"),
        ("end", "2023-01-20"),
    ] {
        let input = page
            .element(format!(
                "#test-dp-form-value input[name={name}][form=test-dp-form-value-form]"
            ))
            .await?;
        assert_that!(input)
            .property("value")
            .await
            .is_equal_to(Some(value.to_owned()));
        assert_that!(input)
            .has_attribute("form")
            .await
            .is_equal_to("test-dp-form-value-form");
    }
    Ok(())
}

/// A required range picker shows the browser's error on both ends and focuses its first segment
/// once its form is checked; the error stays while both dates are entered and goes once the
/// focus leaves ("supports validation errors", `DateRangePicker.test.js`).
#[browser_test]
pub async fn range_validation_errors(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-required"]).await?;
    let start = page
        .element("#test-dp-range-required input[name=start]")
        .await?;
    let end = page
        .element("#test-dp-range-required input[name=end]")
        .await?;
    let group = group(page, "range-required").await?;
    let range_picker = picker(page, "range-required", "DateRangePicker").await?;
    for input in [&start, &end] {
        assert_that!(input).has_attribute("required").await;
        assert_that!(input.is_valid().await?).is_false();
    }
    assert_that!(group)
        .attribute("aria-describedby")
        .await
        .is_none();
    assert_that!(range_picker)
        .attribute("data-invalid")
        .await
        .is_none();
    let message = assert_that!(start)
        .property("validationMessage")
        .await
        .some()
        .is_not_blank()
        .actual()
        .clone();

    assert_that!(
        page.element("#test-dp-range-required-form")
            .await?
            .check_validity()
            .await?
    )
    .is_false();
    wait_for_description(&group, &message, true).await?;
    range_picker
        .wait_for_attr("data-invalid", Some("true"))
        .await?;
    page.wait_for_focus(&spinbuttons(page, "range-required").await?[0])
        .await?;

    for _ in 0..2 {
        page.send_keys(Key::Up).await?;
        page.send_keys(Key::Tab).await?;
        page.send_keys(Key::Up).await?;
        page.send_keys(Key::Tab).await?;
        page.send_keys(Key::Up).await?;
        page.send_keys(Key::Tab).await?;
    }
    assert_that!(|| start.is_valid())
        .eventually_ok()
        .matches(eq(true))
        .await;
    assert_that!(|| end.is_valid())
        .eventually_ok()
        .matches(eq(true))
        .await;
    page.element("#test-dp-range-required-after")
        .await?
        .click()
        .await?;
    wait_for_description(&group, &message, false).await?;
    range_picker.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// Selecting a range closes the range picker's popover; without close on select it stays open
/// ("should support close on select = true", "should support close on select = false",
/// `DateRangePicker.test.js`).
#[browser_test]
pub async fn range_close_on_select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-close-true", "range-close-false"])
        .await?;
    for (section, closes) in [("range-close-true", true), ("range-close-false", false)] {
        open(page, section).await?;
        let selected = page
            .first_element("[role=dialog] [role=gridcell][aria-selected=true] > [role=button]")
            .await?;
        assert_that!(selected)
            .has_attribute("aria-label")
            .await
            .starts_with("Selected Range: Tuesday, January 10, 2023 to Friday, January 20, 2023")
            .ends_with("Tuesday, January 10, 2023 selected");
        calendar_date(page, "Wednesday, January 11, 2023")
            .await?
            .click()
            .await?;
        calendar_date(page, "Thursday, January 12, 2023")
            .await?
            .click()
            .await?;
        wait_for_value(page, section, "2023-01-11 - 2023-01-12").await?;
        if closes {
            page.wait_for_count("[role=dialog]", 0).await?;
        } else {
            page.count_stays("[role=dialog]", 1, std::time::Duration::from_millis(100))
                .await?;
            page.send_keys(Key::Escape).await?;
            page.wait_for_count("[role=dialog]", 0).await?;
        }
    }
    Ok(())
}

/// A disabled range picker disables its button, group and segments ("should disable button and
/// date input when DatePicker is disabled", `DateRangePicker.test.js`).
#[browser_test]
pub async fn range_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-disabled"]).await?;
    assert_that!(button(page, "range-disabled").await?)
        .enabled()
        .await
        .is_false();
    assert_that!(group(page, "range-disabled").await?)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    let segments = spinbuttons(page, "range-disabled").await?;
    assert_that!(segments).has_length(6);
    for segment in segments {
        assert_that!(segment)
            .has_attribute("aria-disabled")
            .await
            .is_equal_to("true");
        assert_that!(segment)
            .attribute("contenteditable")
            .await
            .is_none();
        assert_that!(segment).attribute("inputmode").await.is_none();
        assert_that!(segment).attribute("tabindex").await.is_none();
    }
    Ok(())
}

/// A label and a button inside the popover are not the picker's: the label has no id of the
/// picker's and the button no `aria-expanded` ("should clear contexts inside popover").
async fn check_cleared_contexts(page: &Page<'_>, section: &str) -> Result<(), Report> {
    let picker_label = page
        .element(format!("#test-dp-{section} .leptonic-Label"))
        .await?;
    assert_that!(picker_label)
        .has_attribute("id")
        .await
        .is_not_empty();
    open(page, section).await?;
    let label = page.element("[role=dialog] .leptonic-Label").await?;
    assert_that!(label).attribute("id").await.is_none();
    let popover_button = page.element(css("[role=dialog] button").text("Hi")).await?;
    assert_that!(popover_button)
        .attribute("aria-expanded")
        .await
        .is_none();
    Ok(())
}

/// A label and a button inside a date picker's popover are not the picker's ("should clear
/// contexts inside popover").
#[browser_test]
pub async fn clear_contexts(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["clear-contexts"]).await?;
    check_cleared_contexts(page, "clear-contexts").await
}

/// A label and a button inside a date range picker's popover are not the picker's ("should clear
/// contexts inside popover", `DateRangePicker.test.js`).
#[browser_test]
pub async fn range_clear_contexts(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-clear-contexts"]).await?;
    check_cleared_contexts(page, "range-clear-contexts").await
}

/// Checks a segment's text and spin button values.
async fn check_segment(
    segment: &WebElement,
    text: &str,
    label: &str,
    now: Option<&str>,
    value_text: &str,
    min_max: Option<(&str, &str)>,
) -> Result<(), Report> {
    assert_that!(segment)
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to(text.to_owned());
    assert_that!(segment)
        .has_attribute("aria-label")
        .await
        .is_equal_to(label);
    assert_that!(segment)
        .attribute("aria-valuenow")
        .await
        .derive_owned(|value| value.as_deref())
        .is_equal_to(now);
    assert_that!(segment)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to(value_text);
    if let Some((min, max)) = min_max {
        assert_that!(segment)
            .has_attribute("aria-valuemin")
            .await
            .is_equal_to(min);
        assert_that!(segment)
            .has_attribute("aria-valuemax")
            .await
            .is_equal_to(max);
    }
    Ok(())
}

/// A picker on February 3, 2019 is a valid, enabled group of month, day and year spin buttons
/// with their values, value texts and limits ("should render a datepicker with a specified
/// date").
#[browser_test]
pub async fn specified_date(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["close-true"]).await?;
    let group = group(page, "close-true").await?;
    assert_that!(group)
        .attribute("aria-disabled")
        .await
        .is_none();
    assert_that!(group)
        .attribute("aria-invalid")
        .await
        .is_none();
    let segments = spinbuttons(page, "close-true").await?;
    assert_that!(segments).has_length(3);
    check_segment(
        &segments[0],
        "2",
        "month, ",
        Some("2"),
        "2 – February",
        Some(("1", "12")),
    )
    .await?;
    check_segment(
        &segments[1],
        "3",
        "day, ",
        Some("3"),
        "3",
        Some(("1", "31")),
    )
    .await?;
    check_segment(
        &segments[2],
        "2019",
        "year, ",
        Some("2019"),
        "2019",
        Some(("1", "9999")),
    )
    .await?;
    Ok(())
}

/// A picker of dates and times to the second shows seven spin buttons: the date, the hour of a
/// 12-hour clock, minutes, seconds and the day period ("should render a datepicker with
/// granularity=\"second\"").
#[browser_test]
pub async fn granularity_second(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["second"]).await?;
    let segments = spinbuttons(page, "second").await?;
    assert_that!(segments).has_length(7);
    check_segment(
        &segments[0],
        "2",
        "month, ",
        Some("2"),
        "2 – February",
        Some(("1", "12")),
    )
    .await?;
    check_segment(
        &segments[1],
        "3",
        "day, ",
        Some("3"),
        "3",
        Some(("1", "31")),
    )
    .await?;
    check_segment(
        &segments[2],
        "2019",
        "year, ",
        Some("2019"),
        "2019",
        Some(("1", "9999")),
    )
    .await?;
    check_segment(
        &segments[3],
        "12",
        "hour, ",
        Some("12"),
        "12\u{202f}AM",
        Some(("1", "12")),
    )
    .await?;
    check_segment(
        &segments[4],
        "00",
        "minute, ",
        Some("0"),
        "00",
        Some(("0", "59")),
    )
    .await?;
    check_segment(
        &segments[5],
        "00",
        "second, ",
        Some("0"),
        "00",
        Some(("0", "59")),
    )
    .await?;
    assert_that!(&segments[6])
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to("AM".to_owned());
    assert_that!(&segments[6])
        .has_attribute("aria-label")
        .await
        .is_equal_to("AM/PM, ");
    assert_that!(&segments[6])
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("AM");
    Ok(())
}

/// Pickers render enabled, editable numeric segments and a focusable button ("should render a
/// default datepicker", react-spectrum `DatePickerBase.test.js`).
#[browser_test]
pub async fn default_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty", "range-focus"]).await?;
    let groups = page
        .elements("#test-dp-empty .leptonic-DatePickerGroup, #test-dp-range-focus .leptonic-DatePickerGroup")
        .await?;
    assert_that!(groups).has_length(2);
    for (group, count) in groups.iter().zip([3, 6]) {
        assert_that!(group)
            .attribute("aria-disabled")
            .await
            .is_none();
        assert_that!(group)
            .attribute("aria-invalid")
            .await
            .is_none();
        let segments = group.elements("[role=spinbutton]").await?;
        assert_that!(segments).has_length(count);
        for segment in segments {
            assert_that!(segment)
                .attribute("aria-disabled")
                .await
                .is_none();
            assert_that!(segment)
                .has_attribute("contenteditable")
                .await
                .is_equal_to("true");
            assert_that!(segment)
                .has_attribute("inputmode")
                .await
                .is_equal_to("numeric");
            assert_that!(segment)
                .attribute("aria-readonly")
                .await
                .is_none();
        }
        assert_that!(group.element("button").await?)
            .has_attribute("tabindex")
            .await
            .is_equal_to("0");
    }
    Ok(())
}

/// Read-only pickers mark their segments `aria-readonly` (not the group) and disable their
/// button ("should set aria-readonly when isReadOnly").
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["read-only", "range-read-only"])
        .await?;
    for section in ["read-only", "range-read-only"] {
        assert_that!(group(page, section).await?)
            .attribute("aria-readonly")
            .await
            .is_none();
        for segment in spinbuttons(page, section).await? {
            assert_that!(segment)
                .has_attribute("aria-readonly")
                .await
                .is_equal_to("true");
        }
        assert_that!(button(page, section).await?)
            .enabled()
            .await
            .is_false();
    }
    Ok(())
}

/// Required and invalid pickers mark their segments `aria-required` and `aria-invalid`, not the
/// group ("should set aria-required when isRequired", "should set aria-invalid when
/// validationState=\"invalid\"").
#[browser_test]
pub async fn required_and_invalid_segments(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["form-value", "error"]).await?;
    assert_that!(group(page, "form-value").await?)
        .attribute("aria-required")
        .await
        .is_none();
    for segment in spinbuttons(page, "form-value").await? {
        assert_that!(segment)
            .has_attribute("aria-required")
            .await
            .is_equal_to("true");
    }
    assert_that!(group(page, "error").await?)
        .attribute("aria-invalid")
        .await
        .is_none();
    for segment in spinbuttons(page, "error").await? {
        assert_that!(segment)
            .has_attribute("aria-invalid")
            .await
            .is_equal_to("true");
    }
    Ok(())
}

/// The time zone of a zoned placeholder shows as a read-only text box, not editable ("should set
/// aria-readonly on non-editable segments").
#[browser_test]
pub async fn read_only_time_zone(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["zoned-placeholder"]).await?;
    let zones = page
        .elements("#test-dp-zoned-placeholder [data-type=timeZoneName]")
        .await?;
    assert_that!(zones).has_length(3);
    for zone in zones {
        assert_that!(zone)
            .has_attribute("role")
            .await
            .is_equal_to("textbox");
        assert_that!(zone)
            .has_attribute("aria-readonly")
            .await
            .is_equal_to("true");
        assert_that!(&zone)
            .text_content()
            .await
            .map_owned(Option::unwrap_or_default)
            .map_owned(without_bidi_marks)
            .is_equal_to("PDT".to_owned());
        assert_that!(zone)
            .attribute("contenteditable")
            .await
            .is_none();
    }
    Ok(())
}

/// The calendar opens on the placeholder's month with its date focused ("should focus
/// placeholderValue in calendar").
#[browser_test]
pub async fn placeholder_focused_in_the_calendar(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["placeholder", "range-placeholder"])
        .await?;
    for section in ["placeholder", "range-placeholder"] {
        open(page, section).await?;
        assert_that!(page.element("[role=dialog] [role=grid]").await?)
            .has_attribute("aria-label")
            .await
            .is_equal_to("June 2019");
        page.wait_for_focus(&calendar_date(page, "Wednesday, June 5, 2019").await?)
            .await?;
        page.send_keys(Key::Escape).await?;
        page.wait_for_count("[role=dialog]", 0).await?;
    }
    Ok(())
}

/// The calendar opens on the selected date, not the placeholder ("should focus selected date over
/// placeholderValue").
#[browser_test]
pub async fn selected_date_focused_over_the_placeholder(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["selected"]).await?;
    let buttons = page
        .elements("#test-dp-selected button[aria-haspopup=dialog]")
        .await?;
    for open_button in buttons {
        open_button.click().await?;
        assert_that!(|| async {
            Ok::<_, Report>(
                page.element("[role=dialog] [role=grid]")
                    .await?
                    .attr("aria-label")
                    .await?,
            )
        })
        .eventually_ok()
        .matches(eq(Some("July 2019".to_owned())))
        .await;
        assert_that!(|| async {
            Ok::<_, Report>(page.focused_element().await?.attr("aria-label").await?)
        })
        .eventually_ok()
        .satisfies(|label| {
            label.is_some_satisfying(|label| {
                label.contains("Friday, July 5, 2019");
            });
        })
        .await;
        page.send_keys(Key::Escape).await?;
        page.wait_for_count("[role=dialog]", 0).await?;
    }
    Ok(())
}

/// With forced leading zeros, months and days show two digits ("should support
/// shouldForceLeadingZeros").
#[browser_test]
pub async fn forced_leading_zeros(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["leading-zeros"]).await?;
    for segment in page
        .elements(
            "#test-dp-leading-zeros [data-type=month], #test-dp-leading-zeros [data-type=day]",
        )
        .await?
    {
        assert_that!(&segment)
            .text_content()
            .await
            .map_owned(Option::unwrap_or_default)
            .map_owned(without_bidi_marks)
            .starts_with("0");
    }
    Ok(())
}

/// The button controls the dialog while it is open; opening moves the focus to a date of the
/// calendar; segments have no popup attributes ("should open a calendar popover when clicking
/// the button").
#[browser_test]
pub async fn button_controls_the_dialog(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty", "range-focus"]).await?;
    let buttons = page
        .elements("#test-dp-empty button[aria-haspopup=dialog], #test-dp-range-focus button[aria-haspopup=dialog]")
        .await?;
    for open_button in buttons {
        assert_that!(open_button)
            .has_attribute("aria-expanded")
            .await
            .is_equal_to("false");
        assert_that!(open_button)
            .attribute("aria-controls")
            .await
            .is_none();
        open_button.click().await?;
        let dialog = page.element("[role=dialog]").await?;
        let dialog_id = assert_that!(dialog)
            .has_attribute("id")
            .await
            .is_not_empty()
            .actual()
            .clone();
        open_button
            .wait_for_attr("aria-expanded", Some("true"))
            .await?;
        open_button
            .wait_for_attr("aria-controls", Some(dialog_id.as_str()))
            .await?;
        assert_that!(|| async {
            Ok::<_, Report>(
                page.focused_element()
                    .await?
                    .parent()
                    .await?
                    .attr("role")
                    .await?,
            )
        })
        .eventually_ok()
        .matches(eq(Some("gridcell".to_owned())))
        .await;
        page.send_keys(Key::Escape).await?;
        page.wait_for_count("[role=dialog]", 0).await?;
        open_button.wait_for_attr("aria-controls", None).await?;
    }
    for segment in page
        .elements("#test-dp-empty [role=spinbutton], #test-dp-range-focus [role=spinbutton]")
        .await?
    {
        assert_that!(segment)
            .attribute("aria-expanded")
            .await
            .is_none();
        assert_that!(segment)
            .attribute("aria-controls")
            .await
            .is_none();
    }
    Ok(())
}

/// Alt+ArrowDown in a range picker's field opens its calendar, focusing a date, and the button
/// then controls the dialog ("should open a calendar popover pressing Alt + ArrowDown on the
/// keyboard").
#[browser_test]
pub async fn range_opens_by_alt_arrow_down(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-placeholder"]).await?;
    spinbuttons(page, "range-placeholder").await?[0]
        .focus()
        .await?;
    page.send_keys(Key::Alt + Key::Down).await?;
    let dialog = page.element("[role=dialog]").await?;
    let dialog_id = dialog.id().await?.unwrap_or_default();
    let open_button = button(page, "range-placeholder").await?;
    open_button
        .wait_for_attr("aria-expanded", Some("true"))
        .await?;
    open_button
        .wait_for_attr("aria-controls", Some(dialog_id.as_str()))
        .await?;
    page.wait_for_focus(&calendar_date(page, "Wednesday, June 5, 2019").await?)
        .await?;
    Ok(())
}

/// ArrowRight moves through every segment of the picker on to its button, ArrowLeft back ("should
/// support arrow keys to move between segments").
#[browser_test]
pub async fn arrow_keys_between_segments(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty", "range-focus"]).await?;
    for group in page
        .elements("#test-dp-empty .leptonic-DatePickerGroup, #test-dp-range-focus .leptonic-DatePickerGroup")
        .await?
    {
        let segments = group.elements("[role=spinbutton]").await?;
        let open_button = group.element("button").await?;
        segments[0].focus().await?;
        for segment in &segments {
            page.wait_for_focus(segment).await?;
            page.send_keys(Key::Right).await?;
        }
        page.wait_for_focus(&open_button).await?;
        page.send_keys(Key::Left).await?;
        for segment in segments.iter().rev() {
            page.wait_for_focus(segment).await?;
            page.send_keys(Key::Left).await?;
        }
    }
    Ok(())
}

/// A mouse press on a literal focuses the segment before it ("should focus the previous segment
/// on mouse down on a literal segment").
#[browser_test]
pub async fn press_on_a_literal(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty", "range-focus"]).await?;
    for group in page
        .elements("#test-dp-empty .leptonic-DatePickerGroup, #test-dp-range-focus .leptonic-DatePickerGroup")
        .await?
    {
        let literal = group.first_element(css("[data-type=literal]").text("/")).await?;
        literal
            .dispatch(SyntheticEvent::pointer(PointerKind::Down))
            .await?;
        page.wait_for_focus(&group.first_element("[role=spinbutton]").await?)
            .await?;
        literal
            .dispatch(SyntheticEvent::pointer(PointerKind::Up))
            .await?;
        page.blur_focused().await?;
    }
    Ok(())
}

/// A picker with auto focus focuses its first segment once mounted ("should focus the first
/// segment by default if autoFocus is set", "should support autoFocus").
#[browser_test]
pub async fn auto_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["auto-focus"]).await?;
    page.wait_for_focus(&spinbuttons(page, "auto-focus").await?[0])
        .await?;
    Ok(())
}

/// A range picker with auto focus focuses the start's first segment once mounted ("should focus
/// the first segment by default if autoFocus is set", `DateRangePicker`).
#[browser_test]
pub async fn range_auto_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-auto-focus"]).await?;
    page.wait_for_focus(&spinbuttons(page, "range-auto-focus").await?[0])
        .await?;
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Focus changes (react-spectrum `DatePicker.test.js`, "events").
// ---------------------------------------------------------------------------------------------

/// The log of the focus changes of `#test-dp-events`' picker.
async fn events_log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-dp-events-log").await
}

/// Focus entering the picker reports a focus change once; moving between segments and opening the
/// popover report none ("should focus field, move a segment, and open popover and does not
/// blur").
#[browser_test]
pub async fn focus_change_within_the_picker(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["events"]).await?;
    let log = events_log(page).await?;
    assert_that!(log).inner_text().await.is_equal_to("");
    page.element("#test-dp-events-before")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    let segments = spinbuttons(page, "events").await?;
    page.wait_for_focus(&segments[0]).await?;
    log.wait_for_inner_text("focus:true").await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&segments[1]).await?;
    button(page, "events").await?.click().await?;
    page.element("[role=dialog] [role=grid]").await?;
    log.inner_text_stays("focus:true", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Focus leaving the picker reports a focus change to `false` ("should focus field and leave to
/// blur").
#[browser_test]
pub async fn focus_change_when_leaving(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["events"]).await?;
    let log = events_log(page).await?;
    page.element("#test-dp-events-before")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&spinbuttons(page, "events").await?[0])
        .await?;
    log.wait_for_inner_text("focus:true").await?;
    page.element("#test-dp-events-after").await?.click().await?;
    log.wait_for_inner_text("focus:true focus:false").await?;
    Ok(())
}

/// Opening the popover with the button reports the focus entering the picker ("should open
/// popover and call picker onFocus").
#[browser_test]
pub async fn focus_change_when_opening(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["events"]).await?;
    let log = events_log(page).await?;
    open(page, "events").await?;
    log.wait_for_inner_text("focus:true").await?;
    log.inner_text_stays("focus:true", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Closing the popover with Escape returns the focus to the button without a focus change; only
/// leaving the picker reports one ("should open and close popover and only call blur when focus
/// leaves picker").
#[browser_test]
pub async fn focus_change_after_closing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["events"]).await?;
    let log = events_log(page).await?;
    open(page, "events").await?;
    log.wait_for_inner_text("focus:true").await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    let open_button = button(page, "events").await?;
    page.wait_for_focus(&open_button).await?;
    log.inner_text_stays("focus:true", std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-dp-events-after").await?)
        .await?;
    log.wait_for_inner_text("focus:true focus:false").await?;
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Times in the popover (react-spectrum `DatePicker.test.js` and `DateRangePicker.test.js`,
// "calendar popover"), through the pickers' state contexts.
// ---------------------------------------------------------------------------------------------

/// The `index`th date input of `section` (`0`: a range's start, `1`: its end).
async fn field_input(page: &Page<'_>, section: &str, index: usize) -> Result<WebElement, Report> {
    Ok(page
        .elements(format!("#test-dp-{section} .leptonic-DateInput"))
        .await?
        .swap_remove(index))
}

/// The text of `section`'s `index`th date input, for repeated observations.
async fn field_text(page: &Page<'_>, section: &str, index: usize) -> Result<String, Report> {
    text_without_marks(&field_input(page, section, index).await?).await
}

/// Waits until the text of `section`'s `index`th date input is `expected`.
async fn wait_for_field_text(
    page: &Page<'_>,
    section: &str,
    index: usize,
    expected: &str,
) -> Result<(), Report> {
    assert_that!(|| field_text(page, section, index))
        .eventually_ok()
        .matches(eq(expected.to_owned()))
        .await;
    Ok(())
}

/// The time field labelled `label` in the open popover.
async fn time_field(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(css("[role=dialog] .leptonic-TimeField").has(css(".leptonic-Label").text(label)))
        .await
}

/// The segment of `kind` of `field`.
async fn time_segment(field: &WebElement, kind: &str) -> Result<WebElement, Report> {
    field.element(format!("[data-type={kind}]")).await
}

/// The fixture's count of changes of `section`'s picker.
async fn changes(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-dp-{section}-changes")).await
}

/// Clicks the page margin outside the modal popover, which closes it.
async fn click_outside(page: &Page<'_>, _section: &str) -> Result<(), Report> {
    page.click_at(1, 1).await?;
    page.wait_for_count("[role=dialog]", 0).await
}

/// A picker of dates and times shows a time field in its popover: selecting a date keeps it open
/// and the time, the time field's hour changes the value's ("should display a time field when a
/// CalendarDateTime value is used").
#[browser_test]
pub async fn time_field_in_the_popover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["date-time"]).await?;
    assert_that!(field_input(page, "date-time", 0).await?)
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to("2/3/2019, 10:45\u{202f}AM".to_owned());
    open(page, "date-time").await?;
    let selected = page
        .element("[role=dialog] [role=gridcell][aria-selected=true] > [role=button]")
        .await?;
    assert_that!(selected)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Sunday, February 3, 2019 selected");
    let time = time_field(page, "Time").await?;
    assert_that!(&time.element(".leptonic-DateInput").await?)
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to("10:45\u{202f}AM".to_owned());

    calendar_date(page, "Monday, February 4, 2019")
        .await?
        .click()
        .await?;
    wait_for_value(page, "date-time", "2019-02-04T10:45:00").await?;
    changes(page, "date-time")
        .await?
        .wait_for_inner_text("1")
        .await?;
    wait_for_field_text(page, "date-time", 0, "2/4/2019, 10:45\u{202f}AM").await?;
    page.count_stays("[role=dialog]", 1, std::time::Duration::from_millis(100))
        .await?;

    let hour = time_segment(&time, "hour").await?;
    assert_that!(hour)
        .has_attribute("role")
        .await
        .is_equal_to("spinbutton");
    assert_that!(hour)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("10\u{202f}AM");
    hour.focus().await?;
    page.wait_for_focus(&hour).await?;
    page.send_keys(Key::Up).await?;
    hour.wait_for_attr("aria-valuetext", Some("11\u{202f}AM"))
        .await?;
    wait_for_value(page, "date-time", "2019-02-04T11:45:00").await?;
    changes(page, "date-time")
        .await?
        .wait_for_inner_text("2")
        .await?;
    wait_for_field_text(page, "date-time", 0, "2/4/2019, 11:45\u{202f}AM").await?;
    assert_that!(page.count("[role=dialog]").await?).is_equal_to(1);
    Ok(())
}

/// Deleting the hour of the popover's time field digit by digit changes the value to 1\u{202f}AM, then
/// leaves the hour empty without a change ("should not throw error when deleting values from
/// time field when CalendarDateTime value is used").
#[browser_test]
pub async fn deleting_in_the_popovers_time_field(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["date-time"]).await?;
    open(page, "date-time").await?;
    calendar_date(page, "Monday, February 4, 2019")
        .await?
        .click()
        .await?;
    changes(page, "date-time")
        .await?
        .wait_for_inner_text("1")
        .await?;
    let hour = time_segment(&time_field(page, "Time").await?, "hour").await?;
    hour.focus().await?;
    page.wait_for_focus(&hour).await?;
    page.send_keys(Key::Backspace).await?;
    hour.wait_for_attr("aria-valuetext", Some("1\u{202f}AM"))
        .await?;
    page.send_keys(Key::Backspace).await?;
    hour.wait_for_attr("aria-valuetext", Some("Empty")).await?;
    wait_for_value(page, "date-time", "2019-02-04T01:45:00").await?;
    changes(page, "date-time")
        .await?
        .inner_text_stays("2", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(field_input(page, "date-time", 0).await?)
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to("2/4/2019, 1:45\u{202f}AM".to_owned());
    assert_that!(page.count("[role=dialog]").await?).is_equal_to(1);
    Ok(())
}

/// An empty picker with a time changes only once both a date and a full time are selected in its
/// popover ("should fire onChange until both date and time are selected").
#[browser_test]
pub async fn change_once_date_and_time_are_selected(page: &Page<'_>) -> Result<(), Report> {
    let section = "date-time-empty";
    page.goto_sections(PATH, &[section]).await?;
    assert_that!(field_input(page, section, 0).await?)
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to("mm/dd/yyyy, ––:––\u{202f}AM".to_owned());
    open(page, section).await?;
    assert_that!(
        page.count("[role=dialog] [role=gridcell][aria-selected=true]")
            .await?
    )
    .is_equal_to(0);
    let time = time_field(page, "Time").await?;
    assert_that!(&time.element(".leptonic-DateInput").await?)
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to("––:––\u{202f}AM".to_owned());

    calendar_date(page, "Monday, February 4, 2019")
        .await?
        .click()
        .await?;
    page.wait_for_count("[role=dialog] [role=gridcell][aria-selected=true]", 1)
        .await?;
    let hour = time_segment(&time, "hour").await?;
    assert_that!(hour)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Empty");
    hour.focus().await?;
    page.wait_for_focus(&hour).await?;
    page.send_keys(Key::Up).await?;
    hour.wait_for_attr("aria-valuetext", Some("12\u{202f}AM"))
        .await?;
    changes(page, section)
        .await?
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(field_input(page, section, 0).await?)
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to("mm/dd/yyyy, ––:––\u{202f}AM".to_owned());

    page.send_keys(Key::Right).await?;
    let minute = time_segment(&time, "minute").await?;
    page.wait_for_focus(&minute).await?;
    assert_that!(minute)
        .has_attribute("aria-label")
        .await
        .is_equal_to("minute, ");
    assert_that!(minute)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Empty");
    page.send_keys(Key::Up).await?;
    minute.wait_for_attr("aria-valuetext", Some("00")).await?;
    changes(page, section)
        .await?
        .wait_for_inner_text("1")
        .await?;
    wait_for_value(page, section, "2019-02-04T00:00:00").await?;
    wait_for_field_text(page, section, 0, "2/4/2019, 12:00\u{202f}AM").await?;

    page.send_keys(Key::Right).await?;
    let day_period = time_segment(&time, "dayPeriod").await?;
    page.wait_for_focus(&day_period).await?;
    assert_that!(day_period)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("AM");
    assert_that!(page.count("[role=dialog]").await?).is_equal_to(1);
    assert_that!(changes(page, section).await?)
        .inner_text()
        .await
        .is_equal_to("1");
    Ok(())
}

/// Closing the popover after selecting only a date commits it with the placeholder's time
/// ("should confirm time placeholder on blur if date is selected").
#[browser_test]
pub async fn closing_confirms_the_placeholder_time(page: &Page<'_>) -> Result<(), Report> {
    let section = "date-time-empty";
    page.goto_sections(PATH, &[section]).await?;
    open(page, section).await?;
    calendar_date(page, "Monday, February 4, 2019")
        .await?
        .click()
        .await?;
    page.wait_for_count("[role=dialog] [role=gridcell][aria-selected=true]", 1)
        .await?;
    assert_that!(changes(page, section).await?)
        .inner_text()
        .await
        .is_equal_to("0");
    click_outside(page, section).await?;
    changes(page, section)
        .await?
        .wait_for_inner_text("1")
        .await?;
    wait_for_value(page, section, "2019-02-04T00:00:00").await?;
    wait_for_field_text(page, section, 0, "2/4/2019, 12:00\u{202f}AM").await?;
    Ok(())
}

/// Closing the popover after entering only a time commits nothing ("should not confirm on blur if
/// date is not selected").
#[browser_test]
pub async fn closing_without_a_date_commits_nothing(page: &Page<'_>) -> Result<(), Report> {
    let section = "date-time-empty";
    page.goto_sections(PATH, &[section]).await?;
    open(page, section).await?;
    let hour = time_segment(&time_field(page, "Time").await?, "hour").await?;
    assert_that!(hour)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("Empty");
    hour.focus().await?;
    page.wait_for_focus(&hour).await?;
    page.send_keys(Key::Up).await?;
    hour.wait_for_attr("aria-valuetext", Some("12\u{202f}AM"))
        .await?;
    click_outside(page, section).await?;
    changes(page, section)
        .await?
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(value(page, section).await?)
        .inner_text()
        .await
        .is_equal_to("none");
    Ok(())
}

/// A date and a time entered in the popover are committed as they become complete, and kept
/// when it closes ("should confirm valid date time on dialog close").
#[browser_test]
pub async fn closing_keeps_a_valid_date_time(page: &Page<'_>) -> Result<(), Report> {
    let section = "date-time-empty";
    page.goto_sections(PATH, &[section]).await?;
    open(page, section).await?;
    calendar_date(page, "Monday, February 4, 2019")
        .await?
        .click()
        .await?;
    let time = time_field(page, "Time").await?;
    let hour = time_segment(&time, "hour").await?;
    hour.focus().await?;
    page.wait_for_focus(&hour).await?;
    page.send_keys(Key::Up).await?;
    hour.wait_for_attr("aria-valuetext", Some("12\u{202f}AM"))
        .await?;
    let minute = time_segment(&time, "minute").await?;
    minute.focus().await?;
    page.wait_for_focus(&minute).await?;
    page.send_keys(Key::Up).await?;
    minute.wait_for_attr("aria-valuetext", Some("00")).await?;
    page.send_keys(Key::Up).await?;
    minute.wait_for_attr("aria-valuetext", Some("01")).await?;
    click_outside(page, section).await?;
    changes(page, section)
        .await?
        .wait_for_inner_text("2")
        .await?;
    wait_for_value(page, section, "2019-02-04T00:01:00").await?;
    wait_for_field_text(page, section, 0, "2/4/2019, 12:01\u{202f}AM").await?;
    Ok(())
}

/// A controlled value set to `None` clears the field, the calendar's selection and the popover's
/// time field ("should clear date and time when controlled value is set to null").
#[browser_test]
pub async fn clearing_the_value_clears_date_and_time(page: &Page<'_>) -> Result<(), Report> {
    let section = "date-time-empty";
    page.goto_sections(PATH, &[section]).await?;
    open(page, section).await?;
    calendar_date(page, "Monday, February 4, 2019")
        .await?
        .click()
        .await?;
    let time = time_field(page, "Time").await?;
    let hour = time_segment(&time, "hour").await?;
    hour.focus().await?;
    page.wait_for_focus(&hour).await?;
    page.send_keys(Key::Up).await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Up).await?;
    wait_for_value(page, section, "2019-02-04T00:00:00").await?;
    click_outside(page, section).await?;
    wait_for_field_text(page, section, 0, "2/4/2019, 12:00\u{202f}AM").await?;

    page.element("#test-dp-date-time-empty-clear")
        .await?
        .click()
        .await?;
    wait_for_value(page, section, "none").await?;
    wait_for_field_text(page, section, 0, "mm/dd/yyyy, ––:––\u{202f}AM").await?;
    open(page, section).await?;
    assert_that!(
        page.count("[role=dialog] [role=gridcell][aria-selected=true]")
            .await?
    )
    .is_equal_to(0);
    let time = time_field(page, "Time").await?;
    assert_that!(&time.element(".leptonic-DateInput").await?)
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to("––:––\u{202f}AM".to_owned());
    Ok(())
}

/// A range picker of dates and times shows start and end time fields in its popover: selecting a
/// range keeps it open and the times, and each time field changes its end's time ("should
/// display time fields when a CalendarDateTime value is used").
#[browser_test]
pub async fn range_time_fields_in_the_popover(page: &Page<'_>) -> Result<(), Report> {
    let section = "range-times";
    page.goto_sections(PATH, &[section]).await?;
    assert_that!(field_input(page, section, 0).await?)
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to("2/3/2019, 8:45\u{202f}AM".to_owned());
    assert_that!(field_input(page, section, 1).await?)
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to("5/6/2019, 10:45\u{202f}AM".to_owned());
    open(page, section).await?;
    let start_time = time_field(page, "Start time").await?;
    let end_time = time_field(page, "End time").await?;
    assert_that!(&start_time.element(".leptonic-DateInput").await?)
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to("8:45\u{202f}AM".to_owned());
    assert_that!(&end_time.element(".leptonic-DateInput").await?)
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to("10:45\u{202f}AM".to_owned());

    calendar_date(page, "Sunday, February 10, 2019")
        .await?
        .click()
        .await?;
    calendar_date(page, "Sunday, February 17, 2019")
        .await?
        .click()
        .await?;
    wait_for_value(page, section, "2019-02-10T08:45:00 - 2019-02-17T10:45:00").await?;
    changes(page, section)
        .await?
        .wait_for_inner_text("1")
        .await?;
    assert_that!(page.count("[role=dialog]").await?).is_equal_to(1);

    let start_hour = time_segment(&start_time, "hour").await?;
    assert_that!(start_hour)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("8\u{202f}AM");
    start_hour.focus().await?;
    page.wait_for_focus(&start_hour).await?;
    page.send_keys(Key::Up).await?;
    start_hour
        .wait_for_attr("aria-valuetext", Some("9\u{202f}AM"))
        .await?;
    wait_for_value(page, section, "2019-02-10T09:45:00 - 2019-02-17T10:45:00").await?;
    changes(page, section)
        .await?
        .wait_for_inner_text("2")
        .await?;

    let end_hour = time_segment(&end_time, "hour").await?;
    end_hour.focus().await?;
    page.wait_for_focus(&end_hour).await?;
    page.send_keys(Key::Up).await?;
    end_hour
        .wait_for_attr("aria-valuetext", Some("11\u{202f}AM"))
        .await?;
    wait_for_value(page, section, "2019-02-10T09:45:00 - 2019-02-17T11:45:00").await?;
    changes(page, section)
        .await?
        .wait_for_inner_text("3")
        .await?;
    wait_for_field_text(page, section, 1, "2/17/2019, 11:45\u{202f}AM").await?;
    assert_that!(page.count("[role=dialog]").await?).is_equal_to(1);
    Ok(())
}

/// An empty range picker with times changes only once a range and both times are complete
/// ("should not fire onChange until both date range and time range are selected").
#[browser_test]
pub async fn range_change_once_dates_and_times_are_selected(page: &Page<'_>) -> Result<(), Report> {
    let section = "range-times-empty";
    page.goto_sections(PATH, &[section]).await?;
    open(page, section).await?;
    calendar_date(page, "Friday, February 1, 2019")
        .await?
        .click()
        .await?;
    calendar_date(page, "Saturday, February 2, 2019")
        .await?
        .click()
        .await?;
    page.count_stays("[role=dialog]", 1, std::time::Duration::from_millis(100))
        .await?;
    assert_that!(changes(page, section).await?)
        .inner_text()
        .await
        .is_equal_to("0");
    for (index, label) in ["Start time", "End time"].into_iter().enumerate() {
        let time = time_field(page, label).await?;
        let hour = time_segment(&time, "hour").await?;
        hour.focus().await?;
        page.wait_for_focus(&hour).await?;
        page.send_keys(Key::Up).await?;
        hour.wait_for_attr("aria-valuetext", Some("12\u{202f}AM"))
            .await?;
        page.send_keys(Key::Right).await?;
        let minute = time_segment(&time, "minute").await?;
        page.wait_for_focus(&minute).await?;
        page.send_keys(Key::Up).await?;
        minute.wait_for_attr("aria-valuetext", Some("00")).await?;
        if index == 0 {
            changes(page, section)
                .await?
                .inner_text_stays("0", std::time::Duration::from_millis(100))
                .await?;
        } else {
            changes(page, section)
                .await?
                .wait_for_inner_text("1")
                .await?;
        }
    }
    wait_for_value(page, section, "2019-02-01T00:00:00 - 2019-02-02T00:00:00").await?;
    wait_for_field_text(page, section, 0, "2/1/2019, 12:00\u{202f}AM").await?;
    wait_for_field_text(page, section, 1, "2/2/2019, 12:00\u{202f}AM").await?;
    Ok(())
}

/// Closing a range picker's popover after entering only a time commits nothing ("should not
/// confirm on blur if date range is not selected").
#[browser_test]
pub async fn range_closing_without_dates_commits_nothing(page: &Page<'_>) -> Result<(), Report> {
    let section = "range-times-empty";
    page.goto_sections(PATH, &[section]).await?;
    open(page, section).await?;
    let hour = time_segment(&time_field(page, "Start time").await?, "hour").await?;
    hour.focus().await?;
    page.wait_for_focus(&hour).await?;
    page.send_keys(Key::Up).await?;
    hour.wait_for_attr("aria-valuetext", Some("12\u{202f}AM"))
        .await?;
    click_outside(page, section).await?;
    changes(page, section)
        .await?
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A controlled range set to `None` clears both fields and the popover's selection and times
/// ("should clear date and time when controlled value is set to null", `DateRangePicker`).
#[browser_test]
pub async fn range_clearing_the_value(page: &Page<'_>) -> Result<(), Report> {
    let section = "range-times-empty";
    page.goto_sections(PATH, &[section]).await?;
    open(page, section).await?;
    calendar_date(page, "Friday, February 1, 2019")
        .await?
        .click()
        .await?;
    calendar_date(page, "Saturday, February 2, 2019")
        .await?
        .click()
        .await?;
    click_outside(page, section).await?;
    wait_for_value(page, section, "2019-02-01T00:00:00 - 2019-02-02T00:00:00").await?;
    page.element("#test-dp-range-times-empty-clear")
        .await?
        .click()
        .await?;
    wait_for_value(page, section, "none").await?;
    wait_for_field_text(page, section, 0, "mm/dd/yyyy, ––:––\u{202f}AM").await?;
    wait_for_field_text(page, section, 1, "mm/dd/yyyy, ––:––\u{202f}AM").await?;
    open(page, section).await?;
    assert_that!(
        page.count("[role=dialog] [role=gridcell][aria-selected=true]")
            .await?
    )
    .is_equal_to(0);
    for label in ["Start time", "End time"] {
        let time = time_field(page, label).await?;
        assert_that!(&time.element(".leptonic-DateInput").await?)
            .text_content()
            .await
            .map_owned(Option::unwrap_or_default)
            .map_owned(without_bidi_marks)
            .is_equal_to("––:––\u{202f}AM".to_owned());
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Labelling and descriptions (react-spectrum `DatePicker.test.js`, "labeling").
// ---------------------------------------------------------------------------------------------

/// The texts of the elements an id reference list names, joined with spaces.
async fn referenced_texts(page: &Page<'_>, list: &str) -> Result<String, Report> {
    let mut texts = Vec::new();
    for id in ids(list) {
        texts.push(
            page.element(format!("#{id}"))
                .await?
                .prop("textContent")
                .await?
                .unwrap_or_default(),
        );
    }
    Ok(texts.join(" "))
}

/// The label names the group, the button (with its own "Calendar") and every segment (with its
/// own name) ("should support labeling").
#[browser_test]
pub async fn labelling(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["help"]).await?;
    let label = page
        .element(css("#test-dp-help .leptonic-Label").text("Date"))
        .await?;
    let label_id = label.id().await?.unwrap_or_default();
    assert_that!(group(page, "help").await?)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to(label_id.as_str());
    let open_button = button(page, "help").await?;
    assert_that!(open_button)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Calendar");
    let button_id = assert_that!(open_button)
        .has_attribute("id")
        .await
        .is_not_empty()
        .actual()
        .clone();
    assert_that!(open_button)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to(format!("{button_id} {label_id}"));
    for segment in spinbuttons(page, "help").await? {
        let segment_id = assert_that!(segment)
            .has_attribute("id")
            .await
            .is_not_empty()
            .actual()
            .clone();
        assert_that!(segment)
            .has_attribute("aria-labelledby")
            .await
            .is_equal_to(format!("{segment_id} {label_id}"));
    }
    Ok(())
}

/// Named by `aria-label`, the group has it, the field inside is a presentation without one, the
/// button is labelled by itself and the group, and every segment's own label ends with it
/// ("should support labeling with aria-label").
#[browser_test]
pub async fn labelling_with_aria_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["aria-label"]).await?;
    let group = group(page, "aria-label").await?;
    let group_id = assert_that!(group)
        .has_attribute("id")
        .await
        .is_not_empty()
        .actual()
        .clone();
    assert_that!(group)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Birth date");
    let field = page
        .element("#test-dp-aria-label .leptonic-DateInput")
        .await?;
    assert_that!(field)
        .has_attribute("role")
        .await
        .is_equal_to("presentation");
    assert_that!(field).attribute("aria-label").await.is_none();
    let open_button = button(page, "aria-label").await?;
    let button_id = open_button.id().await?.unwrap_or_default();
    assert_that!(open_button)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to(format!("{button_id} {group_id}"));
    for segment in spinbuttons(page, "aria-label").await? {
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

/// Named by `aria-labelledby`, the group, the button and every segment reference it; the field
/// inside is a presentation ("should support labeling with aria-labelledby").
#[browser_test]
pub async fn labelling_with_aria_labelledby(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["labelledby"]).await?;
    let label_id = "test-dp-labelledby-foo";
    let group = group(page, "labelledby").await?;
    assert_that!(group).attribute("aria-label").await.is_none();
    assert_that!(group)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to(label_id);
    let field = page
        .element("#test-dp-labelledby .leptonic-DateInput")
        .await?;
    assert_that!(field)
        .has_attribute("role")
        .await
        .is_equal_to("presentation");
    assert_that!(field)
        .attribute("aria-labelledby")
        .await
        .is_none();
    let open_button = button(page, "labelledby").await?;
    let button_id = open_button.id().await?.unwrap_or_default();
    assert_that!(open_button)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to(format!("{button_id} {label_id}"));
    for segment in spinbuttons(page, "labelledby").await? {
        let segment_id = segment.id().await?.unwrap_or_default();
        assert_that!(segment)
            .has_attribute("aria-labelledby")
            .await
            .is_equal_to(format!("{segment_id} {label_id}"));
    }
    Ok(())
}

/// A description describes the group and the first segment only, not the field ("should support
/// help text description"); with a value, the value's description comes first ("should support
/// help text with a value").
#[browser_test]
pub async fn help_text(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["help", "help-value"]).await?;
    for (section, description) in [
        ("help", "Help text"),
        ("help-value", "Selected Date: February 3, 2020 Help text"),
    ] {
        let group = group(page, section).await?;
        let describedby = group.attr("aria-describedby").await?.unwrap_or_default();
        assert_that!(referenced_texts(page, &describedby).await?)
            .is_equal_to(description.to_owned());
        let field = page
            .element(format!("#test-dp-{section} .leptonic-DateInput"))
            .await?;
        assert_that!(field)
            .attribute("aria-describedby")
            .await
            .is_none();
        let segments = spinbuttons(page, section).await?;
        assert_that!(&segments[0])
            .has_attribute("aria-describedby")
            .await
            .is_equal_to(describedby.as_str());
        for segment in &segments[1..] {
            assert_that!(segment)
                .attribute("aria-describedby")
                .await
                .is_none();
        }
    }
    Ok(())
}

/// The error message of an invalid picker describes the group and every segment ("should support
/// error message", "should support error message with a value"); a valid picker's isn't shown
/// ("should not display error message if not invalid").
#[browser_test]
pub async fn error_message(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["error", "error-value", "not-invalid"])
        .await?;
    for (section, description) in [
        ("error", "Error message"),
        (
            "error-value",
            "Selected Date: February 3, 2020 Error message",
        ),
    ] {
        let group = group(page, section).await?;
        let describedby = group.attr("aria-describedby").await?.unwrap_or_default();
        assert_that!(referenced_texts(page, &describedby).await?)
            .is_equal_to(description.to_owned());
        for segment in spinbuttons(page, section).await? {
            assert_that!(segment)
                .has_attribute("aria-describedby")
                .await
                .is_equal_to(describedby.as_str());
        }
    }
    assert_that!(group(page, "not-invalid").await?)
        .attribute("aria-describedby")
        .await
        .is_none();
    for segment in spinbuttons(page, "not-invalid").await? {
        assert_that!(segment)
            .attribute("aria-describedby")
            .await
            .is_none();
    }
    Ok(())
}

/// A date before Christ is described with its era, and shows an era segment ("should include era
/// for BC dates").
#[browser_test]
pub async fn era_for_bc_dates(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["era"]).await?;
    assert_that!(group(page, "era").await?)
        .accessible_description()
        .await
        .is_equal_to("Selected Date: February 3, 2020 BC");
    let segments = spinbuttons(page, "era").await?;
    assert_that!(&segments[3])
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to("BC".to_owned());
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Focus management (react-spectrum `DatePicker.test.js`, `DateRangePicker.test.js`).
// ---------------------------------------------------------------------------------------------

/// Presses `element` itself with the mouse (not a segment inside it).
async fn mouse_down_on(element: &WebElement) -> Result<(), Report> {
    element
        .dispatch(SyntheticEvent::pointer(PointerKind::Down))
        .await?;
    element
        .dispatch(SyntheticEvent::pointer(PointerKind::Up))
        .await?;
    Ok(())
}

/// A mouse press on the field of an empty picker focuses its first segment ("should focus the
/// first segment on mouse down in the field").
#[browser_test]
pub async fn mouse_down_focuses_the_first_segment(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty"]).await?;
    let field = page
        .first_element("#test-dp-empty .leptonic-DateInput")
        .await?;
    mouse_down_on(&field).await?;
    page.wait_for_focus(&spinbuttons(page, "empty").await?[0])
        .await?;
    Ok(())
}

/// A mouse press on the field focuses its first unfilled segment ("should focus the first
/// unfilled segment on mouse down in the field").
#[browser_test]
pub async fn mouse_down_focuses_the_first_unfilled_segment(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["empty"]).await?;
    let segments = spinbuttons(page, "empty").await?;
    segments[0].focus().await?;
    page.wait_for_focus(&segments[0]).await?;
    page.send_keys(Key::Up).await?;
    segments[0].wait_for_attr("data-placeholder", None).await?;
    page.focus_stays(&segments[0], std::time::Duration::from_millis(100))
        .await?;
    let field = page
        .first_element("#test-dp-empty .leptonic-DateInput")
        .await?;
    mouse_down_on(&field).await?;
    page.wait_for_focus(&segments[1]).await?;
    Ok(())
}

/// A mouse press on the field of a picker with a value focuses its last segment ("should focus
/// the last segment on mouse down in the field with a value").
#[browser_test]
pub async fn mouse_down_focuses_the_last_segment(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["help-value"]).await?;
    let field = page
        .element("#test-dp-help-value .leptonic-DateInput")
        .await?;
    mouse_down_on(&field).await?;
    page.wait_for_focus(&spinbuttons(page, "help-value").await?[2])
        .await?;
    Ok(())
}

/// A mouse press on each field of a range picker focuses its first segment ("should focus the
/// first segment of each field on mouse down").
#[browser_test]
pub async fn range_mouse_down_on_each_field(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-focus"]).await?;
    let range = page
        .element("#test-dp-range-focus .leptonic-DateRangePicker")
        .await?;
    let fields = range.elements(".leptonic-DateInput").await?;
    let segments = range.elements("[role=spinbutton]").await?;
    mouse_down_on(&fields[0]).await?;
    page.wait_for_focus(&segments[0]).await?;
    page.blur_focused().await?;
    mouse_down_on(&fields[1]).await?;
    page.wait_for_focus(&segments[3]).await?;
    Ok(())
}

/// A mouse press on the dash between a range picker's fields focuses the start's first segment
/// ("should focus the first segment of the end date on mouse down on the dash").
#[browser_test]
pub async fn range_mouse_down_on_the_dash(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-focus"]).await?;
    let range = page
        .element("#test-dp-range-focus .leptonic-DateRangePicker")
        .await?;
    let dash = range
        .element(css("span[aria-hidden=true]").text("–"))
        .await?;
    mouse_down_on(&dash).await?;
    page.wait_for_focus(&range.first_element("[role=spinbutton]").await?)
        .await?;
    Ok(())
}

/// Removing the era (by stepping into the current era) moves the focus to the segment before it
/// ("should focus the previous segment when the era is removed").
#[browser_test]
pub async fn removing_the_era_focuses_the_previous_segment(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["era"]).await?;
    let segments = spinbuttons(page, "era").await?;
    let era = segment(page, "era", "era").await?;
    assert_that!(segments.last()).some().is_equal_to(&era);
    era.focus().await?;
    page.wait_for_focus(&era).await?;
    page.send_keys(Key::Up).await?;
    page.wait_for_count("#test-dp-era [data-type=era]", 0)
        .await?;
    page.wait_for_focus(&segment(page, "era", "year").await?)
        .await?;
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Validation as the user edits (react-spectrum `DatePicker.test.js`, `DateRangePicker.test.js`,
// "validation").
// ---------------------------------------------------------------------------------------------

/// Steps the year of `section`'s field at `index` (`Key::Up`/`Key::Down`).
async fn step_year(page: &Page<'_>, section: &str, index: usize, key: Key) -> Result<(), Report> {
    let years = page
        .elements(format!("#test-dp-{section} [data-type=year]"))
        .await?;
    years[index].focus().await?;
    page.wait_for_focus(&years[index]).await?;
    page.send_keys(key).await?;
    Ok(())
}

/// A picker on its minimum turns invalid when its year goes below it and valid again when it
/// comes back ("should display an error icon when date is less than the minimum
/// (uncontrolled)").
#[browser_test]
pub async fn below_the_minimum(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["min"]).await?;
    let date_picker = picker(page, "min", "DatePicker").await?;
    assert_that!(date_picker)
        .attribute("data-invalid")
        .await
        .is_none();
    step_year(page, "min", 0, Key::Down).await?;
    date_picker
        .wait_for_attr("data-invalid", Some("true"))
        .await?;
    page.send_keys(Key::Up).await?;
    date_picker.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// A picker on its maximum turns invalid when its year goes above it and valid again when it
/// comes back ("should display an error icon when date is greater than the maximum
/// (uncontrolled)").
#[browser_test]
pub async fn above_the_maximum(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["max"]).await?;
    let date_picker = picker(page, "max", "DatePicker").await?;
    assert_that!(date_picker)
        .attribute("data-invalid")
        .await
        .is_none();
    step_year(page, "max", 0, Key::Up).await?;
    date_picker
        .wait_for_attr("data-invalid", Some("true"))
        .await?;
    page.send_keys(Key::Down).await?;
    date_picker.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// A range picker turns invalid when its start goes below the minimum, its start or end above
/// the maximum, or its end before its start ("should display an error icon when the start date
/// is less than the minimum (uncontrolled)", "... the start date is greater than the maximum",
/// "... the end date is greater than the maximum", "... the end date is less than the start
/// date").
#[browser_test]
pub async fn range_limits(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-min", "range-max"])
        .await?;
    let below = picker(page, "range-min", "DateRangePicker").await?;
    assert_that!(below)
        .attribute("data-invalid")
        .await
        .is_none();
    step_year(page, "range-min", 0, Key::Down).await?;
    below.wait_for_attr("data-invalid", Some("true")).await?;
    page.send_keys(Key::Up).await?;
    below.wait_for_attr("data-invalid", None).await?;
    // The end before the start.
    step_year(page, "range-min", 1, Key::Down).await?;
    below.wait_for_attr("data-invalid", Some("true")).await?;

    let above = picker(page, "range-max", "DateRangePicker").await?;
    assert_that!(above)
        .attribute("data-invalid")
        .await
        .is_none();
    step_year(page, "range-max", 1, Key::Up).await?;
    above.wait_for_attr("data-invalid", Some("true")).await?;
    page.send_keys(Key::Down).await?;
    above.wait_for_attr("data-invalid", None).await?;
    step_year(page, "range-max", 0, Key::Up).await?;
    above.wait_for_attr("data-invalid", Some("true")).await?;
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Time zones (react-spectrum `DatePicker.test.js`, "timeZone").
// ---------------------------------------------------------------------------------------------

/// Clearing every segment of a zoned picker keeps showing its time zone ("should keep timeZone
/// from defaultValue when date and time are cleared").
#[browser_test]
pub async fn zone_kept_when_cleared(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["zoned"]).await?;
    assert_that!(field_input(page, "zoned", 0).await?)
        .text_content()
        .await
        .map_owned(Option::unwrap_or_default)
        .map_owned(without_bidi_marks)
        .is_equal_to("9/21/2024, 12:00\u{202f}AM PDT".to_owned());
    let segments = spinbuttons(page, "zoned").await?;
    for segment in &segments {
        segment.focus().await?;
        page.wait_for_focus(segment).await?;
        for _ in 0..4 {
            page.send_keys(Key::Backspace).await?;
        }
    }
    wait_for_value(page, "zoned", "none").await?;
    assert_that!(|| field_text(page, "zoned", 0))
        .eventually_ok()
        .satisfies(|text| {
            text.starts_with("mm/dd/yyyy, ––:––\u{202f}AM P")
                .ends_with("T");
        })
        .await;
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Forms (react-spectrum `DatePicker.test.js`, "forms").
// ---------------------------------------------------------------------------------------------

/// The group's description texts of `section`'s picker.
async fn group_description(page: &Page<'_>, section: &str) -> Result<String, Report> {
    let group = group(page, section).await?;
    let describedby = group.attr("aria-describedby").await?.unwrap_or_default();
    referenced_texts(page, &describedby).await
}

/// Waits until the group's description of `section`'s picker contains `text` (or doesn't).
async fn wait_for_group_description(
    page: &Page<'_>,
    section: &str,
    text: &str,
    present: bool,
) -> Result<(), Report> {
    assert_that!(|| group_description(page, section))
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

/// The hidden input of `section`'s picker named `name`.
async fn hidden_input(page: &Page<'_>, section: &str, name: &str) -> Result<WebElement, Report> {
    page.element(format!(
        "#test-dp-{section} input[name={name}]:not([form=''])"
    ))
    .await
}

/// Resetting the form restores the picker's initial value, also in its description ("supports
/// form reset").
#[browser_test]
pub async fn form_reset(page: &Page<'_>) -> Result<(), Report> {
    let section = "form-reset";
    page.goto_sections(PATH, &[section]).await?;
    let input = hidden_input(page, section, "date").await?;
    assert_that!(group_description(page, section).await?)
        .is_equal_to("Selected Date: February 3, 2020".to_owned());
    assert_that!(input)
        .property("value")
        .await
        .is_equal_to(Some("2020-02-03".to_owned()));
    let segments = spinbuttons(page, section).await?;
    segments[0].focus().await?;
    page.wait_for_focus(&segments[0]).await?;
    page.send_keys(Key::Up).await?;
    input.wait_for_prop("value", "2020-03-03").await?;
    wait_for_group_description(page, section, "Selected Date: March 3, 2020", true).await?;
    page.element("#test-dp-form-reset-reset")
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "2020-02-03").await?;
    wait_for_group_description(page, section, "Selected Date: February 3, 2020", true).await?;
    Ok(())
}

/// Native validation of min and max: the error shows once the form is checked (focusing the
/// first segment), stays while editing and goes when the picker is left valid; above the
/// maximum the same ("supports minValue and maxValue", native).
#[browser_test]
pub async fn native_min_and_max(page: &Page<'_>) -> Result<(), Report> {
    let section = "form-min-max";
    page.goto_sections(PATH, &[section]).await?;
    let input = hidden_input(page, section, "date").await?;
    let form = page.element("#test-dp-form-min-max-form").await?;
    let segments = spinbuttons(page, section).await?;
    let minimum = "Value must be 2/3/2020 or later.";
    let maximum = "Value must be 2/3/2024 or earlier.";
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(group_description(page, section).await?).does_not_contain(minimum);

    assert_that!(form.check_validity().await?).is_false();
    wait_for_group_description(page, section, minimum, true).await?;
    page.wait_for_focus(&segments[0]).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Up).await?;
    assert_that!(|| input.is_valid())
        .eventually_ok()
        .matches(eq(true))
        .await;
    assert_that!(group_description(page, section).await?).contains(minimum);
    page.element("#test-dp-form-min-max-after")
        .await?
        .click()
        .await?;
    wait_for_group_description(page, section, minimum, false).await?;

    segments[2].focus().await?;
    page.wait_for_focus(&segments[2]).await?;
    page.type_text("2025").await?;
    assert_that!(|| input.is_valid())
        .eventually_ok()
        .matches(eq(false))
        .await;
    assert_that!(group_description(page, section).await?).does_not_contain(maximum);
    page.element("#test-dp-form-min-max-after")
        .await?
        .click()
        .await?;
    assert_that!(form.check_validity().await?).is_false();
    wait_for_group_description(page, section, maximum, true).await?;
    page.wait_for_focus(&segments[0]).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Down).await?;
    assert_that!(|| input.is_valid())
        .eventually_ok()
        .matches(eq(true))
        .await;
    assert_that!(group_description(page, section).await?).contains(maximum);
    page.element("#test-dp-form-min-max-after")
        .await?
        .click()
        .await?;
    wait_for_group_description(page, section, maximum, false).await?;
    Ok(())
}

/// Native validation of a `validate` function: the error shows once the form is checked, stays
/// while editing and goes when the picker is left valid ("supports validate function", native).
#[browser_test]
pub async fn native_validate(page: &Page<'_>) -> Result<(), Report> {
    let section = "form-validate";
    page.goto_sections(PATH, &[section]).await?;
    let input = hidden_input(page, section, "date").await?;
    assert_that!(group_description(page, section).await?).does_not_contain("Invalid value");
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(
        page.element("#test-dp-form-validate-form")
            .await?
            .check_validity()
            .await?
    )
    .is_false();
    wait_for_group_description(page, section, "Invalid value", true).await?;
    let segments = spinbuttons(page, section).await?;
    page.wait_for_focus(&segments[0]).await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    page.type_text("2024").await?;
    assert_that!(|| input.is_valid())
        .eventually_ok()
        .matches(eq(true))
        .await;
    assert_that!(group_description(page, section).await?).contains("Invalid value");
    page.element("#test-dp-form-validate-after")
        .await?
        .click()
        .await?;
    wait_for_group_description(page, section, "Invalid value", false).await?;
    Ok(())
}

/// A server error shows after submitting and goes once the value changes ("supports server
/// validation", native).
#[browser_test]
pub async fn native_server_errors(page: &Page<'_>) -> Result<(), Report> {
    let section = "form-server";
    page.goto_sections(PATH, &[section]).await?;
    let input = hidden_input(page, section, "date").await?;
    assert_that!(group(page, section).await?)
        .attribute("aria-describedby")
        .await
        .is_none();
    page.element("#test-dp-form-server-submit")
        .await?
        .click()
        .await?;
    wait_for_group_description(page, section, "Invalid value", true).await?;
    assert_that!(input.is_valid().await?).is_false();
    let segments = spinbuttons(page, section).await?;
    segments[2].focus().await?;
    page.wait_for_focus(&segments[2]).await?;
    page.type_text("2024").await?;
    segments[1].focus().await?;
    page.wait_for_focus(&segments[1]).await?;
    page.type_text("2").await?;
    segments[0].focus().await?;
    page.wait_for_focus(&segments[0]).await?;
    page.type_text("2").await?;
    page.blur_focused().await?;
    wait_for_group_description(page, section, "Invalid value", false).await?;
    assert_that!(|| input.is_valid())
        .eventually_ok()
        .matches(eq(true))
        .await;
    Ok(())
}

/// A custom message replaces the browser's for a missing value ("supports customizing native
/// error messages").
#[browser_test]
pub async fn native_custom_message(page: &Page<'_>) -> Result<(), Report> {
    let section = "form-custom";
    page.goto_sections(PATH, &[section]).await?;
    assert_that!(group(page, section).await?)
        .attribute("aria-describedby")
        .await
        .is_none();
    assert_that!(
        page.element("#test-dp-form-custom-form")
            .await?
            .check_validity()
            .await?
    )
    .is_false();
    assert_that!(|| group_description(page, section))
        .eventually_ok()
        .matches(eq("Please enter a value".to_owned()))
        .await;
    Ok(())
}

/// Resetting the form clears the shown native error ("clears validation on form reset").
#[browser_test]
pub async fn native_error_cleared_on_reset(page: &Page<'_>) -> Result<(), Report> {
    let section = "required";
    page.goto_sections(PATH, &[section]).await?;
    let group = group(page, section).await?;
    assert_that!(group)
        .attribute("aria-describedby")
        .await
        .is_none();
    assert_that!(
        page.element("#test-dp-required-form")
            .await?
            .check_validity()
            .await?
    )
    .is_false();
    assert_that!(|| group.attr("aria-describedby"))
        .eventually_ok()
        .satisfies(|describedby| {
            describedby.is_some();
        })
        .await;
    page.element("#test-dp-required-reset")
        .await?
        .click()
        .await?;
    group.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

/// Selecting a date in the calendar makes a required picker valid and clears its shown error
/// ("updates when selecting a date with the calendar").
#[browser_test]
pub async fn native_error_updated_by_the_calendar(page: &Page<'_>) -> Result<(), Report> {
    let section = "required";
    page.goto_sections(PATH, &[section]).await?;
    let input = hidden_input(page, section, "date").await?;
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(
        page.element("#test-dp-required-form")
            .await?
            .check_validity()
            .await?
    )
    .is_false();
    let message = assert_that!(input)
        .property("validationMessage")
        .await
        .some()
        .is_not_blank()
        .actual()
        .clone();
    wait_for_group_description(page, section, &message, true).await?;
    open(page, section).await?;
    let focused = page.focused_element().await?;
    focused.click().await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    assert_that!(|| input.is_valid())
        .eventually_ok()
        .matches(eq(true))
        .await;
    wait_for_group_description(page, section, &message, false).await?;
    Ok(())
}

/// `Aria` validation of min and max shows as the user edits ("supports minValue and maxValue",
/// aria).
#[browser_test]
pub async fn aria_min_and_max(page: &Page<'_>) -> Result<(), Report> {
    let section = "aria-min-max";
    page.goto_sections(PATH, &[section]).await?;
    let minimum = "Value must be 2/3/2020 or later.";
    let maximum = "Value must be 2/3/2024 or earlier.";
    wait_for_group_description(page, section, minimum, true).await?;
    step_year(page, section, 0, Key::Up).await?;
    wait_for_group_description(page, section, minimum, false).await?;
    for _ in 0..5 {
        page.send_keys(Key::Up).await?;
    }
    wait_for_group_description(page, section, maximum, true).await?;
    page.send_keys(Key::Down).await?;
    wait_for_group_description(page, section, maximum, false).await?;
    Ok(())
}

/// `Aria` validation of a `validate` function shows at once and goes with a valid value
/// ("supports validate function", aria).
#[browser_test]
pub async fn aria_validate(page: &Page<'_>) -> Result<(), Report> {
    let section = "aria-validate";
    page.goto_sections(PATH, &[section]).await?;
    wait_for_group_description(page, section, "Invalid value", true).await?;
    let year = segment(page, section, "year").await?;
    year.focus().await?;
    page.wait_for_focus(&year).await?;
    page.type_text("2024").await?;
    wait_for_group_description(page, section, "Invalid value", false).await?;
    Ok(())
}

/// A server error of the form shows at once and goes once the value changes ("supports server
/// validation", aria).
#[browser_test]
pub async fn aria_server_errors(page: &Page<'_>) -> Result<(), Report> {
    let section = "aria-server";
    page.goto_sections(PATH, &[section]).await?;
    wait_for_group_description(page, section, "Invalid value", true).await?;
    let year = segment(page, section, "year").await?;
    year.focus().await?;
    page.wait_for_focus(&year).await?;
    page.type_text("2024").await?;
    page.send_keys(Key::Tab).await?;
    wait_for_group_description(page, section, "Invalid value", false).await?;
    Ok(())
}

/// A range with times is described with its dates and times, also when both ends are the same,
/// on the group only ("should have selected range description with a time", "should handle
/// selected range description when start and end dates are the same").
#[browser_test]
pub async fn range_description_with_times(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["range-description", "range-description-same"])
        .await?;
    for (section, description) in [
        (
            "range-description",
            "Selected Range: February 3, 2020 at 8:00\u{202f}AM to February 10, 2020 at 10:00\u{202f}AM",
        ),
        (
            "range-description-same",
            "Selected Range: February 3, 2020 at 8:00\u{202f}AM to February 3, 2020 at 8:00\u{202f}AM",
        ),
    ] {
        assert_that!(group_description(page, section).await?).is_equal_to(description.to_owned());
        for field in page
            .elements(format!("#test-dp-{section} .leptonic-DateInput"))
            .await?
        {
            assert_that!(field)
                .attribute("aria-describedby")
                .await
                .is_none();
        }
    }
    Ok(())
}
