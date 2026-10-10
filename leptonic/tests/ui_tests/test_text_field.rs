// Upstream: react-aria-components/test/TextField.test.js @ 99e6102368
//! Behavior of `use_text_field`: labelling, description and validation wiring, the value
//! staying in sync with the hook-owned state in both directions, and form reset.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/hooks/text-field";

/// The text field's input.
async fn input(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-tf-form input").await
}

/// The fixture's mirror of the hook-owned value.
async fn state_value(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-tf-value").await
}

/// The label's `for` points to the input, which is labelled by the label and described by the
/// description.
#[browser_test]
pub async fn labelling(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page).await?;
    let label = page.element("#test-tf-form label").await?;
    let input_id = input.id().await?;
    assert_that!(label)
        .attribute("for")
        .await
        .is_equal_to(input_id);
    assert_that!(input)
        .accessible_name()
        .await
        .is_equal_to("Name");
    assert_that!(input)
        .accessible_description()
        .await
        .is_equal_to("Your first name.");
    assert_that!(input)
        .property("value")
        .await
        .some()
        .is_equal_to("Ada");
    Ok(())
}

/// Typing into the input updates the hook-owned state.
#[browser_test]
pub async fn typing_updates_the_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page).await?;
    input.click().await?;
    input.type_keys(Key::End + "line").await?;
    state_value(page)
        .await?
        .wait_for_inner_text("Adaline")
        .await?;
    Ok(())
}

/// An invalid value marks the input `aria-invalid` and describes it with the error message
/// (aria validation behavior: errors show while typing).
#[browser_test]
pub async fn validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page).await?;
    input.click().await?;
    input.type_keys(Key::End + Key::Backspace).await?;
    state_value(page).await?.wait_for_inner_text("Ad").await?;
    input.wait_for_attr("aria-invalid", Some("true")).await?;
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Your first name. At least 3 characters."))
        .await;

    input.type_keys("a").await?;
    state_value(page).await?.wait_for_inner_text("Ada").await?;
    input.wait_for_attr("aria-invalid", None).await?;
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Your first name."))
        .await;
    Ok(())
}

/// Changing the state from outside updates what the input shows (the DOM property, not just
/// the attribute).
#[browser_test]
pub async fn programmatic_changes_update_the_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-tf-clear").await?.click().await?;
    state_value(page).await?.wait_for_inner_text("").await?;
    input(page).await?.wait_for_prop("value", "").await?;
    Ok(())
}

/// Resetting the form restores the default value, in the state and in what the input shows ("resets
/// to defaultValue when submitting form action" for a native form reset).
#[browser_test]
pub async fn form_reset_restores_the_default(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page).await?;
    input.click().await?;
    input.type_keys(Key::End + "Grace").await?;
    state_value(page)
        .await?
        .wait_for_inner_text("AdaGrace")
        .await?;
    page.element("#test-tf-reset").await?.click().await?;
    state_value(page).await?.wait_for_inner_text("Ada").await?;
    input.wait_for_prop("value", "Ada").await?;
    Ok(())
}
