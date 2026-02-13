use std::time::Duration;

use assertr::prelude::*;
use thirtyfour::{prelude::*, TimeoutConfiguration, WebDriver};

use crate::{pages::focus_scope::FocusScopePage, ui_tests::UiTest};

pub struct FocusScopeTests {}

#[async_trait::async_trait]
impl UiTest for FocusScopeTests {
    fn name(&self) -> String {
        "focus_scope_tests".to_string()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> anyhow::Result<()> {
        let mut timeouts = TimeoutConfiguration::default();
        timeouts.set_implicit(Some(Duration::from_secs(3)));
        driver.update_timeouts(timeouts).await?;

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

        Ok(())
    }
}

/// Auto-focus: on page load, the first button in the auto_focus scope should
/// have focus.
async fn test_auto_focus(page: &FocusScopePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: auto-focus on mount");
    page.goto().await?;

    // The page has auto_focus=true on the second section. After navigating,
    // the first tabbable element in that scope should be focused.
    let active_id = page.get_active_element_id().await?;
    assert_that(active_id).is_equal_to(Some("test-fs-autofocus-btn-1".to_string()));

    Ok(())
}

/// Tab wrapping: Tab cycles through the contained scope and wraps around.
async fn test_tab_wrapping(page: &FocusScopePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: Tab wrapping within containing scope");
    page.goto().await?;

    // Click btn-1 to focus it and activate the containing scope.
    page.click_contain_btn_1().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fs-contain-btn-1".to_string()));

    // Tab -> btn-2
    let active = page.driver.active_element().await?;
    active.send_keys(Key::Tab).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fs-contain-btn-2".to_string()));

    // Tab -> btn-3
    let active = page.driver.active_element().await?;
    active.send_keys(Key::Tab).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fs-contain-btn-3".to_string()));

    // Tab -> btn-1 (wraps)
    let active = page.driver.active_element().await?;
    active.send_keys(Key::Tab).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fs-contain-btn-1".to_string()));

    Ok(())
}

/// Shift+Tab wrapping: Shift+Tab cycles backwards and wraps.
async fn test_shift_tab_wrapping(page: &FocusScopePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: Shift+Tab wrapping within containing scope");
    page.goto().await?;

    // Ensure btn-1 is focused.
    page.click_contain_btn_1().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fs-contain-btn-1".to_string()));

    // Shift+Tab -> btn-3 (wraps backwards)
    let active = page.driver.active_element().await?;
    active.send_keys(Key::Shift + Key::Tab).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fs-contain-btn-3".to_string()));

    Ok(())
}

/// Focus restoration: when a scope with `restore_focus=true` unmounts, focus
/// returns to the element that had focus before the scope appeared.
async fn test_focus_restoration(page: &FocusScopePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: focus restoration on unmount");
    page.goto().await?;

    // Click toggle to show the restore scope (it has auto_focus=true, so
    // focus should move into the scope).
    page.click_restore_toggle().await?;
    tokio::time::sleep(Duration::from_millis(200)).await;

    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fs-restore-btn".to_string()));

    // Click toggle again to hide the scope — focus should return to the toggle button.
    page.click_restore_toggle().await?;
    tokio::time::sleep(Duration::from_millis(200)).await;

    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fs-restore-toggle".to_string()));

    Ok(())
}

/// Nested scopes: inner scope's containment is respected; outer scope doesn't
/// yank focus away from the inner scope.
async fn test_nested_scopes(page: &FocusScopePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: nested scopes — inner containment respected");
    page.goto().await?;

    // Click inner btn-1.
    page.click_nested_inner_btn_1().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fs-nested-inner-btn-1".to_string()));

    // Tab -> inner btn-2.
    let active = page.driver.active_element().await?;
    active.send_keys(Key::Tab).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fs-nested-inner-btn-2".to_string()));

    // Tab -> wraps back to inner btn-1 (stays in inner scope).
    let active = page.driver.active_element().await?;
    active.send_keys(Key::Tab).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fs-nested-inner-btn-1".to_string()));

    Ok(())
}

/// Containment blocks escape: clicking outside a containing scope should
/// pull focus back into the scope.
async fn test_containment_blocks_escape(page: &FocusScopePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: containment blocks focus escape via click outside");
    page.goto().await?;

    // Focus btn-1 in the containing scope.
    page.click_contain_btn_1().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fs-contain-btn-1".to_string()));

    // Click the outside button — focus should be recaptured.
    page.click_outside().await?;
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Focus should be back inside the containing scope (either btn-1 or wherever
    // it was last tracked).
    let active_id = page.get_active_element_id().await?;
    let is_in_scope = active_id
        .as_deref()
        .is_some_and(|id| id.starts_with("test-fs-contain-btn-"));
    assert_that(is_in_scope).is_equal_to(true);

    Ok(())
}

/// Nested restore: when nested scopes (both with restore_focus) unmount,
/// focus should return to the element that was focused before the outermost
/// scope mounted.
async fn test_nested_restore_focuses_outermost(page: &FocusScopePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: nested restore — outermost scope's restore wins");
    page.goto().await?;

    // 1. Click the trigger to show nested scopes.
    //    The trigger button receives focus first, becoming the outer scope's node_to_restore.
    page.click_nested_restore_trigger().await?;
    tokio::time::sleep(Duration::from_millis(300)).await;

    // 2. Auto-focus should have moved focus into the inner scope.
    let active_id = page.get_active_element_id().await?;
    assert_that(active_id).is_equal_to(Some("test-fs-nested-restore-inner-btn".to_string()));

    // 3. Click the trigger again to hide both scopes at once.
    page.click_nested_restore_trigger().await?;
    tokio::time::sleep(Duration::from_millis(300)).await;

    // 4. Focus should be restored to the trigger button (the outermost scope's node_to_restore).
    let active_id = page.get_active_element_id().await?;
    assert_that(active_id).is_equal_to(Some("test-fs-nested-restore-trigger".to_string()));

    Ok(())
}

/// Outer-to-inner navigation: Tab from the outer scope's button should enter
/// the inner scope.
async fn test_outer_to_inner_navigation(page: &FocusScopePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: outer-to-inner navigation via Tab");
    page.goto().await?;

    // Click outer button to focus it.
    page.click_nested_outer_btn().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fs-nested-outer-btn".to_string()));

    // Tab from outer button — should enter the inner scope.
    page.tab_from_active().await?;
    let active_id = page.get_active_element_id().await?;
    let is_in_inner_scope = active_id
        .as_deref()
        .is_some_and(|id| id.starts_with("test-fs-nested-inner-btn-"));
    assert_that(is_in_inner_scope).is_equal_to(true);

    Ok(())
}
