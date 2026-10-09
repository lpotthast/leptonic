// Upstream: react-aria-components/test/SearchField.test.js @ 99e6102368
// Upstream: react-aria/test/searchfield/useSearchField.test.js @ 99e6102368
//! The SearchField atom and `use_search_field`: slots, Enter/Escape, the clear button,
//! validation and states.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, KeyKind, Page, SyntheticEvent};

const PATH: &str = "/atoms/search-field";

/// The `SearchField` in the fixture section matching `section`.
async fn field_in(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.element(format!("{section} .leptonic-SearchField"))
        .await
}

/// The search input shows its value and is named by its label and described by its description
/// and error; the clear button is named "Clear search" and not tabbable ("provides slots", "clear
/// button should not be tabbable").
#[browser_test]
pub async fn provides_slots(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-slots input").await?;
    assert_that!(input)
        .has_attribute("type")
        .await
        .is_equal_to("search");
    assert_that!(input)
        .property("value")
        .await
        .get_some()
        .is_equal_to("test");
    assert_that!(field_in(page, "#sf-slots").await?)
        .has_attribute("data-foo")
        .await
        .is_equal_to("bar");
    assert_that!(input)
        .accessible_name()
        .await
        .is_equal_to("Test");
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Description Error"))
        .await;

    let button = page.element("#sf-slots button").await?;
    assert_that!(button)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Clear search");
    assert_that!(button)
        .has_attribute("tabindex")
        .await
        .is_equal_to("-1");
    Ok(())
}

/// Enter calls `on_submit` with the typed value ("preventDefault and onSubmit are called for Enter
/// if submit is provided").
#[browser_test]
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

/// Escape clears a non-empty field and calls `on_clear` without bubbling; a second Escape on the
/// empty field bubbles and doesn't call `on_clear` ("pressing the Escape key sets the state value
/// to "", ...", "preventDefault and stopPropagation are not called for Escape").
#[browser_test]
pub async fn escape_clears_once(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-keys input").await?;
    let clears = page.element("#sf-clears").await?;
    let escapes_bubbled = page.element("#sf-escapes-bubbled").await?;
    input.send_keys("query").await?;
    page.element("#sf-keys > div:not([data-empty])").await?;
    input.send_keys(Key::Escape).await?;
    clears.wait_for_inner_text("1").await?;
    assert_that!(input)
        .property("value")
        .await
        .get_some()
        .is_empty();
    page.element("#sf-keys > div[data-empty]").await?;
    escapes_bubbled
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;

    input.send_keys(Key::Escape).await?;
    escapes_bubbled.wait_for_inner_text("1").await?;
    clears
        .inner_text_stays("1", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Pressing the clear button empties the field, calls `on_clear` and focuses the input ("sets the
/// state to "" and focuses the search field", "calls the user provided onClear if provided").
#[browser_test]
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
    assert_that!(input)
        .property("value")
        .await
        .get_some()
        .is_empty();
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// The clear button focused while the user uses the keyboard has `data-focus-visible`, as its
/// documentation says.
#[browser_test]
pub async fn clear_button_shows_focus_visible(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-keys input").await?;
    input.send_keys("abc").await?;
    let button = page.element("#sf-keys button").await?;
    assert_that!(button)
        .attribute("data-focus-visible")
        .await
        .is_none();
    button.focus().await?;
    button
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    Ok(())
}

/// Without `on_submit`, Enter in the field submits its form ("preventDefault is not called for
/// Enter if onSubmit is not provided").
#[browser_test]
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

/// Validating the form focuses the empty required field and shows the browser's message as its
/// description; the error stays while typing and goes once focus leaves ("supports validation
/// errors").
#[browser_test]
pub async fn validation_errors(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-native input").await?;
    let field = field_in(page, "#sf-native").await?;
    assert_that!(input).has_attribute("required").await;
    assert_that!(input)
        .attribute("aria-required")
        .await
        .is_none();
    assert_that!(input)
        .attribute("aria-describedby")
        .await
        .is_none();
    assert_that!(input.is_valid().await?).is_false();

    assert_that!(page.element("#sf-native").await?.check_validity().await?).is_false();
    page.wait_for_focus(&input).await?;
    field.wait_for_attr("data-invalid", Some("true")).await?;
    assert_that!(field)
        .has_attribute("data-required")
        .await
        .is_equal_to("true");
    // The browser's validation message.
    let message = assert_that!(input)
        .property("validationMessage")
        .await
        .get_some()
        .is_not_blank()
        .actual()
        .clone();
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq(message))
        .await;

    // The error stays while typing, and goes once the value is committed.
    let described = assert_that!(input)
        .has_attribute("aria-describedby")
        .await
        .actual()
        .clone();
    page.send_keys("Devon").await?;
    input
        .attr_stays(
            "aria-describedby",
            Some(described.as_str()),
            std::time::Duration::from_millis(100),
        )
        .await?;
    assert_that!(input.is_valid().await?).is_true();
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    field.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// A read-only field is marked `data-readonly` and its clear button is disabled ("supports
/// readonly").
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let field = field_in(page, "#sf-read-only").await?;
    assert_that!(field)
        .has_attribute("data-readonly")
        .await
        .is_equal_to("true");
    let button = page.element("#sf-read-only button").await?;
    assert_that!(button).enabled().await.is_false();
    Ok(())
}

/// The field's `form` prop is set on its input ("should support form prop").
#[browser_test]
pub async fn form_attribute(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-form-attribute input").await?;
    assert_that!(input)
        .has_attribute("form")
        .await
        .is_equal_to("sf-attribute-form");
    Ok(())
}

/// A field given the type `text` renders a text input instead of a `search` input ("with base
/// props and value equal to state.value").
#[browser_test]
pub async fn input_type(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-type input").await?;
    assert_that!(input)
        .has_attribute("type")
        .await
        .is_equal_to("text");
    Ok(())
}

/// A disabled field ignores Enter and Escape on its input and presses of its clear button: no
/// submit, no clear, the value stays ("does not return an onKeyDown prop if isDisabled is true",
/// "$Name doesn't submit when disabled", "... doesn't clear when disabled").
#[browser_test]
pub async fn disabled_field_ignores_keys_and_the_clear_button(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#sf-disabled input").await?;
    assert_that!(input).enabled().await.is_false();
    for key in ["Enter", "Escape"] {
        input
            .dispatch(SyntheticEvent::keyboard(KeyKind::Down, key))
            .await?;
    }
    page.element("#sf-disabled button").await?.click().await?;
    page.element("#sf-disabled-events")
        .await?
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(input)
        .property("value")
        .await
        .get_some()
        .is_equal_to("test");
    Ok(())
}
