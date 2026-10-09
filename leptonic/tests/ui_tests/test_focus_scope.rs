// Upstream: react-aria/test/focus/FocusScope.test.js @ 99e6102368
//! The `FocusScope` atom: auto focus, containment (Tab wrapping, clicks outside), focus
//! restoration (nested scopes, fallbacks, cancelled restoration), select on Tab. Every case starts
//! on a fresh page: a containing scope stays active otherwise.
use std::time::Duration;

use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, KeyKind, Modifier, MouseKind, Page, SyntheticEvent, css};

const PATH: &str = "/atoms/focus-scope";

/// Navigate to a fresh page (the original part of the fixture, every scope at once) and wait for
/// the auto-focus scope to take focus.
async fn goto(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["classic"]).await?;
    page.wait_for_focus(&page.element("#test-fs-autofocus-btn-1").await?)
        .await
}

/// Click `selector` and wait until it has focus.
async fn click_to_focus(page: &Page<'_>, selector: &str) -> Result<WebElement, Report> {
    let element = page.element(selector).await?;
    element.click().await?;
    page.wait_for_focus(&element).await?;
    Ok(element)
}

/// Press `key`, then wait until `selector` has focus.
async fn press_to_focus(
    page: &Page<'_>,
    key: impl Into<TypingData> + Send,
    selector: &str,
) -> Result<(), Report> {
    page.send_keys(key).await?;
    page.wait_for_focus(&page.element(selector).await?).await
}

/// On page load, the `auto_focus` scope focuses its first button ("should auto focus the first
/// tabbable element in the scope on mount").
#[browser_test]
pub async fn auto_focus(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    Ok(())
}

/// Tab moves through the inputs of a containing scope and wraps from the last to the first
/// ("should contain focus within the scope").
#[browser_test]
pub async fn tab_wrapping(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    click_to_focus(page, "#test-fs-select-input-1").await?;
    press_to_focus(page, Key::Tab, "#test-fs-select-input-2").await?;
    press_to_focus(page, Key::Tab, "#test-fs-select-input-3").await?;
    // Wraps.
    press_to_focus(page, Key::Tab, "#test-fs-select-input-1").await?;
    Ok(())
}

/// Shift+Tab on the first input of a containing scope wraps to the last ("should contain focus
/// within the scope").
#[browser_test]
pub async fn shift_tab_wrapping(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    click_to_focus(page, "#test-fs-select-input-1").await?;
    press_to_focus(page, Key::Shift + Key::Tab, "#test-fs-select-input-3").await?;
    Ok(())
}

/// When a scope with `restore_focus` unmounts, focus returns to the element that had focus before
/// the scope appeared ("should restore focus to the previously focused node on unmount").
#[browser_test]
pub async fn focus_restoration(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    let toggle = page.element("#test-fs-restore-toggle").await?;

    // The scope auto-focuses: focus moves into it.
    toggle.click().await?;
    page.wait_for_focus(&page.element("#test-fs-restore-btn").await?)
        .await?;

    toggle.click().await?;
    page.wait_for_focus(&toggle).await?;
    Ok(())
}

/// Tab in a containing scope nested in another containing scope wraps within the inner scope
/// ("should lock tab navigation inside direct child focus scope").
#[browser_test]
pub async fn nested_scopes(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    click_to_focus(page, "#test-fs-nested-inner-btn-1").await?;
    press_to_focus(page, Key::Tab, "#test-fs-nested-inner-btn-2").await?;
    // Wraps back to inner btn-1 (stays in the inner scope).
    press_to_focus(page, Key::Tab, "#test-fs-nested-inner-btn-1").await?;
    Ok(())
}

/// Clicking outside a containing scope moves focus back to the scope's element that had it last.
#[browser_test]
pub async fn containment_blocks_escape(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    let inside = click_to_focus(page, "#test-fs-select-input-1").await?;
    page.element("#test-fs-outside").await?.click().await?;

    // Focus goes back to the element that last had it inside the scope, after it really reached
    // the outside button (else this test would pass with focus never leaving the scope).
    page.wait_for_focus(&inside).await?;
    page.element("#test-fs-outside-focused")
        .await?
        .wait_for_inner_text("true")
        .await?;
    Ok(())
}

/// When nested scopes (both with `restore_focus`) unmount, focus returns to the element that was
/// focused before the outermost scope mounted.
#[browser_test]
pub async fn nested_restore_focuses_outermost(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    let trigger = page.element("#test-fs-nested-restore-trigger").await?;

    // The trigger gets focus first, becoming the outer scope's node to restore; auto-focus then
    // moves focus into the inner scope.
    trigger.click().await?;
    page.wait_for_focus(&page.element("#test-fs-nested-restore-inner-btn").await?)
        .await?;

    // Hide both scopes at once: focus goes to the outermost scope's node to restore.
    trigger.click().await?;
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// Tab from the outer scope's button enters the inner scope.
#[browser_test]
pub async fn outer_to_inner_navigation(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    click_to_focus(page, "#test-fs-nested-outer-btn").await?;
    press_to_focus(page, Key::Tab, "#test-fs-nested-inner-btn-1").await?;
    Ok(())
}

/// Removes the scope `<prefix>-inside` opened by `<prefix>-target`, whose node to restore is gone
/// by then, and checks that the focus goes to `fallback` and stays there.
async fn restore_without_the_node_to_restore(
    page: &Page<'_>,
    prefix: &str,
    fallback: &str,
) -> Result<(), Report> {
    goto(page).await?;
    page.element(format!("{prefix}-target"))
        .await?
        .click()
        .await?;
    let inside = page.element(format!("{prefix}-inside")).await?;
    page.wait_for_focus(&inside).await?;
    inside.click().await?;
    page.wait_for_count(format!("{prefix}-inside"), 0).await?;
    let fallback = page.element(fallback).await?;
    page.wait_for_focus(&fallback).await?;
    page.focus_stays(&fallback, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Without a node to restore in the DOM, focus goes to the first tabbable element of the nearest
/// ancestor scope.
#[browser_test]
pub async fn restore_fallback(page: &Page<'_>) -> Result<(), Report> {
    restore_without_the_node_to_restore(page, "#test-fs-fallback", "#test-fs-fallback-other").await
}

/// Without a node to restore and without a tabbable element in the ancestor scope, focus goes to
/// the body and stays there ("does not throw when there is no focusable element to restore focus
/// to").
#[browser_test]
pub async fn restore_fallback_without_tabbables(page: &Page<'_>) -> Result<(), Report> {
    restore_without_the_node_to_restore(page, "#test-fs-fallback-empty", "body").await
}

/// Closing a dialog opened from a (since closed) menu and rendered outside it restores focus to the
/// menu's trigger ("tracks node to restore if the node to restore was removed in another part of
/// the tree").
#[browser_test]
pub async fn dialog_from_menu(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    let open_menu = page.element("#test-fs-open-menu").await?;
    open_menu.focus().await?;
    press_to_focus(page, Key::Enter, "#test-fs-open-dialog").await?;
    press_to_focus(page, Key::Enter, "#test-fs-close-dialog").await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_count("#test-fs-close-dialog", 0).await?;
    page.wait_for_focus(&open_menu).await?;
    Ok(())
}

/// When a script blurs the focused element of a containing scope, focus goes back to that element,
/// not the scope's first one ("should restore focus to the last focused element in the scope on
/// focus out").
#[browser_test]
pub async fn restore_on_blur(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    click_to_focus(page, "#test-fs-select-input-1").await?;
    press_to_focus(page, Key::Tab, "#test-fs-select-input-2").await?;
    page.blur_focused().await?;
    page.wait_for_focus(&page.element("#test-fs-select-input-2").await?)
        .await?;
    Ok(())
}

/// Tabbing in a containing scope, also when wrapping, selects the whole text of the input it moves
/// to, as the browser does ("should select all text in input when tabbing").
#[browser_test]
pub async fn select_on_tab(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    click_to_focus(page, "#test-fs-select-input-1").await?;
    for next in [
        "#test-fs-select-input-2",
        "#test-fs-select-input-3",
        // Wrapping selects too.
        "#test-fs-select-input-1",
    ] {
        let input = page.element(next).await?;
        page.send_keys(Key::Tab).await?;
        page.wait_for_focus(&input).await?;
        assert_that!(input)
            .property("selectionStart")
            .await
            .with_detail_message(next)
            .get_some()
            .is_equal_to("0");
        assert_that!(input)
            .property("selectionEnd")
            .await
            .with_detail_message(next)
            .get_some()
            .is_equal_to("5");
    }
    // Typing replaces the selected text.
    page.send_keys(Key::Delete).await?;
    page.element("#test-fs-select-input-1")
        .await?
        .wait_for_prop("value", "")
        .await?;
    Ok(())
}

/// Tab with focus outside the active containing scope (here in a top layer, where focus may go) is
/// left to the browser and moves to the next element there.
#[browser_test]
pub async fn tab_outside_the_scope_is_native(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    click_to_focus(page, "#test-fs-select-input-1").await?;
    click_to_focus(page, "#test-fs-top-layer-1").await?;
    press_to_focus(page, Key::Tab, "#test-fs-top-layer-2").await?;
    Ok(())
}

/// Changing `contain` while the scope is mounted takes effect: Tab wraps within the scope only
/// while it contains and leaves it otherwise.
#[browser_test]
pub async fn runtime_contain(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    let toggle = page.element("#test-fs-runtime-toggle").await?;

    // Not containing: Tab leaves the scope.
    click_to_focus(page, "#test-fs-runtime-2").await?;
    press_to_focus(page, Key::Tab, "#test-fs-runtime-after").await?;

    // Containing: Tab wraps.
    toggle.click().await?;
    toggle.wait_for_inner_text("Stop containing").await?;
    press_to_focus(page, Key::Tab, "#test-fs-runtime-2").await?;
    press_to_focus(page, Key::Tab, "#test-fs-runtime-toggle").await?;

    // Not containing again: Tab leaves the scope.
    page.send_keys(Key::Enter).await?;
    toggle.wait_for_inner_text("Contain").await?;
    press_to_focus(page, Key::Tab, "#test-fs-runtime-2").await?;
    press_to_focus(page, Key::Tab, "#test-fs-runtime-after").await?;
    Ok(())
}

/// A listener calling `preventDefault` on the restore event keeps focus from being restored, so it
/// stays on the body ("should allow restoration to be overridden with a custom event").
#[browser_test]
pub async fn cancelled_restore(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    page.element("#test-fs-cancel-show").await?.click().await?;
    page.wait_for_focus(&page.element("#test-fs-cancel-input").await?)
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("#test-fs-cancel-input", 0).await?;
    let body = page.element("body").await?;
    page.wait_for_focus(&body).await?;
    page.focus_stays(&body, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A cancelling restore-event listener outside the scope around the node to restore doesn't keep
/// focus from being restored ("should not bubble focus scope restoration event out of nested focus
/// scopes").
#[browser_test]
pub async fn restore_event_stays_in_nested_scopes(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    let show = page.element("#test-fs-nested-cancel-show").await?;
    show.click().await?;
    page.wait_for_focus(&page.element("#test-fs-nested-cancel-input").await?)
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("#test-fs-nested-cancel-input", 0)
        .await?;
    page.wait_for_focus(&show).await?;
    page.focus_stays(&show, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Tab out of a scope that restores focus but doesn't contain it moves to the element after the
/// node to restore, Shift+Tab to the one before it ("should move focus to the element after the
/// previously focused node on Tab", "should move focus to the previous element after the
/// previously focused node on Shift+Tab").
#[browser_test]
pub async fn tab_out_of_restoring_scope(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    let trigger = page.element("#test-fs-tab-trigger").await?;
    trigger.click().await?;
    page.wait_for_focus(&page.element("#test-fs-tab-input-1").await?)
        .await?;
    click_to_focus(page, "#test-fs-tab-input-3").await?;
    // Natively, Tab would go to `test-fs-tab-after`, the next element in the DOM.
    press_to_focus(page, Key::Tab, "#test-fs-tab-after-trigger").await?;

    // Reopen (the trigger restores into the scope again) and leave backwards.
    trigger.click().await?;
    page.wait_for_count("#test-fs-tab-input-1", 0).await?;
    trigger.click().await?;
    page.wait_for_focus(&page.element("#test-fs-tab-input-1").await?)
        .await?;
    press_to_focus(page, Key::Shift + Key::Tab, "#test-fs-tab-before").await?;
    Ok(())
}

/// Focus `selector` from a script (`element.focus()`) and wait until it has it.
async fn focus(page: &Page<'_>, selector: &str) -> Result<WebElement, Report> {
    let element = page.element(selector).await?;
    element.focus().await?;
    page.wait_for_focus(&element).await?;
    Ok(element)
}

/// Starting at the first of `cycle`, Tab visits the elements in order and wraps to the first,
/// then Shift+Tab visits them backwards to the first again.
async fn tab_cycle(page: &Page<'_>, cycle: &[&str]) -> Result<(), Report> {
    focus(page, cycle[0]).await?;
    for next in cycle.iter().skip(1).chain([&cycle[0]]) {
        press_to_focus(page, Key::Tab, next).await?;
    }
    for previous in cycle.iter().rev() {
        press_to_focus(page, Key::Shift + Key::Tab, previous).await?;
    }
    Ok(())
}

/// Tab cycles through a containing scope, skipping hidden inputs; focusing an element of another
/// containing scope from a script moves focus back ("should work with multiple focus scopes").
#[browser_test]
pub async fn multiple_focus_scopes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["multiple"]).await?;
    tab_cycle(
        page,
        &["#test-fs-multi-1", "#test-fs-multi-2", "#test-fs-multi-3"],
    )
    .await?;
    page.element("#test-fs-multi-4").await?.focus().await?;
    page.wait_for_focus(&page.element("#test-fs-multi-1").await?)
        .await?;
    Ok(())
}

/// Tab containment skips elements that aren't tabbable (hidden, invisible, disabled,
/// `tabindex="-1"`) and finds inputs in nested elements ("should skip non-tabbable elements",
/// "should work with nested elements").
#[browser_test]
pub async fn skips_non_tabbable_elements(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["skip"]).await?;
    tab_cycle(
        page,
        &["#test-fs-skip-1", "#test-fs-skip-2", "#test-fs-skip-3"],
    )
    .await
}

/// Tab containment visits contenteditable elements, except those with `contenteditable="false"`
/// ("should only skip content editable which are false").
#[browser_test]
pub async fn skips_only_non_editable_content(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["contenteditable"]).await?;
    focus(page, "#test-fs-ce-1").await?;
    press_to_focus(page, Key::Tab, "#test-fs-ce-2").await?;
    press_to_focus(page, Key::Tab, "#test-fs-ce-3").await?;
    press_to_focus(page, Key::Tab, "#test-fs-ce-4").await?;
    press_to_focus(page, Key::Shift + Key::Tab, "#test-fs-ce-3").await?;
    press_to_focus(page, Key::Shift + Key::Tab, "#test-fs-ce-2").await?;
    press_to_focus(page, Key::Shift + Key::Tab, "#test-fs-ce-1").await?;
    Ok(())
}

/// A Tab with Alt held is left alone: focus stays where it is ("should do nothing if a modifier
/// key is pressed").
#[browser_test]
pub async fn modifier_tab_does_nothing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["multiple"]).await?;
    let first = focus(page, "#test-fs-multi-1").await?;
    let dispatched = first
        .dispatch(SyntheticEvent::keyboard(KeyKind::Down, "Tab").modifiers(&[Modifier::Alt]))
        .await?;
    assert_that!(dispatched.default_prevented).is_false();
    page.settle().await?;
    page.focus_stays(&first, Duration::from_millis(100)).await?;
    Ok(())
}

/// A restoring scope whose children changed (focus on a child added later) still restores focus
/// to the element focused before it mounted ("should restore focus to the previously focused node
/// after children change").
#[browser_test]
pub async fn restore_after_children_change(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["children-change"]).await?;
    let outside = focus(page, "#test-fs-cc-outside").await?;
    page.element("#test-fs-cc-show").await?.click().await?;
    page.wait_for_focus(&page.element("#test-fs-cc-input").await?)
        .await?;
    page.element("#test-fs-cc-show-child")
        .await?
        .click()
        .await?;
    focus(page, "#test-fs-cc-dynamic").await?;
    page.element("#test-fs-cc-hide").await?.click().await?;
    page.wait_for_count("#test-fs-cc-input", 0).await?;
    page.wait_for_focus(&outside).await?;
    Ok(())
}

/// Tab out of a restoring scope that directly follows its trigger skips the scope's own elements
/// and moves to the element after it ("should skip over elements within the scope when moving
/// focus to the next element").
#[browser_test]
pub async fn tab_skips_the_scope_after_its_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["after-trigger"]).await?;
    page.element("#test-fs-at-trigger").await?.click().await?;
    page.wait_for_focus(&page.element("#test-fs-at-1").await?)
        .await?;
    focus(page, "#test-fs-at-3").await?;
    press_to_focus(page, Key::Tab, "#test-fs-at-after").await?;
    Ok(())
}

/// A scope that doesn't restore focus leaves Tab to the browser: from its last element focus
/// moves to the next element in the DOM ("should not handle tabbing if the focus scope does not
/// restore focus").
#[browser_test]
pub async fn no_tab_handling_without_restore(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["no-restore"]).await?;
    page.element("#test-fs-nr-trigger").await?.click().await?;
    page.wait_for_focus(&page.element("#test-fs-nr-1").await?)
        .await?;
    focus(page, "#test-fs-nr-3").await?;
    press_to_focus(page, Key::Tab, "#test-fs-nr-after").await?;
    Ok(())
}

/// Tab and Shift+Tab move in and out of a plain scope in DOM order ("should navigate in and out
/// of scope in DOM order when the nodeToRestore is the document.body").
#[browser_test]
pub async fn dom_order_without_node_to_restore(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["dom-order"]).await?;
    focus(page, "#test-fs-dom-in").await?;
    press_to_focus(page, Key::Tab, "#test-fs-dom-after").await?;
    focus(page, "#test-fs-dom-in").await?;
    press_to_focus(page, Key::Shift + Key::Tab, "#test-fs-dom-before").await?;
    Ok(())
}

/// An auto-focusing scope keeps the focus of an element inside it that has focus already ("should
/// do nothing if something is already focused in the scope").
#[browser_test]
pub async fn auto_focus_keeps_focus_inside(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["autofocused"]).await?;
    let autofocused = page.element("#test-fs-af-2").await?;
    page.wait_for_focus(&autofocused).await?;
    page.settle().await?;
    page.focus_stays(&autofocused, Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Without a tabbable element, auto focus goes to the first focusable one (a `tabindex="-1"`
/// dialog; react-aria's `getFirstInScope`).
#[browser_test]
pub async fn auto_focus_falls_back_to_focusable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["autofocus-fallback"]).await?;
    page.element("#test-fs-aff-show").await?.click().await?;
    page.wait_for_focus(&page.element("#test-fs-aff-dialog").await?)
        .await?;
    Ok(())
}

/// Items that move focus on with the scope's focus manager and remove themselves: focus goes to
/// the next item, and once none is left to the focusable dialog ("should restore focus to the
/// first focusable or tabbable element within the scope when focus is lost within the scope").
#[browser_test]
pub async fn focus_falls_back_to_the_first_focusable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["first-focusable"]).await?;
    let tabbable = page.element("#test-fs-ff-tabbable").await?;
    page.wait_for_focus(&tabbable).await?;
    tabbable.click().await?;
    page.wait_for_count("#test-fs-ff-tabbable", 0).await?;
    let item = page.element("#test-fs-ff-item-1").await?;
    page.wait_for_focus(&item).await?;
    item.click().await?;
    page.wait_for_count("#test-fs-ff-item-1", 0).await?;
    let item = page.element("#test-fs-ff-item-2").await?;
    page.wait_for_focus(&item).await?;
    item.click().await?;
    page.wait_for_count("#test-fs-ff-item-2", 0).await?;
    page.wait_for_focus(&page.element("#test-fs-ff-dialog").await?)
        .await?;
    Ok(())
}

/// Focus may move into a non-containing child scope rendered in a portal and back into the
/// containing parent ("should not lock focus inside a focus scope with a child scope in a
/// portal").
#[browser_test]
pub async fn portal_child_scope_without_contain(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["portal-child"]).await?;
    page.wait_for_focus(&page.element("#test-fs-portal-child-parent").await?)
        .await?;
    focus(page, "#test-fs-portal-child-child").await?;
    let parent = focus(page, "#test-fs-portal-child-parent").await?;
    page.settle().await?;
    page.focus_stays(&parent, Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A containing child scope in a portal keeps focus once it has it: focusing the parent's input
/// moves focus back into the child ("should lock focus inside a child focus scope with contain in
/// a portal").
#[browser_test]
pub async fn portal_child_scope_with_contain(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["portal-child-contain"]).await?;
    page.wait_for_focus(&page.element("#test-fs-portal-child-contain-parent").await?)
        .await?;
    let child = focus(page, "#test-fs-portal-child-contain-child").await?;
    page.element("#test-fs-portal-child-contain-parent")
        .await?
        .focus()
        .await?;
    page.wait_for_focus(&child).await?;
    Ok(())
}

/// A containing child scope rendered elsewhere (a portal) becomes the active scope when it gets
/// focus, so its containing parent doesn't pull focus back ("should make child FocusScopes the
/// active scope regardless of DOM structure").
#[browser_test]
pub async fn child_scope_active_regardless_of_dom(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["child-elsewhere"]).await?;
    focus(page, "#test-fs-ce-input-1").await?;
    page.element("#test-fs-ce-show").await?.click().await?;
    let input = focus(page, "#test-fs-ce-input-3").await?;
    page.settle().await?;
    page.focus_stays(&input, Duration::from_millis(100)).await?;
    Ok(())
}

/// Focus can move from a containing scope into a containing child scope, but not back out; once
/// inner scopes unmount, their parent holds focus again ("should restore to the correct scope on
/// unmount").
#[browser_test]
pub async fn restores_to_the_correct_scope_on_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["correct-scope"]).await?;
    let parent = page.element("#test-fs-cs-parent").await?;
    page.wait_for_focus(&parent).await?;

    page.element("#test-fs-cs-show-1").await?.click().await?;
    let child1 = page.element("#test-fs-cs-child-1").await?;
    page.focus_stays(&parent, Duration::from_millis(100))
        .await?;

    // Can move into a child, but not out.
    press_to_focus(page, Key::Tab, "#test-fs-cs-child-1").await?;
    parent.focus().await?;
    page.wait_for_focus(&child1).await?;

    page.element("#test-fs-cs-show-2").await?.click().await?;
    press_to_focus(page, Key::Tab, "#test-fs-cs-child-2").await?;
    let child2 = page.element("#test-fs-cs-child-2").await?;
    child1.focus().await?;
    page.wait_for_focus(&child2).await?;
    parent.focus().await?;
    page.wait_for_focus(&child2).await?;

    page.element("#test-fs-cs-show-3").await?.click().await?;
    press_to_focus(page, Key::Tab, "#test-fs-cs-child-3").await?;

    page.element("#test-fs-cs-only-1").await?.click().await?;
    page.wait_for_count("#test-fs-cs-child-2", 0).await?;
    child1.focus().await?;
    page.wait_for_focus(&child1).await?;
    parent.focus().await?;
    page.wait_for_focus(&child1).await?;
    Ok(())
}

/// Stacked dialogs (`FocusScope.stories.tsx`): each auto-focuses its first input; closing the
/// middle one (with the innermost inside it) restores focus to the button that opened it, closing
/// the first to the page's button.
async fn stacked_dialogs(page: &Page<'_>, name: &str, contain: bool) -> Result<(), Report> {
    page.goto_sections(PATH, &[name]).await?;
    let root = page.element(format!("#test-fs-{name}")).await?;
    let open_buttons = || root.elements(css("button").text("Open dialog"));
    let inputs = || root.elements(css("input"));

    for (level, first_input) in [(0, 2), (1, 5), (2, 8)] {
        let open = open_buttons().await?.swap_remove(level);
        open.focus().await?;
        open.click().await?;
        root.wait_for_count("input", 2 + 3 * (level + 1)).await?;
        page.wait_for_focus(&inputs().await?.swap_remove(first_input))
            .await?;
    }

    // Close the middle dialog: with containment, without moving focus out of the innermost one.
    let close = root
        .elements(css("button").text("close"))
        .await?
        .swap_remove(1);
    if contain {
        close
            .dispatch(SyntheticEvent::mouse(MouseKind::Click))
            .await?;
    } else {
        close.focus().await?;
        close.click().await?;
    }
    root.wait_for_count("input", 5).await?;
    page.wait_for_focus(&open_buttons().await?.swap_remove(1))
        .await?;

    let close = root
        .elements(css("button").text("close"))
        .await?
        .swap_remove(0);
    close.focus().await?;
    close.click().await?;
    root.wait_for_count("input", 2).await?;
    page.wait_for_focus(&open_buttons().await?.swap_remove(0))
        .await?;
    Ok(())
}

/// Stacked dialogs inline, without containment ("contain=false, isPortaled=false should restore
/// focus to previous nodeToRestore when the nodeToRestore for the unmounting scope in no longer in
/// the DOM").
#[browser_test]
pub async fn stacked_dialogs_inline(page: &Page<'_>) -> Result<(), Report> {
    stacked_dialogs(page, "stacked", false).await
}

/// Stacked dialogs inline, containing focus ("contain=true, isPortaled=false should restore focus
/// to previous nodeToRestore ...").
#[browser_test]
pub async fn stacked_dialogs_inline_containing(page: &Page<'_>) -> Result<(), Report> {
    stacked_dialogs(page, "stacked-contain", true).await
}

/// Stacked dialogs in a portal, without containment ("contain=false, isPortaled=true should
/// restore focus to previous nodeToRestore ...").
#[browser_test]
pub async fn stacked_dialogs_portaled(page: &Page<'_>) -> Result<(), Report> {
    stacked_dialogs(page, "stacked-portaled", false).await
}

/// Stacked dialogs in a portal, containing focus ("contain=true, isPortaled=true should restore
/// focus to previous nodeToRestore ...").
#[browser_test]
pub async fn stacked_dialogs_portaled_containing(page: &Page<'_>) -> Result<(), Report> {
    stacked_dialogs(page, "stacked-contain-portaled", true).await
}

/// The id of the deepest focused element (inside shadow roots too), as the shadow-DOM section
/// shows it.
async fn shadow_focus(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-fs-shadow-focused").await
}

/// Press `key`, then wait until the deepest focused element is `id`.
async fn press_to_shadow_focus(
    page: &Page<'_>,
    key: impl Into<TypingData> + Send,
    id: &str,
) -> Result<(), Report> {
    page.send_keys(key).await?;
    shadow_focus(page).await?.wait_for_inner_text(id).await
}

/// Tab wraps within a containing scope inside a shadow root ("should contain focus within the
/// shadow DOM scope").
#[browser_test]
pub async fn shadow_dom_containment(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["shadow"]).await?;
    page.element("#test-fs-shadow-focus-1")
        .await?
        .click()
        .await?;
    shadow_focus(page)
        .await?
        .wait_for_inner_text("test-fs-shadow-1")
        .await?;
    press_to_shadow_focus(page, Key::Tab, "test-fs-shadow-2").await?;
    press_to_shadow_focus(page, Key::Tab, "test-fs-shadow-button").await?;
    press_to_shadow_focus(page, Key::Tab, "test-fs-shadow-1").await?;
    Ok(())
}

/// Shift+Tab wraps backwards within a containing scope inside a shadow root ("should autofocus and
/// lock tab navigation inside shadow DOM").
#[browser_test]
pub async fn shadow_dom_lock_backwards(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["shadow"]).await?;
    page.element("#test-fs-shadow-focus-1")
        .await?
        .click()
        .await?;
    shadow_focus(page)
        .await?
        .wait_for_inner_text("test-fs-shadow-1")
        .await?;
    press_to_shadow_focus(page, Key::Shift + Key::Tab, "test-fs-shadow-button").await?;
    press_to_shadow_focus(page, Key::Shift + Key::Tab, "test-fs-shadow-2").await?;
    Ok(())
}

/// Tab moves and wraps within a containing scope in a shadow root nested in another one ("should
/// manage focus within nested shadow DOMs").
#[browser_test]
pub async fn nested_shadow_dom(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["shadow"]).await?;
    page.element("#test-fs-shadow-focus-nested")
        .await?
        .click()
        .await?;
    shadow_focus(page)
        .await?
        .wait_for_inner_text("test-fs-nested-shadow-1")
        .await?;
    press_to_shadow_focus(page, Key::Tab, "test-fs-nested-shadow-2").await?;
    press_to_shadow_focus(page, Key::Tab, "test-fs-nested-shadow-1").await?;
    Ok(())
}

/// A restoring scope in a shadow root that unmounts while focus is outside it leaves focus where
/// it is ("should restore focus to the element outside shadow DOM on unmount, with FocusScope
/// outside as well").
#[browser_test]
pub async fn shadow_dom_unmount_keeps_outside_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["shadow"]).await?;
    page.element("#test-fs-shadow-focus-restore")
        .await?
        .click()
        .await?;
    shadow_focus(page)
        .await?
        .wait_for_inner_text("test-fs-shadow-restore-1")
        .await?;
    let outside = focus(page, "#test-fs-shadow-outside").await?;
    page.element("#test-fs-shadow-unmount")
        .await?
        .click()
        .await?;
    page.settle().await?;
    page.focus_stays(&outside, Duration::from_millis(100))
        .await?;
    Ok(())
}
