// Upstream: react-aria-components/test/NumberField.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

pub struct NumberFieldTests {}

#[async_trait]
impl BrowserTest<str> for NumberFieldTests {
    fn name(&self) -> Cow<'_, str> {
        "number_field_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/number-field").await?;

        stepper_buttons(&page).await?;
        click_steps_once_and_focuses_input(&page).await?;
        holding_spins_until_the_limit(&page).await?;
        keyboard(&page).await?;
        steppers_are_not_tab_stops(&page).await?;
        enter_commits_and_submits(&page).await?;

        Ok(())
    }
}

const VALUE: &str = "test-nf-value";
const INCREMENT: &str = "#test-page-hook-number-field button[aria-label=Increase]";
const DECREMENT: &str = "#test-page-hook-number-field button[aria-label=Decrease]";

async fn stepper_buttons(page: &Page<'_>) -> Result<(), Report> {
    let input_id = page
        .driver
        .find(By::Css("[data-testid=input]"))
        .await?
        .id()
        .await?;
    for selector in [INCREMENT, DECREMENT] {
        let button = page.css(selector).await?;
        assert_that!(button.attr("aria-controls").await?).is_equal_to(input_id.clone());
        assert_that!(button.attr("tabindex").await?).is_equal_to(Some("-1".to_owned()));
    }
    // "Increase" + the visible label "Quantity" (through `aria-labelledby`).
    let increment = page.css(INCREMENT).await?;
    let label_id = page
        .css("#test-page-hook-number-field label")
        .await?
        .attr("id")
        .await?;
    let labelled_by = increment.attr("aria-labelledby").await?.unwrap_or_default();
    assert_that!(labelled_by.split(' ').next_back().map(ToOwned::to_owned)).is_equal_to(label_id);
    Ok(())
}

/// A click steps exactly once (no auto-repeat for a short press), and moves focus to the input
/// when using a mouse.
async fn click_steps_once_and_focuses_input(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_text(VALUE, "1").await?;
    page.css(INCREMENT).await?.click().await?;
    page.wait_for_text(VALUE, "2").await?;
    tokio::time::sleep(Duration::from_millis(600)).await;
    assert_that!(page.read_text_of(VALUE).await?).is_equal_to("2".to_owned());

    let active = page.driver.active_element().await?;
    assert_that!(active.attr("data-testid").await?).is_equal_to(Some("input".to_owned()));

    page.css(DECREMENT).await?.click().await?;
    page.wait_for_text(VALUE, "1").await
}

/// Holding the increment button spins: one step, then repeated steps after a delay, until the
/// maximum is reached. There the button disables itself, which ends the press, so spinning stops
/// for good (react-spectrum #9813).
async fn holding_spins_until_the_limit(page: &Page<'_>) -> Result<(), Report> {
    let increment = page.css(INCREMENT).await?;
    page.driver
        .action_chain()
        .move_to_element_center(&increment)
        .click_and_hold()
        .perform()
        .await?;
    page.wait_for_text(VALUE, "5").await?;
    page.wait_for_selector(&format!(
        "{INCREMENT}[aria-disabled=true], {INCREMENT}[disabled]"
    ))
    .await?;
    page.driver.action_chain().release().perform().await?;

    // Back down by keyboard; the increment button works again afterwards.
    page.css(DECREMENT).await?.click().await?;
    page.wait_for_text(VALUE, "4").await
}

async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    let input = page.driver.find(By::Css("[data-testid=input]")).await?;
    input.click().await?;
    input.send_keys(Key::Home).await?;
    page.wait_for_text(VALUE, "0").await?;
    input.send_keys(Key::Up).await?;
    page.wait_for_text(VALUE, "1").await?;
    input.send_keys(Key::End).await?;
    page.wait_for_text(VALUE, "5").await?;
    input.send_keys(Key::Down).await?;
    page.wait_for_text(VALUE, "4").await
}

async fn steppers_are_not_tab_stops(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-nf-before").await?;
    page.press_tab().await?;
    let active = page.driver.active_element().await?;
    assert_that!(active.attr("data-testid").await?).is_equal_to(Some("input".to_owned()));
    page.press_tab().await?;
    assert_that!(page.active_element_id().await?).is_equal_to(Some("test-nf-after".to_owned()));
    Ok(())
}

/// Enter commits the typed text and, as of react-spectrum #10200, keeps its default action:
/// the surrounding form is submitted.
async fn enter_commits_and_submits(page: &Page<'_>) -> Result<(), Report> {
    let input = page.driver.find(By::Css("[data-testid=input]")).await?;
    input.click().await?;
    input.send_keys(Key::Control + "a").await?;
    input.send_keys("3").await?;
    input.send_keys(Key::Enter).await?;
    page.wait_for_text(VALUE, "3").await?;
    page.wait_for_text("test-nf-submits", "1").await
}
