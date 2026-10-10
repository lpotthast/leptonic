// Upstream: react-aria/test/interactions/Focusable.test.js @ 99e6102368
//! `use_focusable`: tab index for disabled and excluded elements, auto focus, keyboard events and
//! the focus handle ("supports isDisabled", "supports excludeFromTabOrder", "supports autoFocus").
//! Every case starts on a fresh page.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/hooks/focusable";
const NORMAL: &str = "#test-fcbl-normal";

/// A normal element has tab index 0, a disabled one none and an excluded one -1, and the auto focus
/// element is focused on load ("supports isDisabled", "supports excludeFromTabOrder", "supports
/// autoFocus").
#[browser_test]
pub async fn tabindex_attributes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let normal = page.element(NORMAL).await?;
    let disabled = page.element("#test-fcbl-disabled").await?;
    let excluded = page.element("#test-fcbl-excluded").await?;
    assert_that!(normal)
        .has_attribute("tabindex")
        .await
        .is_equal_to("0");
    assert_that!(disabled).attribute("tabindex").await.is_none();
    assert_that!(excluded)
        .has_attribute("tabindex")
        .await
        .is_equal_to("-1");

    page.wait_for_focus(&page.element("#test-fcbl-autofocus").await?)
        .await?;
    Ok(())
}

/// Typing a key on the focused element calls its key down and key up handlers once each.
#[browser_test]
pub async fn keyboard_events(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let normal = page.element(NORMAL).await?;
    let keydown_count = page.element("#test-fcbl-keydown-count").await?;
    let keyup_count = page.element("#test-fcbl-keyup-count").await?;

    normal.click().await?;
    page.wait_for_focus(&normal).await?;
    assert_that!(keydown_count)
        .inner_text()
        .await
        .map_owned(|value| value.parse::<u32>())
        .ok()
        .is_equal_to(0);
    assert_that!(keyup_count)
        .inner_text()
        .await
        .map_owned(|value| value.parse::<u32>())
        .ok()
        .is_equal_to(0);

    page.send_keys("a").await?;
    keydown_count.wait_for_inner_text("1").await?;
    keyup_count.wait_for_inner_text("1").await?;
    Ok(())
}

/// Tab from the normal element skips the disabled element (no tab index) and the excluded one (tab
/// index -1).
#[browser_test]
pub async fn tab_skip(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let normal = page.element(NORMAL).await?;
    normal.click().await?;
    page.wait_for_focus(&normal).await?;

    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-fcbl-tab-target").await?)
        .await?;
    Ok(())
}

/// Focusing the element programmatically through its `FocusHandle` (from a button) focuses it.
#[browser_test]
pub async fn focus_handle(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-fcbl-focus-btn").await?.click().await?;
    page.wait_for_focus(&page.element(NORMAL).await?).await?;
    Ok(())
}

/// Disabling the element at runtime removes its tab index, and enabling it again restores tab
/// index 0.
#[browser_test]
pub async fn dynamic_disabled_transition(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dynamic = page.element("#test-fcbl-dynamic").await?;
    let toggle = page.element("#test-fcbl-dynamic-toggle").await?;
    assert_that!(dynamic)
        .has_attribute("tabindex")
        .await
        .is_equal_to("0");

    toggle.click().await?;
    dynamic.wait_for_attr("tabindex", None).await?;

    toggle.click().await?;
    dynamic.wait_for_attr("tabindex", Some("0")).await?;
    Ok(())
}
