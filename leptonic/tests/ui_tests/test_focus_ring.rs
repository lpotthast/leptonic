use std::time::Duration;

use assertr::prelude::*;
use thirtyfour::{TimeoutConfiguration, WebDriver, prelude::*};

use crate::{pages::focus_ring::FocusRingPage, ui_tests::UiTest};

pub struct FocusRingTests {}

#[async_trait::async_trait]
impl UiTest for FocusRingTests {
    fn name(&self) -> String {
        "focus_ring_tests".to_string()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> anyhow::Result<()> {
        let mut timeouts = TimeoutConfiguration::default();
        timeouts.set_implicit(Some(Duration::from_secs(3)));
        driver.update_timeouts(timeouts).await?;

        let page = FocusRingPage { driver, base_url };

        test_basic_click_focus(&page).await?;
        test_basic_tab_focus(&page).await?;
        test_within_click_focus(&page).await?;
        test_within_tab_focus(&page).await?;
        test_modality_switch(&page).await?;
        test_arrow_key_keyboard_modality(&page).await?;
        test_disabled_focus_ring(&page).await?;

        Ok(())
    }
}

/// Click focus: focused=true but focus-visible=false (pointer modality).
async fn test_basic_click_focus(page: &FocusRingPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: basic click focus — no focus ring");
    page.goto().await?;

    // Initial state: not focused, not focus-visible
    assert_that(page.read_is_focused().await?).is_equal_to(false);
    assert_that(page.read_is_focus_visible().await?).is_equal_to(false);

    // Click target: focused=true but focus-visible=false (pointer modality)
    page.click_target().await?;
    assert_that(page.read_is_focused().await?).is_equal_to(true);
    assert_that(page.read_is_focus_visible().await?).is_equal_to(false);
    assert_that(page.read_data_focus_visible_attr().await?).is_equal_to(None);

    // Click elsewhere to unfocus
    page.click_elsewhere().await?;
    assert_that(page.read_is_focused().await?).is_equal_to(false);

    Ok(())
}

/// Tab focus: focused=true AND focus-visible=true (keyboard modality).
async fn test_basic_tab_focus(page: &FocusRingPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: basic tab focus — focus ring visible");
    page.goto().await?;

    // Tab to target from the "before" button: keyboard modality
    page.tab_from_before_to_target().await?;
    assert_that(page.read_is_focused().await?).is_equal_to(true);
    assert_that(page.read_is_focus_visible().await?).is_equal_to(true);
    assert_that(page.read_data_focus_visible_attr().await?).is_equal_to(Some("true".to_string()));

    // Click elsewhere: no longer focused
    page.click_elsewhere().await?;
    assert_that(page.read_is_focused().await?).is_equal_to(false);
    assert_that(page.read_data_focus_visible_attr().await?).is_equal_to(None);

    Ok(())
}

/// within: true, click focus — container shows focused but not focus-visible.
async fn test_within_click_focus(page: &FocusRingPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: within=true click focus — no focus ring");
    page.goto().await?;

    // Initial state
    assert_that(page.read_within_is_focused().await?).is_equal_to(false);
    assert_that(page.read_within_is_focus_visible().await?).is_equal_to(false);

    // Click child-1 (pointer modality): container is focused but not focus-visible
    page.click_within_child_1().await?;
    assert_that(page.read_within_is_focused().await?).is_equal_to(true);
    assert_that(page.read_within_is_focus_visible().await?).is_equal_to(false);

    // Click elsewhere: unfocused
    page.click_within_elsewhere().await?;
    assert_that(page.read_within_is_focused().await?).is_equal_to(false);

    Ok(())
}

/// within: true, Tab focus — container shows focused AND focus-visible.
async fn test_within_tab_focus(page: &FocusRingPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: within=true tab focus — focus ring visible");
    page.goto().await?;

    // Tab from "within-before" button to the first child in the container
    page.tab_from_within_before_to_child().await?;
    assert_that(page.read_within_is_focused().await?).is_equal_to(true);
    assert_that(page.read_within_is_focus_visible().await?).is_equal_to(true);

    // Click elsewhere: unfocused
    page.click_within_elsewhere().await?;
    assert_that(page.read_within_is_focused().await?).is_equal_to(false);
    assert_that(page.read_within_is_focus_visible().await?).is_equal_to(false);

    Ok(())
}

/// Modality switch: Tab-to-focus shows ring, then click same element hides ring.
async fn test_modality_switch(page: &FocusRingPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: modality switch — Tab then click");
    page.goto().await?;

    // Tab to target: keyboard modality → focus ring visible
    page.tab_from_before_to_target().await?;
    assert_that(page.read_is_focused().await?).is_equal_to(true);
    assert_that(page.read_is_focus_visible().await?).is_equal_to(true);

    // Click the same target element: switches to pointer modality → ring disappears
    page.click_target().await?;
    assert_that(page.read_is_focused().await?).is_equal_to(true);
    assert_that(page.read_is_focus_visible().await?).is_equal_to(false);

    Ok(())
}

/// Arrow key after click: switches to keyboard modality, focus ring becomes visible.
async fn test_arrow_key_keyboard_modality(page: &FocusRingPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: arrow key triggers keyboard modality");
    page.goto().await?;

    // Click target: pointer modality, no focus ring
    page.click_target().await?;
    assert_that(page.read_is_focused().await?).is_equal_to(true);
    assert_that(page.read_is_focus_visible().await?).is_equal_to(false);

    // Press ArrowDown: switches to keyboard modality, focus ring appears
    page.send_key_to_active(Key::Down).await?;
    assert_that(page.read_is_focus_visible().await?).is_equal_to(true);
    assert_that(page.read_data_focus_visible_attr().await?).is_equal_to(Some("true".to_string()));

    Ok(())
}

/// Disabled focus ring: click and tab both leave is_focused=false, is_focus_visible=false.
async fn test_disabled_focus_ring(page: &FocusRingPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: disabled focus ring suppresses focus signals");
    page.goto().await?;

    // Click the disabled target: is_focused and is_focus_visible stay false
    page.click_disabled_target().await?;
    assert_that(page.read_disabled_is_focused().await?).is_equal_to(false);
    assert_that(page.read_disabled_is_focus_visible().await?).is_equal_to(false);

    // Tab to disabled target: still false
    // (We click "before" first to set up a known position, then tab forward)
    page.click_before().await?;
    let active = page.driver.active_element().await?;
    active.send_keys(Key::Tab).await?;
    assert_that(page.read_disabled_is_focused().await?).is_equal_to(false);
    assert_that(page.read_disabled_is_focus_visible().await?).is_equal_to(false);

    Ok(())
}
