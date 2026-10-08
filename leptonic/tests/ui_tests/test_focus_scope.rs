// Upstream: react-aria/test/focus/FocusScope.test.js @ 99e6102368
//! The `FocusScope` atom: auto focus, containment (Tab wrapping, clicks outside), focus
//! restoration (nested scopes, fallbacks, cancelled restoration), select on Tab. Every case starts
//! on a fresh page: a containing scope stays active otherwise.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

/// Navigate to a fresh page and wait for the auto-focus scope to take focus.
async fn goto(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/focus-scope").await?;
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

/// Auto-focus: on page load, the first button in the `auto_focus` scope has focus (checked by
/// [`goto`]).
pub async fn auto_focus(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    Ok(())
}

/// Tab cycles through the contained scope and wraps around.
pub async fn tab_wrapping(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    click_to_focus(page, "#test-fs-contain-btn-1").await?;
    press_to_focus(page, Key::Tab, "#test-fs-contain-btn-2").await?;
    press_to_focus(page, Key::Tab, "#test-fs-contain-btn-3").await?;
    // Wraps.
    press_to_focus(page, Key::Tab, "#test-fs-contain-btn-1").await?;
    Ok(())
}

/// Shift+Tab cycles backwards and wraps.
pub async fn shift_tab_wrapping(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    click_to_focus(page, "#test-fs-contain-btn-1").await?;
    press_to_focus(page, Key::Shift + Key::Tab, "#test-fs-contain-btn-3").await?;
    Ok(())
}

/// When a scope with `restore_focus` unmounts, focus returns to the element that had focus before
/// the scope appeared.
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

/// The inner scope's containment is respected; the outer scope doesn't take focus from it.
pub async fn nested_scopes(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    click_to_focus(page, "#test-fs-nested-inner-btn-1").await?;
    press_to_focus(page, Key::Tab, "#test-fs-nested-inner-btn-2").await?;
    // Wraps back to inner btn-1 (stays in the inner scope).
    press_to_focus(page, Key::Tab, "#test-fs-nested-inner-btn-1").await?;
    Ok(())
}

/// Clicking outside a containing scope pulls focus back into the scope.
pub async fn containment_blocks_escape(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    let inside = click_to_focus(page, "#test-fs-contain-btn-1").await?;
    let outside = page.element("#test-fs-outside").await?;

    // Record that focus really reaches the outside button (else this test would pass with focus
    // never leaving the scope).
    page.eval::<()>(
        "window.__outsideFocused = false;
         arguments[0].addEventListener('focus', () => { window.__outsideFocused = true; });",
        vec![outside.to_json()?],
    )
    .await?;
    outside.click().await?;

    // Focus goes back to the element that last had it inside the scope.
    page.wait_for_focus(&inside).await?;
    let left: bool = page.eval("return window.__outsideFocused;", vec![]).await?;
    assert_that!(left).is_true();
    Ok(())
}

/// When nested scopes (both with `restore_focus`) unmount, focus returns to the element that was
/// focused before the outermost scope mounted.
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
pub async fn outer_to_inner_navigation(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    click_to_focus(page, "#test-fs-nested-outer-btn").await?;
    press_to_focus(page, Key::Tab, "#test-fs-nested-inner-btn-1").await?;
    Ok(())
}

/// Without a node to restore in the DOM, focus goes to the first tabbable element of the nearest
/// ancestor scope; without one there, it stays on the body (upstream: "does not throw when there is
/// no focusable element to restore focus to").
pub async fn restore_fallback(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    for (prefix, fallback) in [
        ("#test-fs-fallback", "#test-fs-fallback-other"),
        ("#test-fs-fallback-empty", "body"),
    ] {
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
        page.focus_stays(&fallback).await?;
    }
    Ok(())
}

/// A dialog opened from a menu and rendered outside it restores focus to the menu's trigger (the
/// item it was opened from is gone). Upstream: "tracks node to restore if the node to restore was
/// removed in another part of the tree".
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

/// Focus lost to the body (a script blurs the focused element) goes back to that element, not the
/// first one in the scope. Upstream: "should restore focus to the last focused element in the
/// scope on focus out".
pub async fn restore_on_blur(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    click_to_focus(page, "#test-fs-contain-btn-1").await?;
    press_to_focus(page, Key::Tab, "#test-fs-contain-btn-2").await?;
    page.blur_focused().await?;
    page.wait_for_focus(&page.element("#test-fs-contain-btn-2").await?)
        .await?;
    Ok(())
}

/// Tabbing in a containing scope selects the text of the input it moves to (as the browser does).
/// Upstream: "should select all text in input when tabbing".
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
        assert_that!(input.prop("selectionStart").await?)
            .with_detail_message(next)
            .get_some()
            .is_equal_to("0");
        assert_that!(input.prop("selectionEnd").await?)
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

/// Tab with focus outside the active containing scope (here in a top layer, where focus may go)
/// is left to the browser. Upstream: `useFocusContainment`'s keydown handler returns unless the
/// focused element is in the scope.
pub async fn tab_outside_the_scope_is_native(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    click_to_focus(page, "#test-fs-contain-btn-1").await?;
    click_to_focus(page, "#test-fs-top-layer-1").await?;
    press_to_focus(page, Key::Tab, "#test-fs-top-layer-2").await?;
    Ok(())
}

/// `contain` may change while the scope is mounted: Tab wraps only while it contains.
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

/// A listener calling `preventDefault` on the restore event keeps focus from being restored; the
/// event doesn't leave a scope around the node to restore. Upstream: "should allow restoration to
/// be overridden with a custom event", "should not bubble focus scope restoration event out of
/// nested focus scopes".
pub async fn cancelled_restore(page: &Page<'_>) -> Result<(), Report> {
    goto(page).await?;
    // Cancelled: focus stays on the body.
    page.element("#test-fs-cancel-show").await?.click().await?;
    page.wait_for_focus(&page.element("#test-fs-cancel-input").await?)
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("#test-fs-cancel-input", 0).await?;
    let body = page.element("body").await?;
    page.wait_for_focus(&body).await?;
    page.focus_stays(&body).await?;

    // The cancelling listener sits outside a scope around the node to restore: restored.
    let show = page.element("#test-fs-nested-cancel-show").await?;
    show.click().await?;
    page.wait_for_focus(&page.element("#test-fs-nested-cancel-input").await?)
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("#test-fs-nested-cancel-input", 0)
        .await?;
    page.wait_for_focus(&show).await?;
    Ok(())
}

/// Tab out of a scope that restores focus but doesn't contain it continues after the node to
/// restore; Shift+Tab before it. Upstream: "should move focus to the element after the previously
/// focused node on Tab", "should move focus to the previous element after the previously focused
/// node on Shift+Tab".
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
