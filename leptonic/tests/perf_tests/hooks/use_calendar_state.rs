// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_calendar_state` (`/perf/hooks/use_calendar_state`, a
//! calendar's state on June 2024, rendered as text): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_calendar_state.json`): creating and disposing of it, its methods
//! (moving the focused date by a day, a week, a page, to the month's ends and by a year;
//! selecting; setting the value, the focused date and the focus), what it ignores (selecting
//! while disabled or read-only), and changes of its reactive inputs (limits, visible duration,
//! first day of the week, page behavior).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_calendar_state";

/// Registers the performance tests of `use_calendar_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MoveByDaysAndWeeks))
        .with(fixture_test(MoveByPages))
        .with(fixture_test(MoveToTheMonthEnds))
        .with(fixture_test(MoveByYears))
        .with(fixture_test(Select))
        .with(fixture_test(SetTheFocusedDate))
        .with(fixture_test(SetTheFocus))
        .with(fixture_test(IgnoredWhileDisabled))
        .with(fixture_test(IgnoredWhileReadOnly))
        .with(fixture_test(Limits))
        .with(fixture_test(VisibleDuration))
        .with(fixture_test(FirstDayOfWeek))
        .with(fixture_test(PageBehavior))
}

/// Waits until the fixture shows `expected` in `#perf-<output>` (`value`, `focused`, `range`).
async fn wait_for(page: &Page<'_>, output: &str, expected: &str) -> Result<(), Report> {
    page.element(format!("#perf-{output}"))
        .await?
        .wait_for_inner_text(expected)
        .await
}

/// Creating the state (rendered) and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count("#perf-range", 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for(page, "range", "2024-06-01 - 2024-06-30").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count("#perf-range", 0).await?;
    Ok(())
}

/// Moving the focused date by a day and by a week, forward and back.
#[browser_test]
pub async fn move_by_days_and_weeks(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("next day", page.control("focus-next-day"))
        .await?;
    wait_for(page, "focused", "2024-06-13").await?;
    page.measure("previous day", page.control("focus-previous-day"))
        .await?;
    wait_for(page, "focused", "2024-06-12").await?;
    page.measure("next row", page.control("focus-next-row"))
        .await?;
    wait_for(page, "focused", "2024-06-19").await?;
    page.measure("previous row", page.control("focus-previous-row"))
        .await?;
    wait_for(page, "focused", "2024-06-12").await?;
    Ok(())
}

/// Moving the focused date by a page (a month: the visible range follows) and back.
#[browser_test]
pub async fn move_by_pages(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("next page", page.control("focus-next-page"))
        .await?;
    wait_for(page, "range", "2024-07-01 - 2024-07-31").await?;
    wait_for(page, "focused", "2024-07-12").await?;
    page.measure("previous page", page.control("focus-previous-page"))
        .await?;
    wait_for(page, "range", "2024-06-01 - 2024-06-30").await?;
    Ok(())
}

/// Moving the focused date to the month's start and end.
#[browser_test]
pub async fn move_to_the_month_ends(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("section start", page.control("focus-section-start"))
        .await?;
    wait_for(page, "focused", "2024-06-01").await?;
    page.measure("section end", page.control("focus-section-end"))
        .await?;
    wait_for(page, "focused", "2024-06-30").await?;
    Ok(())
}

/// Moving the focused date by a year (the larger section) and back.
#[browser_test]
pub async fn move_by_years(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("next year", page.control("focus-next-year"))
        .await?;
    wait_for(page, "range", "2025-06-01 - 2025-06-30").await?;
    page.measure("previous year", page.control("focus-previous-year"))
        .await?;
    wait_for(page, "range", "2024-06-01 - 2024-06-30").await?;
    Ok(())
}

/// Selecting the focused date, selecting another date, and clearing the value.
#[browser_test]
pub async fn select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("select focused", page.control("select-focused-date"))
        .await?;
    wait_for(page, "value", "2024-06-12").await?;
    page.measure("select a date", page.control("select-june-14"))
        .await?;
    wait_for(page, "value", "2024-06-14").await?;
    page.measure("clear", page.control("clear-value")).await?;
    wait_for(page, "value", "none").await?;
    Ok(())
}

/// Setting the focused date to another month (the visible range follows).
#[browser_test]
pub async fn set_the_focused_date(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("focus-october")).await?;
    wait_for(page, "range", "2024-10-01 - 2024-10-31").await?;
    wait_for(page, "focused", "2024-10-15").await?;
    Ok(())
}

/// Marking the calendar focused and unfocused.
#[browser_test]
pub async fn set_the_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("focus", page.control("set-focused")).await?;
    assert_that!(page.control("is-focused").await?).is_equal_to(1.0);
    page.measure("blur", page.control("set-unfocused")).await?;
    assert_that!(page.control("is-focused").await?).is_equal_to(0.0);
    Ok(())
}

/// Disabling the state, a selection it ignores, and enabling it.
#[browser_test]
pub async fn ignored_while_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable")).await?;
    page.measure("ignored select", page.control("select-june-14"))
        .await?;
    wait_for(page, "value", "none").await?;
    page.measure("enable", page.control("enable")).await?;
    Ok(())
}

/// Making the state read-only, a selection it ignores, and making it editable.
#[browser_test]
pub async fn ignored_while_read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("make read-only", page.control("make-read-only"))
        .await?;
    page.measure("ignored select", page.control("select-june-14"))
        .await?;
    wait_for(page, "value", "none").await?;
    page.measure("make editable", page.control("make-editable"))
        .await?;
    Ok(())
}

/// A minimum and a maximum (a move past them stops at them), and clearing both.
#[browser_test]
pub async fn limits(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set min", page.control("set-min")).await?;
    page.measure("set max", page.control("set-max")).await?;
    page.control("focus-section-end").await?;
    wait_for(page, "focused", "2024-06-25").await?;
    page.measure("clear limits", page.control("clear-limits"))
        .await?;
    page.control("focus-section-end").await?;
    wait_for(page, "focused", "2024-06-30").await?;
    Ok(())
}

/// Showing three months and one again (the visible range is aligned again).
#[browser_test]
pub async fn visible_duration(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("show three months", page.control("show-three-months"))
        .await?;
    wait_for(page, "range", "2024-05-01 - 2024-07-31").await?;
    page.measure("show one month", page.control("show-one-month"))
        .await?;
    wait_for(page, "range", "2024-06-01 - 2024-06-30").await?;
    Ok(())
}

/// Starting the weeks on Monday and on the locale's first day again.
#[browser_test]
pub async fn first_day_of_week(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("start on monday", page.control("start-on-monday"))
        .await?;
    page.measure(
        "start on the locale's day",
        page.control("start-on-locale-day"),
    )
    .await?;
    wait_for(page, "range", "2024-06-01 - 2024-06-30").await?;
    Ok(())
}

/// Paging by single months in a three-month view, and by the visible three again.
#[browser_test]
pub async fn page_behavior(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("show-three-months").await?;
    wait_for(page, "range", "2024-05-01 - 2024-07-31").await?;
    page.measure("page single", page.control("page-single"))
        .await?;
    page.control("focus-next-page").await?;
    wait_for(page, "range", "2024-06-01 - 2024-08-31").await?;
    page.measure("page visible", page.control("page-visible"))
        .await?;
    page.control("focus-next-page").await?;
    wait_for(page, "range", "2024-09-01 - 2024-11-30").await?;
    Ok(())
}
