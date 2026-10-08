// Upstream: react-aria-components/test/DatePicker.test.js @ 99e6102368
// Upstream: react-aria-components/test/DateRangePicker.test.js @ 99e6102368
// Upstream: react-aria-components/test/DateField.test.js @ 99e6102368
// Upstream: react-aria-components/test/TimeField.test.js @ 99e6102368
// Upstream: react-aria/test/datepicker/useDatePicker.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/datepicker/DatePickerBase.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Date pickers, date fields and time fields beyond `date_field_tests`: closing on select or
/// not, the pressed button and open state while open, a disabled picker, a programmatic value
/// in an empty picker, required pickers and time fields with their errors, a range picker's
/// placeholder time, Enter, held keys, deleting a partial field, the selection while another
/// element has the focus, and fields outside en-US (German order, right-to-left segments and
/// the isolated time, the segment styles following the locale).
/// Spec: react-aria-components `DatePicker.test.js`, `DateRangePicker.test.js`,
/// `DateField.test.js`, `TimeField.test.js`; react-aria `useDatePicker.test.tsx`;
/// react-spectrum `DatePickerBase.test.js` (RTL arrows).
pub struct DatePickerTests {}

#[async_trait]
impl BrowserTest<str> for DatePickerTests {
    fn name(&self) -> Cow<'_, str> {
        "date_picker_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/date-picker").await?;

        close_on_select(&page).await?;
        disabled_picker(&page).await?;
        programmatic_value(&page).await?;
        required_picker(&page).await?;
        required_time_field(&page).await?;
        range_placeholder_times(&page).await?;
        enter_does_nothing(&page).await?;
        held_keys(&page).await?;
        deleting_a_partial_field(&page).await?;
        autofill(&page).await?;
        selection_while_elsewhere(&page).await?;
        german_order(&page).await?;
        twelve_hour_clocks(&page).await?;
        right_to_left(&page).await?;
        switching_to_right_to_left(&page).await?;

        page.expect_no_page_errors().await
    }
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn segment(page: &Page<'_>, section: &str, kind: &str) -> Result<WebElement, Report> {
    page.css(&format!("#test-dp-{section} [data-type='{kind}']"))
        .await
}

async fn segment_types(page: &Page<'_>, section: &str) -> Result<Vec<String>, Report> {
    let mut types = Vec::new();
    for segment in page
        .driver
        .find_all(By::Css(format!(
            "#test-dp-{section} [role=spinbutton], #test-dp-{section} [role=textbox]"
        )))
        .await?
    {
        types.push(attr(&segment, "data-type").await?.unwrap_or_default());
    }
    Ok(types)
}

async fn wait_for_value(page: &Page<'_>, section: &str, expected: &str) -> Result<(), Report> {
    page.wait_for_text(&format!("test-dp-{section}-value"), expected)
        .await
}

/// The value stays `expected`, also once effects had time to run.
async fn expect_value_unchanged(
    page: &Page<'_>,
    section: &str,
    expected: &str,
) -> Result<(), Report> {
    let id = format!("test-dp-{section}-value");
    stays!(id, expected.to_owned(), page.read_text_of(&id).await?);
    Ok(())
}

async fn button(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.css(&format!("#test-dp-{section} button[aria-haspopup=dialog]"))
        .await
}

async fn type_text(page: &Page<'_>, text: &str) -> Result<(), Report> {
    for key in text.chars() {
        page.send_keys_to_active(key.to_string()).await?;
    }
    Ok(())
}

/// The text of the segments of `section`'s first `DateInput` (the isolation marks kept).
async fn input_text(page: &Page<'_>, section: &str) -> Result<String, Report> {
    Ok(page
        .css(&format!("#test-dp-{section} .leptonic-DateInput"))
        .await?
        .prop("textContent")
        .await?
        .unwrap_or_default())
}

/// The texts of the elements describing `element`.
async fn descriptions(page: &Page<'_>, element: &WebElement) -> Result<String, Report> {
    let ids = attr(element, "aria-describedby").await?.unwrap_or_default();
    let mut texts = Vec::new();
    for id in ids.split_whitespace() {
        texts.push(
            page.element(id)
                .await?
                .prop("textContent")
                .await?
                .unwrap_or_default(),
        );
    }
    Ok(texts.join(" "))
}

/// "should support close on select = true/false", "should apply isPressed state to button when
/// expanded", "should support data-open state".
async fn close_on_select(page: &Page<'_>) -> Result<(), Report> {
    let picker = page.css("#test-dp-close-true .leptonic-DatePicker").await?;
    let open_button = button(page, "close-true").await?;
    assert_that!(attr(&open_button, "data-pressed").await?).is_none();
    assert_that!(attr(&picker, "data-open").await?).is_none();
    open_button.click().await?;
    page.wait_for_selector("[role=dialog] [role=grid]").await?;
    page.wait_for_attr(&open_button, "data-pressed", Some("true"))
        .await?;
    page.wait_for_attr(&picker, "data-open", Some("true"))
        .await?;
    let selected = page
        .css("[role=dialog] [role=gridcell][aria-selected=true] > [role=button]")
        .await?;
    assert_that!(attr(&selected, "aria-label").await?)
        .is_equal_to(Some("Sunday, February 3, 2019 selected".to_owned()));
    page.css("[role=dialog] [role=button][aria-label^='Monday, February 4, 2019']")
        .await?
        .click()
        .await?;
    page.wait_for_no_selector("[role=dialog]").await?;
    wait_for_value(page, "close-true", "2019-02-04").await?;
    page.wait_for_attr(&open_button, "data-pressed", None)
        .await?;

    button(page, "close-false").await?.click().await?;
    page.wait_for_selector("[role=dialog] [role=grid]").await?;
    page.css("[role=dialog] [role=button][aria-label^='Monday, February 4, 2019']")
        .await?
        .click()
        .await?;
    wait_for_value(page, "close-false", "2019-02-04").await?;
    stays!(
        "open dialogs",
        1,
        page.count_matching("[role=dialog]").await?
    );
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=dialog]").await
}

/// "should disable button and date input when DatePicker is disabled".
async fn disabled_picker(page: &Page<'_>) -> Result<(), Report> {
    let open_button = button(page, "disabled").await?;
    assert_that!(attr(&open_button, "disabled").await?).is_some();
    let group = page.css("#test-dp-disabled [role=group]").await?;
    assert_that!(attr(&group, "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    for segment in group.find_all(By::Css("[role=spinbutton]")).await? {
        assert_that!(attr(&segment, "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    }
    let input = page
        .css("#test-dp-disabled input[name='disabled-date']")
        .await?;
    assert_that!(attr(&input, "disabled").await?).is_some();
    Ok(())
}

/// `useDatePicker.test.tsx`, "should commit programmatically setValue when field is empty".
async fn programmatic_value(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(input_text(page, "empty").await?.as_str()).contains("mm");
    page.click_element_with_id("test-dp-empty-set").await?;
    wait_for_value(page, "empty", "2020-02-03").await?;
    wait_for!(
        "the field showing the year set",
        true,
        input_text(page, "empty").await?.contains("2020")
    );
    Ok(())
}

async fn check_validity(page: &Page<'_>, form: &str) -> Result<(), Report> {
    page.driver
        .execute(
            "document.getElementById(arguments[0]).checkValidity();",
            vec![serde_json::Value::from(form)],
        )
        .await?;
    Ok(())
}

async fn input_valid(page: &Page<'_>, selector: &str) -> Result<bool, Report> {
    let valid = page
        .driver
        .execute(
            "return document.querySelector(arguments[0]).validity.valid;",
            vec![serde_json::Value::from(selector)],
        )
        .await?;
    Ok(valid.json().as_bool() == Some(true))
}

/// Waits until `element`'s descriptions contain `text` (or don't).
async fn wait_for_description(
    page: &Page<'_>,
    element: &WebElement,
    text: &str,
    present: bool,
) -> Result<(), Report> {
    wait_for!(
        format!("whether the description contains {text:?}"),
        present,
        descriptions(page, element).await?.contains(text)
    );
    Ok(())
}

/// The browser's message for a missing required value.
async fn required_message(page: &Page<'_>, selector: &str) -> Result<String, Report> {
    let message = page
        .driver
        .execute(
            "return document.querySelector(arguments[0]).validationMessage;",
            vec![serde_json::Value::from(selector)],
        )
        .await?;
    Ok(message.json().as_str().unwrap_or_default().to_owned())
}

/// RAC `DatePicker.test.js`, "supports validation errors": a required picker is invalid on
/// submission, the first segment gets the focus; the error stays until the field is left with a
/// value.
async fn required_picker(page: &Page<'_>) -> Result<(), Report> {
    let input = "#test-dp-required input[name=date]";
    let group = page.css("#test-dp-required [role=group]").await?;
    let picker = page.css("#test-dp-required .leptonic-DatePicker").await?;
    assert_that!(attr(&page.css(input).await?, "required").await?).is_some();
    assert_that!(input_valid(page, input).await?).is_false();
    assert_that!(attr(&picker, "data-invalid").await?).is_none();
    let message = required_message(page, input).await?;

    check_validity(page, "test-dp-required-form").await?;
    wait_for_description(page, &group, &message, true).await?;
    page.wait_for_attr(&picker, "data-invalid", Some("true"))
        .await?;
    let month = segment(page, "required", "month").await?;
    page.wait_for_focus_on(&month, "the first segment").await?;

    page.send_keys_to_active(Key::Up).await?;
    page.press_tab().await?;
    page.send_keys_to_active(Key::Up).await?;
    page.press_tab().await?;
    page.send_keys_to_active(Key::Up).await?;
    wait_for!(
        "the picker's input validity",
        true,
        input_valid(page, input).await?
    );
    assert_that!(descriptions(page, &group).await?.as_str()).contains(message.as_str());

    page.click_element_with_id("test-dp-required-after").await?;
    wait_for_description(page, &group, &message, false).await?;
    page.wait_for_attr(&picker, "data-invalid", None).await
}

/// RAC `TimeField.test.js`, "supports validation errors".
async fn required_time_field(page: &Page<'_>) -> Result<(), Report> {
    let input = "#test-dp-time-required input[name=time]";
    let group = page.css("#test-dp-time-required [role=group]").await?;
    assert_that!(attr(&page.css(input).await?, "required").await?).is_some();
    assert_that!(input_valid(page, input).await?).is_false();
    let message = required_message(page, input).await?;

    check_validity(page, "test-dp-time-required-form").await?;
    wait_for_description(page, &group, &message, true).await?;
    let hour = segment(page, "time-required", "hour").await?;
    page.wait_for_focus_on(&hour, "the first segment").await?;

    page.send_keys_to_active(Key::Up).await?;
    page.press_tab().await?;
    page.send_keys_to_active(Key::Up).await?;
    page.press_tab().await?;
    page.send_keys_to_active(Key::Up).await?;
    wait_for!(
        "the time field's input validity",
        true,
        input_valid(page, input).await?
    );
    assert_that!(descriptions(page, &group).await?.as_str()).contains(message.as_str());
    page.click_element_with_id("test-dp-time-required-after")
        .await?;
    wait_for_description(page, &group, &message, false).await
}

/// RAC `DateRangePicker.test.js`, "should set a placeholder time when closing" (closing on
/// select gives a range of dates the placeholder's time), and "should support close on select =
/// false" with times: the range waits for times, closing commits it with the placeholder's time.
async fn range_placeholder_times(page: &Page<'_>) -> Result<(), Report> {
    button(page, "range-time").await?.click().await?;
    page.wait_for_selector("[role=dialog] [role=grid]").await?;
    page.css("[role=dialog] [role=button][aria-label*='Friday, January 6, 2023']")
        .await?
        .click()
        .await?;
    page.css("[role=dialog] [role=button][aria-label*='Wednesday, January 11, 2023']")
        .await?
        .click()
        .await?;
    page.wait_for_no_selector("[role=dialog]").await?;
    wait_for_value(
        page,
        "range-time",
        "2023-01-06T00:00:00 - 2023-01-11T00:00:00",
    )
    .await?;
    let text = input_text(page, "range-time")
        .await?
        .replace(['\u{2066}', '\u{2069}'], "");
    assert_that!(text.as_str()).is_equal_to("1/6/2023, 12:00:00\u{202f}AM");

    button(page, "range-open").await?.click().await?;
    page.wait_for_selector("[role=dialog] [role=grid]").await?;
    page.css("[role=dialog] [role=button][aria-label*='Friday, January 13, 2023']")
        .await?
        .click()
        .await?;
    page.css("[role=dialog] [role=button][aria-label*='Monday, January 16, 2023']")
        .await?
        .click()
        .await?;
    // Waits for the times while open.
    stays!(
        "open dialogs",
        1,
        page.count_matching("[role=dialog]").await?
    );
    expect_value_unchanged(page, "range-open", "none").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=dialog]").await?;
    wait_for_value(
        page,
        "range-open",
        "2023-01-13T10:30:00 - 2023-01-16T10:30:00",
    )
    .await
}

/// RAC `DateField.test.js`, "should do nothing when pressing enter": the focus stays and the
/// form isn't submitted.
async fn enter_does_nothing(page: &Page<'_>) -> Result<(), Report> {
    let year = segment(page, "keys", "year").await?;
    year.click().await?;
    page.wait_for_focus_on(&year, "the year").await?;
    page.send_keys_to_active(Key::Enter).await?;
    stays!(
        "focus on the year",
        true,
        page.driver.active_element().await? == year
    );
    // A submitted form would have reloaded the page with `?keys=...`.
    let url = page.driver.current_url().await?;
    assert_that!(url.query().unwrap_or_default().contains("keys")).is_false();
    expect_value_unchanged(page, "keys", "2024-12-31").await
}

/// Dispatches a keydown, `repeats` repeated keydowns (a held key) and a keyup to the focused
/// element.
async fn hold_key(page: &Page<'_>, key: &str, repeats: usize) -> Result<(), Report> {
    page.driver
        .execute(
            "const [key, repeats] = arguments;
             const fire = (type, repeat) => document.activeElement.dispatchEvent(
                 new KeyboardEvent(type, { key, repeat, bubbles: true, cancelable: true, composed: true }));
             fire('keydown', false);
             for (let i = 0; i < repeats; i++) fire('keydown', true);
             fire('keyup', false);",
            vec![serde_json::Value::from(key), serde_json::Value::from(repeats)],
        )
        .await?;
    Ok(())
}

/// RAC "should support repeat keydown events when holding an arrow key to navigate segments",
/// "... when holding backspace across empty segments".
async fn held_keys(page: &Page<'_>) -> Result<(), Report> {
    let month = segment(page, "keys", "month").await?;
    month.click().await?;
    page.wait_for_focus_on(&month, "the month").await?;
    hold_key(page, "ArrowRight", 1).await?;
    let year = segment(page, "keys", "year").await?;
    page.wait_for_focus_on(&year, "the year").await?;

    let empty_year = segment(page, "empty-field", "year").await?;
    empty_year.click().await?;
    page.wait_for_focus_on(&empty_year, "the empty year")
        .await?;
    hold_key(page, "Backspace", 1).await?;
    let empty_month = segment(page, "empty-field", "month").await?;
    page.wait_for_focus_on(&empty_month, "the empty month")
        .await
}

/// RAC "should reset to placeholders when deleting a partially filled DateField".
async fn deleting_a_partial_field(page: &Page<'_>) -> Result<(), Report> {
    let month = segment(page, "empty-field", "month").await?;
    month.click().await?;
    page.wait_for_focus_on(&month, "the month").await?;
    type_text(page, "11").await?;
    page.wait_for_selector_text("#test-dp-empty-field [data-type=month]", "11")
        .await?;
    month.click().await?;
    page.wait_for_focus_on(&month, "the month").await?;
    page.send_keys_to_active(Key::Backspace).await?;
    page.send_keys_to_active(Key::Backspace).await?;
    page.wait_for_selector_text("#test-dp-empty-field [data-type=month]", "mm")
        .await?;
    assert_that!(segment(page, "empty-field", "day").await?.text().await?)
        .is_equal_to("dd".to_owned());
    assert_that!(segment(page, "empty-field", "year").await?.text().await?)
        .is_equal_to("yyyy".to_owned());
    Ok(())
}

/// Fills the hidden date input of `section` as a browser's autofill does.
async fn fill_hidden_date_input(page: &Page<'_>, section: &str, value: &str) -> Result<(), Report> {
    page.driver
        .execute(
            "const input = document.querySelector(arguments[0]);
             input.value = arguments[1];
             input.dispatchEvent(new Event('input', { bubbles: true }));
             input.dispatchEvent(new Event('change', { bubbles: true }));",
            vec![
                serde_json::Value::from(format!("#test-dp-{section} input[type=date]")),
                serde_json::Value::from(value),
            ],
        )
        .await?;
    Ok(())
}

/// RAC `DateField.test.js`/`DatePicker.test.js`, "should support autofill": a hidden date input
/// (not focusable, hidden from assistive technology, not submitted) takes what the browser fills
/// in.
async fn autofill(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#test-dp-empty-field input[type=date]").await?;
    assert_that!(attr(&input, "tabindex").await?).is_equal_to(Some("-1".to_owned()));
    assert_that!(attr(&input, "form").await?).is_equal_to(Some(String::new()));
    let container = input.find(By::XPath("..")).await?;
    assert_that!(attr(&container, "aria-hidden").await?).is_equal_to(Some("true".to_owned()));
    fill_hidden_date_input(page, "empty-field", "2000-05-30").await?;
    wait_for!(
        "the autofilled field's text",
        "5/30/2000",
        input_text(page, "empty-field").await?
    );

    fill_hidden_date_input(page, "empty", "2000-05-30").await?;
    wait_for_value(page, "empty", "2000-05-30").await
}

/// RAC "does not collapse the selection onto a segment while another element is focused": a
/// selection left inside a segment doesn't take the focus from another element.
async fn selection_while_elsewhere(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-dp-keys-before").await?;
    page.wait_for_active_id("test-dp-keys-before").await?;
    let year = segment(page, "keys", "year").await?;
    page.driver
        .execute(
            "const segment = arguments[0];
             const before = document.getElementById('test-dp-keys-before');
             document.getSelection().collapse(segment.firstChild, 0);
             before.focus();
             document.dispatchEvent(new Event('selectionchange'));",
            vec![year.to_json()?],
        )
        .await?;
    stays!(
        "the focused element",
        Some("test-dp-keys-before".to_owned()),
        page.active_element_id().await?
    );
    Ok(())
}

/// A German date field: day, month, year, two-digit day and month, typed in that order; its
/// segments are named in German ("Tag").
async fn german_order(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(segment_types(page, "de").await?)
        .is_equal_to(["day", "month", "year"].map(str::to_owned).to_vec());
    assert_that!(input_text(page, "de").await?.as_str()).is_equal_to("05.06.2024");
    let day = segment(page, "de", "day").await?;
    // Segment names follow the locale.
    assert_that!(attr(&day, "aria-label").await?.unwrap_or_default().as_str()).starts_with("Tag");
    day.click().await?;
    page.wait_for_focus_on(&day, "the day").await?;
    type_text(page, "17").await?;
    let month = segment(page, "de", "month").await?;
    page.wait_for_focus_on(&month, "the month").await?;
    type_text(page, "3").await?;
    wait_for_value(page, "de", "2024-03-17").await
}

/// A 12-hour time field shows the locale's 12-hour clock as `Intl`'s `hour12: true` does
/// (react-aria's `hourCycle: 'h12'`): German "12:30 AM" and, hour-only, "12 AM" (not the
/// flexible day period "nachts"), Japanese "午前0:30" (h11).
async fn twelve_hour_clocks(page: &Page<'_>) -> Result<(), Report> {
    for (section, hour, day_period) in [
        ("de-12h", "12", "AM"),
        ("de-12h-hour", "12", "AM"),
        ("ja-12h", "0", "午前"),
    ] {
        let hour_segment = segment(page, section, "hour").await?;
        assert_that!(hour_segment.text().await?.as_str())
            .with_detail_message(format!("the hour in {section}"))
            .is_equal_to(hour);
        let day_period_segment = segment(page, section, "dayPeriod").await?;
        assert_that!(day_period_segment.text().await?.as_str())
            .with_detail_message(format!("the day period in {section}"))
            .is_equal_to(day_period);
    }
    Ok(())
}

/// A Hebrew date picker with a time: the time is isolated left to right, the segments are
/// embedded left to right.
async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    let text = input_text(page, "rtl").await?;
    // The time is isolated (LRI ... PDI), so that it reads hour:minute.
    assert_that!(text.contains("\u{2066}9:30\u{2069}") || text.contains("\u{2066}09:30\u{2069}"))
        .with_detail_message(format!("the time isolated in {text:?}"))
        .is_true();
    let types = segment_types(page, "rtl").await?;
    assert_that!(types.clone()).is_equal_to(
        ["day", "month", "year", "hour", "minute"]
            .map(str::to_owned)
            .to_vec(),
    );
    let day = segment(page, "rtl", "day").await?;
    let style = attr(&day, "style")
        .await?
        .unwrap_or_default()
        .replace(' ', "");
    assert_that!(style.as_str()).contains("direction:ltr");
    assert_that!(style.as_str()).contains("unicode-bidi:embed");

    // Arrow keys by position ("DatePicker should support arrow keys to move between segments in
    // an RTL locale", react-spectrum `DatePickerBase.test.js`): ArrowLeft walks leftwards through
    // the segments to the button, ArrowRight back.
    let button = page.css("#test-dp-rtl button").await?;
    // Focused directly, as upstream does (a click at the center of a bidi-embedded segment can
    // land on its neighbor).
    page.driver
        .execute("arguments[0].focus();", vec![day.to_json()?])
        .await?;
    page.wait_for_focus_on(&day, "the day").await?;
    let mut left = active_left(page).await?;
    let mut steps = 0;
    while page.driver.active_element().await? != button {
        steps += 1;
        if steps > 6 {
            leptos_browser_test::bail!("ArrowLeft didn't reach the button from the day");
        }
        page.send_keys_to_active(Key::Left).await?;
        wait_for!("focus moving left", true, active_left(page).await? < left);
        left = active_left(page).await?;
    }
    page.send_keys_to_active(Key::Right).await?;
    wait_for!(
        "focus moving right of the button",
        true,
        active_left(page).await? > left
    );
    Ok(())
}

/// The left edge of the focused element.
async fn active_left(page: &Page<'_>) -> Result<f64, Report> {
    Ok(page
        .driver
        .execute(
            "return document.activeElement.getBoundingClientRect().left;",
            vec![],
        )
        .await?
        .convert()?)
}

/// Switching the locale to a right-to-left one embeds the segments left to right (the styles
/// follow the locale).
async fn switching_to_right_to_left(page: &Page<'_>) -> Result<(), Report> {
    let day = segment(page, "switch", "day").await?;
    let style = attr(&day, "style")
        .await?
        .unwrap_or_default()
        .replace(' ', "");
    assert_that!(style.contains("unicode-bidi")).is_false();
    page.click_element_with_id("test-dp-switch-he").await?;
    wait_for!("the day segment's isolation in he-IL", true, {
        let day = segment(page, "switch", "day").await?;
        let style = attr(&day, "style")
            .await?
            .unwrap_or_default()
            .replace(' ', "");
        style.contains("unicode-bidi:embed") && style.contains("direction:ltr")
    });
    Ok(())
}
