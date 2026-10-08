// Upstream: react-aria/test/focus/FocusScope.test.js @ 99e6102368
//! The focus manager of `FocusScope`: next, previous, first and last, with wrapping, tabbable
//! filtering, an accept filter, radio groups, hidden and inert elements, and from outside the
//! scope.
use rootcause::Report;

use crate::pages::{Page, PageActions, focus_manager::FocusManagerActions};

const PATH: &str = "/hooks/focus-manager";

/// Basic navigation: focus_first, focus_next, focus_previous, focus_last.
pub async fn basic_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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
pub async fn wrap_next(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click("wrap-item-3").await?;
    page.expect_focus("wrap-item-3").await?;
    page.click("wrap-focus-next").await?;
    page.expect_focus("wrap-item-1").await?;
    Ok(())
}

/// wrap: true — focus_previous at the first element wraps to the last.
pub async fn wrap_prev(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click("wrap-item-1").await?;
    page.expect_focus("wrap-item-1").await?;
    page.click("wrap-focus-prev").await?;
    page.expect_focus("wrap-item-3").await?;
    Ok(())
}

/// wrap: false — focus_next at the last element stays put.
pub async fn nowrap_boundary_next(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click("wrap-item-3").await?;
    page.expect_focus("wrap-item-3").await?;
    page.click("nowrap-focus-next").await?;
    page.expect_focus_stays("wrap-item-3").await?;
    Ok(())
}

/// wrap: false — focus_previous at the first element stays put.
pub async fn nowrap_boundary_prev(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click("wrap-item-1").await?;
    page.expect_focus("wrap-item-1").await?;
    page.click("nowrap-focus-prev").await?;
    page.expect_focus_stays("wrap-item-1").await?;
    Ok(())
}

/// tabbable: true — skips the item with tabindex=-1.
pub async fn tabbable_skip(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click("tabbable-item-1").await?;
    page.expect_focus("tabbable-item-1").await?;
    page.click("tabbable-focus-next").await?;
    page.expect_focus("tabbable-item-3").await?;
    page.click("tabbable-focus-prev").await?;
    page.expect_focus("tabbable-item-1").await?;
    Ok(())
}

/// tabbable: false — includes items with tabindex=-1.
pub async fn nontabbable_include(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click("tabbable-item-1").await?;
    page.expect_focus("tabbable-item-1").await?;
    page.click("nontabbable-focus-next").await?;
    page.expect_focus("tabbable-item-2").await?;
    Ok(())
}

/// An accept filter rejects item 2 during navigation.
pub async fn accept_filter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click("accept-item-1").await?;
    page.expect_focus("accept-item-1").await?;
    page.click("accept-focus-next").await?;
    page.expect_focus("accept-item-3").await?;
    page.click("accept-focus-prev").await?;
    page.expect_focus("accept-item-1").await?;
    Ok(())
}

/// A radio group with a checked radio: tabbable navigation stops only at the checked radio.
pub async fn radio_group_checked(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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
pub async fn radio_group_none_checked(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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
pub async fn radio_group_wrap_next(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click("radio-wrap-b").await?;
    page.expect_focus("radio-wrap-b").await?;
    page.click("radio-wrap-focus-next").await?;
    page.expect_focus_stays("radio-wrap-b").await?;
    Ok(())
}

/// focus_previous with wrap and tabbable wraps back to the checked radio.
pub async fn radio_group_wrap_prev(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click("radio-wrap-b").await?;
    page.expect_focus("radio-wrap-b").await?;
    page.click("radio-wrap-focus-prev").await?;
    page.expect_focus_stays("radio-wrap-b").await?;
    Ok(())
}

/// Hidden elements (`display: none`, `hidden`, `visibility: hidden`) are skipped.
pub async fn hidden_elements_skipped(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click("vis-item-1").await?;
    page.expect_focus("vis-item-1").await?;
    page.click("vis-focus-next").await?;
    page.expect_focus("vis-item-5").await?;
    page.click("vis-focus-prev").await?;
    page.expect_focus("vis-item-1").await?;
    Ok(())
}

/// Elements in an inert subtree are skipped.
pub async fn inert_elements_skipped(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click("inert-item-1").await?;
    page.expect_focus("inert-item-1").await?;
    page.click("inert-focus-next").await?;
    page.expect_focus("inert-item-3").await?;
    page.click("inert-focus-prev").await?;
    page.expect_focus("inert-item-1").await?;
    Ok(())
}

/// focus_next from outside the scope focuses the scope's first element.
pub async fn focus_next_from_outside_scope(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click("outside-external").await?;
    page.expect_focus("outside-external").await?;
    page.click("outside-focus-next").await?;
    page.expect_focus("outside-item-1").await?;
    Ok(())
}

/// focus_previous from outside the scope focuses the scope's last element.
pub async fn focus_previous_from_outside_scope(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click("outside-external").await?;
    page.expect_focus("outside-external").await?;
    page.click("outside-focus-prev").await?;
    page.expect_focus("outside-item-3").await?;
    Ok(())
}
