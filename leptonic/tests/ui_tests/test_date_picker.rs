// Upstream: react-aria-components/test/DatePicker.test.js @ 99e6102368
// Upstream: react-aria-components/test/DateRangePicker.test.js @ 99e6102368
// Upstream: react-aria-components/test/DateField.test.js @ 99e6102368
// Upstream: react-aria-components/test/TimeField.test.js @ 99e6102368
// Upstream: react-aria/test/datepicker/useDatePicker.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/datepicker/DatePickerBase.test.js @ 99e6102368
//! Date pickers, date fields and time fields beyond `date_field_tests`: closing on select or
//! not, the pressed button and open state while open, a disabled picker, a programmatic value
//! in an empty picker, required pickers and time fields with their errors, a range picker's
//! placeholder time, Enter, held keys, deleting a partial field, the selection while another
//! element has the focus, and fields outside en-US (German order, right-to-left segments and
//! the isolated time, the segment styles following the locale).
//! Spec: react-aria-components `DatePicker.test.js`, `DateRangePicker.test.js`,
//! `DateField.test.js`, `TimeField.test.js`; react-aria `useDatePicker.test.tsx`;
//! react-spectrum `DatePickerBase.test.js` (RTL arrows).
use assertr::{
    matchers::{eq, gt, lt},
    prelude::*,
};
use browser_test::thirtyfour::prelude::*;
use rootcause::{Report, bail};

use crate::pages::{ElementActions, Page, PageActions, SyntheticEvent};

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
        .element(format!("#test-dp-{section} .leptonic-DateInput"))
        .await?
        .prop("textContent")
        .await?
        .unwrap_or_default())
}

/// "should support close on select = true/false", "should apply isPressed state to button when
/// expanded", "should support data-open state".
pub async fn close_on_select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let picker = page
        .element("#test-dp-close-true .leptonic-DatePicker")
        .await?;
    let open_button = button(page, "close-true").await?;
    assert_that!(open_button.attr("data-pressed").await?).is_none();
    assert_that!(picker.attr("data-open").await?).is_none();
    open_button.click().await?;
    page.element("[role=dialog] [role=grid]").await?;
    open_button
        .wait_for_attr("data-pressed", Some("true"))
        .await?;
    picker.wait_for_attr("data-open", Some("true")).await?;
    let selected = page
        .element("[role=dialog] [role=gridcell][aria-selected=true] > [role=button]")
        .await?;
    assert_that!(selected.attr("aria-label").await?)
        .get_some()
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
    page.count_stays("[role=dialog]", 1).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    Ok(())
}

/// "should disable button and date input when DatePicker is disabled".
pub async fn disabled_picker(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let open_button = button(page, "disabled").await?;
    assert_that!(open_button.is_enabled().await?).is_false();
    let group = page.element("#test-dp-disabled [role=group]").await?;
    assert_that!(group.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    let segments = group.elements("[role=spinbutton]").await?;
    for segment in segments {
        assert_that!(segment.attr("aria-disabled").await?)
            .get_some()
            .is_equal_to("true");
    }
    let input = page
        .element("#test-dp-disabled input[name='disabled-date']")
        .await?;
    assert_that!(input.is_enabled().await?).is_false();
    Ok(())
}

/// `useDatePicker.test.tsx`, "should commit programmatically setValue when field is empty".
pub async fn programmatic_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(input_text(page, "empty").await?).contains("mm");
    page.element("#test-dp-empty-set").await?.click().await?;
    wait_for_value(page, "empty", "2020-02-03").await?;
    assert_that!(|| input_text(page, "empty"))
        .eventually_ok()
        .satisfies(|text| {
            text.contains("2020");
        })
        .await;
    Ok(())
}

/// Waits until `element`'s description contains `text` (or doesn't).
async fn wait_for_description(
    element: &WebElement,
    text: &str,
    present: bool,
) -> Result<(), Report> {
    assert_that!(|| element.referenced_text("aria-describedby"))
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

/// RAC `DatePicker.test.js`, "supports validation errors": a required picker is invalid on
/// submission, the first segment gets the focus; the error stays until the field is left with a
/// value.
pub async fn required_picker(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#test-dp-required input[name=date]").await?;
    let group = page.element("#test-dp-required [role=group]").await?;
    let picker = page
        .element("#test-dp-required .leptonic-DatePicker")
        .await?;
    assert_that!(input.attr("required").await?).is_some();
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(picker.attr("data-invalid").await?).is_none();
    // The browser's message for a missing value.
    let message = input.prop("validationMessage").await?.unwrap_or_default();
    assert_that!(message.as_str()).is_not_blank();

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
    assert_that!(group.referenced_text("aria-describedby").await?).contains(&message);

    page.element("#test-dp-required-after")
        .await?
        .click()
        .await?;
    wait_for_description(&group, &message, false).await?;
    picker.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// RAC `TimeField.test.js`, "supports validation errors".
pub async fn required_time_field(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page
        .element("#test-dp-time-required input[name=time]")
        .await?;
    let group = page.element("#test-dp-time-required [role=group]").await?;
    assert_that!(input.attr("required").await?).is_some();
    assert_that!(input.is_valid().await?).is_false();
    // The browser's message for a missing value.
    let message = input.prop("validationMessage").await?.unwrap_or_default();
    assert_that!(message.as_str()).is_not_blank();

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
    assert_that!(group.referenced_text("aria-describedby").await?).contains(&message);
    page.element("#test-dp-time-required-after")
        .await?
        .click()
        .await?;
    wait_for_description(&group, &message, false).await?;
    Ok(())
}

/// RAC `DateRangePicker.test.js`, "should set a placeholder time when closing" (closing on
/// select gives a range of dates the placeholder's time), and "should support close on select =
/// false" with times: the range waits for times, closing commits it with the placeholder's time.
pub async fn range_placeholder_times(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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
    let text = input_text(page, "range-time")
        .await?
        .replace(['\u{2066}', '\u{2069}'], "");
    assert_that!(text).is_equal_to("1/6/2023, 12:00:00\u{202f}AM");

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
    page.count_stays("[role=dialog]", 1).await?;
    value(page, "range-open")
        .await?
        .inner_text_stays("none")
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

/// RAC `DateField.test.js`, "should do nothing when pressing enter": the focus stays and the
/// form isn't submitted.
pub async fn enter_does_nothing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let year = segment(page, "keys", "year").await?;
    year.click().await?;
    page.wait_for_focus(&year).await?;
    page.send_keys(Key::Enter).await?;
    page.focus_stays(&year).await?;
    // A submitted form would have reloaded the page with `?keys=...`.
    let url = page.driver.current_url().await?;
    assert_that!(url.query().unwrap_or_default()).does_not_contain("keys");
    value(page, "keys")
        .await?
        .inner_text_stays("2024-12-31")
        .await?;
    Ok(())
}

/// RAC "should support repeat keydown events when holding an arrow key to navigate segments",
/// "... when holding backspace across empty segments".
pub async fn held_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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

/// RAC "should reset to placeholders when deleting a partially filled DateField".
pub async fn deleting_a_partial_field(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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
    assert_that!(
        segment(page, "empty-field", "day")
            .await?
            .inner_text()
            .await?
    )
    .is_equal_to("dd");
    assert_that!(
        segment(page, "empty-field", "year")
            .await?
            .inner_text()
            .await?
    )
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
    input.dispatch(SyntheticEvent::plain("change")).await?;
    Ok(())
}

/// RAC `DateField.test.js`/`DatePicker.test.js`, "should support autofill": a hidden date input
/// (not focusable, hidden from assistive technology, not submitted) takes what the browser fills
/// in.
pub async fn autofill(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page
        .element("#test-dp-empty-field input[type=date]")
        .await?;
    assert_that!(input.attr("tabindex").await?)
        .get_some()
        .is_equal_to("-1");
    assert_that!(input.attr("form").await?)
        .get_some()
        .is_empty();
    let container = input.parent().await?;
    assert_that!(container.attr("aria-hidden").await?)
        .get_some()
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

/// RAC "does not collapse the selection onto a segment while another element is focused": a
/// selection left inside a segment doesn't take the focus from another element.
pub async fn selection_while_elsewhere(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let before = page.element("#test-dp-keys-before").await?;
    before.click().await?;
    page.wait_for_focus(&before).await?;
    let year = segment(page, "keys", "year").await?;
    page.eval::<()>(
        "const [segment, before] = arguments;
         document.getSelection().collapse(segment.firstChild, 0);
         before.focus();
         document.dispatchEvent(new Event('selectionchange'));",
        vec![year.to_json()?, before.to_json()?],
    )
    .await?;
    page.focus_stays(&before).await?;
    Ok(())
}

/// A German date field: day, month, year, two-digit day and month, typed in that order; its
/// segments are named in German ("Tag").
pub async fn german_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(segment_types(page, "de").await?).contains_exactly(["day", "month", "year"]);
    assert_that!(input_text(page, "de").await?).is_equal_to("05.06.2024");
    let day = segment(page, "de", "day").await?;
    // Segment names follow the locale.
    assert_that!(day.attr("aria-label").await?)
        .get_some()
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

/// A 12-hour time field shows the locale's 12-hour clock as `Intl`'s `hour12: true` does
/// (react-aria's `hourCycle: 'h12'`): German "12:30 AM" and, hour-only, "12 AM" (not the
/// flexible day period "nachts"), Japanese "午前0:30" (h11).
pub async fn twelve_hour_clocks(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    for (section, hour, day_period) in [
        ("de-12h", "12", "AM"),
        ("de-12h-hour", "12", "AM"),
        ("ja-12h", "0", "午前"),
    ] {
        let hour_segment = segment(page, section, "hour").await?;
        assert_that!(hour_segment.inner_text().await?)
            .with_detail_message(format!("the hour in {section}"))
            .is_equal_to(hour);
        let day_period_segment = segment(page, section, "dayPeriod").await?;
        assert_that!(day_period_segment.inner_text().await?)
            .with_detail_message(format!("the day period in {section}"))
            .is_equal_to(day_period);
    }
    Ok(())
}

/// A Hebrew date picker with a time: the time is isolated left to right, the segments are
/// embedded left to right.
pub async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let text = input_text(page, "rtl").await?;
    // The time is isolated (LRI ... PDI), so that it reads hour:minute (the hour with or without
    // a leading zero).
    assert_that!(text.replace("\u{2066}09:", "\u{2066}9:")).contains("\u{2066}9:30\u{2069}");
    assert_that!(segment_types(page, "rtl").await?)
        .contains_exactly(["day", "month", "year", "hour", "minute"]);
    let day = segment(page, "rtl", "day").await?;
    assert_that!(style_of(&day).await?)
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
pub async fn switching_to_right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let day = segment(page, "switch", "day").await?;
    assert_that!(style_of(&day).await?).does_not_contain("unicode-bidi");
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
