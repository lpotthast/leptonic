// Upstream: react-aria-components/test/ToggleButton.test.js @ 99e6102368
// Upstream: react-aria-components/test/ToggleButtonGroup.test.js @ 99e6102368
// Upstream: react-aria-components/test/Toolbar.test.tsx @ 99e6102368
//! Behavior of the toggle button hooks (through the `ToggleButton` and `ToggleButtonGroup`
//! atoms): `aria-pressed`, press and keyboard toggling, disabled buttons; groups with single
//! selection (`radiogroup` of `radio`s) and multiple selection (`toolbar`), the toolbar's arrow
//! key navigation in both orientations, Tab leaving it and re-entering at the last focused
//! button, and disabled groups. Spec: react-aria-components `ToggleButton.test.js`,
//! `ToggleButtonGroup.test.js`, react-aria-components `Toolbar.test.tsx`.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, role};

const PATH: &str = "/atoms/toggle-button";

/// Pressing a toggle button selects it (`aria-pressed`, `data-selected`), and once tabbed to (with
/// a focus ring) Space and Enter toggle it ("should support selected state", "should support focus
/// ring").
#[browser_test]
pub async fn toggle_button(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page.element(role(AriaRole::Button).text("Toggle")).await?;
    assert_that!(toggle)
        .has_attribute("aria-pressed")
        .await
        .is_equal_to("false");
    assert_that!(toggle)
        .attribute("data-selected")
        .await
        .is_none();
    toggle.click().await?;
    toggle.wait_for_attr("aria-pressed", Some("true")).await?;
    toggle.wait_for_attr("data-selected", Some("true")).await?;
    page.element("#test-tb-value")
        .await?
        .wait_for_inner_text("true")
        .await?;

    page.element("#test-tb-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&toggle).await?;
    toggle
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.send_keys(Key::Space).await?;
    toggle.wait_for_attr("aria-pressed", Some("false")).await?;
    page.send_keys(Key::Enter).await?;
    toggle.wait_for_attr("aria-pressed", Some("true")).await?;
    page.element("#test-tb-value")
        .await?
        .wait_for_inner_text("true")
        .await?;
    Ok(())
}

/// A disabled toggle button is a disabled button with `data-disabled` ("should support disabled
/// state").
#[browser_test]
pub async fn disabled_toggle_button(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page
        .element(role(AriaRole::Button).text("Disabled toggle"))
        .await?;
    assert_that!(toggle).enabled().await.is_false();
    assert_that!(toggle)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    Ok(())
}

/// A single-selection group is a radio group in which selecting a button deselects the other and
/// pressing the selected one deselects it ("should support uncontrolled single selection").
#[browser_test]
pub async fn single_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element("[aria-label='Single']").await?;
    assert_that!(group)
        .has_attribute("role")
        .await
        .is_equal_to("radiogroup");
    let a = page.element(role(AriaRole::Radio).text("Single A")).await?;
    let b = page.element(role(AriaRole::Radio).text("Single B")).await?;
    assert_that!(a)
        .has_attribute("role")
        .await
        .is_equal_to("radio");
    assert_that!(a)
        .has_attribute("aria-checked")
        .await
        .is_equal_to("false");
    assert_that!(a).attribute("aria-pressed").await.is_none();

    a.click().await?;
    a.wait_for_attr("aria-checked", Some("true")).await?;
    page.element("#test-tb-single-value")
        .await?
        .wait_for_inner_text("a")
        .await?;
    b.click().await?;
    b.wait_for_attr("aria-checked", Some("true")).await?;
    a.wait_for_attr("aria-checked", Some("false")).await?;
    page.element("#test-tb-single-value")
        .await?
        .wait_for_inner_text("b")
        .await?;
    b.click().await?;
    b.wait_for_attr("aria-checked", Some("false")).await?;
    page.element("#test-tb-single-value")
        .await?
        .wait_for_inner_text("")
        .await?;
    Ok(())
}

/// A multiple-selection group is a horizontal toolbar whose buttons toggle independently
/// ("should support uncontrolled multiple selection").
#[browser_test]
pub async fn multiple_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element("[aria-label='Multiple']").await?;
    assert_that!(group)
        .has_attribute("role")
        .await
        .is_equal_to("toolbar");
    assert_that!(group)
        .has_attribute("aria-orientation")
        .await
        .is_equal_to("horizontal");
    let a = page
        .element(role(AriaRole::Button).text("Multiple A"))
        .await?;
    let b = page
        .element(role(AriaRole::Button).text("Multiple B"))
        .await?;
    assert_that!(a).attribute("role").await.is_none();
    a.click().await?;
    b.click().await?;
    page.element("#test-tb-multiple-value")
        .await?
        .wait_for_inner_text("a,b")
        .await?;
    assert_that!(a)
        .has_attribute("aria-pressed")
        .await
        .is_equal_to("true");
    assert_that!(b)
        .has_attribute("aria-pressed")
        .await
        .is_equal_to("true");
    a.click().await?;
    page.element("#test-tb-multiple-value")
        .await?
        .wait_for_inner_text("b")
        .await?;
    Ok(())
}

/// ArrowRight and ArrowLeft move focus within the toolbar without wrapping, and ArrowDown does
/// nothing ("should support horizontal keyboard navigation").
#[browser_test]
pub async fn horizontal_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let a = page
        .element(role(AriaRole::Button).text("Multiple A"))
        .await?;
    let b = page
        .element(role(AriaRole::Button).text("Multiple B"))
        .await?;
    let c = page
        .element(role(AriaRole::Button).text("Multiple C"))
        .await?;
    a.click().await?;
    page.wait_for_focus(&a).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&b).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&c).await?;
    // No wrapping.
    page.send_keys(Key::Right).await?;
    page.focus_stays(&c, std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&b).await?;
    // Vertical keys do nothing in a horizontal toolbar.
    page.send_keys(Key::Down).await?;
    page.focus_stays(&b, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Tab leaves the toolbar from wherever focus is; Shift+Tab back into a toolbar restores the
/// button focused last.
#[browser_test]
pub async fn tab_leaves_and_restores(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    // Single B, then Multiple B gets the focus: the button focused last in each group.
    let previous = page.element(role(AriaRole::Radio).text("Single B")).await?;
    previous.click().await?;
    page.wait_for_focus(&previous).await?;
    let b = page
        .element(role(AriaRole::Button).text("Multiple B"))
        .await?;
    b.click().await?;
    page.wait_for_focus(&b).await?;
    page.send_keys(Key::Tab).await?;
    let next = page
        .element(role(AriaRole::Radio).text("Vertical A"))
        .await?;
    page.wait_for_focus(&next).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&b).await?;
    // The single selection group before it is a toolbar too: entering it restores its button
    // focused last (Single B, which focus left for Multiple B).
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&previous).await?;
    Ok(())
}

/// In a vertical group, ArrowDown and ArrowUp move focus between its buttons and ArrowRight does
/// nothing ("should support vertical keyboard navigation").
#[browser_test]
pub async fn vertical_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element("[aria-label='Vertical']").await?;
    assert_that!(group)
        .has_attribute("aria-orientation")
        .await
        .is_equal_to("vertical");
    let a = page
        .element(role(AriaRole::Radio).text("Vertical A"))
        .await?;
    let b = page
        .element(role(AriaRole::Radio).text("Vertical B"))
        .await?;
    a.click().await?;
    page.wait_for_focus(&a).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&b).await?;
    // Right does nothing in a vertical toolbar.
    page.send_keys(Key::Right).await?;
    page.focus_stays(&b, std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Up).await?;
    page.wait_for_focus(&a).await?;
    Ok(())
}

/// A disabled group has `aria-disabled` and `data-disabled`, and its buttons are disabled with
/// `data-disabled` (ToggleButtonGroup's "should support disabled state").
#[browser_test]
pub async fn disabled_group(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element("[aria-label='Disabled']").await?;
    assert_that!(group)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    assert_that!(group)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    for text in ["Disabled A", "Disabled B", "Disabled C"] {
        let button = page.element(role(AriaRole::Radio).text(text)).await?;
        assert_that!(button)
            .enabled()
            .await
            .with_detail_message(text)
            .is_false();
        assert_that!(button)
            .with_detail_message(text)
            .has_attribute("data-disabled")
            .await
            .is_equal_to("true");
    }
    Ok(())
}
