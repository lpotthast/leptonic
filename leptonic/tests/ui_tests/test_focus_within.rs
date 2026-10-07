// Upstream: react-aria/test/interactions/useFocusWithin.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, focus_within::FocusWithinPage};

/// `use_focus_within`: focus entering and leaving an element tree, also when the focused element
/// is removed or disabled.
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
        test_focus_outside_after_a_hidden_blur(&page).await?;
        test_removal_of_the_focused_child(&page).await?;
        test_disabling_the_focused_element(&page).await?;

        Ok(())
    }
}

/// Waits for a counter or flag to show `expected`.
async fn expect(page: &FocusWithinPage<'_>, id: &str, expected: &str) -> Result<(), Report> {
    page.wait_for_text(id, expected).await
}

/// A negative check: give a wrong update time to happen, then check the value again.
async fn expect_stays(page: &FocusWithinPage<'_>, id: &str, expected: &str) -> Result<(), Report> {
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_that!(page.read_text_of(id).await?.trim().to_owned()).is_equal_to(expected.to_owned());
    Ok(())
}

/// Basic focus within behavior: enter, move within, leave, re-enter ("does handle focus events on
/// children").
async fn test_basic_focus_within(page: &FocusWithinPage<'_>) -> Result<(), Report> {
    page.goto().await?;

    assert_that!(page.read_is_focus_within().await?).is_equal_to(false);
    assert_that!(page.read_focus_within_count().await?).is_equal_to(0);
    assert_that!(page.read_blur_within_count().await?).is_equal_to(0);

    page.click_input_a().await?;
    expect(page, "test-fw-is-focus-within", "true").await?;
    expect(page, "test-fw-focus-within-count", "1").await?;

    // Moving within: no new focus within, no blur within.
    page.click_input_b().await?;
    page.wait_for_active_id("test-fw-input-b").await?;
    expect_stays(page, "test-fw-focus-within-count", "1").await?;
    expect_stays(page, "test-fw-blur-within-count", "0").await?;
    expect(page, "test-fw-is-focus-within", "true").await?;

    page.click_outside().await?;
    expect(page, "test-fw-is-focus-within", "false").await?;
    expect(page, "test-fw-blur-within-count", "1").await?;

    page.click_input_a().await?;
    expect(page, "test-fw-is-focus-within", "true").await?;
    expect(page, "test-fw-focus-within-count", "2").await
}

/// "does not handle focus events if disabled".
async fn test_disabled(page: &FocusWithinPage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.click_disabled_input().await?;
    page.wait_for_active_id("test-fw-disabled-input").await?;
    expect_stays(page, "test-fw-disabled-is-focus-within", "false").await?;
    expect_stays(page, "test-fw-disabled-focus-count", "0").await?;

    page.click_outside().await?;
    page.click_disabled_input().await?;
    page.wait_for_active_id("test-fw-disabled-input").await?;
    expect_stays(page, "test-fw-disabled-is-focus-within", "false").await?;
    expect_stays(page, "test-fw-disabled-focus-count", "0").await
}

/// `on_focus_within_change` fires true on focus enter, false on focus leave.
async fn test_change_callback(page: &FocusWithinPage<'_>) -> Result<(), Report> {
    page.goto().await?;

    assert_that!(page.read_change_value().await?).is_equal_to(false);
    assert_that!(page.read_change_count().await?).is_equal_to(0);

    page.click_change_input().await?;
    expect(page, "test-fw-change-value", "true").await?;
    expect(page, "test-fw-change-count", "1").await?;

    page.click_outside().await?;
    expect(page, "test-fw-change-value", "false").await?;
    expect(page, "test-fw-change-count", "2").await?;

    page.click_change_input().await?;
    expect(page, "test-fw-change-value", "true").await?;
    expect(page, "test-fw-change-count", "3").await
}

/// Tab navigation into and out of the container.
async fn test_tab_into_and_out_of_container(page: &FocusWithinPage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.click_before().await?;

    page.press_tab().await?;
    page.wait_for_active_id("test-fw-input-a").await?;
    expect(page, "test-fw-is-focus-within", "true").await?;
    expect(page, "test-fw-focus-within-count", "1").await?;

    page.press_tab().await?;
    page.wait_for_active_id("test-fw-input-b").await?;
    expect_stays(page, "test-fw-focus-within-count", "1").await?;
    expect(page, "test-fw-is-focus-within", "true").await?;

    page.press_tab().await?;
    expect(page, "test-fw-is-focus-within", "false").await?;
    expect(page, "test-fw-blur-within-count", "1").await
}

/// Focusing a deeply nested input sets focus within on both containers ("events bubble by
/// default": focus within doesn't stop the focus events).
async fn test_nested_focus_within(page: &FocusWithinPage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.click_nested_input().await?;
    expect(page, "test-fw-nested-outer-is-focus-within", "true").await?;
    expect(page, "test-fw-nested-inner-is-focus-within", "true").await?;

    page.click_outside().await?;
    expect(page, "test-fw-nested-outer-is-focus-within", "false").await?;
    expect(page, "test-fw-nested-inner-is-focus-within", "false").await
}

/// "should fire onBlur when focus occurs outside": no blur reached the container (a child stopped
/// its `focusout`), so the next focus outside ends focus within, with a blur on the container. The
/// outside input stops its `focusin` too, so this needs the capture-phase `focus` listener.
async fn test_focus_outside_after_a_hidden_blur(page: &FocusWithinPage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.click_element_with_id("test-fw-removal-quiet").await?;
    expect(page, "test-fw-removal-events", "focus,change:true").await?;

    page.click_element_with_id("test-fw-removal-outer").await?;
    page.wait_for_active_id("test-fw-removal-outer").await?;
    expect(
        page,
        "test-fw-removal-events",
        "focus,change:true,blur:test-fw-removal-container,change:false",
    )
    .await
}

/// Removing the focused child ends focus within (Chrome fires a blur for the removed element).
async fn test_removal_of_the_focused_child(page: &FocusWithinPage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.click_element_with_id("test-fw-removal-hide").await?;
    page.wait_for_no_selector("#test-fw-removal-hide").await?;
    expect(
        page,
        "test-fw-removal-events",
        "focus,change:true,blur:test-fw-removal-hide,change:false",
    )
    .await
}

/// "should fire onBlur when a focused element is disabled" (Firefox fires no blur then; the
/// synthetic blur observer dispatches one).
async fn test_disabling_the_focused_element(page: &FocusWithinPage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.click_element_with_id("test-fw-removal-disable")
        .await?;
    page.wait_for_selector("#test-fw-removal-disable[disabled]")
        .await?;
    expect(
        page,
        "test-fw-removal-events",
        "focus,change:true,blur:test-fw-removal-disable,change:false",
    )
    .await?;
    // Exactly one blur (native `focusout` and the observer's don't both count).
    expect_stays(
        page,
        "test-fw-removal-events",
        "focus,change:true,blur:test-fw-removal-disable,change:false",
    )
    .await
}
