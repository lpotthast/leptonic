// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_drop_indicator` (`/perf/hooks/use_drop_indicator`): the drop
//! indicators of a grid list of 10 or 1,000 rows (one before each row, one after the last, one
//! for the list itself; the fixture of `use_droppable_collection`). Measured (`Page::measure`)
//! and checked against its budget (`tests/budgets/hooks/use_drop_indicator.json`): mounting and
//! unmounting them; a keyboard drag starting (the indicators are offered) and being canceled;
//! keyboard navigation between indicators (ArrowDown, End, Home); a native drag moving from one
//! indicator to the next.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    fixtures::dnd::DndActions,
    harness::fixture_test,
    pages::{DragKind, ElementActions, Page},
    perf_tests::hooks::use_droppable_collection::{
        begin_external_drag, cell, drop_target, indicator, mount_and_unmount, open,
        start_keyboard_drag,
    },
};

const PATH: &str = "/perf/hooks/use_drop_indicator";

/// Registers the performance tests of `use_drop_indicator`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(KeyboardDragSmall))
        .with(fixture_test(KeyboardDragLarge))
        .with(fixture_test(KeyboardNavigationSmall))
        .with(fixture_test(KeyboardNavigationLarge))
        .with(fixture_test(NativeDragSmall))
        .with(fixture_test(NativeDragLarge))
}

/// Mounting and unmounting the indicators of 10 rows.
#[browser_test]
pub async fn mount_and_unmount_small(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, PATH, 10).await
}

/// Mounting and unmounting the indicators of 1,000 rows.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, PATH, 1_000).await
}

/// A keyboard drag over `count` rows starting (the indicators are offered: no longer hidden) and
/// being canceled (hidden again).
async fn keyboard_drag(page: &Page<'_>, count: u32) -> Result<(), Report> {
    open(page, PATH, count).await?;
    let draggable = page.element("#perf-draggable").await?;
    let before_first = indicator(page, "before-item-0").await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&draggable).await?;
    page.measure("start", page.send_keys(Key::Enter)).await?;
    page.wait_for_focus(&indicator(page, "root").await?).await?;
    before_first.wait_for_attr("aria-hidden", None).await?;
    page.measure("cancel", page.send_keys(Key::Escape)).await?;
    page.wait_for_focus(&draggable).await?;
    before_first
        .wait_for_attr("aria-hidden", Some("true"))
        .await?;
    Ok(())
}

/// A keyboard drag over 10 rows.
#[browser_test]
pub async fn keyboard_drag_small(page: &Page<'_>) -> Result<(), Report> {
    keyboard_drag(page, 10).await
}

/// A keyboard drag over 1,000 rows.
#[browser_test]
pub async fn keyboard_drag_large(page: &Page<'_>) -> Result<(), Report> {
    keyboard_drag(page, 1_000).await
}

/// Keyboard navigation between the indicators of `count` rows: ArrowDown (before the first
/// row), End (after the last), Home (the list itself).
async fn keyboard_navigation(page: &Page<'_>, count: u32) -> Result<(), Report> {
    open(page, PATH, count).await?;
    start_keyboard_drag(page).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&indicator(page, "before-item-0").await?)
        .await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    let last = format!("after-item-{}", count - 1);
    page.wait_for_focus(&indicator(page, &last).await?).await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&indicator(page, "root").await?).await?;
    page.send_keys(Key::Escape).await?;
    page.element("#perf-draggable")
        .await?
        .wait_for_attr("data-dragging", None)
        .await?;
    Ok(())
}

/// Keyboard navigation between the indicators of 10 rows.
#[browser_test]
pub async fn keyboard_navigation_small(page: &Page<'_>) -> Result<(), Report> {
    keyboard_navigation(page, 10).await
}

/// Keyboard navigation between the indicators of 1,000 rows.
#[browser_test]
pub async fn keyboard_navigation_large(page: &Page<'_>) -> Result<(), Report> {
    keyboard_navigation(page, 1_000).await
}

/// A native drag over `count` rows moving from the indicator before the second row to the one
/// before the third.
async fn native_drag(page: &Page<'_>, count: u32) -> Result<(), Report> {
    open(page, PATH, count).await?;
    let second = cell(page, 1).await?;
    let third = cell(page, 2).await?;
    let dnd = DndActions::new(page);
    begin_external_drag(page).await?;
    dnd.fire_drag_event_at(&second, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    assert_that!(drop_target(page).await?).is_equal_to("before-item-1".to_owned());
    page.measure(
        "next indicator",
        dnd.fire_drag_event_at(&third, DragKind::Over, &[], Some((1.0, 1.0))),
    )
    .await?;
    assert_that!(drop_target(page).await?).is_equal_to("before-item-2".to_owned());
    Ok(())
}

/// A native drag between the indicators of 10 rows.
#[browser_test]
pub async fn native_drag_small(page: &Page<'_>) -> Result<(), Report> {
    native_drag(page, 10).await
}

/// A native drag between the indicators of 1,000 rows.
#[browser_test]
pub async fn native_drag_large(page: &Page<'_>) -> Result<(), Report> {
    native_drag(page, 1_000).await
}
