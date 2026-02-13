use std::time::Duration;

use assertr::prelude::*;
use thirtyfour::{TimeoutConfiguration, WebDriver};

use crate::{pages::focus_manager::FocusManagerPage, ui_tests::UiTest};

pub struct FocusManagerTests {}

#[async_trait::async_trait]
impl UiTest for FocusManagerTests {
    fn name(&self) -> String {
        "focus_manager_tests".to_string()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> anyhow::Result<()> {
        let mut timeouts = TimeoutConfiguration::default();
        timeouts.set_implicit(Some(Duration::from_secs(3)));
        driver.update_timeouts(timeouts).await?;

        let page = FocusManagerPage { driver, base_url };

        test_basic_navigation(&page).await?;
        test_wrap_next(&page).await?;
        test_wrap_prev(&page).await?;
        test_nowrap_boundary_next(&page).await?;
        test_nowrap_boundary_prev(&page).await?;
        test_tabbable_skip(&page).await?;
        test_nontabbable_include(&page).await?;
        test_accept_filter(&page).await?;
        test_radio_group_checked(&page).await?;
        test_radio_group_none_checked(&page).await?;
        test_radio_group_wrap_next(&page).await?;
        test_radio_group_wrap_prev(&page).await?;
        test_hidden_elements_skipped(&page).await?;
        test_inert_elements_skipped(&page).await?;
        test_focus_next_from_outside_scope(&page).await?;
        test_focus_previous_from_outside_scope(&page).await?;

        Ok(())
    }
}

/// Basic navigation: focus_first, focus_next, focus_previous, focus_last.
async fn test_basic_navigation(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: basic navigation");
    page.goto().await?;

    // Focus first: item-1 should be focused
    page.click_focus_first().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-item-1".to_string()));

    // Focus next: item-2 should be focused
    page.click_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-item-2".to_string()));

    // Focus next: item-3 should be focused
    page.click_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-item-3".to_string()));

    // Focus previous: item-2 should be focused
    page.click_focus_prev().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-item-2".to_string()));

    // Focus last: item-3 should be focused
    page.click_focus_last().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-item-3".to_string()));

    Ok(())
}

/// wrap: true — focus_next at last element wraps to first.
async fn test_wrap_next(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: wrap=true focus_next wraps at end");
    page.goto().await?;

    // Focus last item in wrap scope
    page.click_wrap_item(3).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-wrap-item-3".to_string()));

    // wrap focus_next → should wrap to item-1
    page.click_wrap_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-wrap-item-1".to_string()));

    Ok(())
}

/// wrap: true — focus_previous at first element wraps to last.
async fn test_wrap_prev(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: wrap=true focus_previous wraps at start");
    page.goto().await?;

    // Focus first item in wrap scope
    page.click_wrap_item(1).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-wrap-item-1".to_string()));

    // wrap focus_previous → should wrap to item-3
    page.click_wrap_focus_prev().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-wrap-item-3".to_string()));

    Ok(())
}

/// wrap: false — focus_next at last element stays put.
async fn test_nowrap_boundary_next(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: wrap=false focus_next stays at end");
    page.goto().await?;

    // Focus last item in wrap scope
    page.click_wrap_item(3).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-wrap-item-3".to_string()));

    // no-wrap focus_next → should stay on item-3
    page.click_nowrap_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-wrap-item-3".to_string()));

    Ok(())
}

/// wrap: false — focus_previous at first element stays put.
async fn test_nowrap_boundary_prev(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: wrap=false focus_previous stays at start");
    page.goto().await?;

    // Focus first item in wrap scope
    page.click_wrap_item(1).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-wrap-item-1".to_string()));

    // no-wrap focus_previous → should stay on item-1
    page.click_nowrap_focus_prev().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-wrap-item-1".to_string()));

    Ok(())
}

/// tabbable: true — skips item with tabindex=-1.
async fn test_tabbable_skip(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: tabbable=true skips tabindex=-1 items");
    page.goto().await?;

    // Focus item-1 in the tabbable scope
    page.click_tabbable_item(1).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-tabbable-item-1".to_string()));

    // tabbable focus_next → should skip item-2 (tabindex=-1) and land on item-3
    page.click_tabbable_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-tabbable-item-3".to_string()));

    // tabbable focus_prev → should skip item-2 and land on item-1
    page.click_tabbable_focus_prev().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-tabbable-item-1".to_string()));

    Ok(())
}

/// tabbable: false — includes items with tabindex=-1.
async fn test_nontabbable_include(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: tabbable=false includes tabindex=-1 items");
    page.goto().await?;

    // Focus item-1 in the tabbable scope
    page.click_tabbable_item(1).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-tabbable-item-1".to_string()));

    // non-tabbable focus_next → should include item-2 (tabindex=-1)
    page.click_nontabbable_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-tabbable-item-2".to_string()));

    Ok(())
}

/// accept filter — custom predicate rejects item-2 during navigation.
async fn test_accept_filter(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: accept filter skips rejected elements");
    page.goto().await?;

    // Focus item-1 in the accept scope
    page.click_accept_item(1).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-accept-item-1".to_string()));

    // accept focus_next → should skip item-2 (rejected by filter) and land on item-3
    page.click_accept_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-accept-item-3".to_string()));

    // accept focus_prev → should skip item-2 and land on item-1
    page.click_accept_focus_prev().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-accept-item-1".to_string()));

    Ok(())
}

/// Radio group with one checked radio — tabbable navigation only stops at the checked radio.
async fn test_radio_group_checked(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: radio group with checked radio — skips unchecked radios");
    page.goto().await?;

    // Focus the button before the radio group.
    page.click_radio_item("test-fm-radio-btn-before").await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-radio-btn-before".to_string()));

    // Tabbable focus_next → should skip unchecked radios (a, c), land on checked radio (b).
    page.click_radio_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-radio-b".to_string()));

    // Tabbable focus_next again → should skip remaining unchecked radio (c), land on button after.
    page.click_radio_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-radio-btn-after".to_string()));

    // Tabbable focus_prev → should land back on checked radio (b), skipping unchecked (c).
    page.click_radio_focus_prev().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-radio-b".to_string()));

    Ok(())
}

/// Radio group with no checked radio — tabbable navigation stops at the first radio only.
async fn test_radio_group_none_checked(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: radio group with no checked radio — stops at first radio");
    page.goto().await?;

    // Focus the button before the radio group.
    page.click_radio_item("test-fm-radio-none-btn-before")
        .await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-radio-none-btn-before".to_string()));

    // Tabbable focus_next → should land on the first radio (a) since none are checked.
    page.click_radio_none_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-radio-none-a".to_string()));

    // Tabbable focus_next again → should skip radios b and c (same group), land on button after.
    page.click_radio_none_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-radio-none-btn-after".to_string()));

    Ok(())
}

/// Radio group wrap: focus_next with wrap+tabbable wraps back to the checked radio.
async fn test_radio_group_wrap_next(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: radio group wrap — focus_next wraps to checked radio");
    page.goto().await?;

    // Focus the checked radio (b).
    page.click_radio_item("test-fm-radio-wrap-b").await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-radio-wrap-b".to_string()));

    // Tabbable wrap focus_next → no next tabbable (same-group radios filtered),
    // wrap should reset radio group context and find checked radio (b) again.
    page.click_radio_wrap_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-radio-wrap-b".to_string()));

    Ok(())
}

/// Radio group wrap: focus_previous with wrap+tabbable wraps back to the checked radio.
async fn test_radio_group_wrap_prev(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: radio group wrap — focus_previous wraps to checked radio");
    page.goto().await?;

    // Focus the checked radio (b).
    page.click_radio_item("test-fm-radio-wrap-b").await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-radio-wrap-b".to_string()));

    // Tabbable wrap focus_previous → no previous tabbable (same-group radios filtered),
    // wrap should reset radio group context and find checked radio (b) again.
    page.click_radio_wrap_focus_prev().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-radio-wrap-b".to_string()));

    Ok(())
}

/// Hidden elements (display:none, hidden attr, visibility:hidden) are skipped.
async fn test_hidden_elements_skipped(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: hidden elements skipped during navigation");
    page.goto().await?;

    // Focus first visible item
    page.click_vis_item(1).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-vis-item-1".to_string()));

    // Focus next → should skip items 2-4 (hidden) and land on item 5
    page.click_vis_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-vis-item-5".to_string()));

    // Focus prev → should skip items 4-2 (hidden) and land back on item 1
    page.click_vis_focus_prev().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-vis-item-1".to_string()));

    Ok(())
}

/// Inert subtree elements are skipped during navigation.
async fn test_inert_elements_skipped(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: inert elements skipped during navigation");
    page.goto().await?;

    // Focus first item
    page.click_inert_item(1).await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-inert-item-1".to_string()));

    // Focus next → should skip item 2 (inert parent) and land on item 3
    page.click_inert_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-inert-item-3".to_string()));

    // Focus prev → should skip item 2 and land back on item 1
    page.click_inert_focus_prev().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-inert-item-1".to_string()));

    Ok(())
}

/// focus_next from outside scope focuses the first element in the scope.
async fn test_focus_next_from_outside_scope(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: focus_next from outside scope focuses first element");
    page.goto().await?;

    // Click the external button (outside the scope).
    page.click_outside_external().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-outside-external".to_string()));

    // focus_next → should focus the first element in the scope.
    page.click_outside_focus_next().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-outside-item-1".to_string()));

    Ok(())
}

/// focus_previous from outside scope focuses the last element in the scope.
async fn test_focus_previous_from_outside_scope(page: &FocusManagerPage<'_>) -> anyhow::Result<()> {
    tracing::info!("Test: focus_previous from outside scope focuses last element");
    page.goto().await?;

    // Click the external button (outside the scope).
    page.click_outside_external().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-outside-external".to_string()));

    // focus_previous → should focus the last element in the scope.
    page.click_outside_focus_prev().await?;
    assert_that(page.get_active_element_id().await?)
        .is_equal_to(Some("test-fm-outside-item-3".to_string()));

    Ok(())
}
