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
    let active_id = page.active_element_id().await?;
    assert_that!(active_id).is_equal_to(Some("test-fs-autofocus-btn-1".to_string()));

    Ok(())
}

/// Tab wrapping: Tab cycles through the contained scope and wraps around.
async fn test_tab_wrapping(page: &FocusScopePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: Tab wrapping within containing scope");
    page.goto().await?;

    // Click btn-1 to focus it and activate the containing scope.
    page.click_contain_btn_1().await?;
    assert_that!(page.active_element_id().await?)
        .is_equal_to(Some("test-fs-contain-btn-1".to_string()));

    // Tab -> btn-2
    let active = page.driver.active_element().await?;
    active.send_keys(Key::Tab).await?;
    assert_that!(page.active_element_id().await?)
        .is_equal_to(Some("test-fs-contain-btn-2".to_string()));

    // Tab -> btn-3
    let active = page.driver.active_element().await?;
    active.send_keys(Key::Tab).await?;
    assert_that!(page.active_element_id().await?)
        .is_equal_to(Some("test-fs-contain-btn-3".to_string()));

    // Tab -> btn-1 (wraps)
    let active = page.driver.active_element().await?;
    active.send_keys(Key::Tab).await?;
    assert_that!(page.active_element_id().await?)
        .is_equal_to(Some("test-fs-contain-btn-1".to_string()));

    Ok(())
}

/// Shift+Tab wrapping: Shift+Tab cycles backwards and wraps.
async fn test_shift_tab_wrapping(page: &FocusScopePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: Shift+Tab wrapping within containing scope");
    page.goto().await?;

    // Ensure btn-1 is focused.
    page.click_contain_btn_1().await?;
    assert_that!(page.active_element_id().await?)
        .is_equal_to(Some("test-fs-contain-btn-1".to_string()));

    // Shift+Tab -> btn-3 (wraps backwards)
    let active = page.driver.active_element().await?;
    active.send_keys(Key::Shift + Key::Tab).await?;
    assert_that!(page.active_element_id().await?)
        .is_equal_to(Some("test-fs-contain-btn-3".to_string()));

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

    // Click inner btn-1.
    page.click_nested_inner_btn_1().await?;
    assert_that!(page.active_element_id().await?)
        .is_equal_to(Some("test-fs-nested-inner-btn-1".to_string()));

    // Tab -> inner btn-2.
    let active = page.driver.active_element().await?;
    active.send_keys(Key::Tab).await?;
    assert_that!(page.active_element_id().await?)
        .is_equal_to(Some("test-fs-nested-inner-btn-2".to_string()));

    // Tab -> wraps back to inner btn-1 (stays in inner scope).
    let active = page.driver.active_element().await?;
    active.send_keys(Key::Tab).await?;
    assert_that!(page.active_element_id().await?)
        .is_equal_to(Some("test-fs-nested-inner-btn-1".to_string()));

    Ok(())
}

/// Containment blocks escape: clicking outside a containing scope should
/// pull focus back into the scope.
async fn test_containment_blocks_escape(page: &FocusScopePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: containment blocks focus escape via click outside");
    page.goto().await?;

    // Focus btn-1 in the containing scope.
    page.click_contain_btn_1().await?;
    assert_that!(page.active_element_id().await?)
        .is_equal_to(Some("test-fs-contain-btn-1".to_string()));

    // Click the outside button — focus should be recaptured.
    page.click_outside().await?;

    // Focus goes back to the element that last had it inside the scope.
    page.wait_for_active_id("test-fs-contain-btn-1").await?;

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

    // Click outer button to focus it.
    page.click_nested_outer_btn().await?;
    assert_that!(page.active_element_id().await?)
        .is_equal_to(Some("test-fs-nested-outer-btn".to_string()));

    // Tab from outer button — should enter the inner scope.
    page.press_tab().await?;
    let active_id = page.active_element_id().await?;
    let is_in_inner_scope = active_id
        .as_deref()
        .is_some_and(|id| id.starts_with("test-fs-nested-inner-btn-"));
    assert_that!(is_in_inner_scope).is_equal_to(true);

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
