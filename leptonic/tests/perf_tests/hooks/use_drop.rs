// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_drop` (`/perf/hooks/use_drop`): a drop target and a
//! `use_drag` source. Measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_drop.json`): mounting and unmounting it; native drags from outside
//! the page (synthesized `DragEvent`s: entering it, moving over it at the same and at another
//! point, leaving it, dropping on it); keyboard drags (Tab leaving it and entering it again,
//! Enter dropping on it); and disabling and enabling it (a native drag entering it that it
//! ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    fixtures::dnd::{DndActions, ExternalData},
    harness::fixture_test,
    pages::{DragKind, ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_drop";
const TARGET: &str = "#perf-drop";

/// Registers the performance tests of `use_drop`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(NativeEnterMoveAndLeave))
        .with(fixture_test(NativeDrop))
        .with(fixture_test(KeyboardDrag))
        .with(fixture_test(DisableAndEnable))
}

/// Opens the fixture once the live announcer exists (keyboard drags announce); returns the
/// target.
async fn open(page: &Page<'_>) -> Result<WebElement, Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    page.element(TARGET).await
}

/// Starts a native drag of the text "hello world" from outside the page.
async fn begin_external_drag(page: &Page<'_>) -> Result<(), Report> {
    DndActions::new(page)
        .begin_external_drag(&[ExternalData::Text {
            kind: "text/plain",
            data: "hello world",
        }])
        .await
}

/// Mounting the target and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    page.control("unmount").await?;
    page.wait_for_count(TARGET, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TARGET).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TARGET, 0).await?;
    Ok(())
}

/// A native drag entering the target, moving over it (at the same point: nothing to decide; at
/// another point) and leaving it.
#[browser_test]
pub async fn native_enter_move_and_leave(page: &Page<'_>) -> Result<(), Report> {
    let target = open(page).await?;
    let dnd = DndActions::new(page);
    begin_external_drag(page).await?;
    page.measure("enter", dnd.fire_drag_event(&target, DragKind::Enter, &[]))
        .await?;
    target
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    page.measure("over", dnd.fire_drag_event(&target, DragKind::Over, &[]))
        .await?;
    page.measure(
        "over another point",
        dnd.fire_drag_event_at(&target, DragKind::Over, &[], Some((2.0, 2.0))),
    )
    .await?;
    assert_that!(page.control("moves").await?).is_equal_to(1.0);
    page.measure("leave", dnd.fire_drag_event(&target, DragKind::Leave, &[]))
        .await?;
    target.wait_for_attr("data-drop-target", None).await?;
    assert_that!(page.control("enters").await?).is_equal_to(1.0);
    assert_that!(page.control("exits").await?).is_equal_to(1.0);
    Ok(())
}

/// A native drop on the target (a move).
#[browser_test]
pub async fn native_drop(page: &Page<'_>) -> Result<(), Report> {
    let target = open(page).await?;
    let dnd = DndActions::new(page);
    begin_external_drag(page).await?;
    dnd.fire_drag_events(&target, &[DragKind::Enter, DragKind::Over])
        .await?;
    target
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    page.measure("drop", dnd.fire_drag_event(&target, DragKind::Drop, &[]))
        .await?;
    target.wait_for_attr("data-drop-target", None).await?;
    assert_that!(page.control("drops").await?).is_equal_to(1.0);
    assert_that!(page.control("last-operation").await?).is_equal_to(1.0);
    Ok(())
}

/// A keyboard drag (started on the draggable, entering the target): Tab leaving the target (to
/// the draggable) and entering it again, and Enter dropping on it.
#[browser_test]
pub async fn keyboard_drag(page: &Page<'_>) -> Result<(), Report> {
    let target = open(page).await?;
    let draggable = page.element("#perf-draggable").await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&draggable).await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(&target).await?;
    target
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    page.measure("leave", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&draggable).await?;
    target.wait_for_attr("data-drop-target", None).await?;
    page.measure("enter", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&target).await?;
    target
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    page.measure("drop", page.send_keys(Key::Enter)).await?;
    draggable.wait_for_attr("data-dragging", None).await?;
    assert_that!(page.control("drops").await?).is_equal_to(1.0);
    assert_that!(page.control("enters").await?).is_equal_to(2.0);
    Ok(())
}

/// Disabling the target, a native drag entering it that it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    let target = open(page).await?;
    page.measure("disable", page.control("disable")).await?;
    begin_external_drag(page).await?;
    page.measure(
        "ignored enter",
        DndActions::new(page).fire_drag_event(&target, DragKind::Enter, &[]),
    )
    .await?;
    assert_that!(page.control("enters").await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    target.wait_for_attr("data-drop-target", None).await?;
    Ok(())
}
