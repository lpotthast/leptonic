// Upstream: react-aria/test/interactions/useFocus.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, focus::FocusPage};

/// `use_focus`: focus and blur of the element itself (not its children), disabled, and a blur
/// when the focused element becomes disabled.
pub struct FocusTests {}

#[async_trait]
impl BrowserTest<str> for FocusTests {
    fn name(&self) -> Cow<'_, str> {
        "focus_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = FocusPage { driver, base_url };

        test_basic_focus(&page).await?;
        test_tab_focus(&page).await?;
        test_focus_change_count(&page).await?;
        test_child_focus_does_not_trigger_parent(&page).await?;
        test_blur_when_disabled_while_focused(&page).await?;

        Ok(())
    }
}

/// A negative check: give a wrong update time to happen, then check the value again.
async fn expect_stays(page: &FocusPage<'_>, id: &str, expected: &str) -> Result<(), Report> {
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_that!(page.read_text_of(id).await?.trim().to_owned()).is_equal_to(expected.to_owned());
    Ok(())
}

/// "handles focus events on the immediate target", "does not handle focus events if disabled".
async fn test_basic_focus(page: &FocusPage<'_>) -> Result<(), Report> {
    page.goto().await?;

    assert_that!(page.read_focus_count().await?).is_equal_to(0);
    assert_that!(page.read_blur_count().await?).is_equal_to(0);
    assert_that!(page.read_is_focused().await?).is_equal_to(false);

    page.click_target().await?;
    page.wait_for_text("test-focus-count", "1").await?;
    page.wait_for_text("test-is-focused", "true").await?;

    page.click_elsewhere().await?;
    page.wait_for_text("test-blur-count", "1").await?;
    page.wait_for_text("test-is-focused", "false").await?;

    page.click_target().await?;
    page.wait_for_text("test-focus-count", "2").await?;
    page.wait_for_text("test-is-focused", "true").await?;

    page.click_disabled_target().await?;
    page.wait_for_active_id("test-focus-disabled").await?;
    expect_stays(page, "test-disabled-focus-count", "0").await
}

/// The Tab key focuses the target.
async fn test_tab_focus(page: &FocusPage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.tab_from_before_to_target().await?;
    page.wait_for_text("test-focus-count", "1").await?;
    page.wait_for_text("test-is-focused", "true").await
}

/// "does not handle focus events on children".
async fn test_child_focus_does_not_trigger_parent(page: &FocusPage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.click_child().await?;
    page.wait_for_active_id("test-focus-child").await?;
    expect_stays(page, "test-focus-parent-focus-count", "0").await?;

    page.click_parent().await?;
    page.wait_for_text("test-focus-parent-focus-count", "1")
        .await?;

    // Focus moving to the child blurs the parent, but focusing the child isn't the parent's focus.
    page.click_child().await?;
    page.wait_for_text("test-focus-parent-blur-count", "1")
        .await?;
    expect_stays(page, "test-focus-parent-focus-count", "1").await
}

/// `on_focus_change` tracks focus/blur transitions.
async fn test_focus_change_count(page: &FocusPage<'_>) -> Result<(), Report> {
    page.goto().await?;

    assert_that!(page.read_focus_change_count().await?).is_equal_to(0);

    page.click_target().await?;
    page.wait_for_text("test-focus-change-count", "1").await?;

    page.click_elsewhere().await?;
    page.wait_for_text("test-focus-change-count", "2").await?;

    page.click_target().await?;
    page.wait_for_text("test-focus-change-count", "3").await
}

/// "should fire onBlur when a focused element is disabled" (Firefox fires no blur then; the
/// synthetic blur observer dispatches one), exactly once.
async fn test_blur_when_disabled_while_focused(page: &FocusPage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.click_element_with_id("test-focus-disable-me").await?;
    page.wait_for_selector("#test-focus-disable-me[disabled]")
        .await?;
    page.wait_for_text("test-focus-disable-me-blur-count", "1")
        .await?;
    expect_stays(page, "test-focus-disable-me-blur-count", "1").await
}
