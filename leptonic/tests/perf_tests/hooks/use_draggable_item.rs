// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_draggable_item` (`/perf/hooks/use_draggable_item`): the rows
//! of a grid list of 10 or 1,000 draggable rows, and a `use_drop` target after it. Measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_draggable_item.json`): mounting and unmounting the rows; a native
//! drag of a row (synthesized `DragEvent`s: its start, a move, its end) and of all of them (the
//! selection); a keyboard drag of a row (focus, Enter starting it, Escape canceling it, Enter
//! dropping it on the target); and disabling and enabling dragging (a native drag start it
//! ignores). The fixture and the helpers here are also those of
//! `use_draggable_collection_state` and `use_draggable_collection`.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    fixtures::dnd::DndActions,
    harness::fixture_test,
    pages::{DragKind, ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_draggable_item";

/// Registers the performance tests of `use_draggable_item`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(NativeDragSmall))
        .with(fixture_test(NativeDragLarge))
        .with(fixture_test(DragTheSelection))
        .with(fixture_test(KeyboardDragSmall))
        .with(fixture_test(KeyboardDragLarge))
        .with(fixture_test(KeyboardDrop))
        .with(fixture_test(DisableAndEnable))
}

/// The row "Item `index`" (by its number: a lookup by text among 1,000 rows is slow).
pub(crate) async fn row(page: &Page<'_>, index: u32) -> Result<WebElement, Report> {
    page.element(format!("#perf-list [data-row='{index}']"))
        .await
}

/// Opens the draggable list fixture at `path` with `count` rows (10 or 1,000), once the live
/// announcer exists (keyboard drags announce).
pub(crate) async fn open(page: &Page<'_>, path: &str, count: u32) -> Result<(), Report> {
    page.goto_path(path).await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    if count != 10 {
        page.control("unmount").await?;
        page.wait_for_count("#perf-list", 0).await?;
        page.control("large").await?;
        page.control("mount").await?;
    }
    page.wait_for_count("#perf-list [role=row]", usize::try_from(count)?)
        .await
}

/// Mounting and unmounting the list of `count` rows (10 or 1,000) at `path` (once the live
/// announcer exists: it adds its first entry 100ms after the page announced).
pub(crate) async fn mount_and_unmount(
    page: &Page<'_>,
    path: &str,
    count: u32,
) -> Result<(), Report> {
    page.goto_path(path).await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    page.control("unmount").await?;
    page.wait_for_count("#perf-list", 0).await?;
    if count != 10 {
        page.control("large").await?;
    }
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count("#perf-list [role=row]", usize::try_from(count)?)
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count("#perf-list", 0).await?;
    Ok(())
}

/// A native drag of the first of `count` rows at `path`: its start, a move and its end (no
/// drop).
pub(crate) async fn native_drag(page: &Page<'_>, path: &str, count: u32) -> Result<(), Report> {
    open(page, path, count).await?;
    let first = row(page, 0).await?;
    let dnd = DndActions::new(page);
    page.measure("start", dnd.fire_drag_event(&first, DragKind::Start, &[]))
        .await?;
    first.wait_for_attr("data-dragging", Some("true")).await?;
    assert_that!(page.control("dragging").await?).is_equal_to(1.0);
    page.measure(
        "move",
        dnd.fire_drag_event_at(&first, DragKind::Drag, &[], Some((2.0, 2.0))),
    )
    .await?;
    page.measure("end", dnd.fire_drag_event(&first, DragKind::End, &[]))
        .await?;
    first.wait_for_attr("data-dragging", None).await?;
    assert_that!(page.control("ends").await?).is_equal_to(1.0);
    assert_that!(page.control("dragging").await?).is_equal_to(0.0);
    Ok(())
}

/// A keyboard drop of the first of 10 rows at `path` on the target (Enter): the drag ends
/// outside the list.
pub(crate) async fn keyboard_drop_on_the_target(page: &Page<'_>, path: &str) -> Result<(), Report> {
    open(page, path, 10).await?;
    let first = row(page, 0).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&first).await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(&page.element("#perf-drop").await?)
        .await?;
    page.measure("drop", page.send_keys(Key::Enter)).await?;
    first.wait_for_attr("data-dragging", None).await?;
    assert_that!(page.control("drops").await?).is_equal_to(1.0);
    assert_that!(page.control("last-internal").await?).is_equal_to(0.0);
    Ok(())
}

/// Mounting and unmounting 10 rows.
#[browser_test]
pub async fn mount_and_unmount_small(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, PATH, 10).await
}

/// Mounting and unmounting 1,000 rows.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, PATH, 1_000).await
}

/// A native drag of the first of 10 rows.
#[browser_test]
pub async fn native_drag_small(page: &Page<'_>) -> Result<(), Report> {
    native_drag(page, PATH, 10).await
}

/// A native drag of the first of 1,000 rows.
#[browser_test]
pub async fn native_drag_large(page: &Page<'_>) -> Result<(), Report> {
    native_drag(page, PATH, 1_000).await
}

/// A native drag of all 1,000 rows (selected), started on the first: its start and its end.
#[browser_test]
pub async fn drag_the_selection(page: &Page<'_>) -> Result<(), Report> {
    open(page, PATH, 1_000).await?;
    page.control("select-all").await?;
    let first = row(page, 0).await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    let dnd = DndActions::new(page);
    page.measure("start", dnd.fire_drag_event(&first, DragKind::Start, &[]))
        .await?;
    row(page, 999)
        .await?
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    assert_that!(page.control("dragging").await?).is_equal_to(1_000.0);
    page.measure("end", dnd.fire_drag_event(&first, DragKind::End, &[]))
        .await?;
    first.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// A keyboard drag of the first of `count` rows: focusing it (Tab), Enter starting the drag (the
/// target gets focus), Escape canceling it (the row gets focus again).
async fn keyboard_drag(page: &Page<'_>, count: u32) -> Result<(), Report> {
    open(page, PATH, count).await?;
    let first = row(page, 0).await?;
    page.element("#perf-before").await?.click().await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("start", page.send_keys(Key::Enter)).await?;
    page.wait_for_focus(&page.element("#perf-drop").await?)
        .await?;
    first.wait_for_attr("data-dragging", Some("true")).await?;
    page.measure("cancel", page.send_keys(Key::Escape)).await?;
    page.wait_for_focus(&first).await?;
    first.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// A keyboard drag of the first of 10 rows.
#[browser_test]
pub async fn keyboard_drag_small(page: &Page<'_>) -> Result<(), Report> {
    keyboard_drag(page, 10).await
}

/// A keyboard drag of the first of 1,000 rows.
#[browser_test]
pub async fn keyboard_drag_large(page: &Page<'_>) -> Result<(), Report> {
    keyboard_drag(page, 1_000).await
}

/// A keyboard drop of a row on the target.
#[browser_test]
pub async fn keyboard_drop(page: &Page<'_>) -> Result<(), Report> {
    keyboard_drop_on_the_target(page, PATH).await
}

/// Disabling dragging of 1,000 rows, a native drag start it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page, PATH, 1_000).await?;
    let first = row(page, 0).await?;
    page.measure("disable", page.control("disable")).await?;
    first.wait_for_attr("draggable", Some("false")).await?;
    page.measure(
        "ignored start",
        DndActions::new(page).fire_drag_event(&first, DragKind::Start, &[]),
    )
    .await?;
    assert_that!(page.control("starts").await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    first.wait_for_attr("draggable", Some("true")).await?;
    Ok(())
}
