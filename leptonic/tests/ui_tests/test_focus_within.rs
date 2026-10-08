// Upstream: react-aria/test/interactions/useFocusWithin.test.js @ 99e6102368
//! `use_focus_within`: focus entering and leaving an element tree, also when the focused element
//! is removed or disabled. Every case starts on a fresh page.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

const PATH: &str = "/hooks/focus-within";

/// Basic focus within behavior: enter, move within, leave, re-enter ("does handle focus events on
/// children").
pub async fn basic_focus_within(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input_a = page.element("#test-fw-input-a").await?;
    let input_b = page.element("#test-fw-input-b").await?;
    let outside = page.element("#test-fw-outside").await?;
    let is_focus_within = page.element("#test-fw-is-focus-within").await?;
    let focus_count = page.element("#test-fw-focus-within-count").await?;
    let blur_count = page.element("#test-fw-blur-within-count").await?;
    assert_that!(is_focus_within.inner_text().await?.parse::<bool>()?).is_false();
    assert_that!(focus_count.inner_text().await?.parse::<u32>()?).is_equal_to(0);
    assert_that!(blur_count.inner_text().await?.parse::<u32>()?).is_equal_to(0);

    input_a.click().await?;
    is_focus_within.wait_for_inner_text("true").await?;
    focus_count.wait_for_inner_text("1").await?;

    // Moving within: no new focus within, no blur within.
    input_b.click().await?;
    page.wait_for_focus(&input_b).await?;
    focus_count.inner_text_stays("1").await?;
    blur_count.inner_text_stays("0").await?;
    is_focus_within.inner_text_stays("true").await?;

    outside.click().await?;
    is_focus_within.wait_for_inner_text("false").await?;
    blur_count.wait_for_inner_text("1").await?;

    input_a.click().await?;
    is_focus_within.wait_for_inner_text("true").await?;
    focus_count.wait_for_inner_text("2").await?;
    Ok(())
}

/// "does not handle focus events if disabled".
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#test-fw-disabled-input").await?;
    let outside = page.element("#test-fw-outside").await?;
    let is_focus_within = page.element("#test-fw-disabled-is-focus-within").await?;
    let focus_count = page.element("#test-fw-disabled-focus-count").await?;

    for _ in 0..2 {
        input.click().await?;
        page.wait_for_focus(&input).await?;
        is_focus_within.inner_text_stays("false").await?;
        focus_count.inner_text_stays("0").await?;
        outside.click().await?;
    }
    Ok(())
}

/// `on_focus_within_change` fires true on focus enter, false on focus leave.
pub async fn change_callback(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#test-fw-change-input").await?;
    let outside = page.element("#test-fw-outside").await?;
    let value = page.element("#test-fw-change-value").await?;
    let count = page.element("#test-fw-change-count").await?;
    assert_that!(value.inner_text().await?.parse::<bool>()?).is_false();
    assert_that!(count.inner_text().await?.parse::<u32>()?).is_equal_to(0);

    input.click().await?;
    value.wait_for_inner_text("true").await?;
    count.wait_for_inner_text("1").await?;

    outside.click().await?;
    value.wait_for_inner_text("false").await?;
    count.wait_for_inner_text("2").await?;

    input.click().await?;
    value.wait_for_inner_text("true").await?;
    count.wait_for_inner_text("3").await?;
    Ok(())
}

/// Tab navigation into and out of the container.
pub async fn tab_into_and_out_of_container(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input_a = page.element("#test-fw-input-a").await?;
    let input_b = page.element("#test-fw-input-b").await?;
    let is_focus_within = page.element("#test-fw-is-focus-within").await?;
    let focus_count = page.element("#test-fw-focus-within-count").await?;
    let blur_count = page.element("#test-fw-blur-within-count").await?;
    page.element("#test-fw-before").await?.click().await?;

    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&input_a).await?;
    is_focus_within.wait_for_inner_text("true").await?;
    focus_count.wait_for_inner_text("1").await?;

    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&input_b).await?;
    focus_count.inner_text_stays("1").await?;
    is_focus_within.inner_text_stays("true").await?;

    page.send_keys(Key::Tab).await?;
    is_focus_within.wait_for_inner_text("false").await?;
    blur_count.wait_for_inner_text("1").await?;
    Ok(())
}

/// Focusing a deeply nested input sets focus within on both containers ("events bubble by
/// default": focus within doesn't stop the focus events).
pub async fn nested_focus_within(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let outer = page
        .element("#test-fw-nested-outer-is-focus-within")
        .await?;
    let inner = page
        .element("#test-fw-nested-inner-is-focus-within")
        .await?;

    page.element("#test-fw-nested-input").await?.click().await?;
    outer.wait_for_inner_text("true").await?;
    inner.wait_for_inner_text("true").await?;

    page.element("#test-fw-outside").await?.click().await?;
    outer.wait_for_inner_text("false").await?;
    inner.wait_for_inner_text("false").await?;
    Ok(())
}

/// "should fire onBlur when focus occurs outside": no blur reached the container (a child stopped
/// its `focusout`), so the next focus outside ends focus within, with a blur on the container. The
/// outside input stops its `focusin` too, so this needs the capture-phase `focus` listener.
pub async fn focus_outside_after_a_hidden_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let events = page.element("#test-fw-removal-events").await?;
    let outer = page.element("#test-fw-removal-outer").await?;

    page.element("#test-fw-removal-quiet")
        .await?
        .click()
        .await?;
    events.wait_for_inner_text("focus,change:true").await?;

    outer.click().await?;
    page.wait_for_focus(&outer).await?;
    events
        .wait_for_inner_text("focus,change:true,blur:test-fw-removal-container,change:false")
        .await?;
    Ok(())
}

/// Removing the focused child ends focus within (Chrome fires a blur for the removed element).
pub async fn removal_of_the_focused_child(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;

    page.element("#test-fw-removal-hide").await?.click().await?;
    page.wait_for_count("#test-fw-removal-hide", 0).await?;
    page.element("#test-fw-removal-events")
        .await?
        .wait_for_inner_text("focus,change:true,blur:test-fw-removal-hide,change:false")
        .await?;
    Ok(())
}

/// "should fire onBlur when a focused element is disabled" (Firefox fires no blur then; the
/// synthetic blur observer dispatches one).
pub async fn disabling_the_focused_element(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disable = page.element("#test-fw-removal-disable").await?;
    let events = page.element("#test-fw-removal-events").await?;
    let expected = "focus,change:true,blur:test-fw-removal-disable,change:false";

    disable.click().await?;
    disable.wait_for_attr("disabled", Some("true")).await?;
    events.wait_for_inner_text(expected).await?;
    // Exactly one blur (native `focusout` and the observer's don't both count).
    events.inner_text_stays(expected).await?;
    Ok(())
}
