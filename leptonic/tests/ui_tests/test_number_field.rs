// Upstream: react-aria-components/test/NumberField.test.js @ 99e6102368
//! Behavior of `use_number_field`: stepper buttons (wiring, single steps, spinning while held,
//! no tab stops), keyboard steps, and Enter committing the value and submitting the form.
use std::time::Duration;

use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, Platform};

const PATH: &str = "/hooks/number-field";

/// The fixture's mirror of the field's value.
const VALUE: &str = "#test-nf-value";
const INPUT: &str = "[data-testid=input]";
const INCREMENT: &str = "#test-page-hook-number-field button[aria-label=Increase]";
const DECREMENT: &str = "#test-page-hook-number-field button[aria-label=Decrease]";

/// The stepper buttons control the input, have `tabindex="-1"` and are labelled by their own label
/// followed by the field's visible label.
#[browser_test]
pub async fn stepper_buttons(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input_id = page.element(INPUT).await?.id().await?;
    for selector in [INCREMENT, DECREMENT] {
        let button = page.element(selector).await?;
        assert_that!(button)
            .attribute("aria-controls")
            .await
            .with_detail_message(selector)
            .is_equal_to(input_id.clone());
        assert_that!(button)
            .with_detail_message(selector)
            .has_attribute("tabindex")
            .await
            .is_equal_to("-1");
    }
    // "Increase" + the visible label "Quantity" (through `aria-labelledby`).
    let increment = page.element(INCREMENT).await?;
    let label_id = page
        .element("#test-page-hook-number-field label")
        .await?
        .id()
        .await?;
    assert_that!(increment)
        .attribute("aria-labelledby")
        .await
        .map_owned(Option::unwrap_or_default)
        .derive_owned(|labelled_by| labelled_by.split(' ').next_back())
        .is_equal_to(label_id.as_deref());
    assert_that!(increment)
        .accessible_name()
        .await
        .is_equal_to("Increase Quantity");
    Ok(())
}

/// On an iPhone the input has no role description (VoiceOver then announces the required state)
/// and the decimal keyboard of a field without negative values, also when the server rendered it
/// for a desktop (useNumberField: `aria-roledescription` "not on iOS", `inputMode`).
#[browser_test]
pub async fn iphone_input(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::IPhone).await?;
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    input.wait_for_attr("aria-roledescription", None).await?;
    assert_that!(input)
        .has_attribute("inputmode")
        .await
        .is_equal_to("decimal");
    Ok(())
}

/// A click steps exactly once (no auto-repeat for a short press), and moves focus to the input
/// when using a mouse.
#[browser_test]
pub async fn click_steps_once_and_focuses_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let value = page.element(VALUE).await?;
    value.wait_for_inner_text("1").await?;
    page.element(INCREMENT).await?.click().await?;
    value.wait_for_inner_text("2").await?;
    // Past the delay after which a held button starts spinning.
    page.settle().await?;
    assert_that!(|| value.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(600))
        .matches(eq("2"))
        .await;
    page.wait_for_focus(&page.element(INPUT).await?).await?;

    page.element(DECREMENT).await?.click().await?;
    value.wait_for_inner_text("1").await?;
    Ok(())
}

/// Holding the increment button steps repeatedly up to the maximum, where the button disables
/// itself and spinning stops for good ("stops spinning if the associated button is disabled").
#[browser_test]
pub async fn holding_spins_until_the_limit(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let value = page.element(VALUE).await?;
    let increment = page.element(INCREMENT).await?;
    let held = increment.press_and_hold().await?;
    value.wait_for_inner_text("5").await?;
    // A boolean attribute reads "true" while present.
    increment.wait_for_attr("disabled", Some("true")).await?;
    held.release().await?;

    // Back down with the decrement button; the increment button is enabled again.
    page.element(DECREMENT).await?.click().await?;
    value.wait_for_inner_text("4").await?;
    increment.wait_for_attr("disabled", None).await?;
    Ok(())
}

/// In the focused input, Home steps to the minimum, Up one step up, End to the maximum and Down one
/// step down.
#[browser_test]
pub async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let value = page.element(VALUE).await?;
    let input = page.element(INPUT).await?;
    input.click().await?;
    for (key, expected) in [
        (Key::Home, "0"),
        (Key::Up, "1"),
        (Key::End, "5"),
        (Key::Down, "4"),
    ] {
        input.type_keys(key).await?;
        value.wait_for_inner_text(expected).await?;
    }
    Ok(())
}

/// Tab moves from the element before the field to its input and on to the element after it, past
/// both stepper buttons.
#[browser_test]
pub async fn steppers_are_not_tab_stops(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-nf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(INPUT).await?).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-nf-after").await?)
        .await?;
    Ok(())
}

/// Enter commits the typed text and, as of react-spectrum #10200, keeps its default action:
/// the surrounding form is submitted.
#[browser_test]
pub async fn enter_commits_and_submits(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    input.click().await?;
    input
        .type_keys(page.primary_modifier().await? + "a")
        .await?;
    input.type_keys("3").await?;
    input.type_keys(Key::Enter).await?;
    page.element(VALUE).await?.wait_for_inner_text("3").await?;
    page.element("#test-nf-submits")
        .await?
        .wait_for_inner_text("1")
        .await?;
    Ok(())
}
