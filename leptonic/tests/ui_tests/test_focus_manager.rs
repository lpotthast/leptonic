// Upstream: react-aria/test/focus/FocusScope.test.js @ 99e6102368
use std::borrow::Cow;

use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::focus_manager::FocusManagerPage;

/// The focus manager of `FocusScope`: next, previous, first and last, with wrapping, tabbable
/// filtering, an accept filter, radio groups, hidden and inert elements, and from outside the
/// scope.
pub struct FocusManagerTests {}

#[async_trait]
impl BrowserTest<str> for FocusManagerTests {
    fn name(&self) -> Cow<'_, str> {
        "focus_manager_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = FocusManagerPage { driver, base_url };
        cases!(
            basic_navigation(&page),
            wrap_next(&page),
            wrap_prev(&page),
            nowrap_boundary_next(&page),
            nowrap_boundary_prev(&page),
            tabbable_skip(&page),
            nontabbable_include(&page),
            accept_filter(&page),
            radio_group_checked(&page),
            radio_group_none_checked(&page),
            radio_group_wrap_next(&page),
            radio_group_wrap_prev(&page),
            hidden_elements_skipped(&page),
            inert_elements_skipped(&page),
            focus_next_from_outside_scope(&page),
            focus_previous_from_outside_scope(&page),
        );
        Ok(())
    }
}

/// Basic navigation: focus_first, focus_next, focus_previous, focus_last.
async fn basic_navigation(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    for (control, expected) in [
        ("focus-first", "item-1"),
        ("focus-next", "item-2"),
        ("focus-next", "item-3"),
        ("focus-prev", "item-2"),
        ("focus-last", "item-3"),
    ] {
        page.click(control).await?;
        page.expect_focus(expected).await?;
    }
    Ok(())
}

/// wrap: true — focus_next at the last element wraps to the first.
async fn wrap_next(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("wrap-item-3").await?;
    page.expect_focus("wrap-item-3").await?;
    page.click("wrap-focus-next").await?;
    page.expect_focus("wrap-item-1").await?;
    Ok(())
}

/// wrap: true — focus_previous at the first element wraps to the last.
async fn wrap_prev(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("wrap-item-1").await?;
    page.expect_focus("wrap-item-1").await?;
    page.click("wrap-focus-prev").await?;
    page.expect_focus("wrap-item-3").await?;
    Ok(())
}

/// wrap: false — focus_next at the last element stays put.
async fn nowrap_boundary_next(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("wrap-item-3").await?;
    page.expect_focus("wrap-item-3").await?;
    page.click("nowrap-focus-next").await?;
    page.expect_focus_stays("wrap-item-3").await?;
    Ok(())
}

/// wrap: false — focus_previous at the first element stays put.
async fn nowrap_boundary_prev(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("wrap-item-1").await?;
    page.expect_focus("wrap-item-1").await?;
    page.click("nowrap-focus-prev").await?;
    page.expect_focus_stays("wrap-item-1").await?;
    Ok(())
}

/// tabbable: true — skips the item with tabindex=-1.
async fn tabbable_skip(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("tabbable-item-1").await?;
    page.expect_focus("tabbable-item-1").await?;
    page.click("tabbable-focus-next").await?;
    page.expect_focus("tabbable-item-3").await?;
    page.click("tabbable-focus-prev").await?;
    page.expect_focus("tabbable-item-1").await?;
    Ok(())
}

/// tabbable: false — includes items with tabindex=-1.
async fn nontabbable_include(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("tabbable-item-1").await?;
    page.expect_focus("tabbable-item-1").await?;
    page.click("nontabbable-focus-next").await?;
    page.expect_focus("tabbable-item-2").await?;
    Ok(())
}

/// An accept filter rejects item 2 during navigation.
async fn accept_filter(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("accept-item-1").await?;
    page.expect_focus("accept-item-1").await?;
    page.click("accept-focus-next").await?;
    page.expect_focus("accept-item-3").await?;
    page.click("accept-focus-prev").await?;
    page.expect_focus("accept-item-1").await?;
    Ok(())
}

/// A radio group with a checked radio: tabbable navigation stops only at the checked radio.
async fn radio_group_checked(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("radio-btn-before").await?;
    page.expect_focus("radio-btn-before").await?;
    // Skips the unchecked radios a and c.
    page.click("radio-focus-next").await?;
    page.expect_focus("radio-b").await?;
    page.click("radio-focus-next").await?;
    page.expect_focus("radio-btn-after").await?;
    page.click("radio-focus-prev").await?;
    page.expect_focus("radio-b").await?;
    Ok(())
}

/// A radio group without a checked radio: tabbable navigation stops at the first radio only.
async fn radio_group_none_checked(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("radio-none-btn-before").await?;
    page.expect_focus("radio-none-btn-before").await?;
    page.click("radio-none-focus-next").await?;
    page.expect_focus("radio-none-a").await?;
    // Skips radios b and c (same group).
    page.click("radio-none-focus-next").await?;
    page.expect_focus("radio-none-btn-after").await?;
    Ok(())
}

/// focus_next with wrap and tabbable: no next tabbable (same-group radios are filtered), so it
/// wraps and finds the checked radio again.
async fn radio_group_wrap_next(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("radio-wrap-b").await?;
    page.expect_focus("radio-wrap-b").await?;
    page.click("radio-wrap-focus-next").await?;
    page.expect_focus_stays("radio-wrap-b").await?;
    Ok(())
}

/// focus_previous with wrap and tabbable wraps back to the checked radio.
async fn radio_group_wrap_prev(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("radio-wrap-b").await?;
    page.expect_focus("radio-wrap-b").await?;
    page.click("radio-wrap-focus-prev").await?;
    page.expect_focus_stays("radio-wrap-b").await?;
    Ok(())
}

/// Hidden elements (`display: none`, `hidden`, `visibility: hidden`) are skipped.
async fn hidden_elements_skipped(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("vis-item-1").await?;
    page.expect_focus("vis-item-1").await?;
    page.click("vis-focus-next").await?;
    page.expect_focus("vis-item-5").await?;
    page.click("vis-focus-prev").await?;
    page.expect_focus("vis-item-1").await?;
    Ok(())
}

/// Elements in an inert subtree are skipped.
async fn inert_elements_skipped(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("inert-item-1").await?;
    page.expect_focus("inert-item-1").await?;
    page.click("inert-focus-next").await?;
    page.expect_focus("inert-item-3").await?;
    page.click("inert-focus-prev").await?;
    page.expect_focus("inert-item-1").await?;
    Ok(())
}

/// focus_next from outside the scope focuses the scope's first element.
async fn focus_next_from_outside_scope(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("outside-external").await?;
    page.expect_focus("outside-external").await?;
    page.click("outside-focus-next").await?;
    page.expect_focus("outside-item-1").await?;
    Ok(())
}

/// focus_previous from outside the scope focuses the scope's last element.
async fn focus_previous_from_outside_scope(page: &FocusManagerPage<'_>) -> Result<(), Report> {
    page.goto().await?;
    page.click("outside-external").await?;
    page.expect_focus("outside-external").await?;
    page.click("outside-focus-prev").await?;
    page.expect_focus("outside-item-3").await?;
    Ok(())
}
