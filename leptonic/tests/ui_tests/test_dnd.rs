// Upstream: react-aria/test/dnd/dnd.test.js @ 99e6102368
//! `use_drag` / `use_drop` between single elements, and reordering a collection: keyboard drags
//! (through the drag manager) and native drags (synthesized `DragEvent`s with a `DataTransfer`;
//! WebDriver's own pointer actions can't start an HTML5 drag). Spec: react-aria's `dnd.test.js`
//! ("keyboard", "screen reader", "native drag and drop").
//!
//! Keyboard navigation between drop targets during a keyboard drag ("keyboard navigation").
//!
//! Drop targets added, removed or hidden during a keyboard drag, and a hidden drag source.
//!
//! Disabled drag sources and drop targets, drop operations, and activating a drop target.
//!
//! Screen reader drags: started and dropped by (virtual) clicks, navigated by focus alone, the
//! rest of the page inert ("screen reader").
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions, dnd::DndActions, role, xpath},
    polling::wait_for,
};

const LOG: &str = "test-dnd-log";
const TARGETS_LOG: &str = "test-dnd-targets-log";
const TARGETS_PAGE: &str = "test-page-hook-dnd-targets";
const ACTION: &str = "dnd-action";

/// A real mouse click, a few pixels off the element's center.
async fn click_off_center(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .action_chain()
        .move_to_element_with_offset(element, 7, 3)
        .click()
        .perform()
        .await?;
    Ok(())
}

/// The drop target button `label`.
async fn droppable(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(role("button").text(label)).await
}

/// The drag source ("Drag me").
async fn draggable(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element(role("button").text("Drag me")).await
}

/// Opens `/hooks/dnd-targets` with the query and starts a keyboard drag of "Drag me" (focus moves
/// to the first drop target).
async fn start_keyboard_drag(page: &Page<'_>, query: &str) -> Result<WebElement, Report> {
    page.goto_path(&format!("/hooks/dnd-targets{query}"))
        .await?;
    let source = draggable(page).await?;
    page.element(role("button").text("Before"))
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    if query.contains("ancestor") {
        // The ancestor drop target comes first.
        page.send_keys(Key::Tab).await?;
    }
    page.wait_for_focus(&source).await?;
    page.send_keys(Key::Enter).await?;
    source.wait_for_attr("data-dragging", Some("true")).await?;
    Ok(source)
}

// ---- /hooks/dnd ----

/// "should perform basic drag and drop" (keyboard): Enter starts the drag and focuses the nearest
/// drop target, Tab moves between targets, Enter drops; sources and targets are described only
/// while it matters.
pub async fn basic_drag_and_drop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd").await?;
    let draggable = page.element("#test-dnd-draggable").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    assert_that!(draggable.attr("data-dragging").await?)
        .get_some()
        .is_equal_to("false");
    assert_that!(draggable.attr("draggable").await?)
        .get_some()
        .is_equal_to("true");

    page.element("#test-dnd-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&draggable).await?;
    assert_that!(draggable.referenced_text("aria-describedby").await?)
        .is_equal_to("Press Enter to start dragging.");

    page.send_keys(Key::Enter).await?;
    // The drag moves focus to the nearest drop target.
    page.wait_for_focus(&target_1).await?;
    draggable
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    target_1
        .wait_for_attr("data-droptarget", Some("true"))
        .await?;
    assert_that!(target_1.referenced_text("aria-describedby").await?)
        .is_equal_to("Press Enter to drop. Press Escape to cancel drag.");
    page.expect_log(LOG, &["dragstart", "dropenter 1"]).await?;

    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&target_2).await?;
    target_1
        .wait_for_attr("data-droptarget", Some("false"))
        .await?;
    target_2
        .wait_for_attr("data-droptarget", Some("true"))
        .await?;
    page.expect_log(
        LOG,
        &["dragstart", "dropenter 1", "dropexit 1", "dropenter 2"],
    )
    .await?;

    page.send_keys(Key::Enter).await?;
    page.expect_log(
        LOG,
        &[
            "dragstart",
            "dropenter 1",
            "dropexit 1",
            "dropenter 2",
            "drop 2 hello world Move",
            // The drag ends before the drop target is left (react-aria's `DragSession::drop`).
            "dragend Move",
            "dropexit 2",
        ],
    )
    .await?;
    page.wait_for_focus(&target_2).await?;
    draggable
        .wait_for_attr("data-dragging", Some("false"))
        .await?;
    target_2
        .wait_for_attr("data-droptarget", Some("false"))
        .await?;
    // Drop targets are only described during drags.
    assert_that!(target_2.attr("aria-describedby").await?).is_none();
    Ok(())
}

/// "should cancel the drag when pressing the escape key": focus returns to the drag source.
pub async fn escape_cancels(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd").await?;
    let draggable = page.element("#test-dnd-draggable").await?;
    page.element("#test-dnd-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Enter).await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_focus(&draggable).await?;
    page.expect_log(
        LOG,
        &["dragstart", "dropenter 1", "dropexit 1", "dragend Cancel"],
    )
    .await?;
    Ok(())
}

/// The row of the letter `letter` in the reorderable list.
async fn row(page: &Page<'_>, letter: &str) -> Result<WebElement, Report> {
    page.element("[role=grid][aria-label='Letters']")
        .await?
        .element(xpath(format!(
            ".//*[@role='row'][.//*[@role='gridcell'][normalize-space(.)='{letter}']]"
        )))
        .await
}

/// Waits until the drop indicator labelled `label` has focus.
async fn expect_focused_indicator(page: &Page<'_>, label: &str) -> Result<(), Report> {
    let indicator = page
        .element(format!(
            "[aria-roledescription='drop indicator'][aria-label='{label}']"
        ))
        .await?;
    page.wait_for_focus(&indicator).await?;
    Ok(())
}

/// Reordering with the keyboard: the drop target starts after the dragged row, arrow keys move
/// between the valid positions (rows can't be dropped on), Enter drops.
pub async fn reorder_a_list(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd").await?;
    page.element("#test-dnd-before-list").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&row(page, "A").await?).await?;
    page.send_keys(Key::Down).await?;
    let b = row(page, "B").await?;
    page.wait_for_focus(&b).await?;

    page.send_keys(Key::Enter).await?;
    expect_focused_indicator(page, "Insert between B and C").await?;
    // A presence flag on collection rows (`""`), unlike the hooks' `"true"`/`"false"`.
    assert_that!(b.attr("data-dragging").await?).is_some();
    // Rows can't be dropped on: they are hidden (inert) during the drag.
    assert_that!(page.is_inert(&row(page, "A").await?).await?).is_true();
    assert_that!(page.is_inert(&b).await?).is_false();

    page.send_keys(Key::Up).await?;
    expect_focused_indicator(page, "Insert between A and B").await?;
    page.send_keys(Key::Up).await?;
    expect_focused_indicator(page, "Insert before A").await?;

    page.send_keys(Key::Enter).await?;
    page.element("#test-dnd-order")
        .await?
        .wait_for_inner_text("BACD")
        .await?;
    let b = row(page, "B").await?;
    page.wait_for_focus(&b).await?;
    assert_that!(page.is_inert(&row(page, "A").await?).await?).is_false();
    assert_that!(b.attr("aria-hidden").await?).is_none();
    Ok(())
}

/// "native drag and drop: should perform basic drag and drop".
pub async fn native_basic_drag_and_drop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd").await?;
    let draggable = page.element("#test-dnd-draggable").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;

    page.fire_drag_event(&draggable, "dragstart", &[]).await?;
    page.expect_log(LOG, &["dragstart"]).await?;
    assert_that!(page.transfer::<String>("getData('text/plain')").await?)
        .is_equal_to("hello world");
    assert_that!(page.transfer::<String>("effectAllowed").await?).is_equal_to("all");
    draggable
        .wait_for_attr("data-dragging", Some("true"))
        .await?;

    // Entering a drop target accepts the drag (prevents the default).
    assert_that!(page.fire_drag_event(&target_1, "dragenter", &[]).await?).is_true();
    assert_that!(page.fire_drag_event(&target_1, "dragover", &[]).await?).is_true();
    assert_that!(page.transfer::<String>("dropEffect").await?).is_equal_to("move");
    page.expect_log(LOG, &["dragstart", "dropenter 1"]).await?;
    target_1
        .wait_for_attr("data-droptarget", Some("true"))
        .await?;

    page.fire_drag_event(&target_1, "dragleave", &[]).await?;
    page.fire_drag_event(&target_2, "dragenter", &[]).await?;
    page.fire_drag_event(&target_2, "dragover", &[]).await?;
    target_1
        .wait_for_attr("data-droptarget", Some("false"))
        .await?;
    target_2
        .wait_for_attr("data-droptarget", Some("true"))
        .await?;

    assert_that!(page.fire_drag_event(&target_2, "drop", &[]).await?).is_true();
    page.fire_drag_event(&draggable, "dragend", &[]).await?;
    page.expect_log_settled(
        LOG,
        &[
            "dragstart",
            "dropenter 1",
            "dropexit 1",
            "dropenter 2",
            "drop 2 hello world Move",
            "dropexit 2",
            "dragend Move",
        ],
    )
    .await?;
    draggable
        .wait_for_attr("data-dragging", Some("false"))
        .await?;
    target_2
        .wait_for_attr("data-droptarget", Some("false"))
        .await?;
    Ok(())
}

// ---- /hooks/dnd-targets: keyboard navigation ----

/// "should Tab forward and skip non drop target elements": past the last drop target, back to the
/// drag source.
pub async fn tab_forward_skips_non_drop_targets(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&target_2).await?;
    // Past the last drop target: back to the drag source.
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&target_1).await?;
    page.expect_log(
        TARGETS_LOG,
        &[
            "dragstart",
            "dropenter 1",
            "dropexit 1",
            "dropenter 2",
            "dropexit 2",
            "dropenter 1",
        ],
    )
    .await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    Ok(())
}

/// "should Tab backward and skip non drop target elements".
pub async fn tab_backward_skips_non_drop_targets(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&target_2).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    Ok(())
}

/// "should prefer an ancestor drop target over the nearest drop target": it is entered first.
pub async fn prefers_an_ancestor_drop_target(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "?ancestor").await?;
    let ancestor = page
        .element("[role=button][data-droptarget]:has([role=button])")
        .await?;
    page.wait_for_focus(&ancestor).await?;
    ancestor
        .wait_for_attr("data-droptarget", Some("true"))
        .await?;
    page.expect_log_settled(TARGETS_LOG, &["dragstart", "dropenter 0"])
        .await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    Ok(())
}

/// "should cancel the drag when pressing Enter on the original drag target": one drag only, Enter
/// doesn't start another one.
pub async fn enter_on_the_drag_source_cancels(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    page.send_keys(Key::Enter).await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    page.wait_for_focus(&source).await?;
    // One drag only: Enter didn't start another one.
    page.expect_log_settled(
        TARGETS_LOG,
        &["dragstart", "dropenter 1", "dropexit 1", "dragend Cancel"],
    )
    .await?;
    Ok(())
}

/// "should ignore drop targets in aria-hidden trees" (target 9).
pub async fn ignores_drop_targets_in_hidden_trees(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "?hidden-tree").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&target_2).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    // Target 9 in the hidden tree is never entered.
    page.expect_log_settled(
        TARGETS_LOG,
        &[
            "dragstart",
            "dropenter 1",
            "dropexit 1",
            "dropenter 2",
            "dropexit 2",
        ],
    )
    .await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    Ok(())
}

// ---- /hooks/dnd-targets: changing targets ----

/// "should handle when a drop target is removed": the first one takes over.
pub async fn a_removed_drop_target(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&target_2).await?;
    // The current drop target goes: the first one takes over.
    page.dispatch_action(TARGETS_PAGE, ACTION, "remove-target")
        .await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    Ok(())
}

/// "should handle when a drop target is hidden with aria-hidden": the first one takes over.
pub async fn a_drop_target_hidden_during_the_drag(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&target_2).await?;
    page.dispatch_action(TARGETS_PAGE, ACTION, "hide-target")
        .await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    Ok(())
}

/// A drop target registered during a drag runs the targets' `get_drop_operation` (which read a
/// signal, the log): the registration must not subscribe to it, or entering the new target (which
/// logs) re-registers it and loses it as the current drop target.
pub async fn an_added_drop_target_keeps_the_current_target(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus(&target_1).await?;
    page.dispatch_action(TARGETS_PAGE, ACTION, "add-target")
        .await?;
    let target_3 = droppable(page, "Drop here 3").await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&target_3).await?;
    page.expect_log_settled(
        TARGETS_LOG,
        &[
            "dragstart",
            "dropenter 1",
            "dropexit 1",
            "dropenter 2",
            "dropexit 2",
            "dropenter 3",
        ],
    )
    .await?;
    page.wait_for_focus(&target_3).await?;
    page.send_keys(Key::Enter).await?;
    page.expect_log(
        TARGETS_LOG,
        &[
            "dragstart",
            "dropenter 1",
            "dropexit 1",
            "dropenter 2",
            "dropexit 2",
            "dropenter 3",
            "drop 3 hello world Move",
            "dragend Move",
            "dropexit 3",
        ],
    )
    .await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    Ok(())
}

/// "should not tab to the original draggable element if it is in an aria-hidden tree": Tab cycles
/// through the drop targets only.
pub async fn a_hidden_drag_source_is_skipped(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus(&target_1).await?;
    page.dispatch_action(TARGETS_PAGE, ACTION, "hide-draggable")
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&target_2).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&target_2).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    Ok(())
}

/// "should not restore focus to the original draggable element on Escape if it is in an aria-hidden
/// tree".
pub async fn escape_with_a_hidden_drag_source(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus(&target_1).await?;
    page.dispatch_action(TARGETS_PAGE, ACTION, "hide-draggable")
        .await?;
    target_1
        .wait_for_attr("data-droptarget", Some("true"))
        .await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    target_1
        .wait_for_attr("data-droptarget", Some("false"))
        .await?;
    // Focus isn't restored into the hidden tree.
    page.focus_stays(&target_1).await?;
    Ok(())
}

// ---- /hooks/dnd-targets: disabled, operations ----

/// "useDrag should support isDisabled" (keyboard): no description, Enter starts nothing and isn't
/// swallowed.
pub async fn disabled_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd-targets?disabled-drag").await?;
    let source = draggable(page).await?;
    assert_that!(source.attr("draggable").await?)
        .get_some()
        .is_equal_to("false");
    assert_that!(source.attr("data-dragging").await?)
        .get_some()
        .is_equal_to("false");
    page.element(role("button").text("Before"))
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    assert_that!(source.attr("aria-describedby").await?).is_none();
    page.send_keys(Key::Enter).await?;
    page.expect_log_settled(TARGETS_LOG, &["parent keydown Enter"])
        .await?;
    page.wait_for_focus(&source).await?;
    source.attr_stays("data-dragging", Some("false")).await?;
    Ok(())
}

/// "useDrop should support isDisabled" (keyboard): the disabled target isn't one.
pub async fn disabled_drop(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "?disabled-drop").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus(&target_2).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&target_2).await?;
    target_1
        .attr_stays("data-droptarget", Some("false"))
        .await?;
    assert_that!(target_1.attr("aria-describedby").await?).is_none();
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    Ok(())
}

/// "should support getDropOperation to override the default operation".
pub async fn drop_operation_override(page: &Page<'_>) -> Result<(), Report> {
    start_keyboard_drag(page, "?op=copy").await?;
    page.wait_for_focus(&droppable(page, "Drop here").await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    page.expect_log(
        TARGETS_LOG,
        &[
            "dragstart",
            "dropenter 1",
            "drop 1 hello world Copy",
            "dragend Copy",
            "dropexit 1",
        ],
    )
    .await?;
    Ok(())
}

/// "should support getAllowedDropOperations to limit allowed operations".
pub async fn allowed_drop_operations(page: &Page<'_>) -> Result<(), Report> {
    start_keyboard_drag(page, "?allowed=link&op=copy").await?;
    page.wait_for_focus(&droppable(page, "Drop here").await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    page.expect_log(
        TARGETS_LOG,
        &[
            "dragstart",
            "dropenter 1",
            "drop 1 hello world Link",
            "dragend Link",
            "dropexit 1",
        ],
    )
    .await?;
    Ok(())
}

/// "should hide drop targets where getDropOperation returns cancel".
pub async fn canceled_targets_are_hidden(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "?cancel-2").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus(&target_1).await?;
    assert_that!(page.is_inert(&target_2).await?).is_true();
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    wait_for("Drop here 2")
        .observing(|| page.is_inert(&target_2))
        .to_be("not inert", |inert| !inert)
        .await?;
    Ok(())
}

/// Alt + Enter activates the drop target (e.g. opens a folder) without dropping.
pub async fn alt_enter_activates(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Alt + Key::Enter).await?;
    page.expect_log_settled(TARGETS_LOG, &["dragstart", "dropenter 1", "dropactivate 1"])
        .await?;
    assert_that!(source.attr("data-dragging").await?)
        .get_some()
        .is_equal_to("true");
    page.send_keys(Key::Escape).await?;
    page.expect_log(
        TARGETS_LOG,
        &[
            "dragstart",
            "dropenter 1",
            "dropactivate 1",
            "dropexit 1",
            "dragend Cancel",
        ],
    )
    .await?;
    Ok(())
}

/// "useDrag/useDrop should support isDisabled" (native): a disabled source writes no data, a
/// disabled target doesn't accept the drag.
pub async fn native_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd-targets?disabled-drag").await?;
    let source = draggable(page).await?;
    page.fire_drag_event(&source, "dragstart", &[]).await?;
    assert_that!(page.transfer::<u32>("types.length").await?).is_equal_to(0);
    source.attr_stays("data-dragging", Some("false")).await?;

    page.goto_path("/hooks/dnd-targets?disabled-drop").await?;
    let source = draggable(page).await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.fire_drag_event(&source, "dragstart", &[]).await?;
    assert_that!(page.fire_drag_event(&target_1, "dragenter", &[]).await?).is_false();
    page.fire_drag_event(&target_1, "dragover", &[]).await?;
    page.fire_drag_event(&target_1, "drop", &[]).await?;
    page.fire_drag_event(&source, "dragend", &[]).await?;
    page.expect_log_settled(TARGETS_LOG, &["dragstart", "dragend Cancel"])
        .await?;
    target_1
        .attr_stays("data-droptarget", Some("false"))
        .await?;
    Ok(())
}

// ---- /hooks/dnd-targets: screen readers ----

/// Starts a screen reader drag: focus and a virtual click on the drag source.
async fn start_virtual_drag(page: &Page<'_>, query: &str) -> Result<WebElement, Report> {
    page.goto_path(&format!("/hooks/dnd-targets{query}"))
        .await?;
    let source = draggable(page).await?;
    source.focus().await?;
    assert_that!(source.referenced_text("aria-describedby").await?)
        .is_equal_to("Click to start dragging.");
    source.virtual_click().await?;
    source.wait_for_attr("data-dragging", Some("true")).await?;
    // The drag manager sets the session up one frame later (then the page becomes inert); clicks
    // before that would go to the drag source itself.
    let input = page.element("input[aria-label='Text field']").await?;
    wait_for("the text field outside the drag session")
        .observing(|| page.is_inert(&input))
        .to_be("inert (the session started)", |inert| *inert)
        .await?;
    Ok(source)
}

/// "should allow navigating with only focus events".
pub async fn navigating_with_focus_events_only(page: &Page<'_>) -> Result<(), Report> {
    let source = start_virtual_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus(&source).await?;
    wait_for("the drag source's description")
        .observing(|| source.referenced_text("aria-describedby"))
        .to_be_equal_to("Dragging. Click to cancel drag.")
        .await?;
    page.expect_log_settled(TARGETS_LOG, &["dragstart"]).await?;

    target_1.focus().await?;
    target_1
        .wait_for_attr("data-droptarget", Some("true"))
        .await?;
    assert_that!(target_1.referenced_text("aria-describedby").await?).is_equal_to("Click to drop.");
    target_2.focus().await?;
    target_2
        .wait_for_attr("data-droptarget", Some("true"))
        .await?;
    target_1
        .wait_for_attr("data-droptarget", Some("false"))
        .await?;

    target_2.virtual_click().await?;
    page.expect_log(
        TARGETS_LOG,
        &[
            "dragstart",
            "dropenter 1",
            "dropexit 1",
            "dropenter 2",
            "drop 2 hello world Move",
            "dragend Move",
            "dropexit 2",
        ],
    )
    .await?;
    page.wait_for_focus(&target_2).await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    assert_that!(target_1.attr("aria-describedby").await?).is_none();
    assert_that!(target_2.attr("aria-describedby").await?).is_none();
    Ok(())
}

/// "should hide all non drop target elements from screen readers while dragging" (with `inert`,
/// as react-aria's `shouldUseInert`).
pub async fn hides_everything_but_drop_targets(page: &Page<'_>) -> Result<(), Report> {
    let source = start_virtual_drag(page, "").await?;
    let input = page.element("input[aria-label='Text field']").await?;
    wait_for("the text field outside the drag session")
        .observing(|| page.is_inert(&input))
        .to_be("inert", |inert| *inert)
        .await?;
    for label in ["Before", "Not a drop target"] {
        let button = page.element(role("button").text(label)).await?;
        assert_that!(page.is_inert(&button).await?)
            .with_detail_message(label)
            .is_true();
    }
    let text = page
        .element(xpath("//span[normalize-space(.)='Text']"))
        .await?;
    assert_that!(page.is_inert(&text).await?).is_true();
    for target in [
        &source,
        &droppable(page, "Drop here").await?,
        &droppable(page, "Drop here 2").await?,
    ] {
        assert_that!(page.is_inert(target).await?).is_false();
    }
    // Clicking the drag source again cancels; the page isn't inert anymore.
    source.virtual_click().await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    let before = page.element(role("button").text("Before")).await?;
    wait_for("Before")
        .observing(|| page.is_inert(&before))
        .to_be("not inert", |inert| !inert)
        .await?;
    Ok(())
}

/// "should support clicking the original drag target to cancel drag".
pub async fn clicking_the_drag_source_cancels(page: &Page<'_>) -> Result<(), Report> {
    let source = start_virtual_drag(page, "").await?;
    source.virtual_click().await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    wait_for("the drag source's description")
        .observing(|| source.referenced_text("aria-describedby"))
        .to_be_equal_to("Click to start dragging.")
        .await?;
    page.expect_log_settled(TARGETS_LOG, &["dragstart", "dragend Cancel"])
        .await?;
    Ok(())
}

/// "should restore focus to the current drop target (or the drag target) when focusing a non
/// drop target element" and "... when blurring all elements".
pub async fn restores_focus_from_non_drop_targets(page: &Page<'_>) -> Result<(), Report> {
    let source = start_virtual_drag(page, "").await?;
    let input = page.element("input[aria-label='Text field']").await?;
    // No current drop target: back to the drag source.
    input.focus().await?;
    page.wait_for_focus(&source).await?;
    page.blur_focused().await?;
    page.wait_for_focus(&source).await?;

    let target_1 = droppable(page, "Drop here").await?;
    target_1.focus().await?;
    target_1
        .wait_for_attr("data-droptarget", Some("true"))
        .await?;
    input.focus().await?;
    page.wait_for_focus(&target_1).await?;
    page.blur_focused().await?;
    page.wait_for_focus(&target_1).await?;
    source.virtual_click().await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    Ok(())
}

/// "should ignore clicks not from screen readers to start dragging" and "... during dragging".
pub async fn ignores_clicks_not_from_screen_readers(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd-targets").await?;
    let source = draggable(page).await?;
    // Off center: a pointer at the very center counts as a screen reader's (TalkBack).
    click_off_center(page, &source).await?;
    source.attr_stays("data-dragging", Some("false")).await?;

    let source = start_virtual_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    target_1.focus().await?;
    target_1
        .wait_for_attr("data-droptarget", Some("true"))
        .await?;
    click_off_center(page, &target_1).await?;
    source.attr_stays("data-dragging", Some("true")).await?;
    page.expect_log_settled(TARGETS_LOG, &["dragstart", "dropenter 1"])
        .await?;
    source.virtual_click().await?;
    source.wait_for_attr("data-dragging", Some("false")).await?;
    Ok(())
}
