// Upstream: react-aria/test/interactions/Focusable.test.js @ 99e6102368
// Upstream: react-aria-components/test/Tooltip.test.js @ 99e6102368
//! The `Focusable` atom: its focus handling goes onto the child, which gets a `tabindex` unless it
//! has one; `Focusable` and `Pressable` children are custom tooltip triggers.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

const PATH: &str = "/atoms/focusable";

/// "should apply focusable props to child element": focusable by Tab, focus handlers.
pub async fn focusable_child(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let focusable = page.element("#test-focusable").await?;
    focusable.wait_for_attr("tabindex", Some("0")).await?;
    page.element("#test-focusable-before")
        .await?
        .focus()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&focusable).await?;
    page.element("#test-focusable-log")
        .await?
        .wait_for_inner_text("focus")
        .await?;
    Ok(())
}

/// "supports isDisabled", "supports excludeFromTabOrder".
pub async fn disabled_and_excluded(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disabled = page.element("#test-focusable-disabled").await?;
    assert_that!(disabled.attr("tabindex").await?).is_none();
    let excluded = page.element("#test-focusable-excluded").await?;
    excluded.wait_for_attr("tabindex", Some("-1")).await?;
    Ok(())
}

/// Tab from the first `Focusable` skips the disabled, the excluded and the merged one
/// (`tabindex="-1"` of its own) onto the trigger: "should support custom Focusable trigger on
/// focus".
pub async fn focusable_tooltip_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-focusable-trigger").await?;
    let focusable = page.element("#test-focusable").await?;
    focusable.focus().await?;
    page.wait_for_focus(&focusable).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&trigger).await?;
    // Visible once it faded in.
    let tooltip = page.element("[role=tooltip]").await?;
    tooltip.wait_for_inner_text("Focusable tooltip").await?;
    let tooltip_id = tooltip.id().await?;
    assert_that!(trigger.attr("aria-describedby").await?).is_equal_to(tooltip_id);
    Ok(())
}

/// "should should merge with existing props": the child's own `tabindex` stays, both focus
/// handlers run.
pub async fn merged_props(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let merged = page.element("#test-focusable-merged").await?;
    assert_that!(merged.attr("tabindex").await?)
        .get_some()
        .is_equal_to("-1");
    merged.focus().await?;
    page.element("#test-focusable-log")
        .await?
        .wait_for_inner_text("own focus, merged focus")
        .await?;
    Ok(())
}

/// "should support custom Pressable trigger": on hover.
pub async fn pressable_tooltip_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-focusable-log").await?.hover().await?;
    page.element("#test-focusable-before")
        .await?
        .click()
        .await?;
    page.wait_for_count("[role=tooltip]", 0).await?;
    let pressable = page.element("#test-pressable-trigger").await?;
    pressable.hover().await?;
    let tooltip = page.element("[role=tooltip]").await?;
    tooltip.wait_for_inner_text("Pressable tooltip").await?;
    let tooltip_id = tooltip.id().await?;
    assert_that!(pressable.attr("aria-describedby").await?).is_equal_to(tooltip_id);
    Ok(())
}

/// "supports autoFocus".
pub async fn auto_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-focusable-mount-auto")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&page.element("#test-focusable-auto").await?)
        .await?;
    Ok(())
}
