// Upstream: react-aria/test/interactions/Focusable.test.js @ 99e6102368
// Upstream: react-aria-components/test/Tooltip.test.js @ 99e6102368
//! The `Focusable` atom: its focus handling goes onto the child, which gets a `tabindex` unless it
//! has one; `Focusable` and `Pressable` children are custom tooltip triggers.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/atoms/focusable";

/// The `Focusable` child gets `tabindex="0"`, is reached by Tab and runs the focus handler
/// ("should apply focusable props to child element").
#[browser_test]
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

/// A disabled `Focusable` child gets no `tabindex`, and one excluded from the tab order gets
/// `tabindex="-1"` ("supports isDisabled", "supports excludeFromTabOrder").
#[browser_test]
pub async fn disabled_and_excluded(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disabled = page.element("#test-focusable-disabled").await?;
    assert_that!(disabled).attribute("tabindex").await.is_none();
    let excluded = page.element("#test-focusable-excluded").await?;
    excluded.wait_for_attr("tabindex", Some("-1")).await?;
    Ok(())
}

/// Tab from the first `Focusable` skips the disabled, the excluded and the `tabindex="-1"` one and
/// focuses a custom `Focusable` tooltip trigger, which shows its tooltip ("should support custom
/// Focusable trigger on focus").
#[browser_test]
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
    assert_that!(trigger)
        .attribute("aria-describedby")
        .await
        .is_equal_to(tooltip_id);
    Ok(())
}

/// The child keeps its own `tabindex`, and focusing it runs both its own and the `Focusable`'s
/// focus handler ("should should merge with existing props").
#[browser_test]
pub async fn merged_props(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let merged = page.element("#test-focusable-merged").await?;
    assert_that!(merged)
        .has_attribute("tabindex")
        .await
        .is_equal_to("-1");
    merged.focus().await?;
    page.element("#test-focusable-log")
        .await?
        .wait_for_inner_text("own focus, merged focus")
        .await?;
    Ok(())
}

/// Hovering a custom `Pressable` tooltip trigger shows its tooltip, which describes the trigger
/// ("should support custom Pressable trigger").
#[browser_test]
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
    assert_that!(pressable)
        .attribute("aria-describedby")
        .await
        .is_equal_to(tooltip_id);
    Ok(())
}

/// A `Focusable` with `auto_focus` takes the focus when it mounts ("supports autoFocus").
#[browser_test]
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

/// A trigger's tooltip follows its `is_disabled`: a `Focusable` created disabled shows no tooltip
/// on hover, shows it once enabled, and none again once disabled.
#[browser_test]
pub async fn tooltip_follows_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-focusable-toggled-trigger").await?;
    let toggle = page.element("#test-focusable-toggle-disabled").await?;
    let park = page.element("#test-focusable-log").await?;

    trigger.hover().await?;
    page.settle().await?;
    assert_that!(|| page.count("[role=tooltip]"))
        .consistently_ok()
        // Past the tooltip's delay (100 ms).
        .for_at_least(std::time::Duration::from_millis(300))
        .matches(eq(0))
        .await;

    toggle.click().await?;
    toggle.wait_for_inner_text("Disable trigger").await?;
    trigger.hover().await?;
    page.element("[role=tooltip]")
        .await?
        .wait_for_inner_text("Toggled tooltip")
        .await?;

    park.hover().await?;
    page.wait_for_count("[role=tooltip]", 0).await?;
    toggle.click().await?;
    toggle.wait_for_inner_text("Enable trigger").await?;
    trigger.hover().await?;
    page.settle().await?;
    assert_that!(|| page.count("[role=tooltip]"))
        .consistently_ok()
        // Past the tooltip's delay (100 ms).
        .for_at_least(std::time::Duration::from_millis(300))
        .matches(eq(0))
        .await;
    Ok(())
}
