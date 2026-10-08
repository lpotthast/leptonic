// Upstream: react-aria/test/interactions/useFocusVisible.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

/// `use_focus_ring` and the `FocusRing` atom: focus is visible after keyboard focus, not after
/// pointer focus; `within`, modality switches, disabled, text inputs. Every case starts on a fresh
/// page.
pub struct FocusRingTests {}

#[async_trait]
impl BrowserTest<str> for FocusRingTests {
    fn name(&self) -> Cow<'_, str> {
        "focus_ring_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        cases!(
            basic_click_focus(&page),
            basic_tab_focus(&page),
            within_click_focus(&page),
            within_tab_focus(&page),
            modality_switch(&page),
            arrow_key_keyboard_modality(&page),
            disabled_focus_ring(&page),
            atom_text_input(&page),
        );
        Ok(())
    }
}

const PATH: &str = "/hooks/focus-ring";

/// Click `before`, then Tab: keyboard focus on the element after it.
async fn tab_after(page: &Page<'_>, before: &str) -> Result<(), Report> {
    page.element(before).await?.click().await?;
    page.send_keys(Key::Tab).await
}

/// Click focus: focused, but focus is not visible (pointer modality).
async fn basic_click_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-fr-target").await?;
    let is_focused = page.element("#test-fr-is-focused").await?;
    let is_focus_visible = page.element("#test-fr-is-focus-visible").await?;
    assert_that!(is_focused.inner_text().await?).is_equal_to("false");
    assert_that!(is_focus_visible.inner_text().await?).is_equal_to("false");

    target.click().await?;
    is_focused.wait_for_inner_text("true").await?;
    is_focus_visible.inner_text_stays("false").await?;
    assert_that!(target.attr("data-focus-visible").await?).is_none();

    page.element("#test-fr-elsewhere").await?.click().await?;
    is_focused.wait_for_inner_text("false").await?;
    Ok(())
}

/// Tab focus: focused, and focus is visible (keyboard modality).
async fn basic_tab_focus(page: &Page<'_>) -> Result<(), Report> {
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

/// `within: true`, click focus: the container is focused, its focus not visible.
async fn within_click_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let is_focused = page.element("#test-fr-within-is-focused").await?;
    let is_focus_visible = page.element("#test-fr-within-is-focus-visible").await?;
    assert_that!(is_focused.inner_text().await?).is_equal_to("false");
    assert_that!(is_focus_visible.inner_text().await?).is_equal_to("false");

    page.element("#test-fr-within-child-1")
        .await?
        .click()
        .await?;
    is_focused.wait_for_inner_text("true").await?;
    is_focus_visible.inner_text_stays("false").await?;

    page.element("#test-fr-within-elsewhere")
        .await?
        .click()
        .await?;
    is_focused.wait_for_inner_text("false").await?;
    Ok(())
}

/// `within: true`, Tab focus: the container is focused and its focus visible.
async fn within_tab_focus(page: &Page<'_>) -> Result<(), Report> {
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
async fn modality_switch(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-fr-target").await?;
    let is_focused = page.element("#test-fr-is-focused").await?;
    let is_focus_visible = page.element("#test-fr-is-focus-visible").await?;

    tab_after(page, "#test-fr-before").await?;
    is_focused.wait_for_inner_text("true").await?;
    is_focus_visible.wait_for_inner_text("true").await?;

    target.click().await?;
    is_focused.inner_text_stays("true").await?;
    is_focus_visible.wait_for_inner_text("false").await?;
    Ok(())
}

/// An arrow key after a click switches to keyboard modality: the focus ring appears.
async fn arrow_key_keyboard_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-fr-target").await?;
    let is_focused = page.element("#test-fr-is-focused").await?;
    let is_focus_visible = page.element("#test-fr-is-focus-visible").await?;

    target.click().await?;
    is_focused.wait_for_inner_text("true").await?;
    is_focus_visible.inner_text_stays("false").await?;

    page.send_keys(Key::Down).await?;
    is_focus_visible.wait_for_inner_text("true").await?;
    target
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    Ok(())
}

/// Disabled: neither a click nor Tab makes the element focused or its focus visible.
async fn disabled_focus_ring(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let is_focused = page.element("#test-fr-disabled-is-focused").await?;
    let is_focus_visible = page.element("#test-fr-disabled-is-focus-visible").await?;

    page.element("#test-fr-disabled-target")
        .await?
        .click()
        .await?;
    is_focused.inner_text_stays("false").await?;
    is_focus_visible.inner_text_stays("false").await?;

    tab_after(page, "#test-fr-before").await?;
    is_focused.inner_text_stays("false").await?;
    is_focus_visible.inner_text_stays("false").await?;
    Ok(())
}

/// The `FocusRing` atom: `data-focused` follows focus. Typing after a click makes focus visible,
/// but not on a text input (only Tab and Escape do there; upstream's "emits on modality change
/// (text input)").
async fn atom_text_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    for (selector, is_text_input) in [("#test-fr-atom", false), ("#test-fr-atom-text", true)] {
        let element = page.element(selector).await?;
        element.click().await?;
        element.wait_for_attr("data-focused", Some("true")).await?;
        assert_that!(element.attr("data-focus-visible").await?)
            .with_detail_message(selector)
            .is_none();

        page.send_keys("a").await?;
        if is_text_input {
            element.attr_stays("data-focus-visible", None).await?;
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
    page.focus_stays(&element).await?;
    Ok(())
}
