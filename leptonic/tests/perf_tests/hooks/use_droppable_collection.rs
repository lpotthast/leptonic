// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_droppable_collection` (`/perf/hooks/use_droppable_collection`):
//! a grid list of 10 or 1,000 rows taking drops on itself, between its rows (drop indicators) and
//! on them. Measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_droppable_collection.json`): mounting and unmounting the list; native
//! drags from outside the page (synthesized `DragEvent`s: entering it, moving to another target
//! and over the same point, leaving it, dropping between rows); keyboard drags (ArrowDown, End,
//! Home and PageDown between its drop targets, Enter dropping between rows); and disabling and
//! enabling it (a native drag entering it that it ignores). The fixture and the helpers here are
//! also those of `use_droppable_collection_state`, `use_droppable_item` and `use_drop_indicator`.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    fixtures::dnd::{DndActions, ExternalData},
    harness::fixture_test,
    pages::{DragKind, ElementActions, Page, css},
};

const PATH: &str = "/perf/hooks/use_droppable_collection";

/// Registers the performance tests of `use_droppable_collection`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(NativeDragSmall))
        .with(fixture_test(NativeDragLarge))
        .with(fixture_test(NativeDrop))
        .with(fixture_test(KeyboardNavigationSmall))
        .with(fixture_test(KeyboardNavigationLarge))
        .with(fixture_test(KeyboardDrop))
        .with(fixture_test(DisableAndEnable))
}

/// The cell of the row "Item `index`" (by its number: a lookup by text among 1,000 rows is
/// slow).
pub(crate) async fn cell(page: &Page<'_>, index: u32) -> Result<WebElement, Report> {
    page.element(format!("#perf-list [data-row='{index}'] [role=gridcell]"))
        .await
}

/// The row "Item `index`" (by its number: a lookup by text among 1,000 rows is slow).
pub(crate) async fn row(page: &Page<'_>, index: u32) -> Result<WebElement, Report> {
    page.element(format!("#perf-list [data-row='{index}']"))
        .await
}

/// The drop indicator `name` (`root`, `before-item-<i>`, `after-item-<i>`).
pub(crate) async fn indicator(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(format!("#perf-list [data-target='{name}']"))
        .await
}

/// The current drop target: the `data-target` of the drop indicator that is it, or `row <text>`
/// for a row, or `none`. Read in one script.
pub(crate) async fn drop_target(page: &Page<'_>) -> Result<String, Report> {
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
    page.wait_for_count(
        "#perf-list [role=row][aria-selected]",
        usize::try_from(count)?,
    )
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
pub(crate) async fn begin_external_drag(page: &Page<'_>) -> Result<(), Report> {
    DndActions::new(page)
        .begin_external_drag(&[ExternalData::Text {
            kind: "text/plain",
            data: "hello world",
        }])
        .await
}

/// Starts a keyboard drag on the drag source ("Drag me"): the list itself becomes the drop
/// target (its root drop indicator gets focus).
pub(crate) async fn start_keyboard_drag(page: &Page<'_>) -> Result<(), Report> {
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

/// A native drag over a list of `count` rows: entering it (before the first row), moving to
/// another target (on the first row), over the same point again, and leaving it.
async fn native_drag(page: &Page<'_>, count: u32) -> Result<(), Report> {
    open(page, PATH, count).await?;
    let first = cell(page, 0).await?;
    let dnd = DndActions::new(page);
    begin_external_drag(page).await?;
    page.measure(
        "enter",
        dnd.fire_drag_event_at(&first, DragKind::Enter, &[], Some((1.0, 1.0))),
    )
    .await?;
    assert_that!(drop_target(page).await?).is_equal_to("before-item-0".to_owned());
    page.measure(
        "move to another target",
        dnd.fire_drag_event_at(&first, DragKind::Over, &[], Some((1.0, 15.0))),
    )
    .await?;
    assert_that!(drop_target(page).await?).is_equal_to("row Item 0".to_owned());
    page.measure(
        "over the same point",
        dnd.fire_drag_event_at(&first, DragKind::Over, &[], Some((1.0, 15.0))),
    )
    .await?;
    page.measure(
        "leave",
        dnd.fire_drag_event_at(&first, DragKind::Leave, &[], Some((1.0, 15.0))),
    )
    .await?;
    assert_that!(drop_target(page).await?).is_equal_to("none".to_owned());
    assert_that!(page.control("exits").await?).is_equal_to(2.0);
    Ok(())
}

/// A native drag over 10 rows.
#[browser_test]
pub async fn native_drag_small(page: &Page<'_>) -> Result<(), Report> {
    native_drag(page, 10).await
}

/// A native drag over 1,000 rows.
#[browser_test]
pub async fn native_drag_large(page: &Page<'_>) -> Result<(), Report> {
    native_drag(page, 1_000).await
}

/// A native drop between the first two of 10 rows: the dropped text is inserted as a row.
#[browser_test]
pub async fn native_drop(page: &Page<'_>) -> Result<(), Report> {
    open(page, PATH, 10).await?;
    let second = cell(page, 1).await?;
    let dnd = DndActions::new(page);
    begin_external_drag(page).await?;
    dnd.fire_drag_event_at(&second, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    assert_that!(drop_target(page).await?).is_equal_to("before-item-1".to_owned());
    page.measure(
        "drop",
        dnd.fire_drag_event_at(&second, DragKind::Drop, &[], Some((1.0, 1.0))),
    )
    .await?;
    page.element(css("#perf-list [role=gridcell]").text("hello world"))
        .await?;
    assert_that!(page.control("inserts").await?).is_equal_to(1.0);
    Ok(())
}

/// Keyboard navigation between the drop targets of `count` rows during a keyboard drag:
/// ArrowDown (before the first row), ArrowDown (on it), End (after the last row), Home (the list
/// itself), PageDown (a page further).
async fn keyboard_navigation(page: &Page<'_>, count: u32) -> Result<(), Report> {
    open(page, PATH, count).await?;
    start_keyboard_drag(page).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&indicator(page, "before-item-0").await?)
        .await?;
    page.measure("arrow down to a row", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&row(page, 0).await?).await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    let last = format!("after-item-{}", count - 1);
    page.wait_for_focus(&indicator(page, &last).await?).await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&indicator(page, "root").await?).await?;
    page.measure("page down", page.send_keys(Key::PageDown))
        .await?;
    assert_that!(drop_target(page).await?).is_not_equal_to("root".to_owned());
    page.send_keys(Key::Escape).await?;
    page.element("#perf-draggable")
        .await?
        .wait_for_attr("data-dragging", None)
        .await?;
    Ok(())
}

/// Keyboard navigation between the drop targets of 10 rows.
#[browser_test]
pub async fn keyboard_navigation_small(page: &Page<'_>) -> Result<(), Report> {
    keyboard_navigation(page, 10).await
}

/// Keyboard navigation between the drop targets of 1,000 rows.
#[browser_test]
pub async fn keyboard_navigation_large(page: &Page<'_>) -> Result<(), Report> {
    keyboard_navigation(page, 1_000).await
}

/// A keyboard drop before the first of 10 rows (Enter): the dropped text is inserted as a row.
#[browser_test]
pub async fn keyboard_drop(page: &Page<'_>) -> Result<(), Report> {
    open(page, PATH, 10).await?;
    start_keyboard_drag(page).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&indicator(page, "before-item-0").await?)
        .await?;
    page.measure("drop", page.send_keys(Key::Enter)).await?;
    page.element(css("#perf-list [role=gridcell]").text("hello world"))
        .await?;
    assert_that!(page.control("inserts").await?).is_equal_to(1.0);
    Ok(())
}

/// Disabling dropping on 10 rows, a native drag entering the list that it ignores, and enabling
/// it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page, PATH, 10).await?;
    let first = cell(page, 0).await?;
    page.measure("disable", page.control("disable")).await?;
    begin_external_drag(page).await?;
    page.measure(
        "ignored enter",
        DndActions::new(page).fire_drag_event_at(&first, DragKind::Enter, &[], Some((1.0, 1.0))),
    )
    .await?;
    assert_that!(drop_target(page).await?).is_equal_to("none".to_owned());
    page.measure("enable", page.control("enable")).await?;
    assert_that!(page.control("enters").await?).is_equal_to(0.0);
    Ok(())
}
