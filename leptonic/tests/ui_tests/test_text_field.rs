// Upstream: react-aria-components/test/TextField.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Behavior of `use_text_field`: labelling, description and validation wiring, the value
/// staying in sync with the hook-owned state in both directions, and form reset.
pub struct TextFieldTests {}

#[async_trait]
impl BrowserTest<str> for TextFieldTests {
    fn name(&self) -> Cow<'_, str> {
        "text_field_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/text-field").await?;

        labelling(&page).await?;
        typing_updates_the_state(&page).await?;
        validation(&page).await?;
        programmatic_changes_update_the_input(&page).await?;
        form_reset_restores_the_default(&page).await?;

        Ok(())
    }
}

async fn input(page: &Page<'_>) -> Result<WebElement, Report> {
    page.css("#test-tf-form input").await
}

async fn input_value(page: &Page<'_>) -> Result<String, Report> {
    Ok(input(page).await?.prop("value").await?.unwrap_or_default())
}

/// The label's `for` points to the input, which is labelled by the label and described by the
/// description.
async fn labelling(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page).await?;
    let id = input.attr("id").await?.unwrap_or_default();
    let label = page.driver.find(By::Css("#test-tf-form label")).await?;
    assert_that!(label.attr("for").await?).is_equal_to(Some(id));
    let label_id = label.attr("id").await?.unwrap_or_default();
    assert_that!(input.attr("aria-labelledby").await?).is_equal_to(Some(label_id));

    let description = page
        .driver
        .find(By::XPath("//div[text()='Your first name.']"))
        .await?;
    let description_id = description.attr("id").await?.unwrap_or_default();
    assert_that!(input.attr("aria-describedby").await?).is_equal_to(Some(description_id));
    assert_that!(input_value(page).await?).is_equal_to("Ada".to_owned());
    Ok(())
}

async fn typing_updates_the_state(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page).await?;
    input.click().await?;
    input.send_keys(Key::End + "line").await?;
    page.wait_for_text("test-tf-value", "Adaline").await
}

/// An invalid value marks the input `aria-invalid` and describes it with the error message
/// (aria validation behavior: errors show while typing).
async fn validation(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page).await?;
    for _ in 0..5 {
        input.send_keys(Key::Backspace).await?;
    }
    page.wait_for_text("test-tf-value", "Ad").await?;
    page.wait_for_selector("#test-tf-form [aria-invalid=true]")
        .await?;
    let error = page
        .driver
        .find(By::XPath("//div[text()='At least 3 characters.']"))
        .await?;
    let error_id = error.attr("id").await?.unwrap_or_default();
    let described_by = input.attr("aria-describedby").await?.unwrap_or_default();
    assert_that!(described_by.split(' ').any(|id| id == error_id)).is_true();

    input.send_keys("a").await?;
    page.wait_for_text("test-tf-value", "Ada").await?;
    page.wait_for_no_selector("#test-tf-form [aria-invalid=true]")
        .await
}

/// Changing the state from outside updates what the input shows (the DOM property, not just
/// the attribute).
async fn programmatic_changes_update_the_input(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-tf-clear").await?;
    page.wait_for_text("test-tf-value", "").await?;
    assert_that!(input_value(page).await?).is_equal_to(String::new());
    Ok(())
}

async fn form_reset_restores_the_default(page: &Page<'_>) -> Result<(), Report> {
    input(page).await?.send_keys("Grace").await?;
    page.wait_for_text("test-tf-value", "Grace").await?;
    page.click_element_with_id("test-tf-reset").await?;
    page.wait_for_text("test-tf-value", "Ada").await?;
    assert_that!(input_value(page).await?).is_equal_to("Ada".to_owned());
    Ok(())
}
