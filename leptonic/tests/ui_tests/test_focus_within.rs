// Upstream: react-aria/test/interactions/useFocusWithin.test.js @ 99e6102368
//! `use_focus_within`: focus entering and leaving an element tree, also when the focused element
//! is removed or disabled. Every case starts on a fresh page.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/hooks/focus-within";

/// Clicking a child starts focus within, moving between children changes nothing, and clicking
/// outside ends it ("does handle focus events on children").
#[browser_test]
pub async fn basic_focus_within(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input_a = page.element("#test-fw-input-a").await?;
    let input_b = page.element("#test-fw-input-b").await?;
    let outside = page.element("#test-fw-outside").await?;
    let is_focus_within = page.element("#test-fw-is-focus-within").await?;
    let focus_count = page.element("#test-fw-focus-within-count").await?;
    let blur_count = page.element("#test-fw-blur-within-count").await?;
    assert_that!(is_focus_within)
        .inner_text()
        .await
        .map_owned(|value| value.parse::<bool>())
        .get_ok()
        .is_false();
    assert_that!(focus_count)
        .inner_text()
        .await
        .map_owned(|value| value.parse::<u32>())
        .get_ok()
        .is_equal_to(0);
    assert_that!(blur_count)
        .inner_text()
        .await
        .map_owned(|value| value.parse::<u32>())
        .get_ok()
        .is_equal_to(0);

    input_a.click().await?;
    is_focus_within.wait_for_inner_text("true").await?;
    focus_count.wait_for_inner_text("1").await?;

    // Moving within: no new focus within, no blur within.
    input_b.click().await?;
    page.wait_for_focus(&input_b).await?;
    focus_count
        .inner_text_stays("1", std::time::Duration::from_millis(100))
        .await?;
    blur_count
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;
    is_focus_within
        .inner_text_stays("true", std::time::Duration::from_millis(100))
        .await?;

    outside.click().await?;
    is_focus_within.wait_for_inner_text("false").await?;
    blur_count.wait_for_inner_text("1").await?;

    input_a.click().await?;
    is_focus_within.wait_for_inner_text("true").await?;
    focus_count.wait_for_inner_text("2").await?;
    Ok(())
}

/// With `disabled`, focusing a child neither starts focus within nor calls the focus handler ("does
/// not handle focus events if disabled").
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#test-fw-disabled-input").await?;
    let outside = page.element("#test-fw-outside").await?;
    let is_focus_within = page.element("#test-fw-disabled-is-focus-within").await?;
    let focus_count = page.element("#test-fw-disabled-focus-count").await?;

    for _ in 0..2 {
        input.click().await?;
        page.wait_for_focus(&input).await?;
        is_focus_within
            .inner_text_stays("false", std::time::Duration::from_millis(100))
            .await?;
        focus_count
            .inner_text_stays("0", std::time::Duration::from_millis(100))
            .await?;
        outside.click().await?;
    }
    Ok(())
}

/// `on_focus_within_change` is called with `true` whenever focus enters the container and with
/// `false` whenever it leaves.
#[browser_test]
pub async fn change_callback(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#test-fw-change-input").await?;
    let outside = page.element("#test-fw-outside").await?;
    let value = page.element("#test-fw-change-value").await?;
    let count = page.element("#test-fw-change-count").await?;
    assert_that!(value)
        .inner_text()
        .await
        .map_owned(|value| value.parse::<bool>())
        .get_ok()
        .is_false();
    assert_that!(count)
        .inner_text()
        .await
        .map_owned(|value| value.parse::<u32>())
        .get_ok()
        .is_equal_to(0);

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

/// Tabbing into the container starts focus within, tabbing between its children changes nothing,
/// and tabbing out ends it.
#[browser_test]
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
    focus_count
        .inner_text_stays("1", std::time::Duration::from_millis(100))
        .await?;
    is_focus_within
        .inner_text_stays("true", std::time::Duration::from_millis(100))
        .await?;

    page.send_keys(Key::Tab).await?;
    is_focus_within.wait_for_inner_text("false").await?;
    blur_count.wait_for_inner_text("1").await?;
    Ok(())
}

/// Focusing an input in two nested containers starts focus within on both, and clicking outside
/// ends it on both ("events bubble by default").
#[browser_test]
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

/// When a child kept its blur from reaching the container, the next focus outside (which stops its
/// `focusin` too) still ends focus within with a blur ("should fire onBlur when focus occurs
/// outside").
#[browser_test]
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

/// Removing the focused child ends focus within, with a blur for the removed element.
#[browser_test]
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

/// Disabling the focused child ends focus within with exactly one blur ("should fire onBlur when a
/// focused element is disabled").
#[browser_test]
pub async fn disabling_the_focused_element(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disable = page.element("#test-fw-removal-disable").await?;
    let events = page.element("#test-fw-removal-events").await?;
    let expected = "focus,change:true,blur:test-fw-removal-disable,change:false";

    disable.click().await?;
    disable.wait_for_attr("disabled", Some("true")).await?;
    events.wait_for_inner_text(expected).await?;
    // Exactly one blur (native `focusout` and the observer's don't both count).
    events
        .inner_text_stays(expected, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}
