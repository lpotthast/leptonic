// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_drag` (`/perf/hooks/use_drag`): a draggable element and a
//! `use_drop` target. Measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_drag.json`): mounting and unmounting it; native drags (synthesized
//! `DragEvent`s: the start, a move, the end without and after a drop); keyboard drags (focus, Enter
//! starting one, Escape canceling it, Enter dropping on the target); screen reader drags (virtual
//! clicks starting one and dropping); a mouse click (no drag), and disabling and enabling it
//! (a native drag start it ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    fixtures::dnd::DndActions,
    harness::fixture_test,
    pages::{DragKind, ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_drag";
const DRAGGABLE: &str = "#perf-draggable";
const TARGET: &str = "#perf-drop";

/// Registers the performance tests of `use_drag`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(NativeDrag))
        .with(fixture_test(NativeDrop))
        .with(fixture_test(KeyboardDrag))
        .with(fixture_test(KeyboardDrop))
        .with(fixture_test(ScreenReaderDrag))
        .with(fixture_test(MouseClick))
        .with(fixture_test(DisableAndEnable))
}

/// Opens the fixture once the live announcer exists (keyboard drags announce); returns the
/// draggable.
async fn open(page: &Page<'_>) -> Result<WebElement, Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    page.element(DRAGGABLE).await
}

/// Focuses the draggable with the keyboard (Tab from the plain button before it).
async fn focus_with_tab(page: &Page<'_>, draggable: &WebElement) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(draggable).await
}

/// Mounting the draggable and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    page.control("unmount").await?;
    page.wait_for_count(DRAGGABLE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(DRAGGABLE).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(DRAGGABLE, 0).await?;
    Ok(())
}

/// A native drag: its start (marked dragging a frame later), a move and its end (no drop).
#[browser_test]
pub async fn native_drag(page: &Page<'_>) -> Result<(), Report> {
    let draggable = open(page).await?;
    let dnd = DndActions::new(page);
    page.measure(
        "start",
        dnd.fire_drag_event(&draggable, DragKind::Start, &[]),
    )
    .await?;
    draggable
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    assert_that!(page.control("starts").await?).is_equal_to(1.0);
    page.measure(
        "move",
        dnd.fire_drag_event_at(&draggable, DragKind::Drag, &[], Some((2.0, 2.0))),
    )
    .await?;
    assert_that!(page.control("moves").await?).is_equal_to(1.0);
    page.measure("end", dnd.fire_drag_event(&draggable, DragKind::End, &[]))
        .await?;
    draggable.wait_for_attr("data-dragging", None).await?;
    assert_that!(page.control("ends").await?).is_equal_to(1.0);
    assert_that!(page.control("last-operation").await?).is_equal_to(0.0);
    Ok(())
}

/// The end of a native drag after a drop on the target: a move.
#[browser_test]
pub async fn native_drop(page: &Page<'_>) -> Result<(), Report> {
    let draggable = open(page).await?;
    let target = page.element(TARGET).await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&draggable, DragKind::Start, &[])
        .await?;
    draggable
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    dnd.fire_drag_events(&target, &[DragKind::Enter, DragKind::Over, DragKind::Drop])
        .await?;
    assert_that!(page.control("drops").await?).is_equal_to(1.0);
    page.measure("end", dnd.fire_drag_event(&draggable, DragKind::End, &[]))
        .await?;
    draggable.wait_for_attr("data-dragging", None).await?;
    assert_that!(page.control("last-operation").await?).is_equal_to(1.0);
    Ok(())
}

/// A keyboard drag: focusing the draggable (Tab, its description), Enter starting the drag (the
/// target gets focus) and Escape canceling it (the draggable gets focus again).
#[browser_test]
pub async fn keyboard_drag(page: &Page<'_>) -> Result<(), Report> {
    let draggable = open(page).await?;
    page.element("#perf-before").await?.click().await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&draggable).await?;
    page.measure("start", page.send_keys(Key::Enter)).await?;
    page.wait_for_focus(&page.element(TARGET).await?).await?;
    draggable
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    page.measure("cancel", page.send_keys(Key::Escape)).await?;
    page.wait_for_focus(&draggable).await?;
    draggable.wait_for_attr("data-dragging", None).await?;
    assert_that!(page.control("ends").await?).is_equal_to(1.0);
    assert_that!(page.control("last-operation").await?).is_equal_to(0.0);
    Ok(())
}

/// A keyboard drop: Enter on the focused target ends the drag as a move.
#[browser_test]
pub async fn keyboard_drop(page: &Page<'_>) -> Result<(), Report> {
    let draggable = open(page).await?;
    focus_with_tab(page, &draggable).await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(&page.element(TARGET).await?).await?;
    page.measure("drop", page.send_keys(Key::Enter)).await?;
    draggable.wait_for_attr("data-dragging", None).await?;
    assert_that!(page.control("drops").await?).is_equal_to(1.0);
    assert_that!(page.control("last-operation").await?).is_equal_to(1.0);
    Ok(())
}

/// A screen reader drag: a virtual click on the draggable starts it, one on the target drops.
#[browser_test]
pub async fn screen_reader_drag(page: &Page<'_>) -> Result<(), Report> {
    let draggable = open(page).await?;
    let target = page.element(TARGET).await?;
    draggable.focus().await?;
    page.measure("start", draggable.virtual_click()).await?;
    draggable
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    target.focus().await?;
    page.measure("drop", target.virtual_click()).await?;
    draggable.wait_for_attr("data-dragging", None).await?;
    assert_that!(page.control("drops").await?).is_equal_to(1.0);
    Ok(())
}

/// A mouse click off the draggable's center (at its center, it counts as a screen reader's
/// virtual click): no drag.
#[browser_test]
pub async fn mouse_click(page: &Page<'_>) -> Result<(), Report> {
    let draggable = open(page).await?;
    page.measure(
        "click",
        page.low_level()
            .driver()
            .action_chain()
            .move_to_element_with_offset(&draggable, 7, 3)
            .click()
            .perform(),
    )
    .await?;
    assert_that!(page.control("starts").await?).is_equal_to(0.0);
    Ok(())
}

/// Disabling the draggable, a native drag start it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    let draggable = open(page).await?;
    page.measure("disable", page.control("disable")).await?;
    draggable.wait_for_attr("draggable", Some("false")).await?;
    page.measure(
        "ignored start",
        DndActions::new(page).fire_drag_event(&draggable, DragKind::Start, &[]),
    )
    .await?;
    assert_that!(page.control("starts").await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    draggable.wait_for_attr("draggable", Some("true")).await?;
    Ok(())
}
