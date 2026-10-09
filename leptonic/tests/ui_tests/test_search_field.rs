// Upstream: react-aria-components/test/SearchField.test.js @ 99e6102368
// Upstream: react-aria/test/searchfield/useSearchField.test.js @ 99e6102368
//! The SearchField atom and `use_search_field`: slots, Enter/Escape, the clear button,
//! validation and states.
use assertr::{matchers::eq, prelude::*};
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

const PATH: &str = "/atoms/search-field";

/// The `SearchField` in the fixture section matching `section`.
async fn field_in(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.element(format!("{section} .leptonic-SearchField"))
        .await
}

/// "provides slots": a searchbox with value, label, description and error message, and a clear
/// button named "Clear search" that is not tabbable.
pub async fn provides_slots(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-slots input").await?;
    assert_that!(input.attr("type").await?)
        .get_some()
        .is_equal_to("search");
    assert_that!(input.value().await?)
        .get_some()
        .is_equal_to("test");
    assert_that!(field_in(page, "#sf-slots").await?.attr("data-foo").await?)
        .get_some()
        .is_equal_to("bar");
    assert_that!(input.referenced_text("aria-labelledby").await?).is_equal_to("Test");
    assert_that!(|| input.referenced_text("aria-describedby"))
        .eventually_ok()
        .matches(eq("Description Error"))
        .await;

    let button = page.element("#sf-slots button").await?;
    assert_that!(button.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Clear search");
    assert_that!(button.attr("tabindex").await?)
        .get_some()
        .is_equal_to("-1");
    Ok(())
}

/// "preventDefault and onSubmit are called for Enter if submit is provided".
pub async fn enter_submits(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-keys input").await?;
    input.click().await?;
    input.send_keys("query").await?;
    input.send_keys(Key::Enter).await?;
    page.element("#sf-submitted")
        .await?
        .wait_for_inner_text("query")
        .await?;
    Ok(())
}

/// "pressing the Escape key sets the state value to "", if state.value is not empty, and calls
/// onClear ... and will not call onClear if escape pressed again"; an unhandled Escape keeps
/// propagating ("preventDefault and stopPropagation are not called for Escape").
pub async fn escape_clears_once(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-keys input").await?;
    let clears = page.element("#sf-clears").await?;
    let escapes_bubbled = page.element("#sf-escapes-bubbled").await?;
    input.send_keys("query").await?;
    page.element("#sf-keys > div:not([data-empty])").await?;
    input.send_keys(Key::Escape).await?;
    clears.wait_for_inner_text("1").await?;
    assert_that!(input.value().await?).get_some().is_empty();
    page.element("#sf-keys > div[data-empty]").await?;
    escapes_bubbled.inner_text_stays("0").await?;

    input.send_keys(Key::Escape).await?;
    escapes_bubbled.wait_for_inner_text("1").await?;
    clears.inner_text_stays("1").await?;
    Ok(())
}

/// "sets the state to "" and focuses the search field" / "calls the user provided onClear".
pub async fn clear_button_clears_and_focuses_the_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-keys input").await?;
    input.send_keys("abc").await?;
    page.element("h1").await?.click().await?;
    page.element("#sf-keys button").await?.click().await?;
    page.element("#sf-clears")
        .await?
        .wait_for_inner_text("1")
        .await?;
    assert_that!(input.value().await?).get_some().is_empty();
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// "preventDefault is not called for Enter if onSubmit is not provided".
pub async fn enter_without_on_submit_submits_the_form(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-form input").await?;
    input.click().await?;
    input.send_keys("q").await?;
    input.send_keys(Key::Enter).await?;
    page.element("#sf-form-submits")
        .await?
        .wait_for_inner_text("1")
        .await?;
    Ok(())
}

/// "supports validation errors": the native error shows once the form is validated (focusing the
/// field), and stays until the value is committed (focus leaves).
pub async fn validation_errors(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-native input").await?;
    let field = field_in(page, "#sf-native").await?;
    assert_that!(input.attr("required").await?).is_some();
    assert_that!(input.attr("aria-required").await?).is_none();
    assert_that!(input.attr("aria-describedby").await?).is_none();
    assert_that!(input.is_valid().await?).is_false();

    assert_that!(page.element("#sf-native").await?.check_validity().await?).is_false();
    page.wait_for_focus(&input).await?;
    field.wait_for_attr("data-invalid", Some("true")).await?;
    assert_that!(field.attr("data-required").await?)
        .get_some()
        .is_equal_to("true");
    // The browser's validation message.
    assert_that!(input.referenced_text("aria-describedby").await?).is_not_blank();

    page.send_keys("Devon").await?;
    assert_that!(input.is_valid().await?).is_true();
    assert_that!(input.attr("aria-describedby").await?).is_some();
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    field.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// "supports readonly"; the clear button is disabled while read-only.
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let field = field_in(page, "#sf-read-only").await?;
    assert_that!(field.attr("data-readonly").await?)
        .get_some()
        .is_equal_to("true");
    let button = page.element("#sf-read-only button").await?;
    assert_that!(button.is_enabled().await?).is_false();
    Ok(())
}

/// The `form` attribute reaches the input.
pub async fn form_attribute(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-form-attribute input").await?;
    assert_that!(input.attr("form").await?)
        .get_some()
        .is_equal_to("test");
    Ok(())
}

/// "with base props": the input is a `search` input unless another type is given.
pub async fn input_type(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-type input").await?;
    assert_that!(input.attr("type").await?)
        .get_some()
        .is_equal_to("text");
    Ok(())
}
