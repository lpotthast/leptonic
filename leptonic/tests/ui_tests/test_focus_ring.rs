// Upstream: react-aria/test/interactions/useFocusVisible.test.js @ 99e6102368
//! `use_focus_ring` and the `FocusRing` atom: focus is visible after keyboard focus, not after
//! pointer focus; `within`, modality switches, disabled, text inputs. Every case starts on a fresh
//! page.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/hooks/focus-ring";

/// Click `before`, then Tab: keyboard focus on the element after it.
async fn tab_after(page: &Page<'_>, before: &str) -> Result<(), Report> {
    page.element(before).await?.click().await?;
    page.send_keys(Key::Tab).await
}

/// Clicking the element focuses it without making its focus visible (pointer modality), and
/// clicking elsewhere blurs it.
#[browser_test]
pub async fn basic_click_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-fr-target").await?;
    let is_focused = page.element("#test-fr-is-focused").await?;
    let is_focus_visible = page.element("#test-fr-is-focus-visible").await?;
    assert_that!(is_focused)
        .inner_text()
        .await
        .is_equal_to("false");
    assert_that!(is_focus_visible)
        .inner_text()
        .await
        .is_equal_to("false");

    target.click().await?;
    is_focused.wait_for_inner_text("true").await?;
    is_focus_visible
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(target)
        .attribute("data-focus-visible")
        .await
        .is_none();

    page.element("#test-fr-elsewhere").await?.click().await?;
    is_focused.wait_for_inner_text("false").await?;
    Ok(())
}

/// Tabbing to the element focuses it and makes its focus visible (`data-focus-visible`), and
/// clicking elsewhere clears both.
#[browser_test]
pub async fn basic_tab_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-fr-target").await?;
    let is_focused = page.element("#test-fr-is-focused").await?;
    let is_focus_visible = page.element("#test-fr-is-focus-visible").await?;

    tab_after(page, "#test-fr-before").await?;
    page.wait_for_focus(&target).await?;
    is_focused.wait_for_inner_text("true").await?;
    is_focus_visible.wait_for_inner_text("true").await?;
    target
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;

    page.element("#test-fr-elsewhere").await?.click().await?;
    is_focused.wait_for_inner_text("false").await?;
    target.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// With `within`, clicking a child makes the container focused without visible focus, and clicking
/// elsewhere blurs it.
#[browser_test]
pub async fn within_click_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let is_focused = page.element("#test-fr-within-is-focused").await?;
    let is_focus_visible = page.element("#test-fr-within-is-focus-visible").await?;
    assert_that!(is_focused)
        .inner_text()
        .await
        .is_equal_to("false");
    assert_that!(is_focus_visible)
        .inner_text()
        .await
        .is_equal_to("false");

    page.element("#test-fr-within-child-1")
        .await?
        .click()
        .await?;
    is_focused.wait_for_inner_text("true").await?;
    is_focus_visible
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;

    page.element("#test-fr-within-elsewhere")
        .await?
        .click()
        .await?;
    is_focused.wait_for_inner_text("false").await?;
    Ok(())
}

/// With `within`, tabbing to a child makes the container focused with visible focus, and clicking
/// elsewhere clears both.
#[browser_test]
pub async fn within_tab_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let is_focused = page.element("#test-fr-within-is-focused").await?;
    let is_focus_visible = page.element("#test-fr-within-is-focus-visible").await?;

    tab_after(page, "#test-fr-within-before").await?;
    is_focused.wait_for_inner_text("true").await?;
    is_focus_visible.wait_for_inner_text("true").await?;

    page.element("#test-fr-within-elsewhere")
        .await?
        .click()
        .await?;
    is_focused.wait_for_inner_text("false").await?;
    is_focus_visible.wait_for_inner_text("false").await?;
    Ok(())
}

/// Tab focus shows the focus ring; clicking the same element switches to pointer modality and
/// hides it.
#[browser_test]
pub async fn modality_switch(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-fr-target").await?;
    let is_focused = page.element("#test-fr-is-focused").await?;
    let is_focus_visible = page.element("#test-fr-is-focus-visible").await?;

    tab_after(page, "#test-fr-before").await?;
    is_focused.wait_for_inner_text("true").await?;
    is_focus_visible.wait_for_inner_text("true").await?;

    target.click().await?;
    is_focused
        .inner_text_stays("true", std::time::Duration::from_millis(100))
        .await?;
    is_focus_visible.wait_for_inner_text("false").await?;
    Ok(())
}

/// Pressing an arrow key on a clicked element switches to keyboard modality and makes its focus
/// visible.
#[browser_test]
pub async fn arrow_key_keyboard_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-fr-target").await?;
    let is_focused = page.element("#test-fr-is-focused").await?;
    let is_focus_visible = page.element("#test-fr-is-focus-visible").await?;

    target.click().await?;
    is_focused.wait_for_inner_text("true").await?;
    is_focus_visible
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;

    page.send_keys(Key::Down).await?;
    is_focus_visible.wait_for_inner_text("true").await?;
    target
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    Ok(())
}

/// A disabled element becomes neither focused nor focus visible, by a click or by Tab.
#[browser_test]
pub async fn disabled_focus_ring(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let is_focused = page.element("#test-fr-disabled-is-focused").await?;
    let is_focus_visible = page.element("#test-fr-disabled-is-focus-visible").await?;

    page.element("#test-fr-disabled-target")
        .await?
        .click()
        .await?;
    is_focused
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;
    is_focus_visible
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;

    tab_after(page, "#test-fr-before").await?;
    is_focused
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;
    is_focus_visible
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Typing makes the focus of a clicked `FocusRing` atom visible (on a text input only Escape does),
/// and disabling a focused atom clears its `data-focused` ("emits on modality change (non-text
/// input)", "emits on modality change (text input)").
#[browser_test]
pub async fn atom_text_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    for (selector, is_text_input) in [("#test-fr-atom", false), ("#test-fr-atom-text", true)] {
        let element = page.element(selector).await?;
        element.click().await?;
        element.wait_for_attr("data-focused", Some("true")).await?;
        element
            .attr_stays(
                "data-focus-visible",
                None,
                std::time::Duration::from_millis(100),
            )
            .await?;

        page.send_keys("a").await?;
        if is_text_input {
            element
                .attr_stays(
                    "data-focus-visible",
                    None,
                    std::time::Duration::from_millis(100),
                )
                .await?;
            page.send_keys(Key::Escape).await?;
        }
        element
            .wait_for_attr("data-focus-visible", Some("true"))
            .await?;
    }
    page.element("#test-fr-elsewhere").await?.click().await?;
    page.element("#test-fr-atom-text")
        .await?
        .wait_for_attr("data-focused", None)
        .await?;

    // Disabled while focused: no longer focused.
    let element = page.element("#test-fr-atom-disable").await?;
    element.click().await?;
    element.wait_for_attr("data-focused", Some("true")).await?;
    // A virtual click keeps focus on the element.
    page.element("#test-fr-atom-disable-toggle")
        .await?
        .virtual_click()
        .await?;
    element.wait_for_attr("data-focused", None).await?;
    page.focus_stays(&element, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}
