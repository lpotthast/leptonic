// Upstream: react-aria/test/focus/FocusScope.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{WebDriver, prelude::*},
};
use rootcause::Report;

use crate::pages::{BaseActions, focus_scope::FocusScopePage};

pub struct FocusScopeTests {}

#[async_trait]
impl BrowserTest<str> for FocusScopeTests {
    fn name(&self) -> Cow<'_, str> {
        "focus_scope_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = FocusScopePage { driver, base_url };

        // Each test navigates to the page fresh to avoid state leakage
        // (e.g., a containing scope staying active from a previous test).
        test_auto_focus(&page).await?;
        test_tab_wrapping(&page).await?;
        test_shift_tab_wrapping(&page).await?;
        test_focus_restoration(&page).await?;
        test_nested_scopes(&page).await?;
        test_containment_blocks_escape(&page).await?;
        test_outer_to_inner_navigation(&page).await?;
        test_nested_restore_focuses_outermost(&page).await?;
        test_restore_fallback(&page).await?;
        test_dialog_from_menu(&page).await?;
        test_restore_on_blur(&page).await?;
        test_select_on_tab(&page).await?;
        test_tab_outside_the_scope_is_native(&page).await?;
        test_runtime_contain(&page).await?;
        test_cancelled_restore(&page).await?;
        test_tab_out_of_restoring_scope(&page).await?;

        Ok(())
    }
}

/// Auto-focus: on page load, the first button in the auto_focus scope should
/// have focus.
async fn test_auto_focus(page: &FocusScopePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: auto-focus on mount");
    page.goto().await?;

    // The page has auto_focus=true on the second section. After navigating,
    // the first tabbable element in that scope should be focused.
    page.wait_for_active_id("test-fs-autofocus-btn-1").await?;

    Ok(())
}

/// Tab wrapping: Tab cycles through the contained scope and wraps around.
async fn test_tab_wrapping(page: &FocusScopePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: Tab wrapping within containing scope");
    page.goto().await?;

    // Click btn-1 to focus it and activate the containing scope.
    page.click_contain_btn_1().await?;
    page.wait_for_active_id("test-fs-contain-btn-1").await?;

    for next in [
        "test-fs-contain-btn-2",
        "test-fs-contain-btn-3",
        // Wraps.
        "test-fs-contain-btn-1",
    ] {
        page.send_keys_to_active(Key::Tab).await?;
        page.wait_for_active_id(next).await?;
    }

    Ok(())
}

/// Shift+Tab wrapping: Shift+Tab cycles backwards and wraps.
async fn test_shift_tab_wrapping(page: &FocusScopePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: Shift+Tab wrapping within containing scope");
    page.goto().await?;

    page.click_contain_btn_1().await?;
    page.wait_for_active_id("test-fs-contain-btn-1").await?;

    // Shift+Tab -> btn-3 (wraps backwards)
    page.send_keys_to_active(Key::Shift + Key::Tab).await?;
    page.wait_for_active_id("test-fs-contain-btn-3").await?;

    Ok(())
}

/// Focus restoration: when a scope with `restore_focus=true` unmounts, focus
/// returns to the element that had focus before the scope appeared.
async fn test_focus_restoration(page: &FocusScopePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: focus restoration on unmount");
    page.goto().await?;

    // Click toggle to show the restore scope (it has auto_focus=true, so
    // focus should move into the scope).
    page.click_restore_toggle().await?;
    page.wait_for_active_id("test-fs-restore-btn").await?;

    // Click toggle again to hide the scope — focus should return to the toggle button.
    page.click_restore_toggle().await?;
    page.wait_for_active_id("test-fs-restore-toggle").await?;

    Ok(())
}

/// Nested scopes: inner scope's containment is respected; outer scope doesn't
/// yank focus away from the inner scope.
async fn test_nested_scopes(page: &FocusScopePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: nested scopes — inner containment respected");
    page.goto().await?;

    page.click_nested_inner_btn_1().await?;
    page.wait_for_active_id("test-fs-nested-inner-btn-1")
        .await?;

    page.send_keys_to_active(Key::Tab).await?;
    page.wait_for_active_id("test-fs-nested-inner-btn-2")
        .await?;

    // Wraps back to inner btn-1 (stays in inner scope).
    page.send_keys_to_active(Key::Tab).await?;
    page.wait_for_active_id("test-fs-nested-inner-btn-1")
        .await?;

    Ok(())
}

/// Containment blocks escape: clicking outside a containing scope should
/// pull focus back into the scope.
async fn test_containment_blocks_escape(page: &FocusScopePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: containment blocks focus escape via click outside");
    page.goto().await?;

    page.click_contain_btn_1().await?;
    page.wait_for_active_id("test-fs-contain-btn-1").await?;

    // Record that focus really reaches the outside button (else this test would pass with focus
    // never leaving the scope).
    page.driver
        .execute(
            "window.__outsideFocused = false; \
             document.getElementById('test-fs-outside') \
                 .addEventListener('focus', () => { window.__outsideFocused = true; });",
            vec![],
        )
        .await?;
    page.click_outside().await?;

    // Focus goes back to the element that last had it inside the scope.
    page.wait_for_active_id("test-fs-contain-btn-1").await?;
    let left = page
        .driver
        .execute("return window.__outsideFocused;", vec![])
        .await?;
    assert_that!(left.json().as_bool()).is_equal_to(Some(true));

    Ok(())
}

/// Nested restore: when nested scopes (both with restore_focus) unmount,
/// focus should return to the element that was focused before the outermost
/// scope mounted.
async fn test_nested_restore_focuses_outermost(page: &FocusScopePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: nested restore — outermost scope's restore wins");
    page.goto().await?;

    // 1. Click the trigger to show nested scopes.
    //    The trigger button receives focus first, becoming the outer scope's node_to_restore.
    page.click_nested_restore_trigger().await?;

    // 2. Auto-focus should have moved focus into the inner scope.
    page.wait_for_active_id("test-fs-nested-restore-inner-btn")
        .await?;

    // 3. Click the trigger again to hide both scopes at once.
    page.click_nested_restore_trigger().await?;

    // 4. Focus should be restored to the trigger button (the outermost scope's node_to_restore).
    page.wait_for_active_id("test-fs-nested-restore-trigger")
        .await?;

    Ok(())
}

/// Outer-to-inner navigation: Tab from the outer scope's button should enter
/// the inner scope.
async fn test_outer_to_inner_navigation(page: &FocusScopePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: outer-to-inner navigation via Tab");
    page.goto().await?;

    page.click_nested_outer_btn().await?;
    page.wait_for_active_id("test-fs-nested-outer-btn").await?;

    // Tab from outer button — enters the inner scope.
    page.press_tab().await?;
    page.wait_for_active_id("test-fs-nested-inner-btn-1")
        .await?;

    Ok(())
}

/// Without a node to restore in the DOM, focus goes to the first tabbable element of the nearest
/// ancestor scope; without one there, it stays on the body (upstream: "does not throw when there is
/// no focusable element to restore focus to").
async fn test_restore_fallback(page: &FocusScopePage<'_>) -> Result<(), Report> {
    page.goto().await?;
    for (prefix, expected) in [
        ("test-fs-fallback", Some("test-fs-fallback-other")),
        ("test-fs-fallback-empty", None),
    ] {
        page.element(&format!("{prefix}-target"))
            .await?
            .click()
            .await?;
        page.wait_for_active_id(&format!("{prefix}-inside")).await?;
        page.element(&format!("{prefix}-inside"))
            .await?
            .click()
            .await?;
        page.wait_for_no_selector(&format!("#{prefix}-inside"))
            .await?;
        if let Some(id) = expected {
            page.wait_for_active_id(id).await?;
        } else {
            // Give a wrong restoration the frame it would take.
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            let tag = page
                .driver
                .execute("return document.activeElement.tagName;", vec![])
                .await?;
            assert_that!(tag.json().as_str()).is_equal_to(Some("BODY"));
        }
    }
    Ok(())
}

/// A dialog opened from a menu and rendered outside it restores focus to the menu's trigger (the
/// item it was opened from is gone). Upstream: "tracks node to restore if the node to restore was
/// removed in another part of the tree".
async fn test_dialog_from_menu(page: &FocusScopePage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.element("test-fs-open-menu").await?.focus().await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_active_id("test-fs-open-dialog").await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_active_id("test-fs-close-dialog").await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_no_selector("#test-fs-close-dialog").await?;
    page.wait_for_active_id("test-fs-open-menu").await
}

/// Focus lost to the body (a script blurs the focused element) goes back to that element, not the
/// first one in the scope. Upstream: "should restore focus to the last focused element in the
/// scope on focus out".
async fn test_restore_on_blur(page: &FocusScopePage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click_contain_btn_1().await?;
    page.send_keys_to_active(Key::Tab).await?;
    page.wait_for_active_id("test-fs-contain-btn-2").await?;
    page.driver
        .execute("document.activeElement.blur();", vec![])
        .await?;
    page.wait_for_active_id("test-fs-contain-btn-2").await
}

/// Tabbing in a containing scope selects the text of the input it moves to (as the browser does).
/// Upstream: "should select all text in input when tabbing".
async fn test_select_on_tab(page: &FocusScopePage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.element("test-fs-select-input-1")
        .await?
        .click()
        .await?;
    page.wait_for_active_id("test-fs-select-input-1").await?;
    for next in [
        "test-fs-select-input-2",
        "test-fs-select-input-3",
        // Wrapping selects too.
        "test-fs-select-input-1",
    ] {
        page.send_keys_to_active(Key::Tab).await?;
        page.wait_for_active_id(next).await?;
        let selection = page
            .driver
            .execute(
                "const e = document.activeElement; return [e.selectionStart, e.selectionEnd];",
                vec![],
            )
            .await?;
        assert_that!(selection.json().clone()).is_equal_to(serde_json::json!([0, 5]));
    }
    // Typing replaces the selected text.
    page.send_keys_to_active(Key::Delete).await?;
    let value = page
        .driver
        .execute("return document.activeElement.value;", vec![])
        .await?;
    assert_that!(value.json().as_str()).is_equal_to(Some(""));
    Ok(())
}

/// Tab with focus outside the active containing scope (here in a top layer, where focus may go)
/// is left to the browser. Upstream: `useFocusContainment`'s keydown handler returns unless the
/// focused element is in the scope.
async fn test_tab_outside_the_scope_is_native(page: &FocusScopePage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click_contain_btn_1().await?;
    page.wait_for_active_id("test-fs-contain-btn-1").await?;
    page.click_element_with_id("test-fs-top-layer-1").await?;
    page.wait_for_active_id("test-fs-top-layer-1").await?;
    page.send_keys_to_active(Key::Tab).await?;
    page.wait_for_active_id("test-fs-top-layer-2").await
}

/// `contain` may change while the scope is mounted: Tab wraps only while it contains.
async fn test_runtime_contain(page: &FocusScopePage<'_>) -> Result<(), Report> {
    page.goto().await?;
    // Not containing: Tab leaves the scope.
    page.click_element_with_id("test-fs-runtime-2").await?;
    page.wait_for_active_id("test-fs-runtime-2").await?;
    page.send_keys_to_active(Key::Tab).await?;
    page.wait_for_active_id("test-fs-runtime-after").await?;

    // Containing: Tab wraps.
    page.click_element_with_id("test-fs-runtime-toggle").await?;
    page.wait_for_text("test-fs-runtime-toggle", "Stop containing")
        .await?;
    page.send_keys_to_active(Key::Tab).await?;
    page.wait_for_active_id("test-fs-runtime-2").await?;
    page.send_keys_to_active(Key::Tab).await?;
    page.wait_for_active_id("test-fs-runtime-toggle").await?;

    // Not containing again: Tab leaves the scope.
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-fs-runtime-toggle", "Contain")
        .await?;
    page.send_keys_to_active(Key::Tab).await?;
    page.wait_for_active_id("test-fs-runtime-2").await?;
    page.send_keys_to_active(Key::Tab).await?;
    page.wait_for_active_id("test-fs-runtime-after").await
}

/// A listener calling `preventDefault` on the restore event keeps focus from being restored; the
/// event doesn't leave a scope around the node to restore. Upstream: "should allow restoration to
/// be overridden with a custom event", "should not bubble focus scope restoration event out of
/// nested focus scopes".
async fn test_cancelled_restore(page: &FocusScopePage<'_>) -> Result<(), Report> {
    page.goto().await?;
    // Cancelled: focus stays on the body.
    page.click_element_with_id("test-fs-cancel-show").await?;
    page.wait_for_active_id("test-fs-cancel-input").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("#test-fs-cancel-input").await?;
    // Give a restoration the frame it would take.
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    let tag = page
        .driver
        .execute("return document.activeElement.tagName;", vec![])
        .await?;
    assert_that!(tag.json().as_str()).is_equal_to(Some("BODY"));

    // The cancelling listener sits outside a scope around the node to restore: restored.
    page.click_element_with_id("test-fs-nested-cancel-show")
        .await?;
    page.wait_for_active_id("test-fs-nested-cancel-input")
        .await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("#test-fs-nested-cancel-input")
        .await?;
    page.wait_for_active_id("test-fs-nested-cancel-show").await
}

/// Tab out of a scope that restores focus but doesn't contain it continues after the node to
/// restore; Shift+Tab before it. Upstream: "should move focus to the element after the previously
/// focused node on Tab", "should move focus to the previous element after the previously focused
/// node on Shift+Tab".
async fn test_tab_out_of_restoring_scope(page: &FocusScopePage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click_element_with_id("test-fs-tab-trigger").await?;
    page.wait_for_active_id("test-fs-tab-input-1").await?;
    page.element("test-fs-tab-input-3").await?.click().await?;
    page.wait_for_active_id("test-fs-tab-input-3").await?;
    // Natively, Tab would go to `test-fs-tab-after`, the next element in the DOM.
    page.send_keys_to_active(Key::Tab).await?;
    page.wait_for_active_id("test-fs-tab-after-trigger").await?;

    // Reopen (the trigger restores into the scope again) and leave backwards.
    page.click_element_with_id("test-fs-tab-trigger").await?;
    page.wait_for_no_selector("#test-fs-tab-input-1").await?;
    page.click_element_with_id("test-fs-tab-trigger").await?;
    page.wait_for_active_id("test-fs-tab-input-1").await?;
    page.send_keys_to_active(Key::Shift + Key::Tab).await?;
    page.wait_for_active_id("test-fs-tab-before").await
}
