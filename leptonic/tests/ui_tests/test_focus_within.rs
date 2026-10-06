use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, focus_within::FocusWithinPage};

pub struct FocusWithinTests {}

#[async_trait]
impl BrowserTest<str> for FocusWithinTests {
    fn name(&self) -> Cow<'_, str> {
        "focus_within_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = FocusWithinPage { driver, base_url };

        test_basic_focus_within(&page).await?;
        test_disabled(&page).await?;
        test_change_callback(&page).await?;
        test_tab_into_and_out_of_container(&page).await?;
        test_nested_focus_within(&page).await?;

        Ok(())
    }
}

/// Basic focus within behavior: enter, move within, leave, re-enter.
async fn test_basic_focus_within(page: &FocusWithinPage<'_>) -> Result<(), Report> {
    tracing::info!("Test: basic focus within");
    page.goto().await?;

    // Initial state: no focus within
    assert_that!(page.read_is_focus_within().await?).is_equal_to(false);
    assert_that!(page.read_focus_within_count().await?).is_equal_to(0);
    assert_that!(page.read_blur_within_count().await?).is_equal_to(0);

    // Click input A: focus enters container
    page.click_input_a().await?;
    assert_that!(page.read_is_focus_within().await?).is_equal_to(true);
    assert_that!(page.read_focus_within_count().await?).is_equal_to(1);

    // Click input B: focus stays within (no additional focus-within event, no blur-within)
    page.click_input_b().await?;
    assert_that!(page.read_is_focus_within().await?).is_equal_to(true);
    assert_that!(page.read_focus_within_count().await?).is_equal_to(1);
    assert_that!(page.read_blur_within_count().await?).is_equal_to(0);

    // Click outside: focus leaves container
    page.click_outside().await?;
    assert_that!(page.read_is_focus_within().await?).is_equal_to(false);
    assert_that!(page.read_blur_within_count().await?).is_equal_to(1);

    // Click input A again: focus re-enters container
    page.click_input_a().await?;
    assert_that!(page.read_is_focus_within().await?).is_equal_to(true);
    assert_that!(page.read_focus_within_count().await?).is_equal_to(2);

    Ok(())
}

/// disabled: true — no callbacks fire, is_focus_within stays false.
async fn test_disabled(page: &FocusWithinPage<'_>) -> Result<(), Report> {
    tracing::info!("Test: disabled focus within");
    page.goto().await?;

    // Initial state
    assert_that!(page.read_disabled_is_focus_within().await?).is_equal_to(false);
    assert_that!(page.read_disabled_focus_count().await?).is_equal_to(0);

    // Click disabled input: focus count should not increment, is_focus_within stays false
    page.click_disabled_input().await?;
    assert_that!(page.read_disabled_is_focus_within().await?).is_equal_to(false);
    assert_that!(page.read_disabled_focus_count().await?).is_equal_to(0);

    // Click outside and back: still no change
    page.click_outside().await?;
    page.click_disabled_input().await?;
    assert_that!(page.read_disabled_is_focus_within().await?).is_equal_to(false);
    assert_that!(page.read_disabled_focus_count().await?).is_equal_to(0);

    Ok(())
}

/// on_focus_within_change — fires true on focus enter, false on focus leave.
async fn test_change_callback(page: &FocusWithinPage<'_>) -> Result<(), Report> {
    tracing::info!("Test: on_focus_within_change callback");
    page.goto().await?;

    // Initial state
    assert_that!(page.read_change_value().await?).is_equal_to(false);
    assert_that!(page.read_change_count().await?).is_equal_to(0);

    // Click change input: change callback fires with true
    page.click_change_input().await?;
    assert_that!(page.read_change_value().await?).is_equal_to(true);
    assert_that!(page.read_change_count().await?).is_equal_to(1);

    // Click outside: change callback fires with false
    page.click_outside().await?;
    assert_that!(page.read_change_value().await?).is_equal_to(false);
    assert_that!(page.read_change_count().await?).is_equal_to(2);

    // Re-enter: change callback fires with true again
    page.click_change_input().await?;
    assert_that!(page.read_change_value().await?).is_equal_to(true);
    assert_that!(page.read_change_count().await?).is_equal_to(3);

    Ok(())
}

/// Tab navigation into and out of the container.
async fn test_tab_into_and_out_of_container(page: &FocusWithinPage<'_>) -> Result<(), Report> {
    tracing::info!("Test: tab into and out of container");
    page.goto().await?;

    // Initial state: no focus within
    assert_that!(page.read_is_focus_within().await?).is_equal_to(false);

    // Click "before" button (outside container)
    page.click_before().await?;

    // Tab into container (should land on input-a)
    page.press_tab().await?;
    assert_that!(page.read_is_focus_within().await?).is_equal_to(true);
    assert_that!(page.read_focus_within_count().await?).is_equal_to(1);

    // Tab from input-a to input-b (still within container)
    page.press_tab().await?;
    assert_that!(page.read_is_focus_within().await?).is_equal_to(true);
    assert_that!(page.read_focus_within_count().await?).is_equal_to(1); // No re-trigger

    // Tab out of container
    page.press_tab().await?;
    assert_that!(page.read_is_focus_within().await?).is_equal_to(false);
    assert_that!(page.read_blur_within_count().await?).is_equal_to(1);

    Ok(())
}

/// Nested focus within: focusing a deeply nested input sets is_focus_within=true on both containers.
async fn test_nested_focus_within(page: &FocusWithinPage<'_>) -> Result<(), Report> {
    tracing::info!("Test: nested focus within");
    page.goto().await?;

    // Initial state: neither container reports focus within
    assert_that!(page.read_nested_outer_is_focus_within().await?).is_equal_to(false);
    assert_that!(page.read_nested_inner_is_focus_within().await?).is_equal_to(false);

    // Click nested input: both outer and inner report focus within
    page.click_nested_input().await?;
    assert_that!(page.read_nested_outer_is_focus_within().await?).is_equal_to(true);
    assert_that!(page.read_nested_inner_is_focus_within().await?).is_equal_to(true);

    // Click outside: both become false
    page.click_outside().await?;
    assert_that!(page.read_nested_outer_is_focus_within().await?).is_equal_to(false);
    assert_that!(page.read_nested_inner_is_focus_within().await?).is_equal_to(false);

    Ok(())
}
