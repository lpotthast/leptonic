// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_droppable_item` (`/perf/hooks/use_droppable_item`): the rows of
//! a grid list of 10 or 1,000 rows taking drops on them . Measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_droppable_item.json`): mounting and unmounting the
//! rows; a keyboard drag starting (every row checks whether it takes the drop) and being canceled;
//! a row becoming the target of a keyboard drag (it gets focus) and Enter dropping on it; a native
//! drop on a row.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    fixtures::dnd::{DndActions, ExternalData},
    harness::fixture_test,
    pages::{DragKind, ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_droppable_item";

/// Registers the performance tests of `use_droppable_item`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(KeyboardDragSmall))
        .with(fixture_test(KeyboardDragLarge))
        .with(fixture_test(KeyboardDropOnARow))
        .with(fixture_test(NativeDropOnARow))
}
/// The cell of the row "Item `index`" (by its number: a lookup by text among 1,000 rows is
/// slow).
async fn cell(page: &Page<'_>, index: u32) -> Result<WebElement, Report> {
    page.element(format!("#perf-list [data-row='{index}'] [role=gridcell]"))
        .await
}

/// The row "Item `index`" (by its number: a lookup by text among 1,000 rows is slow).
async fn row(page: &Page<'_>, index: u32) -> Result<WebElement, Report> {
    page.element(format!("#perf-list [data-row='{index}']"))
        .await
}

/// The drop indicator `name` (`root`, `before-item-<i>`, `after-item-<i>`).
async fn indicator(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(format!("#perf-list [data-target='{name}']"))
        .await
}

/// The current drop target: the `data-target` of the drop indicator that is it, or `row <text>`
/// for a row, or `none`. Read in one script.
async fn drop_target(page: &Page<'_>) -> Result<String, Report> {
    page.low_level()
        .eval(
            "const target = document.querySelector('#perf-list [data-drop-target]');
             if (!target) return 'none';
             return target.dataset.target ?? `row ${target.textContent}`;",
            vec![],
        )
        .await
}

/// Opens the droppable list fixture at `path` with `count` rows (10 or 1,000), once the live
/// announcer exists (keyboard drags announce).
async fn open(page: &Page<'_>, path: &str, count: u32) -> Result<(), Report> {
    page.goto_path(path).await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    if count != 10 {
        page.control("unmount").await?;
        page.wait_for_count("#perf-list", 0).await?;
        page.control("large").await?;
        page.control("mount").await?;
    }
    page.wait_for_count(
        "#perf-list [role=row][aria-selected]",
        usize::try_from(count)?,
    )
    .await
}

/// Mounting and unmounting the list of `count` rows (10 or 1,000) at `path` (once the live
/// announcer exists: it adds its first entry 100ms after the page announced).
async fn mount_and_unmount(page: &Page<'_>, path: &str, count: u32) -> Result<(), Report> {
    page.goto_path(path).await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    page.control("unmount").await?;
    page.wait_for_count("#perf-list", 0).await?;
    if count != 10 {
        page.control("large").await?;
    }
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(
        "#perf-list [role=row][aria-selected]",
        usize::try_from(count)?,
    )
    .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count("#perf-list", 0).await?;
    Ok(())
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

/// Starts a keyboard drag on the drag source ("Drag me"): the list itself becomes the drop
/// target (its root drop indicator gets focus).
async fn start_keyboard_drag(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#perf-draggable").await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(&indicator(page, "root").await?).await
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

/// A keyboard drag over `count` rows starting (Enter on the drag source) and being canceled
/// (Escape).
async fn keyboard_drag(page: &Page<'_>, count: u32) -> Result<(), Report> {
    open(page, PATH, count).await?;
    let draggable = page.element("#perf-draggable").await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&draggable).await?;
    page.measure("start", page.send_keys(Key::Enter)).await?;
    page.wait_for_focus(&indicator(page, "root").await?).await?;
    page.measure("cancel", page.send_keys(Key::Escape)).await?;
    page.wait_for_focus(&draggable).await?;
    draggable.wait_for_attr("data-dragging", None).await?;
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

/// The first of 10 rows becoming the target of a keyboard drag (ArrowDown: it gets focus), and
/// Enter dropping on it.
#[browser_test]
pub async fn keyboard_drop_on_a_row(page: &Page<'_>) -> Result<(), Report> {
    open(page, PATH, 10).await?;
    start_keyboard_drag(page).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&indicator(page, "before-item-0").await?)
        .await?;
    page.measure("target", page.send_keys(Key::Down)).await?;
    page.wait_for_focus(&row(page, 0).await?).await?;
    page.measure("drop", page.send_keys(Key::Enter)).await?;
    assert_that!(|| page.control("item-drops"))
        .eventually_ok()
        .satisfies(|drops| {
            drops.is_equal_to(1.0);
        })
        .await;
    Ok(())
}

/// A native drop on the first of 10 rows.
#[browser_test]
pub async fn native_drop_on_a_row(page: &Page<'_>) -> Result<(), Report> {
    open(page, PATH, 10).await?;
    let first = cell(page, 0).await?;
    let dnd = DndActions::new(page);
    begin_external_drag(page).await?;
    dnd.fire_drag_event_at(&first, DragKind::Enter, &[], Some((1.0, 15.0)))
        .await?;
    assert_that!(drop_target(page).await?).is_equal_to("row Item 0".to_owned());
    page.measure(
        "drop",
        dnd.fire_drag_event_at(&first, DragKind::Drop, &[], Some((1.0, 15.0))),
    )
    .await?;
    assert_that!(page.control("item-drops").await?).is_equal_to(1.0);
    Ok(())
}
