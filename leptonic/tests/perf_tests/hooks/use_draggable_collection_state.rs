// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_draggable_collection_state`
//! (`/perf/hooks/use_draggable_collection_state`): the drag state of a grid list of 10 or 1,000
//! draggable rows (the fixture of `use_draggable_item`). Measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_draggable_collection_state.json`): mounting and
//! unmounting the list; the selection it follows changing (all rows, none); its reads (how many
//! rows and items a drag of the selection carries); a native drag starting and ending (the
//! dragged keys); a keyboard drop outside the list.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    fixtures::dnd::DndActions,
    harness::fixture_test,
    pages::{DragKind, ElementActions, Page},
    perf_tests::hooks::use_draggable_item::{
        keyboard_drop_on_the_target, mount_and_unmount, native_drag, open, row,
    },
};

const PATH: &str = "/perf/hooks/use_draggable_collection_state";

/// Registers the performance tests of `use_draggable_collection_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(SelectionSmall))
        .with(fixture_test(SelectionLarge))
        .with(fixture_test(ReadsOfTheSelection))
        .with(fixture_test(NativeDragSmall))
        .with(fixture_test(NativeDragLarge))
        .with(fixture_test(DragTheSelection))
        .with(fixture_test(KeyboardDrop))
}

/// Mounting and unmounting the state of 10 rows.
#[browser_test]
pub async fn mount_and_unmount_small(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, PATH, 10).await
}

/// Mounting and unmounting the state of 1,000 rows.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, PATH, 1_000).await
}

/// Selecting all of `count` rows (a drag of the first drags all of them) and none.
async fn selection(page: &Page<'_>, count: u32) -> Result<(), Report> {
    open(page, PATH, count).await?;
    page.measure("select all", page.control("select-all"))
        .await?;
    assert_that!(page.control("drag-count").await?).is_equal_to(f64::from(count));
    page.measure("select none", page.control("clear-selection"))
        .await?;
    assert_that!(page.control("drag-count").await?).is_equal_to(1.0);
    Ok(())
}

/// Selecting all of 10 rows and none.
#[browser_test]
pub async fn selection_small(page: &Page<'_>) -> Result<(), Report> {
    selection(page, 10).await
}

/// Selecting all of 1,000 rows and none.
#[browser_test]
pub async fn selection_large(page: &Page<'_>) -> Result<(), Report> {
    selection(page, 1_000).await
}

/// With all 1,000 rows selected: how many rows a drag of the first drags, and its items.
#[browser_test]
pub async fn reads_of_the_selection(page: &Page<'_>) -> Result<(), Report> {
    open(page, PATH, 1_000).await?;
    page.control("select-all").await?;
    let count = page
        .measure("drag count", page.control("drag-count"))
        .await?;
    assert_that!(count).is_equal_to(1_000.0);
    let items = page.measure("items", page.control("items")).await?;
    assert_that!(items).is_equal_to(1_000.0);
    Ok(())
}

/// A native drag of the first of 10 rows: its start, a move and its end.
#[browser_test]
pub async fn native_drag_small(page: &Page<'_>) -> Result<(), Report> {
    native_drag(page, PATH, 10).await
}

/// A native drag of the first of 1,000 rows: its start, a move and its end.
#[browser_test]
pub async fn native_drag_large(page: &Page<'_>) -> Result<(), Report> {
    native_drag(page, PATH, 1_000).await
}

/// A native drag of all 1,000 rows (selected): its start and its end.
#[browser_test]
pub async fn drag_the_selection(page: &Page<'_>) -> Result<(), Report> {
    open(page, PATH, 1_000).await?;
    page.control("select-all").await?;
    let first = row(page, 0).await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    let dnd = DndActions::new(page);
    page.measure("start", dnd.fire_drag_event(&first, DragKind::Start, &[]))
        .await?;
    assert_that!(page.control("dragging").await?).is_equal_to(1_000.0);
    page.measure("end", dnd.fire_drag_event(&first, DragKind::End, &[]))
        .await?;
    assert_that!(page.control("dragging").await?).is_equal_to(0.0);
    Ok(())
}

/// A keyboard drop of a row on the target outside the list (not internal).
#[browser_test]
pub async fn keyboard_drop(page: &Page<'_>) -> Result<(), Report> {
    keyboard_drop_on_the_target(page, PATH).await
}
