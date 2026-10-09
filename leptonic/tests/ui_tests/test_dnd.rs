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
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::{
    fixtures::dnd::DndActions,
    pages::{
        DragKind, ElementActions, MouseKind, Page, Platform, PointerKind, SyntheticEvent, css, role,
    },
};

const LOG: &str = "test-dnd-log";
const TARGETS_LOG: &str = "test-dnd-targets-log";
const TARGETS_PAGE: &str = "test-page-hook-dnd-targets";
const ACTION: &str = "dnd-action";
const PATH: &str = "/hooks/dnd";
const TARGETS_PATH: &str = "/hooks/dnd-targets";

/// A real mouse click, a few pixels off the element's center.
async fn click_off_center(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_with_offset(element, 7, 3)
        .click()
        .perform()
        .await?;
    Ok(())
}

/// The drop target button `label`, including targets made inert during a drag.
async fn droppable(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(css("[role=button]").text(label)).await
}

/// The drag source ("Drag me").
async fn draggable(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Button).text("Drag me")).await
}

/// Opens `/hooks/dnd-targets` with the query and starts a keyboard drag of "Drag me" (focus moves
/// to the first drop target).
async fn start_keyboard_drag(page: &Page<'_>, query: &str) -> Result<WebElement, Report> {
    page.goto_path(&format!("{TARGETS_PATH}{query}")).await?;
    let source = draggable(page).await?;
    page.element(role(AriaRole::Button).text("Before"))
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

/// Enter on the drag source starts a keyboard drag and focuses the nearest drop target, Tab moves
/// to the next one and Enter drops there; drop targets are described only during the drag
/// ("should perform basic drag and drop").
#[browser_test]
pub async fn basic_drag_and_drop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let draggable = page.element("#test-dnd-draggable").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    assert_that!(draggable)
        .attribute("data-dragging")
        .await
        .is_none();
    assert_that!(draggable)
        .has_attribute("draggable")
        .await
        .is_equal_to("true");

    page.element("#test-dnd-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&draggable).await?;
    assert_that!(draggable)
        .accessible_description()
        .await
        .is_equal_to("Press Enter to start dragging.");

    page.send_keys(Key::Enter).await?;
    // The drag moves focus to the nearest drop target.
    page.wait_for_focus(&target_1).await?;
    draggable
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    target_1
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    assert_that!(target_1)
        .accessible_description()
        .await
        .is_equal_to("Press Enter to drop. Press Escape to cancel drag.");
    DndActions::new(page)
        .wait_for_log(LOG, &["dragstart", "dropenter 1"])
        .await?;

    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&target_2).await?;
    target_1.wait_for_attr("data-drop-target", None).await?;
    target_2
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &["dragstart", "dropenter 1", "dropexit 1", "dropenter 2"],
        )
        .await?;

    page.send_keys(Key::Enter).await?;
    DndActions::new(page)
        .wait_for_log(
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
    draggable.wait_for_attr("data-dragging", None).await?;
    target_2.wait_for_attr("data-drop-target", None).await?;
    // Drop targets are only described during drags.
    assert_that!(target_2)
        .attribute("aria-describedby")
        .await
        .is_none();
    Ok(())
}

/// Escape cancels a keyboard drag and returns focus to the drag source ("should cancel the drag
/// when pressing the escape key").
#[browser_test]
pub async fn escape_cancels(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let draggable = page.element("#test-dnd-draggable").await?;
    page.element("#test-dnd-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Enter).await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_focus(&draggable).await?;
    DndActions::new(page)
        .wait_for_log(
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
        .element(css("[role=row]").has(css("[role=gridcell]").text(letter)))
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

/// A keyboard drag of a row starts after it and makes the other rows inert; arrow keys move
/// between the insertion positions, and Enter moves the row there.
#[browser_test]
pub async fn reorder_a_list(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-dnd-before-list").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&row(page, "A").await?).await?;
    page.send_keys(Key::Down).await?;
    let b = row(page, "B").await?;
    page.wait_for_focus(&b).await?;

    page.send_keys(Key::Enter).await?;
    expect_focused_indicator(page, "Insert between B and C").await?;
    assert_that!(b)
        .has_attribute("data-dragging")
        .await
        .is_equal_to("true");
    // Rows can't be dropped on: they are hidden (inert) during the drag.
    assert_that!(row(page, "A").await?.is_within("[inert]").await?).is_true();
    assert_that!(b.is_within("[inert]").await?).is_false();

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
    assert_that!(row(page, "A").await?.is_within("[inert]").await?).is_false();
    assert_that!(b).attribute("aria-hidden").await.is_none();
    Ok(())
}

/// A native drag carries the source's text, every drop target it enters accepts it as a move, and
/// the drop on the second target ends it as a move ("should perform basic drag and drop").
#[browser_test]
pub async fn native_basic_drag_and_drop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let draggable = page.element("#test-dnd-draggable").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;

    DndActions::new(page)
        .fire_drag_event(&draggable, DragKind::Start, &[])
        .await?;
    DndActions::new(page)
        .wait_for_log(LOG, &["dragstart"])
        .await?;
    let transfer = DndActions::new(page).transfer().await?;
    assert_that!(transfer.data)
        .contains_exactly([("text/plain".to_owned(), "hello world".to_owned())]);
    assert_that!(transfer.effect_allowed).is_equal_to("all");
    draggable
        .wait_for_attr("data-dragging", Some("true"))
        .await?;

    // Entering a drop target accepts the drag (prevents the default).
    assert_that!(
        DndActions::new(page)
            .fire_drag_event(&target_1, DragKind::Enter, &[])
            .await?
    )
    .is_true();
    assert_that!(
        DndActions::new(page)
            .fire_drag_event(&target_1, DragKind::Over, &[])
            .await?
    )
    .is_true();
    let transfer = DndActions::new(page).transfer().await?;
    assert_that!(transfer.drop_effect).is_equal_to("move");
    DndActions::new(page)
        .wait_for_log(LOG, &["dragstart", "dropenter 1"])
        .await?;
    target_1
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;

    DndActions::new(page)
        .fire_drag_event(&target_1, DragKind::Leave, &[])
        .await?;
    DndActions::new(page)
        .fire_drag_event(&target_2, DragKind::Enter, &[])
        .await?;
    DndActions::new(page)
        .fire_drag_event(&target_2, DragKind::Over, &[])
        .await?;
    target_1.wait_for_attr("data-drop-target", None).await?;
    target_2
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;

    assert_that!(
        DndActions::new(page)
            .fire_drag_event(&target_2, DragKind::Drop, &[])
            .await?
    )
    .is_true();
    DndActions::new(page)
        .fire_drag_event(&draggable, DragKind::End, &[])
        .await?;
    DndActions::new(page)
        .wait_for_log(
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
    DndActions::new(page)
        .log_stays(
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
            std::time::Duration::from_millis(100),
        )
        .await?;
    draggable.wait_for_attr("data-dragging", None).await?;
    target_2.wait_for_attr("data-drop-target", None).await?;
    Ok(())
}

// ---- /hooks/dnd-targets: keyboard navigation ----

/// During a keyboard drag, Tab moves through the drop targets only and from the last one back to
/// the drag source ("should Tab forward and skip non drop target elements").
#[browser_test]
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
    DndActions::new(page)
        .wait_for_log(
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
    source.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// During a keyboard drag, Shift+Tab moves backwards through the drop targets and the drag source
/// only ("should Tab backward and skip non drop target elements").
#[browser_test]
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
    source.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// A keyboard drag starts on the ancestor drop target around the drag source and enters only it
/// ("should prefer an ancestor drop target over the nearest drop target").
#[browser_test]
pub async fn prefers_an_ancestor_drop_target(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "?ancestor").await?;
    let ancestor = page
        .element(role(AriaRole::Button).has(role(AriaRole::Button).text("Drag me")))
        .await?;
    page.wait_for_focus(&ancestor).await?;
    ancestor
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    DndActions::new(page)
        .wait_for_log(TARGETS_LOG, &["dragstart", "dropenter 0"])
        .await?;
    DndActions::new(page)
        .log_stays(
            TARGETS_LOG,
            &["dragstart", "dropenter 0"],
            std::time::Duration::from_millis(100),
        )
        .await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// Enter on the drag source during a keyboard drag cancels it instead of starting another one
/// ("should cancel the drag when pressing Enter on the original drag target").
#[browser_test]
pub async fn enter_on_the_drag_source_cancels(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    page.send_keys(Key::Enter).await?;
    source.wait_for_attr("data-dragging", None).await?;
    page.wait_for_focus(&source).await?;
    // One drag only: Enter didn't start another one.
    DndActions::new(page)
        .wait_for_log(
            TARGETS_LOG,
            &["dragstart", "dropenter 1", "dropexit 1", "dragend Cancel"],
        )
        .await?;
    DndActions::new(page)
        .log_stays(
            TARGETS_LOG,
            &["dragstart", "dropenter 1", "dropexit 1", "dragend Cancel"],
            std::time::Duration::from_millis(100),
        )
        .await?;
    Ok(())
}

/// Tab during a keyboard drag skips a drop target in an `aria-hidden` tree ("should ignore drop
/// targets in aria-hidden trees").
#[browser_test]
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
    DndActions::new(page)
        .wait_for_log(
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
    DndActions::new(page)
        .log_stays(
            TARGETS_LOG,
            &[
                "dragstart",
                "dropenter 1",
                "dropexit 1",
                "dropenter 2",
                "dropexit 2",
            ],
            std::time::Duration::from_millis(100),
        )
        .await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

// ---- /hooks/dnd-targets: changing targets ----

/// When the current drop target is removed during a keyboard drag, the first drop target takes
/// the focus ("should handle when a drop target is removed").
#[browser_test]
pub async fn a_removed_drop_target(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&target_2).await?;
    // The current drop target goes: the first one takes over.
    DndActions::new(page)
        .dispatch_action(TARGETS_PAGE, ACTION, "remove-target")
        .await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// When the current drop target is hidden with `aria-hidden` during a keyboard drag, the first
/// drop target takes the focus ("should handle when a drop target is hidden with aria-hidden").
#[browser_test]
pub async fn a_drop_target_hidden_during_the_drag(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&target_2).await?;
    DndActions::new(page)
        .dispatch_action(TARGETS_PAGE, ACTION, "hide-target")
        .await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// A drop target added during a keyboard drag can be tabbed to and stays the current target once
/// entered, so Enter drops on it ("should handle when a drop target is added").
#[browser_test]
pub async fn an_added_drop_target_keeps_the_current_target(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus(&target_1).await?;
    DndActions::new(page)
        .dispatch_action(TARGETS_PAGE, ACTION, "add-target")
        .await?;
    let target_3 = droppable(page, "Drop here 3").await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&target_3).await?;
    DndActions::new(page)
        .wait_for_log(
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
    DndActions::new(page)
        .log_stays(
            TARGETS_LOG,
            &[
                "dragstart",
                "dropenter 1",
                "dropexit 1",
                "dropenter 2",
                "dropexit 2",
                "dropenter 3",
            ],
            std::time::Duration::from_millis(100),
        )
        .await?;
    page.wait_for_focus(&target_3).await?;
    page.send_keys(Key::Enter).await?;
    DndActions::new(page)
        .wait_for_log(
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
    source.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// Once the drag source is hidden with `aria-hidden`, Tab and Shift+Tab cycle through the drop
/// targets only ("should not tab to the original draggable element if it is in an aria-hidden
/// tree").
#[browser_test]
pub async fn a_hidden_drag_source_is_skipped(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus(&target_1).await?;
    DndActions::new(page)
        .dispatch_action(TARGETS_PAGE, ACTION, "hide-draggable")
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
    source.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// Escape cancels a keyboard drag whose source was hidden with `aria-hidden` and leaves the focus
/// on the drop target ("should not restore focus to the original draggable element on Escape if
/// it is in an aria-hidden tree").
#[browser_test]
pub async fn escape_with_a_hidden_drag_source(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus(&target_1).await?;
    DndActions::new(page)
        .dispatch_action(TARGETS_PAGE, ACTION, "hide-draggable")
        .await?;
    target_1
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", None).await?;
    target_1.wait_for_attr("data-drop-target", None).await?;
    // Focus isn't restored into the hidden tree.
    page.focus_stays(&target_1, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

// ---- /hooks/dnd-targets: disabled, operations ----

/// A disabled drag source is neither draggable nor described, and Enter on it starts no drag but
/// reaches its parent ("useDrag should support isDisabled").
#[browser_test]
pub async fn disabled_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{TARGETS_PATH}?disabled-drag"))
        .await?;
    let source = draggable(page).await?;
    assert_that!(source)
        .has_attribute("draggable")
        .await
        .is_equal_to("false");
    assert_that!(source)
        .attribute("data-dragging")
        .await
        .is_none();
    page.element(role(AriaRole::Button).text("Before"))
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    assert_that!(source)
        .attribute("aria-describedby")
        .await
        .is_none();
    page.send_keys(Key::Enter).await?;
    DndActions::new(page)
        .wait_for_log(TARGETS_LOG, &["parent keydown Enter"])
        .await?;
    DndActions::new(page)
        .log_stays(
            TARGETS_LOG,
            &["parent keydown Enter"],
            std::time::Duration::from_millis(100),
        )
        .await?;
    page.wait_for_focus(&source).await?;
    source
        .attr_stays("data-dragging", None, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A keyboard drag skips a disabled drop target: it starts on the other target, and Tab cycles
/// between that target and the drag source ("useDrop should support isDisabled").
#[browser_test]
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
        .attr_stays(
            "data-drop-target",
            None,
            std::time::Duration::from_millis(100),
        )
        .await?;
    assert_that!(target_1)
        .attribute("aria-describedby")
        .await
        .is_none();
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// A keyboard drop on a target whose `get_drop_operation` returns copy is a copy ("should support
/// getDropOperation to override the default operation").
#[browser_test]
pub async fn drop_operation_override(page: &Page<'_>) -> Result<(), Report> {
    start_keyboard_drag(page, "?op=copy").await?;
    page.wait_for_focus(&droppable(page, "Drop here").await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    DndActions::new(page)
        .wait_for_log(
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

/// A drag whose source allows only links drops as a link, even on a target preferring copy
/// ("should support getAllowedDropOperations to limit allowed operations").
#[browser_test]
pub async fn allowed_drop_operations(page: &Page<'_>) -> Result<(), Report> {
    start_keyboard_drag(page, "?allowed=link&op=copy").await?;
    page.wait_for_focus(&droppable(page, "Drop here").await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    DndActions::new(page)
        .wait_for_log(
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

/// A drop target whose `get_drop_operation` returns cancel is inert during a keyboard drag, so
/// Tab skips it ("should hide drop targets where getDropOperation returns "cancel"").
#[browser_test]
pub async fn canceled_targets_are_hidden(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "?cancel-2").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus(&target_1).await?;
    assert_that!(target_2.is_within("[inert]").await?).is_true();
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    page.send_keys(Key::Escape).await?;
    source.wait_for_attr("data-dragging", None).await?;
    assert_that!(|| target_2.is_within("[inert]"))
        .eventually_ok()
        .matches(eq(false))
        .await;
    Ok(())
}

/// Alt + Enter activates the drop target (e.g. opens a folder) without dropping.
#[browser_test]
pub async fn alt_enter_activates(page: &Page<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus(&target_1).await?;
    page.send_keys(Key::Alt + Key::Enter).await?;
    DndActions::new(page)
        .wait_for_log(TARGETS_LOG, &["dragstart", "dropenter 1", "dropactivate 1"])
        .await?;
    DndActions::new(page)
        .log_stays(
            TARGETS_LOG,
            &["dragstart", "dropenter 1", "dropactivate 1"],
            std::time::Duration::from_millis(100),
        )
        .await?;
    assert_that!(source)
        .has_attribute("data-dragging")
        .await
        .is_equal_to("true");
    page.send_keys(Key::Escape).await?;
    DndActions::new(page)
        .wait_for_log(
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

/// A native drag from a disabled drag source writes no data and doesn't start dragging ("useDrag
/// should support isDisabled").
#[browser_test]
pub async fn native_disabled_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{TARGETS_PATH}?disabled-drag"))
        .await?;
    let source = draggable(page).await?;
    DndActions::new(page)
        .fire_drag_event(&source, DragKind::Start, &[])
        .await?;
    let transfer = DndActions::new(page).transfer().await?;
    assert_that!(transfer.types).is_empty();
    source
        .attr_stays("data-dragging", None, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A disabled drop target doesn't accept a native drag, so the drop is cancelled ("useDrop should
/// support isDisabled").
#[browser_test]
pub async fn native_disabled_drop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{TARGETS_PATH}?disabled-drop"))
        .await?;
    let source = draggable(page).await?;
    let target_1 = droppable(page, "Drop here").await?;
    DndActions::new(page)
        .fire_drag_event(&source, DragKind::Start, &[])
        .await?;
    assert_that!(
        DndActions::new(page)
            .fire_drag_event(&target_1, DragKind::Enter, &[])
            .await?
    )
    .is_false();
    DndActions::new(page)
        .fire_drag_event(&target_1, DragKind::Over, &[])
        .await?;
    DndActions::new(page)
        .fire_drag_event(&target_1, DragKind::Drop, &[])
        .await?;
    DndActions::new(page)
        .fire_drag_event(&source, DragKind::End, &[])
        .await?;
    DndActions::new(page)
        .wait_for_log(TARGETS_LOG, &["dragstart", "dragend Cancel"])
        .await?;
    DndActions::new(page)
        .log_stays(
            TARGETS_LOG,
            &["dragstart", "dragend Cancel"],
            std::time::Duration::from_millis(100),
        )
        .await?;
    target_1
        .attr_stays(
            "data-drop-target",
            None,
            std::time::Duration::from_millis(100),
        )
        .await?;
    // The drop reached no `use_drop` target: `use_drag` warns, as upstream's `useDrag` does.
    assert_that!(crate::pages::health::diagnostics(page.low_level().driver()).await?.console_warnings).contains_exactly([
        "Drags initiated from use_drag may only be dropped on a target created with use_drop. This \
         ensures that a keyboard and screen reader accessible alternative is available."
            .to_owned(),
    ]);
    crate::fixtures::take_warnings(page, "Drags initiated from use_drag", 1).await?;
    Ok(())
}

// ---- /hooks/dnd-targets: screen readers ----

/// Starts a screen reader drag: focus and a virtual click on the drag source.
async fn start_virtual_drag(page: &Page<'_>, query: &str) -> Result<WebElement, Report> {
    page.goto_path(&format!("{TARGETS_PATH}{query}")).await?;
    let source = draggable(page).await?;
    source.focus().await?;
    assert_that!(source)
        .accessible_description()
        .await
        .is_equal_to("Click to start dragging.");
    source.virtual_click().await?;
    source.wait_for_attr("data-dragging", Some("true")).await?;
    // The drag manager sets the session up one frame later (then the page becomes inert); clicks
    // before that would go to the drag source itself.
    let input = page.element("input[aria-label='Text field']").await?;
    // The session started.
    assert_that!(|| input.is_within("[inert]"))
        .eventually_ok()
        .matches(eq(true))
        .await;
    Ok(source)
}

/// In a screen reader drag, focusing a drop target makes it the current one and clicking it drops
/// there ("should allow navigating with only focus events").
#[browser_test]
pub async fn navigating_with_focus_events_only(page: &Page<'_>) -> Result<(), Report> {
    let source = start_virtual_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus(&source).await?;
    assert_that!(|| source.accessible_description())
        .eventually_ok()
        .matches(eq("Dragging. Click to cancel drag."))
        .await;
    DndActions::new(page)
        .wait_for_log(TARGETS_LOG, &["dragstart"])
        .await?;
    DndActions::new(page)
        .log_stays(
            TARGETS_LOG,
            &["dragstart"],
            std::time::Duration::from_millis(100),
        )
        .await?;

    target_1.focus().await?;
    target_1
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    assert_that!(target_1)
        .accessible_description()
        .await
        .is_equal_to("Click to drop.");
    target_2.focus().await?;
    target_2
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    target_1.wait_for_attr("data-drop-target", None).await?;

    target_2.virtual_click().await?;
    DndActions::new(page)
        .wait_for_log(
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
    source.wait_for_attr("data-dragging", None).await?;
    assert_that!(target_1)
        .attribute("aria-describedby")
        .await
        .is_none();
    assert_that!(target_2)
        .attribute("aria-describedby")
        .await
        .is_none();
    Ok(())
}

/// A screen reader drag makes everything but the drag source and the drop targets inert until
/// clicking the source cancels it ("should hide all non drop target elements from screen readers
/// while dragging").
#[browser_test]
pub async fn hides_everything_but_drop_targets(page: &Page<'_>) -> Result<(), Report> {
    let source = start_virtual_drag(page, "").await?;
    let input = page.element("input[aria-label='Text field']").await?;
    assert_that!(|| input.is_within("[inert]"))
        .eventually_ok()
        .matches(eq(true))
        .await;
    for label in ["Before", "Not a drop target"] {
        let button = page
            .element(css("button, [role=button]").text(label))
            .await?;
        assert_that!(button.is_within("[inert]").await?)
            .with_detail_message(label)
            .is_true();
    }
    let text = page.element(css("span").text("Text")).await?;
    assert_that!(text.is_within("[inert]").await?).is_true();
    for target in [
        &source,
        &droppable(page, "Drop here").await?,
        &droppable(page, "Drop here 2").await?,
    ] {
        assert_that!(target.is_within("[inert]").await?).is_false();
    }
    // Clicking the drag source again cancels; the page isn't inert anymore.
    source.virtual_click().await?;
    source.wait_for_attr("data-dragging", None).await?;
    let before = page.element(role(AriaRole::Button).text("Before")).await?;
    assert_that!(|| before.is_within("[inert]"))
        .eventually_ok()
        .matches(eq(false))
        .await;
    Ok(())
}

/// Clicking the drag source again cancels a screen reader drag ("should support clicking the
/// original drag target to cancel drag").
#[browser_test]
pub async fn clicking_the_drag_source_cancels(page: &Page<'_>) -> Result<(), Report> {
    let source = start_virtual_drag(page, "").await?;
    source.virtual_click().await?;
    source.wait_for_attr("data-dragging", None).await?;
    assert_that!(|| source.accessible_description())
        .eventually_ok()
        .matches(eq("Click to start dragging."))
        .await;
    DndActions::new(page)
        .wait_for_log(TARGETS_LOG, &["dragstart", "dragend Cancel"])
        .await?;
    DndActions::new(page)
        .log_stays(
            TARGETS_LOG,
            &["dragstart", "dragend Cancel"],
            std::time::Duration::from_millis(100),
        )
        .await?;
    Ok(())
}

/// Focusing a non drop target or blurring everything during a screen reader drag puts the focus
/// back on the current drop target, or on the drag source without one ("should restore focus to
/// the current drop target when focusing a non drop target element", "should restore focus to the
/// drag target when focusing a non drop target element and there is no current drop target",
/// "should restore focus to the current drop target when blurring all elements", "should restore
/// focus to the drag target when blurring all elements and there is no current drop target").
#[browser_test]
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
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    input.focus().await?;
    page.wait_for_focus(&target_1).await?;
    page.blur_focused().await?;
    page.wait_for_focus(&target_1).await?;
    source.virtual_click().await?;
    source.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// Mouse clicks that aren't a screen reader's neither start a drag nor drop during one ("should
/// ignore clicks not from screen readers to start dragging", "should ignore clicks not from screen
/// readers during dragging").
#[browser_test]
pub async fn ignores_clicks_not_from_screen_readers(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(TARGETS_PATH).await?;
    let source = draggable(page).await?;
    // Off center: a pointer at the very center counts as a screen reader's (TalkBack).
    click_off_center(page, &source).await?;
    source
        .attr_stays("data-dragging", None, std::time::Duration::from_millis(100))
        .await?;

    let source = start_virtual_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    target_1.focus().await?;
    target_1
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    click_off_center(page, &target_1).await?;
    source
        .attr_stays(
            "data-dragging",
            Some("true"),
            std::time::Duration::from_millis(100),
        )
        .await?;
    DndActions::new(page)
        .wait_for_log(TARGETS_LOG, &["dragstart", "dropenter 1"])
        .await?;
    DndActions::new(page)
        .log_stays(
            TARGETS_LOG,
            &["dragstart", "dropenter 1"],
            std::time::Duration::from_millis(100),
        )
        .await?;
    source.virtual_click().await?;
    source.wait_for_attr("data-dragging", None).await?;
    Ok(())
}

/// TalkBack's pointer events on Android (`pointerType` mouse, 1×1, no pressure) followed by a
/// click with `detail` 1.
async fn talkback_click(element: &WebElement) -> Result<(), Report> {
    element
        .dispatch(SyntheticEvent::pointer(PointerKind::Down))
        .await?;
    element
        .dispatch(SyntheticEvent::pointer(PointerKind::Up))
        .await?;
    element
        .dispatch(SyntheticEvent::mouse(MouseKind::Click).detail(1))
        .await?;
    Ok(())
}

/// Starts a TalkBack drag on Android: focus and a TalkBack click on the drag source.
async fn start_talkback_drag(page: &Page<'_>) -> Result<WebElement, Report> {
    page.emulate_platform(Platform::Android).await?;
    page.goto_path(TARGETS_PATH).await?;
    let source = draggable(page).await?;
    source.focus().await?;
    talkback_click(&source).await?;
    source.wait_for_attr("data-dragging", Some("true")).await?;
    assert_that!(|| source.accessible_description())
        .eventually_ok()
        .matches(eq("Dragging. Click to cancel drag."))
        .await;
    Ok(source)
}

/// On Android, a TalkBack click on the drag source (a click with `detail` 1 after virtual pointer
/// events) cancels the drag ("should support clicking the original drag target to cancel drag
/// (virtual pointer event)").
#[browser_test]
pub async fn talkback_click_on_the_drag_source_cancels(page: &Page<'_>) -> Result<(), Report> {
    let source = start_talkback_drag(page).await?;
    talkback_click(&source).await?;
    source.wait_for_attr("data-dragging", None).await?;
    assert_that!(|| source.accessible_description())
        .eventually_ok()
        .matches(eq("Click to start dragging."))
        .await;
    DndActions::new(page)
        .wait_for_log(TARGETS_LOG, &["dragstart", "dragend Cancel"])
        .await?;
    Ok(())
}

/// On Android, a TalkBack double tap (click) on a drop target drops there ("should support
/// double tapping the drop target to complete drag (virtual pointer event)").
#[browser_test]
pub async fn talkback_double_tap_on_a_drop_target_drops(page: &Page<'_>) -> Result<(), Report> {
    let source = start_talkback_drag(page).await?;
    let target_1 = droppable(page, "Drop here").await?;
    target_1.focus().await?;
    target_1
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    talkback_click(&target_1).await?;
    source.wait_for_attr("data-dragging", None).await?;
    assert_that!(|| source.accessible_description())
        .eventually_ok()
        .matches(eq("Click to start dragging."))
        .await;
    DndActions::new(page)
        .wait_for_log(
            TARGETS_LOG,
            &[
                "dragstart",
                "dropenter 1",
                "drop 1 hello world Move",
                "dragend Move",
                "dropexit 1",
            ],
        )
        .await?;
    Ok(())
}

/// A drop target added during a screen reader drag is available to the screen reader (not
/// inert) ("should handle when a drop target is added").
#[browser_test]
pub async fn screen_reader_an_added_drop_target(page: &Page<'_>) -> Result<(), Report> {
    start_virtual_drag(page, "").await?;
    DndActions::new(page)
        .dispatch_action(TARGETS_PAGE, ACTION, "add-target")
        .await?;
    let target_3 = droppable(page, "Drop here 3").await?;
    // The session re-validates its drop targets when the page changes.
    page.settle().await?;
    assert_that!(|| target_3.is_within("[inert]"))
        .consistently_ok()
        .matches(eq(false))
        .await;
    Ok(())
}

/// An element that isn't a drop target, added during a screen reader drag, is hidden from the
/// screen reader (inert) until the drag ends ("should handle when a non drop target element is
/// added").
#[browser_test]
pub async fn screen_reader_an_added_non_drop_target(page: &Page<'_>) -> Result<(), Report> {
    let source = start_virtual_drag(page, "").await?;
    DndActions::new(page)
        .dispatch_action(TARGETS_PAGE, ACTION, "add-input")
        .await?;
    let input = page.element("input[aria-label='Text field 2']").await?;
    assert_that!(|| input.is_within("[inert]"))
        .eventually_ok()
        .matches(eq(true))
        .await;
    source.virtual_click().await?;
    source.wait_for_attr("data-dragging", None).await?;
    assert_that!(|| input.is_within("[inert]"))
        .eventually_ok()
        .matches(eq(false))
        .await;
    let first = page.element("input[aria-label='Text field']").await?;
    assert_that!(first.is_within("[inert]").await?).is_false();
    Ok(())
}

/// A drop target removed during a screen reader drag leaves the others available and the drag
/// going ("should handle when a drop target is removed").
#[browser_test]
pub async fn screen_reader_a_removed_drop_target(page: &Page<'_>) -> Result<(), Report> {
    let source = start_virtual_drag(page, "").await?;
    DndActions::new(page)
        .dispatch_action(TARGETS_PAGE, ACTION, "remove-target")
        .await?;
    page.wait_for_count(css("[role=button]").text("Drop here 2"), 0)
        .await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.settle().await?;
    assert_that!(|| target_1.is_within("[inert]"))
        .consistently_ok()
        .matches(eq(false))
        .await;
    source
        .attr_stays(
            "data-dragging",
            Some("true"),
            std::time::Duration::from_millis(100),
        )
        .await?;
    Ok(())
}

/// A drop target hidden with `aria-hidden` during a screen reader drag stays hidden; the others
/// stay available ("should handle when a drop target is hidden with aria-hidden").
#[browser_test]
pub async fn screen_reader_a_hidden_drop_target(page: &Page<'_>) -> Result<(), Report> {
    start_virtual_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    assert_that!(target_2.is_within("[inert], [aria-hidden=true]").await?).is_false();
    DndActions::new(page)
        .dispatch_action(TARGETS_PAGE, ACTION, "hide-target")
        .await?;
    assert_that!(|| target_2.is_within("[inert], [aria-hidden=true]"))
        .eventually_ok()
        .matches(eq(true))
        .await;
    page.settle().await?;
    assert_that!(|| target_1.is_within("[inert]"))
        .consistently_ok()
        .matches(eq(false))
        .await;
    Ok(())
}
