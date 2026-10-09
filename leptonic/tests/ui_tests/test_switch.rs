// Upstream: react-aria-components/test/Switch.test.js @ 99e6102368
//! Behavior of the switch hooks (through the `Switch` atom): a native `role="switch"` checkbox
//! inside a label, toggled by press and Space, focus ring, disabled and read-only states, and a
//! switch bound to a signal. Spec: react-aria-components `Switch.test.js`.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, KeyKind, Page, SyntheticEvent, css};

const PATH: &str = "/atoms/switch";

/// The `<label>` of the switch with the visible text `text`.
async fn label(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(css("label").text(text)).await
}

/// The fixture's mirror of the basic switch's state.
async fn value(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-sw-value").await
}

/// The native checkbox inside `label`.
async fn input(label: &WebElement) -> Result<WebElement, Report> {
    label.element("input").await
}

/// The switch is a native checkbox with `role=switch`; pressing its label turns it on
/// (`data-selected`, checked input) and off again ("should support selected state").
#[browser_test]
pub async fn selected_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Basic").await?;
    let input = input(&label).await?;
    assert_that!(input)
        .has_attribute("role")
        .await
        .is_equal_to("switch");
    assert_that!(input)
        .has_attribute("type")
        .await
        .is_equal_to("checkbox");
    label.click().await?;
    label.wait_for_attr("data-selected", Some("true")).await?;
    value(page).await?.wait_for_inner_text("true").await?;
    assert_that!(input).selected().await.is_true();
    label.click().await?;
    label.wait_for_attr("data-selected", None).await?;
    value(page).await?.wait_for_inner_text("false").await?;
    Ok(())
}

/// Tab focuses the switch's input (focus visible), and Space toggles it on and off ("should support
/// focus ring", "should support press state with keyboard").
#[browser_test]
pub async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Basic").await?;
    let input = input(&label).await?;
    page.element("#test-sw-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&input).await?;
    label
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.send_keys(Key::Space).await?;
    value(page).await?.wait_for_inner_text("true").await?;
    page.send_keys(Key::Space).await?;
    value(page).await?.wait_for_inner_text("false").await?;
    Ok(())
}

/// Hovering the switch's label sets `data-hovered`, and leaving it clears it ("should support
/// hover").
#[browser_test]
pub async fn hover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Basic").await?;
    assert_that!(label)
        .attribute("data-hovered")
        .await
        .is_none();
    label.hover().await?;
    label.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("h1").await?.hover().await?;
    label.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Holding the pointer down on the label sets `data-pressed` until it is released ("should support
/// press state").
#[browser_test]
pub async fn press_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Basic").await?;
    assert_that!(label)
        .attribute("data-pressed")
        .await
        .is_none();
    let held = label.press_and_hold().await?;
    label.wait_for_attr("data-pressed", Some("true")).await?;
    held.release().await?;
    label.wait_for_attr("data-pressed", None).await?;
    value(page).await?.wait_for_inner_text("true").await?;
    Ok(())
}

/// Holding Space on the focused switch sets `data-pressed` until the key is released ("should
/// support press state with keyboard").
#[browser_test]
pub async fn press_state_with_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Basic").await?;
    let input = input(&label).await?;
    input.focus().await?;
    input
        .dispatch(SyntheticEvent::keyboard(KeyKind::Down, " "))
        .await?;
    label.wait_for_attr("data-pressed", Some("true")).await?;
    input
        .dispatch(SyntheticEvent::keyboard(KeyKind::Up, " "))
        .await?;
    label.wait_for_attr("data-pressed", None).await?;
    Ok(())
}

/// A virtual click on the label toggles the switch on and off, as with a native label.
#[browser_test]
pub async fn virtual_label_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Basic").await?;
    label.virtual_click().await?;
    value(page).await?.wait_for_inner_text("true").await?;
    label.virtual_click().await?;
    value(page).await?.wait_for_inner_text("false").await?;
    Ok(())
}

/// A disabled switch has `data-disabled` and a disabled input, and pressing its label doesn't turn
/// it on ("should support disabled state").
#[browser_test]
pub async fn disabled_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Disabled").await?;
    assert_that!(label)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    assert_that!(input(&label).await?)
        .enabled()
        .await
        .is_false();
    label.click().await?;
    label
        .attr_stays("data-selected", None, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A read-only switch has `data-readonly` and `aria-readonly`, and pressing its label doesn't turn
/// it off ("should support read only state").
#[browser_test]
pub async fn read_only_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Read only").await?;
    let input = input(&label).await?;
    assert_that!(label)
        .has_attribute("data-readonly")
        .await
        .is_equal_to("true");
    assert_that!(input)
        .has_attribute("aria-readonly")
        .await
        .is_equal_to("true");
    label.click().await?;
    label
        .attr_stays(
            "data-selected",
            Some("true"),
            std::time::Duration::from_millis(100),
        )
        .await?;
    assert_that!(input).selected().await.is_true();
    Ok(())
}

/// A switch bound to app state follows a change of the state, and pressing it writes the new value
/// back to the state.
#[browser_test]
pub async fn bound_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Bound").await?;
    page.element("#test-sw-bound-flip").await?.click().await?;
    label.wait_for_attr("data-selected", Some("true")).await?;
    label.click().await?;
    label.wait_for_attr("data-selected", None).await?;
    page.element("#test-sw-bound-value")
        .await?
        .wait_for_inner_text("false")
        .await?;
    Ok(())
}

/// A read-only switch bound to app state stays off when its label is pressed or Space is pressed
/// on its focused input.
#[browser_test]
pub async fn bound_read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Bound read only").await?;
    let input = input(&label).await?;
    label.click().await?;
    input.focus().await?;
    page.send_keys(Key::Space).await?;
    page.element("#test-sw-bound-read-only-value")
        .await?
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(input).selected().await.is_false();
    Ok(())
}
