// Upstream: react-aria-components/test/SearchField.test.js @ 99e6102368
// Upstream: react-aria/test/searchfield/useSearchField.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use super::test_text_field_atoms::{
    check_validity, expect_focused, field_of, is_valid, referenced_texts, wait_for_referenced_texts,
};
use crate::pages::{BaseActions, Page};

/// The SearchField atom and `use_search_field`: slots, Enter/Escape, the clear button,
/// validation and states.
pub struct SearchFieldTests {}

#[async_trait]
impl BrowserTest<str> for SearchFieldTests {
    fn name(&self) -> Cow<'_, str> {
        "search_field_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/search-field").await?;

        provides_slots(&page).await?;
        enter_submits(&page).await?;
        escape_clears_once(&page).await?;
        clear_button_clears_and_focuses_the_input(&page).await?;
        enter_without_on_submit_submits_the_form(&page).await?;
        validation_errors(&page).await?;
        read_only(&page).await?;
        form_attribute(&page).await?;
        input_type(&page).await?;

        Ok(())
    }
}

/// "provides slots": a searchbox with value, label, description and error message, and a clear
/// button named "Clear search" that is not tabbable.
async fn provides_slots(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#sf-slots input").await?;
    assert_that!(input.attr("type").await?).is_equal_to(Some("search".to_owned()));
    assert_that!(input.prop("value").await?).is_equal_to(Some("test".to_owned()));
    assert_that!(field_of(page, "#sf-slots").await?.attr("data-foo").await?)
        .is_equal_to(Some("bar".to_owned()));
    assert_that!(referenced_texts(page, &input, "aria-labelledby").await?)
        .is_equal_to("Test".to_owned());
    wait_for_referenced_texts(page, &input, "aria-describedby", "Description Error").await?;

    let button = page.css("#sf-slots button").await?;
    assert_that!(button.attr("aria-label").await?).is_equal_to(Some("Clear search".to_owned()));
    assert_that!(button.attr("tabindex").await?).is_equal_to(Some("-1".to_owned()));
    Ok(())
}

/// "preventDefault and onSubmit are called for Enter if submit is provided".
async fn enter_submits(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#sf-keys input").await?;
    input.click().await?;
    input.send_keys("query").await?;
    input.send_keys(Key::Enter).await?;
    page.wait_for_text("sf-submitted", "query").await
}

/// "pressing the Escape key sets the state value to "", if state.value is not empty, and calls
/// onClear ... and will not call onClear if escape pressed again"; an unhandled Escape keeps
/// propagating ("preventDefault and stopPropagation are not called for Escape").
async fn escape_clears_once(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#sf-keys input").await?;
    page.wait_for_selector("#sf-keys > div:not([data-empty])")
        .await?;
    input.send_keys(Key::Escape).await?;
    page.wait_for_text("sf-clears", "1").await?;
    assert_that!(input.prop("value").await?).is_equal_to(Some(String::new()));
    page.wait_for_selector("#sf-keys > div[data-empty]").await?;
    assert_that!(page.read_text_of("sf-escapes-bubbled").await?).is_equal_to("0".to_owned());

    input.send_keys(Key::Escape).await?;
    page.wait_for_text("sf-escapes-bubbled", "1").await?;
    assert_that!(page.read_text_of("sf-clears").await?).is_equal_to("1".to_owned());
    Ok(())
}

/// "sets the state to "" and focuses the search field" / "calls the user provided onClear".
async fn clear_button_clears_and_focuses_the_input(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#sf-keys input").await?;
    input.send_keys("abc").await?;
    page.driver
        .find(browser_test::thirtyfour::By::Css("h1"))
        .await?
        .click()
        .await?;
    page.css("#sf-keys button").await?.click().await?;
    page.wait_for_text("sf-clears", "2").await?;
    assert_that!(input.prop("value").await?).is_equal_to(Some(String::new()));
    expect_focused(page, &input).await
}

/// "preventDefault is not called for Enter if onSubmit is not provided".
async fn enter_without_on_submit_submits_the_form(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#sf-form input").await?;
    input.click().await?;
    input.send_keys("q").await?;
    input.send_keys(Key::Enter).await?;
    page.wait_for_text("sf-form-submits", "1").await
}

/// "supports validation errors".
async fn validation_errors(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#sf-native input").await?;
    assert_that!(input.attr("required").await?).is_some();
    assert_that!(input.attr("aria-required").await?).is_none();
    assert_that!(input.attr("aria-describedby").await?).is_none();
    assert_that!(is_valid(page, &input).await?).is_false();

    check_validity(page, "sf-native").await?;
    page.wait_for_selector("#sf-native input[aria-describedby]")
        .await?;
    expect_focused(page, &input).await?;
    let field = field_of(page, "#sf-native").await?;
    assert_that!(field.attr("data-invalid").await?).is_some();
    assert_that!(field.attr("data-required").await?).is_some();

    page.send_keys_to_active("Devon").await?;
    assert_that!(input.attr("aria-describedby").await?).is_some();
    assert_that!(is_valid(page, &input).await?).is_true();
    page.press_tab().await?;
    page.wait_for_no_selector("#sf-native input[aria-describedby]")
        .await
}

/// "supports readonly"; the clear button is disabled while read-only.
async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(
        field_of(page, "#sf-read-only")
            .await?
            .attr("data-readonly")
            .await?
    )
    .is_some();
    let button = page.css("#sf-read-only button").await?;
    assert_that!(button.attr("disabled").await?).is_some();
    Ok(())
}

async fn form_attribute(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#sf-form-attribute input").await?;
    assert_that!(input.attr("form").await?).is_equal_to(Some("test".to_owned()));
    Ok(())
}

/// "with base props": the input is a `search` input unless another type is given.
async fn input_type(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#sf-type input").await?;
    assert_that!(input.attr("type").await?).is_equal_to(Some("text".to_owned()));
    Ok(())
}
