// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `DateField` (`/perf/atoms/date_field`, a date field labelled
//! "Birthday"): what each thing it offers costs, measured (`Page::measure`) and checked against
//! its budget (`tests/budgets/atoms/date_field.json`): mounting and unmounting it (empty and with
//! a date), typing a date, the keys of its segments (arrows, Page Up/Down, Home/End, Backspace),
//! moving between segments, focus by the keyboard, the pointer and the label, hover, and changes
//! of its reactive props (value, placeholder, limits, disabled, read-only, required, invalid,
//! leading zeros). Fixed dates: the counts don't depend on the day the tests run.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, css},
};

const PATH: &str = "/perf/atoms/date_field";

/// Registers the performance tests of `DateField`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountWithADate))
        .with(fixture_test(TypeADate))
        .with(fixture_test(StepWithTheArrowKeys))
        .with(fixture_test(StepWithPageHomeAndEnd))
        .with(fixture_test(MoveBetweenSegments))
        .with(fixture_test(DeleteWithBackspace))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(FocusWithThePointer))
        .with(fixture_test(ClickTheLabel))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ReadOnly))
        .with(fixture_test(Required))
        .with(fixture_test(Invalid))
        .with(fixture_test(ControlledValue))
        .with(fixture_test(Placeholder))
        .with(fixture_test(Limits))
        .with(fixture_test(LeadingZeros))
}
/// The field's group of segments.
const FIELD: &str = "#perf-field";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// The segment of `kind` (`month`, `day`, `year`).
async fn segment(page: &Page<'_>, kind: &str) -> Result<WebElement, Report> {
    page.element(format!("{FIELD} [data-type='{kind}']")).await
}

/// The field's value (`yyyymmdd`, 0 without).
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Focuses the month segment with the keyboard (Tab from the plain button before the field).
async fn focus_the_month(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let month = segment(page, "month").await?;
    page.wait_for_focus(&month).await?;
    Ok(month)
}

/// Mounting the empty field and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(FIELD, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    segment(page, "month")
        .await?
        .wait_for_inner_text("mm")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FIELD, 0).await?;
    Ok(())
}

/// Mounting the field with a date (June 5, 2024) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_with_a_date(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(FIELD, 0).await?;
    page.control("set-value").await?;
    page.measure("mount", page.control("mount")).await?;
    segment(page, "year")
        .await?
        .wait_for_inner_text("2024")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FIELD, 0).await?;
    Ok(())
}

/// Typing June 15, 2024: each full segment moves the focus on; the last makes the date.
#[browser_test]
pub async fn type_a_date(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    focus_the_month(page).await?;
    page.measure("type the month", page.type_text("6")).await?;
    page.wait_for_focus(&segment(page, "day").await?).await?;
    page.measure("type the day", page.type_text("15")).await?;
    page.wait_for_focus(&segment(page, "year").await?).await?;
    page.measure("type the year", page.type_text("2024"))
        .await?;
    assert_that!(value(page).await?).is_equal_to(20_240_615.0);
    Ok(())
}

/// Arrow Up on the empty month (it takes the placeholder's, June), Up again and Down.
#[browser_test]
pub async fn step_with_the_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let month = focus_the_month(page).await?;
    page.measure("up from the placeholder", page.send_keys(Key::Up))
        .await?;
    month.wait_for_inner_text("6").await?;
    page.measure("up", page.send_keys(Key::Up)).await?;
    month.wait_for_inner_text("7").await?;
    page.measure("down", page.send_keys(Key::Down)).await?;
    month.wait_for_inner_text("6").await?;
    Ok(())
}

/// Page Up/Down (by 2 months), Home and End on the month of a date.
#[browser_test]
pub async fn step_with_page_home_and_end(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("set-value").await?;
    let month = focus_the_month(page).await?;
    month.wait_for_inner_text("6").await?;
    page.measure("page up", page.send_keys(Key::PageUp)).await?;
    month.wait_for_inner_text("8").await?;
    page.measure("page down", page.send_keys(Key::PageDown))
        .await?;
    month.wait_for_inner_text("6").await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    month.wait_for_inner_text("12").await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    month.wait_for_inner_text("1").await?;
    assert_that!(value(page).await?).is_equal_to(20_240_105.0);
    Ok(())
}

/// Arrow Right and Left move the focus between the segments.
#[browser_test]
pub async fn move_between_segments(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    focus_the_month(page).await?;
    page.measure("right", page.send_keys(Key::Right)).await?;
    page.wait_for_focus(&segment(page, "day").await?).await?;
    page.measure("left", page.send_keys(Key::Left)).await?;
    page.wait_for_focus(&segment(page, "month").await?).await?;
    Ok(())
}

/// Backspace on the year of a date: it clears the year (the field's value goes).
#[browser_test]
pub async fn delete_with_backspace(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("set-value").await?;
    let year = segment(page, "year").await?;
    year.wait_for_inner_text("2024").await?;
    year.click().await?;
    page.wait_for_focus(&year).await?;
    page.measure("backspace", page.send_keys(Key::Backspace))
        .await?;
    year.wait_for_inner_text("202").await?;
    page.measure("backspace again", page.send_keys(Key::Backspace))
        .await?;
    year.wait_for_inner_text("20").await?;
    Ok(())
}

/// Tab moving focus into the field (its month, with a focus ring) and Shift+Tab out.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    let month = segment(page, "month").await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    month
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    month.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// A click on the day segment focuses it; a click on the plain button after the field leaves.
#[browser_test]
pub async fn focus_with_the_pointer(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let day = segment(page, "day").await?;
    page.measure("click", day.click()).await?;
    page.wait_for_focus(&day).await?;
    day.wait_for_attr("data-focused", Some("true")).await?;
    let after = page.element("#perf-after").await?;
    page.measure("click outside", after.click()).await?;
    page.wait_for_focus(&after).await?;
    day.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// A click on the label focuses the first segment.
#[browser_test]
pub async fn click_the_label(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let label = page.element(css("span").text("Birthday")).await?;
    page.measure("click", label.click()).await?;
    page.wait_for_focus(&segment(page, "month").await?).await?;
    Ok(())
}

/// The pointer entering and leaving the field's segments.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let field = page.element(FIELD).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", field.hover()).await?;
    field.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    field.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Disabling the field (its segments leave the tab order), a click it ignores, and enabling it.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let field = page.element(FIELD).await?;
    let month = segment(page, "month").await?;
    page.measure("disable", page.control("disable")).await?;
    field.wait_for_attr("data-disabled", Some("true")).await?;
    month.wait_for_attr("tabindex", None).await?;
    page.measure("ignored click", month.click()).await?;
    month
        .attr_stays("data-focused", None, std::time::Duration::from_millis(100))
        .await?;
    page.measure("enable", page.control("enable")).await?;
    field.wait_for_attr("data-disabled", None).await?;
    month.wait_for_attr("tabindex", Some("0")).await?;
    Ok(())
}

/// Making the field read-only, an arrow key it ignores, and making it editable.
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("set-value").await?;
    let month = focus_the_month(page).await?;
    month.wait_for_inner_text("6").await?;
    page.measure("make read-only", page.control("make-read-only"))
        .await?;
    month.wait_for_attr("aria-readonly", Some("true")).await?;
    page.measure("ignored up", page.send_keys(Key::Up)).await?;
    month
        .inner_text_stays("6", std::time::Duration::from_millis(100))
        .await?;
    page.measure("make editable", page.control("make-editable"))
        .await?;
    month.wait_for_attr("aria-readonly", None).await?;
    Ok(())
}

/// Making the field required and optional again.
#[browser_test]
pub async fn required(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let month = segment(page, "month").await?;
    page.measure("require", page.control("require")).await?;
    month.wait_for_attr("aria-required", Some("true")).await?;
    page.measure("make optional", page.control("make-optional"))
        .await?;
    month.wait_for_attr("aria-required", None).await?;
    Ok(())
}

/// Marking the field invalid and valid again.
#[browser_test]
pub async fn invalid(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let field = page.element(FIELD).await?;
    page.measure("invalidate", page.control("invalidate"))
        .await?;
    field.wait_for_attr("data-invalid", Some("true")).await?;
    page.measure("validate", page.control("validate")).await?;
    field.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// The app setting the field's date (June 5, 2024) and clearing it.
#[browser_test]
pub async fn controlled_value(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let year = segment(page, "year").await?;
    page.measure("set", page.control("set-value")).await?;
    year.wait_for_inner_text("2024").await?;
    page.measure("clear", page.control("clear-value")).await?;
    year.wait_for_inner_text("yyyy").await?;
    Ok(())
}

/// Moving the placeholder (where stepping starts: January 2030) and back; Arrow Up on the empty
/// month then takes January.
#[browser_test]
pub async fn placeholder(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("move", page.control("move-placeholder"))
        .await?;
    let month = focus_the_month(page).await?;
    page.send_keys(Key::Up).await?;
    month.wait_for_inner_text("1").await?;
    page.measure("back", page.control("placeholder-back"))
        .await?;
    month.wait_for_inner_text("1").await?;
    Ok(())
}

/// A minimum after the date and a maximum before it (the field invalid), and clearing them.
#[browser_test]
pub async fn limits(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("set-value").await?;
    // Native validation (the default): the field's input reports the error to the form.
    let input = page.first_element("#perf-field ~ input").await?;
    segment(page, "year")
        .await?
        .wait_for_inner_text("2024")
        .await?;
    page.measure("set min", page.control("set-min")).await?;
    assert_that!(input.is_valid().await?).is_false();
    page.measure("set max", page.control("set-max")).await?;
    page.measure("clear limits", page.control("clear-limits"))
        .await?;
    assert_that!(input.is_valid().await?).is_true();
    Ok(())
}

/// Forcing leading zeros (June shows as 06) and not.
#[browser_test]
pub async fn leading_zeros(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("set-value").await?;
    let month = segment(page, "month").await?;
    month.wait_for_inner_text("6").await?;
    page.measure("force", page.control("force-leading-zeros"))
        .await?;
    month.wait_for_inner_text("06").await?;
    page.measure("unforce", page.control("no-leading-zeros"))
        .await?;
    month.wait_for_inner_text("6").await?;
    Ok(())
}
