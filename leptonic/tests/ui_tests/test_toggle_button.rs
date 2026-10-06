// Upstream: react-aria-components/test/ToggleButton.test.js @ 99e6102368
// Upstream: react-aria-components/test/ToggleButtonGroup.test.js @ 99e6102368
// Upstream: react-aria/test/toolbar/useToolbar.test.tsx @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Behavior of the toggle button hooks (through the `ToggleButton` and `ToggleButtonGroup`
/// atoms): `aria-pressed`, press and keyboard toggling, disabled buttons; groups with single
/// selection (`radiogroup` of `radio`s) and multiple selection (`toolbar`), the toolbar's arrow
/// key navigation in both orientations, Tab leaving it and re-entering at the last focused
/// button, and disabled groups. Spec: react-aria-components `ToggleButton.test.js`,
/// `ToggleButtonGroup.test.js`, react-aria `useToolbar.test.tsx`.
pub struct ToggleButtonTests {}

#[async_trait]
impl BrowserTest<str> for ToggleButtonTests {
    fn name(&self) -> Cow<'_, str> {
        "toggle_button_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/toggle-button").await?;

        toggle_button(&page).await?;
        disabled_toggle_button(&page).await?;
        single_selection(&page).await?;
        multiple_selection(&page).await?;
        horizontal_navigation(&page).await?;
        tab_leaves_and_restores(&page).await?;
        vertical_navigation(&page).await?;
        disabled_group(&page).await?;

        Ok(())
    }
}

async fn button(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    Ok(page
        .driver
        .find(By::XPath(format!("//button[normalize-space(.)='{text}']")))
        .await?)
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn toggle_button(page: &Page<'_>) -> Result<(), Report> {
    let toggle = button(page, "Toggle").await?;
    assert_that!(attr(&toggle, "aria-pressed").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(attr(&toggle, "data-selected").await?).is_none();
    toggle.click().await?;
    page.wait_for_attr(&toggle, "aria-pressed", Some("true"))
        .await?;
    page.wait_for_attr(&toggle, "data-selected", Some("true"))
        .await?;
    page.wait_for_text("test-tb-value", "true").await?;

    page.click_element_with_id("test-tb-before").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&toggle, "the toggle button").await?;
    page.wait_for_attr(&toggle, "data-focus-visible", Some("true"))
        .await?;
    page.send_keys_to_active(Key::Space).await?;
    page.wait_for_attr(&toggle, "aria-pressed", Some("false"))
        .await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_attr(&toggle, "aria-pressed", Some("true"))
        .await?;
    page.wait_for_text("test-tb-value", "true").await
}

async fn disabled_toggle_button(page: &Page<'_>) -> Result<(), Report> {
    let toggle = button(page, "Disabled toggle").await?;
    assert_that!(attr(&toggle, "disabled").await?).is_some();
    assert_that!(attr(&toggle, "data-disabled").await?).is_equal_to(Some("true".to_owned()));
    Ok(())
}

/// Single selection: a `radiogroup` of `radio`s with `aria-checked`; selecting one deselects
/// the other, pressing the selected one deselects it.
async fn single_selection(page: &Page<'_>) -> Result<(), Report> {
    let group = page.css("[aria-label='Single']").await?;
    assert_that!(attr(&group, "role").await?).is_equal_to(Some("radiogroup".to_owned()));
    let a = button(page, "Single A").await?;
    let b = button(page, "Single B").await?;
    assert_that!(attr(&a, "role").await?).is_equal_to(Some("radio".to_owned()));
    assert_that!(attr(&a, "aria-checked").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(attr(&a, "aria-pressed").await?).is_none();

    a.click().await?;
    page.wait_for_attr(&a, "aria-checked", Some("true")).await?;
    page.wait_for_text("test-tb-single-value", "a").await?;
    b.click().await?;
    page.wait_for_attr(&b, "aria-checked", Some("true")).await?;
    page.wait_for_attr(&a, "aria-checked", Some("false"))
        .await?;
    page.wait_for_text("test-tb-single-value", "b").await?;
    b.click().await?;
    page.wait_for_attr(&b, "aria-checked", Some("false"))
        .await?;
    page.wait_for_text("test-tb-single-value", "").await
}

/// Multiple selection: a `toolbar` of buttons with `aria-pressed`.
async fn multiple_selection(page: &Page<'_>) -> Result<(), Report> {
    let group = page.css("[aria-label='Multiple']").await?;
    assert_that!(attr(&group, "role").await?).is_equal_to(Some("toolbar".to_owned()));
    assert_that!(attr(&group, "aria-orientation").await?)
        .is_equal_to(Some("horizontal".to_owned()));
    let a = button(page, "Multiple A").await?;
    let b = button(page, "Multiple B").await?;
    assert_that!(attr(&a, "role").await?).is_none();
    a.click().await?;
    b.click().await?;
    page.wait_for_text("test-tb-multiple-value", "a,b").await?;
    assert_that!(attr(&a, "aria-pressed").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&b, "aria-pressed").await?).is_equal_to(Some("true".to_owned()));
    a.click().await?;
    page.wait_for_text("test-tb-multiple-value", "b").await
}

/// The arrow keys move focus within the toolbar (without wrapping).
async fn horizontal_navigation(page: &Page<'_>) -> Result<(), Report> {
    let a = button(page, "Multiple A").await?;
    let b = button(page, "Multiple B").await?;
    let c = button(page, "Multiple C").await?;
    a.click().await?;
    page.wait_for_focus_on(&a, "Multiple A").await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_focus_on(&b, "Multiple B").await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_focus_on(&c, "Multiple C").await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_focus_on(&c, "Multiple C (no wrapping)")
        .await?;
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_focus_on(&b, "Multiple B").await?;
    // Vertical keys do nothing in a horizontal toolbar.
    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_focus_on(&b, "Multiple B").await
}

/// Tab leaves the toolbar from wherever focus is; Shift+Tab back into a toolbar restores the
/// button focused last.
async fn tab_leaves_and_restores(page: &Page<'_>) -> Result<(), Report> {
    // Focus is on Multiple B (previous step).
    let b = button(page, "Multiple B").await?;
    page.press_tab().await?;
    let next = button(page, "Vertical A").await?;
    page.wait_for_focus_on(&next, "the first button after the toolbar")
        .await?;
    page.press_shift_tab().await?;
    page.wait_for_focus_on(&b, "Multiple B (restored)").await?;
    // The single selection group before it is a toolbar too: entering it restores its button
    // focused last (Single B, which focus left for Multiple A).
    page.press_shift_tab().await?;
    let previous = button(page, "Single B").await?;
    page.wait_for_focus_on(&previous, "Single B (restored)")
        .await
}

async fn vertical_navigation(page: &Page<'_>) -> Result<(), Report> {
    let group = page.css("[aria-label='Vertical']").await?;
    assert_that!(attr(&group, "aria-orientation").await?).is_equal_to(Some("vertical".to_owned()));
    let a = button(page, "Vertical A").await?;
    let b = button(page, "Vertical B").await?;
    a.click().await?;
    page.wait_for_focus_on(&a, "Vertical A").await?;
    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_focus_on(&b, "Vertical B").await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_focus_on(&b, "Vertical B (Right does nothing)")
        .await?;
    page.send_keys_to_active(Key::Up).await?;
    page.wait_for_focus_on(&a, "Vertical A").await
}

async fn disabled_group(page: &Page<'_>) -> Result<(), Report> {
    let group = page.css("[aria-label='Disabled']").await?;
    assert_that!(attr(&group, "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&group, "data-disabled").await?).is_equal_to(Some("true".to_owned()));
    let a = button(page, "Disabled A").await?;
    assert_that!(attr(&a, "disabled").await?).is_some();
    assert_that!(attr(&a, "data-disabled").await?).is_equal_to(Some("true".to_owned()));
    Ok(())
}
