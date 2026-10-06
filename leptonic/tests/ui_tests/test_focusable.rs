use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, focusable::FocusablePage};

pub struct FocusableTests {}

#[async_trait]
impl BrowserTest<str> for FocusableTests {
    fn name(&self) -> Cow<'_, str> {
        "focusable_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = FocusablePage { driver, base_url };

        test_tabindex_attributes(&page).await?;
        test_keyboard_events(&page).await?;
        test_tab_skip(&page).await?;
        test_focus_handle(&page).await?;
        test_dynamic_disabled_transition(&page).await?;

        Ok(())
    }
}

/// Tab index attributes: normal=0, disabled=none, excluded=-1. Auto-focus on load.
async fn test_tabindex_attributes(page: &FocusablePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: tabindex attributes and auto-focus");
    page.goto().await?;

    // Normal element has tabindex="0"
    assert_that!(page.read_normal_tabindex().await?).is_equal_to(Some("0".to_string()));

    // Disabled element has no tabindex attribute
    assert_that!(page.read_disabled_tabindex().await?).is_equal_to(None);

    // Excluded element has tabindex="-1"
    assert_that!(page.read_excluded_tabindex().await?).is_equal_to(Some("-1".to_string()));

    // Auto-focus element is focused on page load
    assert_that!(page.active_element_id().await?)
        .is_equal_to(Some("test-fcbl-autofocus".to_string()));

    Ok(())
}

/// Keyboard events: keydown/keyup counters increment.
async fn test_keyboard_events(page: &FocusablePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: keyboard events on focusable element");
    page.goto().await?;

    // Click the normal element and press a key: keydown/keyup counters increment
    page.click_normal().await?;
    assert_that!(page.read_keydown_count().await?).is_equal_to(0);
    assert_that!(page.read_keyup_count().await?).is_equal_to(0);

    page.send_keys_to_active("a").await?;
    assert_that!(page.read_keydown_count().await?).is_equal_to(1);
    assert_that!(page.read_keyup_count().await?).is_equal_to(1);

    Ok(())
}

/// Tab-key skip: Tab from normal element skips disabled and excluded, lands on
/// the next tabbable element (test-fcbl-tab-target).
async fn test_tab_skip(page: &FocusablePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: Tab skips disabled and excluded elements");
    page.goto().await?;

    // Focus the normal element
    page.click_normal().await?;
    assert_that!(page.active_element_id().await?).is_equal_to(Some("test-fcbl-normal".to_string()));

    // Tab: should skip disabled (no tabindex) and excluded (tabindex=-1),
    // landing on the next tabbable element
    page.press_tab().await?;
    assert_that!(page.active_element_id().await?)
        .is_equal_to(Some("test-fcbl-tab-target".to_string()));

    Ok(())
}

/// Programmatic focus via FocusHandle.
async fn test_focus_handle(page: &FocusablePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: FocusHandle programmatic focus");
    page.goto().await?;

    // Click the programmatic focus button
    page.click_focus_btn().await?;

    // The normal focusable element should now be focused
    assert_that!(page.active_element_id().await?).is_equal_to(Some("test-fcbl-normal".to_string()));

    Ok(())
}

/// Dynamic disabled transition: toggling disabled reactively updates tabindex.
async fn test_dynamic_disabled_transition(page: &FocusablePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: dynamic disabled transition updates tabindex");
    page.goto().await?;

    // Initial: enabled, tabindex="0"
    assert_that!(page.read_dynamic_tabindex().await?).is_equal_to(Some("0".to_string()));

    // Toggle to disabled: tabindex becomes None
    page.click_dynamic_toggle().await?;
    assert_that!(page.read_dynamic_tabindex().await?).is_equal_to(None);

    // Toggle back to enabled: tabindex="0"
    page.click_dynamic_toggle().await?;
    assert_that!(page.read_dynamic_tabindex().await?).is_equal_to(Some("0".to_string()));

    Ok(())
}
