// Upstream: react-aria/test/interactions/useFocus.test.js @ 99e6102368
//! `use_focus`: focus and blur of the element itself (not its children), disabled, and a blur
//! when the focused element becomes disabled. Every case starts on a fresh page.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

const PATH: &str = "/hooks/focus";

/// "handles focus events on the immediate target", "does not handle focus events if disabled".
pub async fn basic_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-focus-target").await?;
    let elsewhere = page.element("#test-focus-elsewhere").await?;
    let disabled = page.element("#test-focus-disabled").await?;
    let focus_count = page.element("#test-focus-count").await?;
    let blur_count = page.element("#test-blur-count").await?;
    let is_focused = page.element("#test-is-focused").await?;
    assert_that!(focus_count.inner_text().await?.parse::<u32>()?).is_equal_to(0);
    assert_that!(blur_count.inner_text().await?.parse::<u32>()?).is_equal_to(0);
    assert_that!(is_focused.inner_text().await?.parse::<bool>()?).is_false();

    target.click().await?;
    focus_count.wait_for_inner_text("1").await?;
    is_focused.wait_for_inner_text("true").await?;

    elsewhere.click().await?;
    blur_count.wait_for_inner_text("1").await?;
    is_focused.wait_for_inner_text("false").await?;

    target.click().await?;
    focus_count.wait_for_inner_text("2").await?;
    is_focused.wait_for_inner_text("true").await?;

    disabled.click().await?;
    page.wait_for_focus(&disabled).await?;
    page.element("#test-disabled-focus-count")
        .await?
        .inner_text_stays("0")
        .await?;
    Ok(())
}

/// The Tab key focuses the target.
pub async fn tab_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-focus-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-focus-target").await?)
        .await?;
    page.element("#test-focus-count")
        .await?
        .wait_for_inner_text("1")
        .await?;
    page.element("#test-is-focused")
        .await?
        .wait_for_inner_text("true")
        .await?;
    Ok(())
}

/// "does not handle focus events on children".
pub async fn child_focus_does_not_trigger_parent(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let child = page.element("#test-focus-child").await?;
    let parent = page.element("#test-focus-parent").await?;
    let parent_focus_count = page.element("#test-focus-parent-focus-count").await?;
    let parent_blur_count = page.element("#test-focus-parent-blur-count").await?;

    child.click().await?;
    page.wait_for_focus(&child).await?;
    parent_focus_count.inner_text_stays("0").await?;

    parent.click().await?;
    parent_focus_count.wait_for_inner_text("1").await?;

    // Focus moving to the child blurs the parent, but focusing the child isn't the parent's focus.
    child.click().await?;
    parent_blur_count.wait_for_inner_text("1").await?;
    parent_focus_count.inner_text_stays("1").await?;
    Ok(())
}

/// `on_focus_change` tracks focus/blur transitions.
pub async fn focus_change_count(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-focus-target").await?;
    let elsewhere = page.element("#test-focus-elsewhere").await?;
    let changes = page.element("#test-focus-change-count").await?;
    assert_that!(changes.inner_text().await?.parse::<u32>()?).is_equal_to(0);

    target.click().await?;
    changes.wait_for_inner_text("1").await?;

    elsewhere.click().await?;
    changes.wait_for_inner_text("2").await?;

    target.click().await?;
    changes.wait_for_inner_text("3").await?;
    Ok(())
}

/// "should fire onBlur when a focused element is disabled" (Firefox fires no blur then; the
/// synthetic blur observer dispatches one), exactly once.
pub async fn blur_when_disabled_while_focused(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disable_me = page.element("#test-focus-disable-me").await?;
    let blur_count = page.element("#test-focus-disable-me-blur-count").await?;

    disable_me.click().await?;
    disable_me.wait_for_attr("disabled", Some("true")).await?;
    blur_count.wait_for_inner_text("1").await?;
    blur_count.inner_text_stays("1").await?;
    Ok(())
}
