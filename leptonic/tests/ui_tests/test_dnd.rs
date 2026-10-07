// Upstream: react-aria/test/dnd/dnd.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, dnd::DndPage};

const LOG: &str = "test-dnd-log";
const TARGETS_LOG: &str = "test-dnd-targets-log";
const TARGETS_PAGE: &str = "test-page-hook-dnd-targets";
const ACTION: &str = "dnd-action";

/// `use_drag` / `use_drop` between single elements, and reordering a collection: keyboard drags
/// (through the drag manager) and native drags (synthesized `DragEvent`s with a `DataTransfer`;
/// WebDriver's own pointer actions can't start an HTML5 drag). Spec: react-aria's `dnd.test.js`
/// ("keyboard", "screen reader", "native drag and drop").
pub struct DndTests {}

#[async_trait]
impl BrowserTest<str> for DndTests {
    fn name(&self) -> Cow<'_, str> {
        "dnd_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = DndPage { driver, base_url };
        basic_drag_and_drop(&page).await?;
        escape_cancels(&page).await?;
        reorder_a_list(&page).await?;
        native_basic_drag_and_drop(&page).await?;
        page.expect_no_page_errors().await
    }
}

/// Keyboard navigation between drop targets during a keyboard drag ("keyboard navigation").
pub struct DndKeyboardNavigationTests {}

#[async_trait]
impl BrowserTest<str> for DndKeyboardNavigationTests {
    fn name(&self) -> Cow<'_, str> {
        "dnd_keyboard_navigation_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = DndPage { driver, base_url };
        tab_forward_skips_non_drop_targets(&page).await?;
        tab_backward_skips_non_drop_targets(&page).await?;
        prefers_an_ancestor_drop_target(&page).await?;
        enter_on_the_drag_source_cancels(&page).await?;
        ignores_drop_targets_in_hidden_trees(&page).await?;
        page.expect_no_page_errors().await
    }
}

/// Drop targets added, removed or hidden during a keyboard drag, and a hidden drag source.
pub struct DndChangingTargetsTests {}

#[async_trait]
impl BrowserTest<str> for DndChangingTargetsTests {
    fn name(&self) -> Cow<'_, str> {
        "dnd_changing_targets_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = DndPage { driver, base_url };
        a_removed_drop_target(&page).await?;
        a_drop_target_hidden_during_the_drag(&page).await?;
        an_added_drop_target_keeps_the_current_target(&page).await?;
        a_hidden_drag_source_is_skipped(&page).await?;
        escape_with_a_hidden_drag_source(&page).await?;
        page.expect_no_page_errors().await
    }
}

/// Disabled drag sources and drop targets, drop operations, and activating a drop target.
pub struct DndOperationsTests {}

#[async_trait]
impl BrowserTest<str> for DndOperationsTests {
    fn name(&self) -> Cow<'_, str> {
        "dnd_operations_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = DndPage { driver, base_url };
        disabled_drag(&page).await?;
        disabled_drop(&page).await?;
        drop_operation_override(&page).await?;
        allowed_drop_operations(&page).await?;
        canceled_targets_are_hidden(&page).await?;
        alt_enter_activates(&page).await?;
        native_disabled(&page).await?;
        page.expect_no_page_errors().await
    }
}

/// Screen reader drags: started and dropped by (virtual) clicks, navigated by focus alone, the
/// rest of the page inert ("screen reader").
pub struct DndScreenReaderTests {}

#[async_trait]
impl BrowserTest<str> for DndScreenReaderTests {
    fn name(&self) -> Cow<'_, str> {
        "dnd_screen_reader_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = DndPage { driver, base_url };
        navigating_with_focus_events_only(&page).await?;
        hides_everything_but_drop_targets(&page).await?;
        clicking_the_drag_source_cancels(&page).await?;
        restores_focus_from_non_drop_targets(&page).await?;
        ignores_clicks_not_from_screen_readers(&page).await?;
        page.expect_no_page_errors().await
    }
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

/// A real mouse click, a few pixels off the element's center.
async fn click_off_center(page: &DndPage<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .action_chain()
        .move_to_element_with_offset(element, 7, 3)
        .click()
        .perform()
        .await?;
    Ok(())
}

async fn droppable(page: &DndPage<'_>, label: &str) -> Result<WebElement, Report> {
    page.by_role_and_text("button", label).await
}

async fn draggable(page: &DndPage<'_>) -> Result<WebElement, Report> {
    page.by_role_and_text("button", "Drag me").await
}

/// Opens `/hooks/dnd-targets` with the query and starts a keyboard drag of "Drag me" (focus moves
/// to the first drop target).
async fn start_keyboard_drag(page: &DndPage<'_>, query: &str) -> Result<WebElement, Report> {
    page.goto_path(&format!("/hooks/dnd-targets{query}"))
        .await?;
    let source = draggable(page).await?;
    page.by_role_and_text("button", "Before")
        .await?
        .click()
        .await?;
    page.press_tab().await?;
    if query.contains("ancestor") {
        // The ancestor drop target comes first.
        page.press_tab().await?;
    }
    page.wait_for_focus_on(&source, "the draggable").await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_attr(&source, "data-dragging", Some("true"))
        .await?;
    Ok(source)
}

// ---- /hooks/dnd ----

async fn basic_drag_and_drop(page: &DndPage<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd").await?;
    let draggable = page.element("test-dnd-draggable").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    assert_that!(attr(&draggable, "data-dragging").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(attr(&draggable, "draggable").await?).is_equal_to(Some("true".to_owned()));

    page.click_element_with_id("test-dnd-before").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&draggable, "the draggable").await?;
    assert_that!(page.description(&draggable).await?)
        .is_equal_to("Press Enter to start dragging.".to_owned());

    page.send_keys_to_active(Key::Enter).await?;
    // The drag moves focus to the nearest drop target.
    page.wait_for_focus_on(&target_1, "the first drop target")
        .await?;
    assert_that!(attr(&draggable, "data-dragging").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&target_1, "data-droptarget").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(page.description(&target_1).await?)
        .is_equal_to("Press Enter to drop. Press Escape to cancel drag.".to_owned());
    page.expect_log(LOG, &["dragstart", "dropenter 1"]).await?;

    page.press_tab().await?;
    page.wait_for_focus_on(&target_2, "the second drop target")
        .await?;
    assert_that!(attr(&target_1, "data-droptarget").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(attr(&target_2, "data-droptarget").await?).is_equal_to(Some("true".to_owned()));
    page.expect_log(
        LOG,
        &["dragstart", "dropenter 1", "dropexit 1", "dropenter 2"],
    )
    .await?;

    page.send_keys_to_active(Key::Enter).await?;
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
    page.wait_for_focus_on(&target_2, "the second drop target")
        .await?;
    assert_that!(attr(&draggable, "data-dragging").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(attr(&target_2, "data-droptarget").await?).is_equal_to(Some("false".to_owned()));
    // Drop targets are only described during drags.
    assert_that!(attr(&target_2, "aria-describedby").await?).is_none();
    Ok(())
}

async fn escape_cancels(page: &DndPage<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd").await?;
    let draggable = page.element("test-dnd-draggable").await?;
    page.click_element_with_id("test-dnd-before").await?;
    page.press_tab().await?;
    page.send_keys_to_active(Key::Enter).await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus_on(&target_1, "the first drop target")
        .await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_focus_on(&draggable, "the draggable").await?;
    page.expect_log(
        LOG,
        &["dragstart", "dropenter 1", "dropexit 1", "dragend Cancel"],
    )
    .await
}

async fn row(page: &DndPage<'_>, letter: &str) -> Result<WebElement, Report> {
    page.css("[role=grid][aria-label='Letters']")
        .await?
        .find(By::XPath(format!(
            ".//*[@role='row'][.//*[@role='gridcell'][normalize-space(.)='{letter}']]"
        )))
        .await
        .map_err(Into::into)
}

async fn expect_focused_indicator(page: &DndPage<'_>, label: &str) -> Result<(), Report> {
    page.wait_until(
        &format!("the drop indicator {label:?} to have focus"),
        || async {
            let active = page.driver.active_element().await?;
            Ok(attr(&active, "aria-label").await?.as_deref() == Some(label)
                && attr(&active, "aria-roledescription").await?.as_deref()
                    == Some("drop indicator"))
        },
    )
    .await
}

/// Reordering with the keyboard: the drop target starts after the dragged row, arrow keys move
/// between the valid positions (rows can't be dropped on), Enter drops.
async fn reorder_a_list(page: &DndPage<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd").await?;
    page.click_element_with_id("test-dnd-before-list").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&row(page, "A").await?, "row A")
        .await?;
    page.send_keys_to_active(Key::Down).await?;
    let b = row(page, "B").await?;
    page.wait_for_focus_on(&b, "row B").await?;

    page.send_keys_to_active(Key::Enter).await?;
    expect_focused_indicator(page, "Insert between B and C").await?;
    assert_that!(attr(&b, "data-dragging").await?).is_some();
    // Rows can't be dropped on: they are hidden (inert) during the drag.
    assert_that!(page.is_inert(&row(page, "A").await?).await?).is_true();
    assert_that!(page.is_inert(&b).await?).is_false();

    page.send_keys_to_active(Key::Up).await?;
    expect_focused_indicator(page, "Insert between A and B").await?;
    page.send_keys_to_active(Key::Up).await?;
    expect_focused_indicator(page, "Insert before A").await?;

    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-dnd-order", "BACD").await?;
    let b = row(page, "B").await?;
    page.wait_for_focus_on(&b, "row B").await?;
    assert_that!(page.is_inert(&row(page, "A").await?).await?).is_false();
    assert_that!(attr(&b, "aria-hidden").await?).is_none();
    Ok(())
}

/// "native drag and drop: should perform basic drag and drop".
async fn native_basic_drag_and_drop(page: &DndPage<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd").await?;
    let draggable = page.element("test-dnd-draggable").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;

    page.fire_drag_event(&draggable, "dragstart", &[]).await?;
    page.expect_log(LOG, &["dragstart"]).await?;
    assert_that!(page.transfer("getData('text/plain')").await?)
        .is_equal_to(serde_json::Value::from("hello world"));
    assert_that!(page.transfer("effectAllowed").await?).is_equal_to(serde_json::Value::from("all"));
    page.wait_for_attr(&draggable, "data-dragging", Some("true"))
        .await?;

    // Entering a drop target accepts the drag (prevents the default).
    assert_that!(page.fire_drag_event(&target_1, "dragenter", &[]).await?).is_true();
    assert_that!(page.fire_drag_event(&target_1, "dragover", &[]).await?).is_true();
    assert_that!(page.transfer("dropEffect").await?).is_equal_to(serde_json::Value::from("move"));
    page.expect_log(LOG, &["dragstart", "dropenter 1"]).await?;
    page.wait_for_attr(&target_1, "data-droptarget", Some("true"))
        .await?;

    page.fire_drag_event(&target_1, "dragleave", &[]).await?;
    page.fire_drag_event(&target_2, "dragenter", &[]).await?;
    page.fire_drag_event(&target_2, "dragover", &[]).await?;
    page.wait_for_attr(&target_1, "data-droptarget", Some("false"))
        .await?;
    page.wait_for_attr(&target_2, "data-droptarget", Some("true"))
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
    page.wait_for_attr(&draggable, "data-dragging", Some("false"))
        .await?;
    page.wait_for_attr(&target_2, "data-droptarget", Some("false"))
        .await
}

// ---- /hooks/dnd-targets: keyboard navigation ----

async fn tab_forward_skips_non_drop_targets(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&target_2, "Drop here 2").await?;
    // Past the last drop target: back to the drag source.
    page.press_tab().await?;
    page.wait_for_focus_on(&source, "the draggable").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
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
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await
}

async fn tab_backward_skips_non_drop_targets(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.press_shift_tab().await?;
    page.wait_for_focus_on(&source, "the draggable").await?;
    page.press_shift_tab().await?;
    page.wait_for_focus_on(&target_2, "Drop here 2").await?;
    page.press_shift_tab().await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await
}

async fn prefers_an_ancestor_drop_target(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "?ancestor").await?;
    let ancestor = page
        .css("[role=button][data-droptarget]:has([role=button])")
        .await?;
    page.wait_for_focus_on(&ancestor, "the ancestor drop target")
        .await?;
    page.wait_for_attr(&ancestor, "data-droptarget", Some("true"))
        .await?;
    page.expect_log_settled(TARGETS_LOG, &["dragstart", "dropenter 0"])
        .await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await
}

async fn enter_on_the_drag_source_cancels(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.press_shift_tab().await?;
    page.wait_for_focus_on(&source, "the draggable").await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await?;
    page.wait_for_focus_on(&source, "the draggable").await?;
    // One drag only: Enter didn't start another one.
    page.expect_log_settled(
        TARGETS_LOG,
        &["dragstart", "dropenter 1", "dropexit 1", "dragend Cancel"],
    )
    .await
}

async fn ignores_drop_targets_in_hidden_trees(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "?hidden-tree").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&target_2, "Drop here 2").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&source, "the draggable").await?;
    assert_that!(page.log(TARGETS_LOG).await?.join(",").contains(" 9")).is_false();
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await
}

// ---- /hooks/dnd-targets: changing targets ----

async fn a_removed_drop_target(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&target_2, "Drop here 2").await?;
    // The current drop target goes: the first one takes over.
    page.dispatch_action(TARGETS_PAGE, ACTION, "remove-target")
        .await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&source, "the draggable").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await
}

async fn a_drop_target_hidden_during_the_drag(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&target_2, "Drop here 2").await?;
    page.dispatch_action(TARGETS_PAGE, ACTION, "hide-target")
        .await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&source, "the draggable").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await
}

/// A drop target registered during a drag runs the targets' `get_drop_operation` (which read a
/// signal, the log): the registration must not subscribe to it, or entering the new target (which
/// logs) re-registers it and loses it as the current drop target.
async fn an_added_drop_target_keeps_the_current_target(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.dispatch_action(TARGETS_PAGE, ACTION, "add-target")
        .await?;
    let target_3 = droppable(page, "Drop here 3").await?;
    page.press_tab().await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&target_3, "Drop here 3").await?;
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
    page.wait_for_focus_on(&target_3, "Drop here 3").await?;
    page.send_keys_to_active(Key::Enter).await?;
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
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await
}

async fn a_hidden_drag_source_is_skipped(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.dispatch_action(TARGETS_PAGE, ACTION, "hide-draggable")
        .await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&target_2, "Drop here 2").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.press_shift_tab().await?;
    page.wait_for_focus_on(&target_2, "Drop here 2").await?;
    page.press_shift_tab().await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await
}

async fn escape_with_a_hidden_drag_source(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.dispatch_action(TARGETS_PAGE, ACTION, "hide-draggable")
        .await?;
    page.wait_for_attr(&target_1, "data-droptarget", Some("true"))
        .await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await?;
    page.wait_for_attr(&target_1, "data-droptarget", Some("false"))
        .await?;
    // Focus isn't restored into the hidden tree.
    page.settle().await;
    page.wait_for_focus_on(&target_1, "Drop here").await
}

// ---- /hooks/dnd-targets: disabled, operations ----

/// "useDrag should support isDisabled" (keyboard): no description, Enter starts nothing and isn't
/// swallowed.
async fn disabled_drag(page: &DndPage<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd-targets?disabled-drag").await?;
    let source = draggable(page).await?;
    assert_that!(attr(&source, "draggable").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(attr(&source, "data-dragging").await?).is_equal_to(Some("false".to_owned()));
    page.by_role_and_text("button", "Before")
        .await?
        .click()
        .await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&source, "the draggable").await?;
    assert_that!(attr(&source, "aria-describedby").await?).is_none();
    page.send_keys_to_active(Key::Enter).await?;
    page.expect_log_settled(TARGETS_LOG, &["parent keydown Enter"])
        .await?;
    page.wait_for_focus_on(&source, "the draggable").await?;
    page.expect_attr_settled(&source, "data-dragging", Some("false"))
        .await
}

/// "useDrop should support isDisabled" (keyboard): the disabled target isn't one.
async fn disabled_drop(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "?disabled-drop").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus_on(&target_2, "Drop here 2").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&source, "the draggable").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&target_2, "Drop here 2").await?;
    page.expect_attr_settled(&target_1, "data-droptarget", Some("false"))
        .await?;
    assert_that!(attr(&target_1, "aria-describedby").await?).is_none();
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await
}

/// "should support getDropOperation to override the default operation".
async fn drop_operation_override(page: &DndPage<'_>) -> Result<(), Report> {
    start_keyboard_drag(page, "?op=copy").await?;
    page.wait_for_focus_on(&droppable(page, "Drop here").await?, "Drop here")
        .await?;
    page.send_keys_to_active(Key::Enter).await?;
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
    .await
}

/// "should support getAllowedDropOperations to limit allowed operations".
async fn allowed_drop_operations(page: &DndPage<'_>) -> Result<(), Report> {
    start_keyboard_drag(page, "?allowed=link&op=copy").await?;
    page.wait_for_focus_on(&droppable(page, "Drop here").await?, "Drop here")
        .await?;
    page.send_keys_to_active(Key::Enter).await?;
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
    .await
}

/// "should hide drop targets where getDropOperation returns cancel".
async fn canceled_targets_are_hidden(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "?cancel-2").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    assert_that!(page.is_inert(&target_2).await?).is_true();
    page.press_tab().await?;
    page.wait_for_focus_on(&source, "the draggable").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await?;
    assert_that!(page.is_inert(&target_2).await?).is_false();
    Ok(())
}

/// Alt + Enter activates the drop target (e.g. opens a folder) without dropping.
async fn alt_enter_activates(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_keyboard_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.send_keys_to_active(Key::Alt + Key::Enter).await?;
    page.expect_log_settled(TARGETS_LOG, &["dragstart", "dropenter 1", "dropactivate 1"])
        .await?;
    assert_that!(attr(&source, "data-dragging").await?).is_equal_to(Some("true".to_owned()));
    page.send_keys_to_active(Key::Escape).await?;
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
    .await
}

/// "useDrag/useDrop should support isDisabled" (native): a disabled source writes no data, a
/// disabled target doesn't accept the drag.
async fn native_disabled(page: &DndPage<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd-targets?disabled-drag").await?;
    let source = draggable(page).await?;
    page.fire_drag_event(&source, "dragstart", &[]).await?;
    assert_that!(page.transfer("types.length").await?).is_equal_to(serde_json::Value::from(0));
    page.expect_attr_settled(&source, "data-dragging", Some("false"))
        .await?;

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
    page.expect_attr_settled(&target_1, "data-droptarget", Some("false"))
        .await
}

// ---- /hooks/dnd-targets: screen readers ----

/// Starts a screen reader drag: focus and a virtual click on the drag source.
async fn start_virtual_drag(page: &DndPage<'_>, query: &str) -> Result<WebElement, Report> {
    page.goto_path(&format!("/hooks/dnd-targets{query}"))
        .await?;
    let source = draggable(page).await?;
    page.focus_by_script(&source).await?;
    assert_that!(page.description(&source).await?)
        .is_equal_to("Click to start dragging.".to_owned());
    page.virtual_click(&source).await?;
    page.wait_for_attr(&source, "data-dragging", Some("true"))
        .await?;
    // The drag manager sets the session up one frame later (then the page becomes inert); clicks
    // before that would go to the drag source itself.
    let input = page.css("input[aria-label='Text field']").await?;
    page.wait_until("the drag session to start", || page.is_inert(&input))
        .await?;
    Ok(source)
}

/// "should allow navigating with only focus events".
async fn navigating_with_focus_events_only(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_virtual_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    page.wait_for_focus_on(&source, "the draggable").await?;
    page.wait_until("the drag source's description", || async {
        Ok(page.description(&source).await? == "Dragging. Click to cancel drag.")
    })
    .await?;
    page.expect_log_settled(TARGETS_LOG, &["dragstart"]).await?;

    page.focus_by_script(&target_1).await?;
    page.wait_for_attr(&target_1, "data-droptarget", Some("true"))
        .await?;
    assert_that!(page.description(&target_1).await?).is_equal_to("Click to drop.".to_owned());
    page.focus_by_script(&target_2).await?;
    page.wait_for_attr(&target_2, "data-droptarget", Some("true"))
        .await?;
    page.wait_for_attr(&target_1, "data-droptarget", Some("false"))
        .await?;

    page.virtual_click(&target_2).await?;
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
    page.wait_for_focus_on(&target_2, "Drop here 2").await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await?;
    assert_that!(attr(&target_1, "aria-describedby").await?).is_none();
    assert_that!(attr(&target_2, "aria-describedby").await?).is_none();
    Ok(())
}

/// "should hide all non drop target elements from screen readers while dragging" (with `inert`,
/// as react-aria's `shouldUseInert`).
async fn hides_everything_but_drop_targets(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_virtual_drag(page, "").await?;
    let input = page.css("input[aria-label='Text field']").await?;
    page.wait_until("the page to become inert", || page.is_inert(&input))
        .await?;
    for label in ["Before", "Not a drop target"] {
        let button = page.by_role_and_text("button", label).await?;
        assert_that!(page.is_inert(&button).await?)
            .with_detail_message(label)
            .is_true();
    }
    let text = page
        .driver
        .find(By::XPath("//span[normalize-space(.)='Text']"))
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
    page.virtual_click(&source).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await?;
    let before = page.by_role_and_text("button", "Before").await?;
    assert_that!(page.is_inert(&before).await?).is_false();
    Ok(())
}

/// "should support clicking the original drag target to cancel drag".
async fn clicking_the_drag_source_cancels(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_virtual_drag(page, "").await?;
    page.virtual_click(&source).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await?;
    page.wait_until("the drag source's description", || async {
        Ok(page.description(&source).await? == "Click to start dragging.")
    })
    .await?;
    page.expect_log_settled(TARGETS_LOG, &["dragstart", "dragend Cancel"])
        .await
}

/// "should restore focus to the current drop target (or the drag target) when focusing a non
/// drop target element" and "... when blurring all elements".
async fn restores_focus_from_non_drop_targets(page: &DndPage<'_>) -> Result<(), Report> {
    let source = start_virtual_drag(page, "").await?;
    let input = page.css("input[aria-label='Text field']").await?;
    // No current drop target: back to the drag source.
    page.focus_by_script(&input).await?;
    page.wait_for_focus_on(&source, "the draggable").await?;
    page.driver
        .execute("document.activeElement.blur();", vec![])
        .await?;
    page.wait_for_focus_on(&source, "the draggable").await?;

    let target_1 = droppable(page, "Drop here").await?;
    page.focus_by_script(&target_1).await?;
    page.wait_for_attr(&target_1, "data-droptarget", Some("true"))
        .await?;
    page.focus_by_script(&input).await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.driver
        .execute("document.activeElement.blur();", vec![])
        .await?;
    page.wait_for_focus_on(&target_1, "Drop here").await?;
    page.virtual_click(&source).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await
}

/// "should ignore clicks not from screen readers to start dragging" and "... during dragging".
async fn ignores_clicks_not_from_screen_readers(page: &DndPage<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd-targets").await?;
    let source = draggable(page).await?;
    // Off center: a pointer at the very center counts as a screen reader's (TalkBack).
    click_off_center(page, &source).await?;
    page.expect_attr_settled(&source, "data-dragging", Some("false"))
        .await?;

    let source = start_virtual_drag(page, "").await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.focus_by_script(&target_1).await?;
    page.wait_for_attr(&target_1, "data-droptarget", Some("true"))
        .await?;
    click_off_center(page, &target_1).await?;
    page.expect_attr_settled(&source, "data-dragging", Some("true"))
        .await?;
    page.expect_log_settled(TARGETS_LOG, &["dragstart", "dropenter 1"])
        .await?;
    page.virtual_click(&source).await?;
    page.wait_for_attr(&source, "data-dragging", Some("false"))
        .await
}
