// Upstream: react-aria-components/test/NumberField.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions},
    polling::expect,
};

/// Behavior of `use_number_field`: stepper buttons (wiring, single steps, spinning while held,
/// no tab stops), keyboard steps, and Enter committing the value and submitting the form.
pub struct NumberFieldTests {}

#[async_trait]
impl BrowserTest<str> for NumberFieldTests {
    fn name(&self) -> Cow<'_, str> {
        "number_field_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/number-field").await?;

        cases!(
            stepper_buttons(&page),
            click_steps_once_and_focuses_input(&page),
            holding_spins_until_the_limit(&page),
            keyboard(&page),
            steppers_are_not_tab_stops(&page),
            enter_commits_and_submits(&page),
        );

        Ok(())
    }
}

/// The fixture's mirror of the field's value.
const VALUE: &str = "#test-nf-value";
const INPUT: &str = "[data-testid=input]";
const INCREMENT: &str = "#test-page-hook-number-field button[aria-label=Increase]";
const DECREMENT: &str = "#test-page-hook-number-field button[aria-label=Decrease]";

async fn stepper_buttons(page: &Page<'_>) -> Result<(), Report> {
    let input_id = page.element(INPUT).await?.id().await?;
    for selector in [INCREMENT, DECREMENT] {
        let button = page.element(selector).await?;
        assert_that!(button.attr("aria-controls").await?)
            .with_detail_message(selector)
            .is_equal_to(input_id.clone());
        assert_that!(button.attr("tabindex").await?)
            .with_detail_message(selector)
            .get_some()
            .is_equal_to("-1");
    }
    // "Increase" + the visible label "Quantity" (through `aria-labelledby`).
    let increment = page.element(INCREMENT).await?;
    let label_id = page
        .element("#test-page-hook-number-field label")
        .await?
        .id()
        .await?;
    let labelled_by = increment.attr("aria-labelledby").await?.unwrap_or_default();
    assert_that!(labelled_by.split(' ').next_back()).is_equal_to(label_id.as_deref());
    Ok(())
}

/// A click steps exactly once (no auto-repeat for a short press), and moves focus to the input
/// when using a mouse.
async fn click_steps_once_and_focuses_input(page: &Page<'_>) -> Result<(), Report> {
    let value = page.element(VALUE).await?;
    value.wait_for_inner_text("1").await?;
    page.element(INCREMENT).await?.click().await?;
    value.wait_for_inner_text("2").await?;
    // Past the delay after which a held button starts spinning.
    expect("the value")
        .observing(|| value.inner_text())
        .for_at_least(Duration::from_millis(600))
        .to_stay_equal_to("2")
        .await?;
    page.wait_for_focus(&page.element(INPUT).await?).await?;

    page.element(DECREMENT).await?.click().await?;
    value.wait_for_inner_text("1").await?;
    Ok(())
}

/// Holding the increment button spins: one step, then repeated steps after a delay, until the
/// maximum is reached. There the button disables itself, which ends the press, so spinning stops
/// for good (react-spectrum #9813).
async fn holding_spins_until_the_limit(page: &Page<'_>) -> Result<(), Report> {
    let value = page.element(VALUE).await?;
    let increment = page.element(INCREMENT).await?;
    page.driver
        .action_chain()
        .move_to_element_center(&increment)
        .click_and_hold()
        .perform()
        .await?;
    value.wait_for_inner_text("5").await?;
    // A boolean attribute reads "true" while present.
    increment.wait_for_attr("disabled", Some("true")).await?;
    page.driver.action_chain().release().perform().await?;

    // Back down with the decrement button; the increment button is enabled again.
    page.element(DECREMENT).await?.click().await?;
    value.wait_for_inner_text("4").await?;
    increment.wait_for_attr("disabled", None).await?;
    Ok(())
}

async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    let value = page.element(VALUE).await?;
    let input = page.element(INPUT).await?;
    input.click().await?;
    for (key, expected) in [
        (Key::Home, "0"),
        (Key::Up, "1"),
        (Key::End, "5"),
        (Key::Down, "4"),
    ] {
        input.send_keys(key).await?;
        value.wait_for_inner_text(expected).await?;
    }
    Ok(())
}

async fn steppers_are_not_tab_stops(page: &Page<'_>) -> Result<(), Report> {
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
async fn enter_commits_and_submits(page: &Page<'_>) -> Result<(), Report> {
    let input = page.element(INPUT).await?;
    input.click().await?;
    input.send_keys(Key::Control + "a").await?;
    input.send_keys("3").await?;
    input.send_keys(Key::Enter).await?;
    page.element(VALUE).await?.wait_for_inner_text("3").await?;
    page.element("#test-nf-submits")
        .await?
        .wait_for_inner_text("1")
        .await?;
    Ok(())
}
