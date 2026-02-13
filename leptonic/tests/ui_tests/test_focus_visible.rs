use std::time::Duration;

use assertr::prelude::*;
use thirtyfour::{prelude::*, TimeoutConfiguration, WebDriver};

use crate::{pages::focus_visible::FocusVisiblePage, ui_tests::UiTest};

pub struct FocusVisibleTests {}

#[async_trait::async_trait]
impl UiTest for FocusVisibleTests {
    fn name(&self) -> String {
        "focus_visible_tests".to_string()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> anyhow::Result<()> {
        let mut timeouts = TimeoutConfiguration::default();
        timeouts.set_implicit(Some(Duration::from_secs(3)));
        driver.update_timeouts(timeouts).await?;

        let page = FocusVisiblePage { driver, base_url };

        test_click_sets_pointer_modality(&page).await?;
        test_tab_sets_keyboard_modality(&page).await?;
        test_arrow_key_sets_keyboard_modality(&page).await?;
        test_typing_on_non_text_input_sets_keyboard_modality(&page).await?;
        test_typing_in_text_input_does_not_set_keyboard_modality(&page).await?;
        test_typing_in_text_input_silently_updates_stored_modality(&page).await?;
        test_pointer_after_keyboard(&page).await?;
        test_escape_sets_keyboard_modality(&page).await?;
        test_enter_sets_keyboard_modality(&page).await?;
        test_space_sets_keyboard_modality(&page).await?;

        Ok(())
    }
}

/// Click target: modality=Pointer, visible=false.
async fn test_click_sets_pointer_modality(page: &FocusVisiblePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: click sets pointer modality");
    page.goto().await?;

    page.click_target().await?;
    assert_that(page.read_modality().await?).is_equal_to("Pointer".to_string());
    assert_that(page.read_visible().await?).is_equal_to(false);

    Ok(())
}

/// Tab to target: modality=Keyboard, visible=true.
async fn test_tab_sets_keyboard_modality(page: &FocusVisiblePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: tab sets keyboard modality");
    page.goto().await?;

    page.tab_from_before_to_target().await?;
    assert_that(page.read_modality().await?).is_equal_to("Keyboard".to_string());
    assert_that(page.read_visible().await?).is_equal_to(true);

    Ok(())
}

/// Click target (pointer), then press ArrowDown: modality switches to Keyboard, visible=true.
async fn test_arrow_key_sets_keyboard_modality(page: &FocusVisiblePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: arrow key sets keyboard modality");
    page.goto().await?;

    // Click sets pointer modality
    page.click_target().await?;
    assert_that(page.read_modality().await?).is_equal_to("Pointer".to_string());

    // ArrowDown switches to keyboard modality
    page.send_key_to_active(Key::Down).await?;
    assert_that(page.read_modality().await?).is_equal_to("Keyboard".to_string());
    assert_that(page.read_visible().await?).is_equal_to(true);

    Ok(())
}

/// Click target (non-text-input), then type "a": modality switches to Keyboard (type-ahead).
/// With the negative key filter, any non-modifier key on a non-text-input triggers keyboard modality.
async fn test_typing_on_non_text_input_sets_keyboard_modality(
    page: &FocusVisiblePage<'_>,
) -> anyhow::Result<()> {
    tracing::info!("Test: typing on non-text-input sets keyboard modality");
    page.goto().await?;

    // Click sets pointer modality
    page.click_target().await?;
    assert_that(page.read_modality().await?).is_equal_to("Pointer".to_string());

    // Typing "a" on a non-text-input SHOULD switch to keyboard modality (type-ahead)
    page.send_key_to_active("a").await?;
    assert_that(page.read_modality().await?).is_equal_to("Keyboard".to_string());
    assert_that(page.read_visible().await?).is_equal_to(true);

    Ok(())
}

/// Click text input (pointer), then type "a": modality stays Pointer (text input guard).
/// The is_text_input guard suppresses keyboard modality for regular typing in text fields.
async fn test_typing_in_text_input_does_not_set_keyboard_modality(
    page: &FocusVisiblePage<'_>,
) -> anyhow::Result<()> {
    tracing::info!("Test: typing in text input does not set keyboard modality");
    page.goto().await?;

    // Click text input sets pointer modality
    page.click_text_input().await?;
    assert_that(page.read_modality().await?).is_equal_to("Pointer".to_string());

    // Typing "a" in a text input should NOT switch to keyboard modality
    page.send_key_to_active("a").await?;
    assert_that(page.read_modality().await?).is_equal_to("Pointer".to_string());
    assert_that(page.read_visible().await?).is_equal_to(false);

    Ok(())
}

/// Click text input, type "a": subscriber modality stays Pointer (not notified),
/// but stored (global) modality is silently updated to Keyboard.
/// This matches react-aria where `currentModality` is always updated for valid keys,
/// but per-subscriber filtering suppresses focus-visible updates for text input typing.
async fn test_typing_in_text_input_silently_updates_stored_modality(
    page: &FocusVisiblePage<'_>,
) -> anyhow::Result<()> {
    tracing::info!("Test: typing in text input silently updates stored modality");
    page.goto().await?;

    // Click text input sets pointer modality (both stored and subscriber)
    page.click_text_input().await?;
    assert_that(page.read_modality().await?).is_equal_to("Pointer".to_string());
    assert_that(page.read_stored_modality().await?).is_equal_to("Pointer".to_string());

    // Typing "a" in text input:
    // - Subscriber modality stays Pointer (not notified)
    // - Stored modality silently updates to Keyboard
    // - Focus visibility stays false
    page.send_key_to_active("a").await?;
    assert_that(page.read_modality().await?).is_equal_to("Pointer".to_string());
    assert_that(page.read_stored_modality().await?).is_equal_to("Keyboard".to_string());
    assert_that(page.read_visible().await?).is_equal_to(false);

    Ok(())
}

/// Click target (pointer), then press Escape: modality switches to Keyboard, visible=true.
async fn test_escape_sets_keyboard_modality(page: &FocusVisiblePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: Escape sets keyboard modality");
    page.goto().await?;

    page.click_target().await?;
    assert_that(page.read_modality().await?).is_equal_to("Pointer".to_string());

    page.send_key_to_active(Key::Escape).await?;
    assert_that(page.read_modality().await?).is_equal_to("Keyboard".to_string());
    assert_that(page.read_visible().await?).is_equal_to(true);

    Ok(())
}

/// Click target (pointer), then press Enter: modality switches to Keyboard, visible=true.
async fn test_enter_sets_keyboard_modality(page: &FocusVisiblePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: Enter sets keyboard modality");
    page.goto().await?;

    page.click_target().await?;
    assert_that(page.read_modality().await?).is_equal_to("Pointer".to_string());

    page.send_key_to_active(Key::Enter).await?;
    assert_that(page.read_modality().await?).is_equal_to("Keyboard".to_string());
    assert_that(page.read_visible().await?).is_equal_to(true);

    Ok(())
}

/// Click target (pointer), then press Space: modality switches to Keyboard, visible=true.
async fn test_space_sets_keyboard_modality(page: &FocusVisiblePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: Space sets keyboard modality");
    page.goto().await?;

    page.click_target().await?;
    assert_that(page.read_modality().await?).is_equal_to("Pointer".to_string());

    page.send_key_to_active(" ").await?;
    assert_that(page.read_modality().await?).is_equal_to("Keyboard".to_string());
    assert_that(page.read_visible().await?).is_equal_to(true);

    Ok(())
}

/// Tab to target (keyboard), then click target: modality switches to Pointer, visible=false.
async fn test_pointer_after_keyboard(page: &FocusVisiblePage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: pointer after keyboard");
    page.goto().await?;

    // Tab sets keyboard modality
    page.tab_from_before_to_target().await?;
    assert_that(page.read_modality().await?).is_equal_to("Keyboard".to_string());
    assert_that(page.read_visible().await?).is_equal_to(true);

    // Click switches to pointer modality
    page.click_target().await?;
    assert_that(page.read_modality().await?).is_equal_to("Pointer".to_string());
    assert_that(page.read_visible().await?).is_equal_to(false);

    Ok(())
}
