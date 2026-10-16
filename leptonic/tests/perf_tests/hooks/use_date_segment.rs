// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_date_segment` (`/perf/hooks/use_date_segment`, the segments
//! of a date field of hooks on June 5, 2024): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_date_segment.json`): creating and disposing of the segments, the keys
//! a segment handles (arrows, Page Up/Down, Home/End, digits, Backspace, Delete), typing into an
//! empty segment, focus by the pointer and the keyboard, and a click it ignores while disabled.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_date_segment";

/// Registers the performance tests of `use_date_segment`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(StepWithTheArrowKeys))
        .with(fixture_test(StepWithPageUpAndDown))
        .with(fixture_test(StepToTheEnds))
        .with(fixture_test(TypeADigit))
        .with(fixture_test(TypeIntoAnEmptySegment))
        .with(fixture_test(DeleteWithBackspaceAndDelete))
        .with(fixture_test(FocusWithThePointer))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(Disabled))
}
const SEGMENTS: &str = "#perf-field [data-type]";
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
    page.element(format!("#perf-field [data-type='{kind}']"))
        .await
}

/// Focuses the segment of `kind` with a click and returns it.
async fn focus(page: &Page<'_>, kind: &str) -> Result<WebElement, Report> {
    let segment = segment(page, kind).await?;
    segment.click().await?;
    page.wait_for_focus(&segment).await?;
    Ok(segment)
}

/// The field's value (`yyyymmdd`, 0 without).
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Mounting the five segments (month, day, year and two literals) and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(SEGMENTS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(SEGMENTS, 5).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SEGMENTS, 0).await?;
    Ok(())
}

/// Arrow Up and Down on the day.
#[browser_test]
pub async fn step_with_the_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let day = focus(page, "day").await?;
    page.measure("up", page.send_keys(Key::Up)).await?;
    day.wait_for_inner_text("6").await?;
    page.measure("down", page.send_keys(Key::Down)).await?;
    day.wait_for_inner_text("5").await?;
    assert_that!(value(page).await?).is_equal_to(20_240_605.0);
    Ok(())
}

/// Page Up and Down on the day (by 7).
#[browser_test]
pub async fn step_with_page_up_and_down(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let day = focus(page, "day").await?;
    page.measure("page up", page.send_keys(Key::PageUp)).await?;
    day.wait_for_inner_text("12").await?;
    page.measure("page down", page.send_keys(Key::PageDown))
        .await?;
    day.wait_for_inner_text("5").await?;
    Ok(())
}

/// End and Home on the day: its last value (31: of any month, as in react-aria) and its first.
#[browser_test]
pub async fn step_to_the_ends(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let day = focus(page, "day").await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    day.wait_for_inner_text("31").await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    day.wait_for_inner_text("1").await?;
    assert_that!(value(page).await?).is_equal_to(20_240_601.0);
    Ok(())
}

/// Typing a digit into the day (2; it waits for a second digit), then a second (5: the 25th,
/// moving on to the year).
#[browser_test]
pub async fn type_a_digit(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let day = focus(page, "day").await?;
    page.measure("first digit", page.type_text("2")).await?;
    day.wait_for_inner_text("2").await?;
    page.measure("second digit", page.type_text("5")).await?;
    page.wait_for_focus(&segment(page, "year").await?).await?;
    assert_that!(value(page).await?).is_equal_to(20_240_625.0);
    Ok(())
}

/// Typing into the empty month of an empty field: from its placeholder to a value.
#[browser_test]
pub async fn type_into_an_empty_segment(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("clear-value").await?;
    let month = segment(page, "month").await?;
    month.wait_for_inner_text("mm").await?;
    month.click().await?;
    page.wait_for_focus(&month).await?;
    page.measure("type", page.type_text("9")).await?;
    month.wait_for_attr("data-placeholder", None).await?;
    page.wait_for_focus(&segment(page, "day").await?).await?;
    Ok(())
}

/// Backspace on the year (its last digit goes) and Delete (the rest).
#[browser_test]
pub async fn delete_with_backspace_and_delete(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let year = focus(page, "year").await?;
    page.measure("backspace", page.send_keys(Key::Backspace))
        .await?;
    year.wait_for_inner_text("202").await?;
    page.measure("delete", page.send_keys(Key::Delete)).await?;
    year.wait_for_inner_text("20").await?;
    Ok(())
}

/// A click on the day focuses it; a click on the plain button after the field leaves it.
#[browser_test]
pub async fn focus_with_the_pointer(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let day = segment(page, "day").await?;
    page.measure("click", day.click()).await?;
    page.wait_for_focus(&day).await?;
    let after = page.element("#perf-after").await?;
    page.measure("click outside", after.click()).await?;
    page.wait_for_focus(&after).await?;
    Ok(())
}

/// Tab moving focus to the month, Tab on to the day, Shift+Tab back.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    let month = segment(page, "month").await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&month).await?;
    page.measure("tab on", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&segment(page, "day").await?).await?;
    page.measure("tab back", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_focus(&month).await?;
    Ok(())
}

/// Disabling the field (the segments leave the tab order), a click on the day it ignores, and
/// enabling it.
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let day = segment(page, "day").await?;
    page.measure("disable", page.control("disable")).await?;
    day.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("ignored click", day.click()).await?;
    page.focus_stays(
        &page.element("body").await?,
        std::time::Duration::from_millis(100),
    )
    .await?;
    page.measure("enable", page.control("enable")).await?;
    day.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}
