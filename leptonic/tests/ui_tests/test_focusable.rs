// Upstream: react-aria/test/interactions/Focusable.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, focusable::FocusablePage};

/// `use_focusable`: tab index for disabled and excluded elements, auto focus, keyboard events and
/// the focus handle ("supports isDisabled", "supports excludeFromTabOrder", "supports autoFocus").
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
    page.wait_for_active_id("test-fcbl-autofocus").await?;

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
    page.wait_for_text("test-fcbl-keydown-count", "1").await?;
    page.wait_for_text("test-fcbl-keyup-count", "1").await?;

    Ok(())
}

/// Tab-key skip: Tab from normal element skips disabled and excluded, lands on
/// the next tabbable element (test-fcbl-tab-target).
async fn test_tab_skip(page: &FocusablePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: Tab skips disabled and excluded elements");
    page.goto().await?;

    // Focus the normal element
    page.click_normal().await?;
    page.wait_for_active_id("test-fcbl-normal").await?;

    // Tab: should skip disabled (no tabindex) and excluded (tabindex=-1),
    // landing on the next tabbable element
    page.press_tab().await?;
    page.wait_for_active_id("test-fcbl-tab-target").await?;

    Ok(())
}

/// Programmatic focus via FocusHandle.
async fn test_focus_handle(page: &FocusablePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: FocusHandle programmatic focus");
    page.goto().await?;

    // Click the programmatic focus button
    page.click_focus_btn().await?;

    // The normal focusable element should now be focused
    page.wait_for_active_id("test-fcbl-normal").await?;

    Ok(())
}

/// Dynamic disabled transition: toggling disabled reactively updates tabindex.
async fn test_dynamic_disabled_transition(page: &FocusablePage<'_>) -> Result<(), Report> {
    tracing::info!("Test: dynamic disabled transition updates tabindex");
    page.goto().await?;

    // Initial: enabled, tabindex="0"
    assert_that!(page.read_dynamic_tabindex().await?).is_equal_to(Some("0".to_string()));

    let dynamic = page.driver.find(By::Id("test-fcbl-dynamic")).await?;

    // Toggle to disabled: tabindex becomes None
    page.click_dynamic_toggle().await?;
    page.wait_for_attr(&dynamic, "tabindex", None).await?;

    // Toggle back to enabled: tabindex="0"
    page.click_dynamic_toggle().await?;
    page.wait_for_attr(&dynamic, "tabindex", Some("0")).await?;

    Ok(())
}
