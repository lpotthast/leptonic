// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `TimeField` (`/perf/atoms/time_field`, a time field labelled
//! "Pickup", 12-hour in en-US): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/time_field.json`): mounting and unmounting it
//! (empty and with a time), typing a time, the arrow keys on the hour and the day period, moving
//! between segments, keyboard focus, and changes of its reactive props (value, limits,
//! granularity, hour cycle, disabled, read-only, required, invalid, leading zeros).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/time_field";

/// Registers the performance tests of `TimeField`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountWithATime))
        .with(fixture_test(TypeATime))
        .with(fixture_test(StepWithTheArrowKeys))
        .with(fixture_test(ToggleTheDayPeriod))
        .with(fixture_test(MoveBetweenSegments))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(Granularity))
        .with(fixture_test(HourCycle))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ReadOnly))
        .with(fixture_test(Required))
        .with(fixture_test(Invalid))
        .with(fixture_test(ControlledValue))
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

/// The segment of `kind` (`hour`, `minute`, `second`, `dayPeriod`).
async fn segment(page: &Page<'_>, kind: &str) -> Result<WebElement, Report> {
    page.element(format!("{FIELD} [data-type='{kind}']")).await
}

/// The field's value (`hhmmss`, -1 without).
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Focuses the hour segment with the keyboard (Tab from the plain button before the field).
async fn focus_the_hour(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let hour = segment(page, "hour").await?;
    page.wait_for_focus(&hour).await?;
    Ok(hour)
}

/// Mounting the empty field and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(FIELD, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    segment(page, "hour")
        .await?
        .wait_for_inner_text("––")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FIELD, 0).await?;
    Ok(())
}

/// Mounting the field with a time (8:30 AM) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_with_a_time(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(FIELD, 0).await?;
    page.control("set-value").await?;
    page.measure("mount", page.control("mount")).await?;
    segment(page, "minute")
        .await?
        .wait_for_inner_text("30")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FIELD, 0).await?;
    Ok(())
}

/// Typing 8:30 PM: the hour (one digit: no hour of two starts with 8), the minute and the day
/// period, each full segment moving the focus on.
#[browser_test]
pub async fn type_a_time(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    focus_the_hour(page).await?;
    page.measure("type the hour", page.type_text("8")).await?;
    page.wait_for_focus(&segment(page, "minute").await?).await?;
    page.measure("type the minute", page.type_text("30"))
        .await?;
    page.wait_for_focus(&segment(page, "dayPeriod").await?)
        .await?;
    page.measure("type the day period", page.type_text("p"))
        .await?;
    assert_that!(value(page).await?).is_equal_to(203_000.0);
    Ok(())
}

/// Arrow Up on the empty hour (it takes the placeholder's, 12 AM), Up again and Down.
#[browser_test]
pub async fn step_with_the_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let hour = focus_the_hour(page).await?;
    page.measure("up from the placeholder", page.send_keys(Key::Up))
        .await?;
    hour.wait_for_inner_text("12").await?;
    page.measure("up", page.send_keys(Key::Up)).await?;
    hour.wait_for_inner_text("1").await?;
    page.measure("down", page.send_keys(Key::Down)).await?;
    hour.wait_for_inner_text("12").await?;
    Ok(())
}

/// Arrow Up and Down on the day period of 8:30 AM: PM and AM again.
#[browser_test]
pub async fn toggle_the_day_period(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("set-value").await?;
    let period = segment(page, "dayPeriod").await?;
    period.wait_for_inner_text("AM").await?;
    period.click().await?;
    page.wait_for_focus(&period).await?;
    page.measure("up", page.send_keys(Key::Up)).await?;
    period.wait_for_inner_text("PM").await?;
    assert_that!(value(page).await?).is_equal_to(203_000.0);
    page.measure("down", page.send_keys(Key::Down)).await?;
    period.wait_for_inner_text("AM").await?;
    assert_that!(value(page).await?).is_equal_to(83_000.0);
    Ok(())
}

/// Arrow Right and Left move the focus between the segments.
#[browser_test]
pub async fn move_between_segments(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    focus_the_hour(page).await?;
    page.measure("right", page.send_keys(Key::Right)).await?;
    page.wait_for_focus(&segment(page, "minute").await?).await?;
    page.measure("left", page.send_keys(Key::Left)).await?;
    page.wait_for_focus(&segment(page, "hour").await?).await?;
    Ok(())
}

/// Tab moving focus into the field (its hour, with a focus ring) and Shift+Tab out.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    let hour = segment(page, "hour").await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    hour.wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    hour.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// Showing seconds (a segment more) and minutes again.
#[browser_test]
pub async fn granularity(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("set-value").await?;
    segment(page, "minute")
        .await?
        .wait_for_inner_text("30")
        .await?;
    let seconds = "#perf-field [data-type='second']";
    page.measure("show seconds", page.control("show-seconds"))
        .await?;
    page.wait_for_count(seconds, 1).await?;
    page.measure("show minutes", page.control("show-minutes"))
        .await?;
    page.wait_for_count(seconds, 0).await?;
    Ok(())
}

/// A 24-hour clock (no day period) and the locale's 12 hours again.
#[browser_test]
pub async fn hour_cycle(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("set-value").await?;
    segment(page, "minute")
        .await?
        .wait_for_inner_text("30")
        .await?;
    let period = "#perf-field [data-type='dayPeriod']";
    page.measure("use 24 hours", page.control("use-24-hours"))
        .await?;
    page.wait_for_count(period, 0).await?;
    page.measure("use the locale's hours", page.control("use-locale-hours"))
        .await?;
    page.wait_for_count(period, 1).await?;
    Ok(())
}

/// Disabling the field, a click it ignores, and enabling it.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let field = page.element(FIELD).await?;
    let hour = segment(page, "hour").await?;
    page.measure("disable", page.control("disable")).await?;
    field.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", hour.click()).await?;
    hour.attr_stays("data-focused", None, std::time::Duration::from_millis(100))
        .await?;
    page.measure("enable", page.control("enable")).await?;
    field.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// Making the field read-only, an arrow key it ignores, and making it editable.
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("set-value").await?;
    let hour = focus_the_hour(page).await?;
    hour.wait_for_inner_text("8").await?;
    page.measure("make read-only", page.control("make-read-only"))
        .await?;
    hour.wait_for_attr("aria-readonly", Some("true")).await?;
    page.measure("ignored up", page.send_keys(Key::Up)).await?;
    hour.inner_text_stays("8", std::time::Duration::from_millis(100))
        .await?;
    page.measure("make editable", page.control("make-editable"))
        .await?;
    hour.wait_for_attr("aria-readonly", None).await?;
    Ok(())
}

/// Making the field required and optional again.
#[browser_test]
pub async fn required(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let hour = segment(page, "hour").await?;
    page.measure("require", page.control("require")).await?;
    hour.wait_for_attr("aria-required", Some("true")).await?;
    page.measure("make optional", page.control("make-optional"))
        .await?;
    hour.wait_for_attr("aria-required", None).await?;
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

/// The app setting the field's time (8:30 AM) and clearing it.
#[browser_test]
pub async fn controlled_value(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let minute = segment(page, "minute").await?;
    page.measure("set", page.control("set-value")).await?;
    minute.wait_for_inner_text("30").await?;
    page.measure("clear", page.control("clear-value")).await?;
    minute.wait_for_inner_text("––").await?;
    Ok(())
}

/// A minimum after the time and a maximum before it (the field invalid), and clearing them.
#[browser_test]
pub async fn limits(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("set-value").await?;
    // Native validation (the default): the field's input reports the error to the form.
    let input = page.first_element("#perf-field ~ input").await?;
    segment(page, "minute")
        .await?
        .wait_for_inner_text("30")
        .await?;
    page.measure("set min", page.control("set-min")).await?;
    assert_that!(input.is_valid().await?).is_false();
    page.measure("set max", page.control("set-max")).await?;
    page.measure("clear limits", page.control("clear-limits"))
        .await?;
    assert_that!(input.is_valid().await?).is_true();
    Ok(())
}

/// Forcing leading zeros (8 shows as 08) and not.
#[browser_test]
pub async fn leading_zeros(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("set-value").await?;
    let hour = segment(page, "hour").await?;
    hour.wait_for_inner_text("8").await?;
    page.measure("force", page.control("force-leading-zeros"))
        .await?;
    hour.wait_for_inner_text("08").await?;
    page.measure("unforce", page.control("no-leading-zeros"))
        .await?;
    hour.wait_for_inner_text("8").await?;
    Ok(())
}
