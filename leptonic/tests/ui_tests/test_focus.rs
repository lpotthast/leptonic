use std::time::Duration;

use assertr::prelude::*;
use thirtyfour::{TimeoutConfiguration, WebDriver};

use crate::{pages::focus::FocusPage, ui_tests::UiTest};

pub struct FocusTests {}

#[async_trait::async_trait]
impl UiTest for FocusTests {
    fn name(&self) -> String {
        "focus_tests".to_string()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> anyhow::Result<()> {
        let mut timeouts = TimeoutConfiguration::default();
        timeouts.set_implicit(Some(Duration::from_secs(3)));
        driver.update_timeouts(timeouts).await?;

        let page = FocusPage { driver, base_url };

        test_basic_focus(&page).await?;
        test_tab_focus(&page).await?;
        test_focus_change_count(&page).await?;
        test_child_focus_does_not_trigger_parent(&page).await?;

        Ok(())
    }
}

/// Basic click focus/blur behavior and disabled target.
async fn test_basic_focus(page: &FocusPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: basic click focus");
    page.goto().await?;

    // Initial state: nothing focused
    assert_that(page.read_focus_count().await?).is_equal_to(0);
    assert_that(page.read_blur_count().await?).is_equal_to(0);
    assert_that(page.read_is_focused().await?).is_equal_to(false);

    // Click target: focus count increments, is_focused becomes true
    page.click_target().await?;
    assert_that(page.read_focus_count().await?).is_equal_to(1);
    assert_that(page.read_is_focused().await?).is_equal_to(true);

    // Click elsewhere: blur count increments, is_focused becomes false
    page.click_elsewhere().await?;
    assert_that(page.read_blur_count().await?).is_equal_to(1);
    assert_that(page.read_is_focused().await?).is_equal_to(false);

    // Click target again: focus count increments to 2
    page.click_target().await?;
    assert_that(page.read_focus_count().await?).is_equal_to(2);
    assert_that(page.read_is_focused().await?).is_equal_to(true);

    // Disabled target: click doesn't increment disabled focus count
    assert_that(page.read_disabled_focus_count().await?).is_equal_to(0);
    page.click_disabled_target().await?;
    assert_that(page.read_disabled_focus_count().await?).is_equal_to(0);

    Ok(())
}

/// Tab key triggers focus on the target element.
async fn test_tab_focus(page: &FocusPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: tab key focus");
    page.goto().await?;

    // Tab from "before" button to focus target
    page.tab_from_before_to_target().await?;
    assert_that(page.read_focus_count().await?).is_equal_to(1);
    assert_that(page.read_is_focused().await?).is_equal_to(true);

    Ok(())
}

/// Child focus events should NOT fire parent's use_focus callbacks.
async fn test_child_focus_does_not_trigger_parent(page: &FocusPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: child focus does not trigger parent");
    page.goto().await?;

    // Initial state: parent focus count is 0
    assert_that(page.read_parent_focus_count().await?).is_equal_to(0);

    // Click the child button: parent focus count should stay 0
    page.click_child().await?;
    assert_that(page.read_parent_focus_count().await?).is_equal_to(0);

    // Click the parent div directly: parent focus count becomes 1
    page.click_parent().await?;
    assert_that(page.read_parent_focus_count().await?).is_equal_to(1);

    // Click child again: parent receives blur (target != current_target for focus, so no parent focus callback)
    page.click_child().await?;
    assert_that(page.read_parent_blur_count().await?).is_equal_to(1);
    assert_that(page.read_parent_focus_count().await?).is_equal_to(1);

    Ok(())
}

/// on_focus_change callback count tracks focus/blur transitions.
async fn test_focus_change_count(page: &FocusPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: on_focus_change callback count");
    page.goto().await?;

    assert_that(page.read_focus_change_count().await?).is_equal_to(0);

    // Click target: focus change fires (true)
    page.click_target().await?;
    assert_that(page.read_focus_change_count().await?).is_equal_to(1);

    // Click elsewhere: focus change fires (false)
    page.click_elsewhere().await?;
    assert_that(page.read_focus_change_count().await?).is_equal_to(2);

    // Re-focus: focus change fires (true)
    page.click_target().await?;
    assert_that(page.read_focus_change_count().await?).is_equal_to(3);

    Ok(())
}
