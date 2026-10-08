// Upstream: react-aria-components/test/DateField.test.js @ 99e6102368
// Upstream: react-aria-components/test/TimeField.test.js @ 99e6102368
// Upstream: react-aria-components/test/DatePicker.test.js @ 99e6102368
// Upstream: react-aria-components/test/DateRangePicker.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions, xpath},
    polling::wait_for,
};

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

        cases!(
            structure(&page),
            typing(&page),
            arrows_and_backspace(&page),
            invalid_date_committed_when_left(&page),
            form_reset(&page),
            min_validation(&page),
            disabled_and_read_only(&page),
            date_and_time(&page),
            zoned(&page),
            time_field(&page),
            date_picker(&page),
            date_range_picker(&page),
            group_states(&page),
        );

        Ok(())
    }
}

/// The segment of `kind` (`month`, `day`, `hour`, `literal`, ...) of the field in
/// `#test-df-<section>`.
async fn segment(page: &Page<'_>, section: &str, kind: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-df-{section} [data-type='{kind}']"))
        .await
}

/// Waits until the segment of `kind` in `#test-df-<section>` has focus.
async fn expect_focus(page: &Page<'_>, section: &str, kind: &str) -> Result<(), Report> {
    let element = segment(page, section, kind).await?;
    page.wait_for_focus(&element).await?;
    Ok(())
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

/// The id of the label with the text `text` in the section `#test-df-<section>`.
async fn label_id(page: &Page<'_>, section: &str, text: &str) -> Result<String, Report> {
    let label = page
        .element(xpath(format!(
            "//section[@id='test-df-{section}']//span[normalize-space()='{text}']"
        )))
        .await?;
    Ok(label.id().await?.unwrap_or_default())
}

/// The ids of an id reference list (`aria-labelledby`, ...).
fn ids(list: &str) -> Vec<&str> {
    list.split_whitespace().collect()
}

/// A group labelled by its label, segments as spin buttons named by kind and labelled by the
/// field, placeholders, hidden literals, the description on the first segment only, a hidden
/// input with the field's name.
async fn structure(page: &Page<'_>) -> Result<(), Report> {
    let group = page.element("#test-df-basic [role=group]").await?;
    let label_id = label_id(page, "basic", "Birthday").await?;
    let labelledby = group.attr("aria-labelledby").await?.unwrap_or_default();
    assert_that!(ids(&labelledby)).contains(label_id.as_str());

    let month = segment(page, "basic", "month").await?;
    assert_that!(month.attr("role").await?)
        .get_some()
        .is_equal_to("spinbutton");
    assert_that!(month.attr("aria-label").await?)
        .get_some()
        .is_equal_to("month, ");
    let month_labelledby = month.attr("aria-labelledby").await?.unwrap_or_default();
    assert_that!(ids(&month_labelledby)).contains(label_id.as_str());
    assert_that!(month.inner_text().await?).is_equal_to("mm");
    assert_that!(month.attr("data-placeholder").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(month.attr("contenteditable").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(month.attr("inputmode").await?)
        .get_some()
        .is_equal_to("numeric");
    assert_that!(segment(page, "basic", "day").await?.inner_text().await?).is_equal_to("dd");
    assert_that!(segment(page, "basic", "year").await?.inner_text().await?).is_equal_to("yyyy");
    let literal = segment(page, "basic", "literal").await?;
    assert_that!(literal.attr("aria-hidden").await?)
        .get_some()
        .is_equal_to("true");

    // The description: on the first segment, not on the others.
    assert_that!(month.attr("aria-describedby").await?).is_some();
    let day = segment(page, "basic", "day").await?;
    assert_that!(day.attr("aria-describedby").await?).is_none();

    let input = page.element("#test-df-basic input[name=birthday]").await?;
    assert_that!(input.value().await?).get_some().is_empty();
    Ok(())
}

/// Typing fills a segment and moves on once no further digit fits.
async fn typing(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-df-basic-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    expect_focus(page, "basic", "month").await?;
    page.type_text("6").await?;
    expect_focus(page, "basic", "day").await?;
    page.type_text("1").await?;
    // 1 may still become 10 to 19.
    expect_focus(page, "basic", "day").await?;
    page.type_text("5").await?;
    expect_focus(page, "basic", "year").await?;
    page.type_text("2024").await?;
    // The last segment keeps the focus (no era shows for the years before 1000 typed on the
    // way: it would take the focus, then go away with it).
    expect_focus(page, "basic", "year").await?;
    wait_for_value(page, "basic", "2024-06-15").await?;
    let input = page.element("#test-df-basic input[name=birthday]").await?;
    assert_that!(input.value().await?)
        .get_some()
        .is_equal_to("2024-06-15");

    // The group describes the value.
    let group = page.element("#test-df-basic [role=group]").await?;
    assert_that!(group.referenced_text("aria-describedby").await?)
        .contains("Selected Date: June 15, 2024");
    Ok(())
}

/// Arrows step a segment and move between them; Backspace deletes a digit, and on an empty
/// segment moves to the previous one.
async fn arrows_and_backspace(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys(Key::Up).await?;
    wait_for_value(page, "basic", "2025-06-15").await?;
    page.send_keys(Key::Left).await?;
    expect_focus(page, "basic", "day").await?;
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
        .inner_text_stays("2025-06-01")
        .await?;
    page.send_keys(Key::Backspace).await?;
    expect_focus(page, "basic", "month").await?;
    page.send_keys(Key::Right).await?;
    page.type_text("5").await?;
    wait_for_value(page, "basic", "2025-06-05").await?;
    Ok(())
}

/// February 30 stays as typed while the field is edited, and is committed as February 28 when
/// the field is left.
async fn invalid_date_committed_when_left(page: &Page<'_>) -> Result<(), Report> {
    segment(page, "basic", "month").await?.click().await?;
    expect_focus(page, "basic", "month").await?;
    page.type_text("2").await?;
    wait_for_value(page, "basic", "2025-02-05").await?;
    expect_focus(page, "basic", "day").await?;
    page.type_text("30").await?;
    segment(page, "basic", "day")
        .await?
        .wait_for_inner_text("30")
        .await?;
    // "3" made a valid date (February 3), "30" does not.
    value(page, "basic")
        .await?
        .inner_text_stays("2025-02-03")
        .await?;
    page.element("#test-df-basic-before").await?.click().await?;
    wait_for_value(page, "basic", "2025-02-28").await?;
    segment(page, "basic", "day")
        .await?
        .wait_for_inner_text("28")
        .await?;
    Ok(())
}

/// Resetting the form restores the initial value.
async fn form_reset(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-df-reset").await?.click().await?;
    wait_for_value(page, "basic", "none").await?;
    segment(page, "basic", "month")
        .await?
        .wait_for_inner_text("mm")
        .await?;
    Ok(())
}

/// A value before the minimum is invalid (shown right away with ARIA validation) until fixed.
async fn min_validation(page: &Page<'_>) -> Result<(), Report> {
    let month = segment(page, "min", "month").await?;
    month.wait_for_attr("aria-invalid", Some("true")).await?;
    let section = page.element("#test-df-min").await?;
    assert_that!(section.inner_text().await?).contains("Value must be 5/1/2024 or later.");
    month.click().await?;
    page.send_keys(Key::Up).await?;
    wait_for_value(page, "min", "2024-05-30").await?;
    month.wait_for_attr("aria-invalid", None).await?;
    wait_for("the min field's text")
        .observing(|| section.inner_text())
        .to_be("without the min error", |text| !text.contains("or later"))
        .await?;
    Ok(())
}

/// A disabled field's segments aren't tabbable; a read-only field's don't change.
async fn disabled_and_read_only(page: &Page<'_>) -> Result<(), Report> {
    let disabled = segment(page, "disabled", "month").await?;
    assert_that!(disabled.attr("tabindex").await?).is_none();
    assert_that!(disabled.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(disabled.attr("contenteditable").await?).is_none();
    let group = page.element("#test-df-disabled [role=group]").await?;
    assert_that!(group.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");

    let read_only = segment(page, "read-only", "month").await?;
    assert_that!(read_only.attr("aria-readonly").await?)
        .get_some()
        .is_equal_to("true");
    read_only.click().await?;
    page.send_keys(Key::Up).await?;
    read_only.inner_text_stays("6").await?;
    Ok(())
}

/// A date with a time (24 hours): hours and minutes follow the date.
async fn date_and_time(page: &Page<'_>) -> Result<(), Report> {
    segment(page, "date-time", "month").await?.click().await?;
    page.type_text("6152024").await?;
    expect_focus(page, "date-time", "hour").await?;
    page.type_text("14").await?;
    expect_focus(page, "date-time", "minute").await?;
    page.type_text("30").await?;
    wait_for_value(page, "date-time", "2024-06-15T14:30:00").await?;
    Ok(())
}

/// A zoned value shows its time zone (not editable); stepping the hour keeps the zone.
async fn zoned(page: &Page<'_>) -> Result<(), Report> {
    let zone = segment(page, "zoned", "timeZoneName").await?;
    assert_that!(zone.inner_text().await?).is_equal_to("EDT");
    assert_that!(zone.attr("role").await?)
        .get_some()
        .is_equal_to("textbox");
    assert_that!(
        segment(page, "zoned", "dayPeriod")
            .await?
            .inner_text()
            .await?
    )
    .is_equal_to("AM");
    segment(page, "zoned", "hour").await?.click().await?;
    page.send_keys(Key::Up).await?;
    wait_for_value(page, "zoned", "2024-06-05T10:30:00-04:00[America/New_York]").await?;
    Ok(())
}

/// A 12-hour time field: the hour moves on after a digit that can't start a two-digit hour,
/// "p" picks PM.
async fn time_field(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(segment(page, "time", "hour").await?.inner_text().await?).is_equal_to("––");
    segment(page, "time", "hour").await?.click().await?;
    page.type_text("9").await?;
    expect_focus(page, "time", "minute").await?;
    page.type_text("30").await?;
    expect_focus(page, "time", "dayPeriod").await?;
    page.type_text("p").await?;
    wait_for_value(page, "time", "21:30:00").await?;
    // The form gets the time ("should support form value").
    let input = page.element("#test-df-time input[name=alarm]").await?;
    assert_that!(input.value().await?)
        .get_some()
        .is_equal_to("21:30:00");
    Ok(())
}

/// A date picker: a labelled group with the field and a button opening a dialog with a calendar;
/// selecting a date closes it and fills the field; Alt+ArrowDown opens it from the field; Escape
/// closes it unchanged.
async fn date_picker(page: &Page<'_>) -> Result<(), Report> {
    let group = page.element("#test-df-picker [role=group]").await?;
    assert_that!(group.attr("aria-labelledby").await?).is_some();
    // The field inside has no role of its own (the group labels and describes it); its
    // segments are labelled by the picker's label.
    page.element("#test-df-picker [role=presentation]").await?;
    let label_id = label_id(page, "picker", "Event").await?;
    let month_labelledby = segment(page, "picker", "month")
        .await?
        .attr("aria-labelledby")
        .await?
        .unwrap_or_default();
    assert_that!(ids(&month_labelledby)).contains(label_id.as_str());
    let button = page
        .element("#test-df-picker button[aria-haspopup=dialog]")
        .await?;
    assert_that!(button.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Calendar");
    assert_that!(button.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("false");

    // Opening: the calendar shows the placeholder's month, its date focused.
    button.click().await?;
    page.element("[role=dialog] [role=grid]").await?;
    button.wait_for_attr("aria-expanded", Some("true")).await?;
    let june1 = page
        .element("[role=dialog] [role=button][aria-label^='Saturday, June 1, 2024']")
        .await?;
    page.wait_for_focus(&june1).await?;
    page.element("[role=dialog] [role=button][aria-label^='Saturday, June 15, 2024']")
        .await?
        .click()
        .await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    wait_for_value(page, "picker", "2024-06-15").await?;
    segment(page, "picker", "day")
        .await?
        .wait_for_inner_text("15")
        .await?;

    // From the field: Alt+ArrowDown opens on the value, arrows and Enter select.
    segment(page, "picker", "month").await?.click().await?;
    page.send_keys(Key::Alt + Key::Down).await?;
    page.element("[role=dialog] [role=grid]").await?;
    let june15 = page
        .element("[role=dialog] [role=button][aria-label^='Saturday, June 15, 2024']")
        .await?;
    page.wait_for_focus(&june15).await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    wait_for_value(page, "picker", "2024-06-16").await?;

    // Escape closes without a change.
    button.click().await?;
    page.element("[role=dialog] [role=grid]").await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    value(page, "picker")
        .await?
        .inner_text_stays("2024-06-16")
        .await?;

    // Typing in the field changes the value.
    segment(page, "picker", "day").await?.click().await?;
    page.send_keys(Key::Up).await?;
    wait_for_value(page, "picker", "2024-06-17").await?;

    // An edit doesn't come back when the value returns to what it was edited from.
    page.element("#test-df-picker-clear").await?.click().await?;
    segment(page, "picker", "month")
        .await?
        .wait_for_inner_text("mm")
        .await?;
    segment(page, "picker", "month").await?.click().await?;
    page.type_text("7").await?;
    segment(page, "picker", "month")
        .await?
        .wait_for_inner_text("7")
        .await?;
    button.click().await?;
    page.element("[role=dialog] [role=grid]").await?;
    page.element("[role=dialog] [role=button][aria-label^='Wednesday, June 12, 2024']")
        .await?
        .click()
        .await?;
    wait_for_value(page, "picker", "2024-06-12").await?;
    page.element("#test-df-picker-clear").await?.click().await?;
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

/// A date range picker: start and end fields named as such; a range selected in the popover
/// fills both; arrows move across both fields (not to the button); a reversed range is invalid.
async fn date_range_picker(page: &Page<'_>) -> Result<(), Report> {
    let start_month = range_segment(page, 0, "month").await?;
    assert_that!(start_month.attr("aria-label").await?)
        .get_some()
        .starts_with("month, Start Date");
    let end_month = range_segment(page, 1, "month").await?;
    assert_that!(end_month.attr("aria-label").await?)
        .get_some()
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
    assert_that!(range_segment(page, 1, "day").await?.inner_text().await?).is_equal_to("14");

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
    wait_for("the range picker's text")
        .observing(|| section.inner_text())
        .to_be("showing the range order error", |text| {
            text.contains("Start date must be before end date.")
        })
        .await?;
    Ok(())
}

/// The field and the picker's group show hover and focus like react-aria-components' `Group`:
/// `data-focus-within`, and `data-focus-visible` only with the keyboard.
async fn group_states(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/date-field").await?;
    let input = page
        .element(xpath(
            "//section[@id='test-df-basic']//*[@data-type='month']/..",
        ))
        .await?;
    assert_that!(input.attr("data-focus-within").await?).is_none();
    page.element("#test-df-basic-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    expect_focus(page, "basic", "month").await?;
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
    assert_that!(group.attr("data-focus-visible").await?).is_none();
    group.wait_for_attr("data-hovered", Some("true")).await?;
    input.wait_for_attr("data-focus-within", None).await?;
    Ok(())
}
