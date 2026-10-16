// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_range_calendar_state` (`/perf/hooks/use_range_calendar_state`,
//! a range calendar's state on June 2024, rendered as text): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_range_calendar_state.json`): creating and disposing of it, its
//! methods (selecting a range by its ends, highlighting while selecting, selecting the focused
//! date and committing, dropping the anchor, setting and clearing the range, dragging, focusing
//! the nearest available date, paging while selecting), what it ignores (selecting while
//! read-only), and changes of its inputs (disabled, limits).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_range_calendar_state";

/// Registers the performance tests of `use_range_calendar_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SelectARange))
        .with(fixture_test(SelectTheFocusedDateAndCommit))
        .with(fixture_test(DropTheAnchor))
        .with(fixture_test(SetAndClear))
        .with(fixture_test(Dragging))
        .with(fixture_test(FocusTheNearestAvailableDate))
        .with(fixture_test(PageWhileSelecting))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(IgnoredWhileReadOnly))
        .with(fixture_test(Limits))
}

/// Waits until the fixture shows `expected` in `#perf-<output>` (`value`, `anchor`,
/// `highlighted`, `focused`, `range`).
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

/// Selecting the first end (the anchor), highlighting a date, and selecting the other end.
#[browser_test]
pub async fn select_a_range(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("select start", page.control("select-june-10"))
        .await?;
    wait_for(page, "anchor", "2024-06-10").await?;
    page.measure("highlight", page.control("highlight-june-13"))
        .await?;
    wait_for(page, "highlighted", "2024-06-10 - 2024-06-13").await?;
    page.measure("select end", page.control("select-june-14"))
        .await?;
    wait_for(page, "value", "2024-06-10 - 2024-06-14").await?;
    wait_for(page, "anchor", "none").await?;
    Ok(())
}

/// Selecting the focused date (the anchor) and committing the range at the focused date.
#[browser_test]
pub async fn select_the_focused_date_and_commit(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("select focused", page.control("select-focused-date"))
        .await?;
    wait_for(page, "anchor", "2024-06-12").await?;
    page.control("highlight-june-13").await?;
    wait_for(page, "highlighted", "2024-06-12 - 2024-06-13").await?;
    page.measure("commit", page.control("commit")).await?;
    wait_for(page, "value", "2024-06-12 - 2024-06-13").await?;
    Ok(())
}

/// Dropping the anchor of a range being selected (the value stays).
#[browser_test]
pub async fn drop_the_anchor(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("select-june-10").await?;
    wait_for(page, "anchor", "2024-06-10").await?;
    page.measure("drop", page.control("drop-anchor")).await?;
    wait_for(page, "anchor", "none").await?;
    wait_for(page, "value", "none").await?;
    Ok(())
}

/// Setting the range (June 17 to 19) and clearing the selection.
#[browser_test]
pub async fn set_and_clear(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-value")).await?;
    wait_for(page, "value", "2024-06-17 - 2024-06-19").await?;
    page.measure("clear", page.control("clear")).await?;
    wait_for(page, "value", "none").await?;
    Ok(())
}

/// Starting and stopping to drag.
#[browser_test]
pub async fn dragging(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("start", page.control("start-dragging"))
        .await?;
    assert_that!(page.control("is-dragging").await?).is_equal_to(1.0);
    page.measure("stop", page.control("stop-dragging")).await?;
    assert_that!(page.control("is-dragging").await?).is_equal_to(0.0);
    Ok(())
}

/// Focusing the date after June 14 (a range may continue there).
#[browser_test]
pub async fn focus_the_nearest_available_date(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("focus", page.control("focus-after-june-14"))
        .await?;
    wait_for(page, "focused", "2024-06-15").await?;
    Ok(())
}

/// Paging to July while selecting from June 10: the highlighted range follows the focused date.
#[browser_test]
pub async fn page_while_selecting(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("select-june-10").await?;
    wait_for(page, "anchor", "2024-06-10").await?;
    page.measure("next page", page.control("next-page")).await?;
    wait_for(page, "range", "2024-07-01 - 2024-07-31").await?;
    wait_for(page, "highlighted", "2024-06-10 - 2024-07-12").await?;
    Ok(())
}

/// Disabling the state and enabling it (selecting stays possible, as in react-stately: the
/// cells of a disabled calendar don't select).
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable")).await?;
    page.measure("enable", page.control("enable")).await?;
    wait_for(page, "anchor", "none").await?;
    Ok(())
}

/// Making the state read-only, a selection it ignores, and making it editable.
#[browser_test]
pub async fn ignored_while_read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("make read-only", page.control("make-read-only"))
        .await?;
    page.measure("ignored select", page.control("select-june-10"))
        .await?;
    wait_for(page, "anchor", "none").await?;
    page.measure("make editable", page.control("make-editable"))
        .await?;
    Ok(())
}

/// A minimum and a maximum, and clearing both.
#[browser_test]
pub async fn limits(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set min", page.control("set-min")).await?;
    page.measure("set max", page.control("set-max")).await?;
    page.measure("clear limits", page.control("clear-limits"))
        .await?;
    wait_for(page, "range", "2024-06-01 - 2024-06-30").await?;
    Ok(())
}
