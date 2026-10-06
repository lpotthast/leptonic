// Upstream: react-aria-components/test/TextField.test.js @ 99e6102368
// Upstream: react-aria-components/test/Form.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The TextField, Input, TextArea, Label, Description, FieldError and Form atoms.
pub struct TextFieldAtomTests {}

#[async_trait]
impl BrowserTest<str> for TextFieldAtomTests {
    fn name(&self) -> Cow<'_, str> {
        "text_field_atom_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/text-field").await?;

        for element in ["input", "textarea"] {
            provides_slots(&page, element).await?;
        }
        hover_state(&page).await?;
        focus_visible_state(&page).await?;
        read_only_and_required_state(&page).await?;
        for form in ["tf-native-input", "tf-native-textarea"] {
            native_validation_errors(&page, form).await?;
        }
        customized_validation_errors(&page).await?;
        invalid_without_message_renders_no_error(&page).await?;
        id_goes_on_the_input(&page).await?;
        form_attribute(&page).await?;
        server_validation_errors(&page).await?;
        form_validation_behavior(&page).await?;

        Ok(())
    }
}

/// The field (`<div>`) around the element matching `selector`.
pub(crate) async fn field_of(page: &Page<'_>, selector: &str) -> Result<WebElement, Report> {
    page.css(&format!("{selector} > div")).await
}

/// The texts of the elements an attribute (`aria-labelledby`, `aria-describedby`) refers to.
pub(crate) async fn referenced_texts(
    page: &Page<'_>,
    element: &WebElement,
    attr: &str,
) -> Result<String, Report> {
    let ids = element.attr(attr).await?.unwrap_or_default();
    let mut texts = Vec::new();
    for id in ids.split_whitespace() {
        texts.push(page.element(id).await?.text().await?);
    }
    Ok(texts.join(" "))
}

/// Waits until `element`'s `attr` refers to elements with the texts `expected`.
pub(crate) async fn wait_for_referenced_texts(
    page: &Page<'_>,
    element: &WebElement,
    attr: &str,
    expected: &str,
) -> Result<(), Report> {
    for _ in 0..50 {
        if referenced_texts(page, element, attr).await? == expected {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    assert_that!(referenced_texts(page, element, attr).await?).is_equal_to(expected.to_owned());
    Ok(())
}

pub(crate) async fn is_valid(page: &Page<'_>, element: &WebElement) -> Result<bool, Report> {
    let valid = page
        .driver
        .execute(
            "return arguments[0].validity.valid;",
            vec![element.to_json()?],
        )
        .await?;
    Ok(valid.json().as_bool().unwrap_or_default())
}

pub(crate) async fn check_validity(page: &Page<'_>, form_id: &str) -> Result<(), Report> {
    page.driver
        .execute(
            &format!("document.getElementById('{form_id}').checkValidity();"),
            vec![],
        )
        .await?;
    Ok(())
}

pub(crate) async fn expect_focused(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.wait_for_focus_on(element, "the field's input").await
}

/// "provides slots": the value, `type` (inputs only), data attributes on the field, the label
/// and the description and error message referenced by the input.
async fn provides_slots(page: &Page<'_>, element: &str) -> Result<(), Report> {
    let container = format!("#tf-slots-{element}");
    let input = page.css(&format!("{container} {element}")).await?;
    assert_that!(input.prop("value").await?).is_equal_to(Some("test".to_owned()));
    let expected_type = (element == "input").then(|| "text".to_owned());
    assert_that!(input.attr("type").await?).is_equal_to(expected_type);
    assert_that!(field_of(page, &container).await?.attr("data-foo").await?)
        .is_equal_to(Some("bar".to_owned()));

    assert_that!(referenced_texts(page, &input, "aria-labelledby").await?)
        .is_equal_to("Test".to_owned());
    let label = page.css(&format!("{container} label")).await?;
    let id = input.attr("id").await?;
    assert_that!(label.attr("for").await?).is_equal_to(id);
    wait_for_referenced_texts(page, &input, "aria-describedby", "Description Error").await
}

async fn hover_state(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#tf-slots-input input").await?;
    assert_that!(input.attr("data-hovered").await?).is_none();
    page.driver
        .action_chain()
        .move_to_element_center(&input)
        .perform()
        .await?;
    page.wait_for_selector("#tf-slots-input input[data-hovered]")
        .await?;
    let heading = page.driver.find(By::Css("h1")).await?;
    page.driver
        .action_chain()
        .move_to_element_center(&heading)
        .perform()
        .await?;
    page.wait_for_no_selector("#tf-slots-input input[data-hovered]")
        .await
}

async fn focus_visible_state(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#tf-slots-input input").await?;
    assert_that!(input.attr("data-focus-visible").await?).is_none();
    // Tab from the element before the input.
    page.driver
        .execute(
            "document.querySelector('h1').setAttribute('tabindex', '-1'); \
             document.querySelector('h1').focus();",
            vec![],
        )
        .await?;
    page.press_tab().await?;
    expect_focused(page, &input).await?;
    page.wait_for_selector("#tf-slots-input input[data-focus-visible][data-focused]")
        .await?;
    page.press_tab().await?;
    page.wait_for_no_selector("#tf-slots-input input[data-focus-visible]")
        .await
}

async fn read_only_and_required_state(page: &Page<'_>) -> Result<(), Report> {
    let plain = field_of(page, "#tf-slots-input").await?;
    assert_that!(plain.attr("data-readonly").await?).is_none();
    assert_that!(plain.attr("data-required").await?).is_none();
    assert_that!(
        field_of(page, "#tf-read-only")
            .await?
            .attr("data-readonly")
            .await?
    )
    .is_some();
    assert_that!(
        field_of(page, "#tf-required")
            .await?
            .attr("data-required")
            .await?
    )
    .is_some();
    Ok(())
}

/// "supports validation errors": with native validation, the error shows once the form is
/// checked (focusing the input), stays while typing and goes once the value is committed.
async fn native_validation_errors(page: &Page<'_>, form: &str) -> Result<(), Report> {
    let container = format!("#{form}");
    let input = page
        .css(&format!("{container} input, {container} textarea"))
        .await?;
    assert_that!(input.attr("required").await?).is_some();
    assert_that!(input.attr("aria-required").await?).is_none();
    assert_that!(input.attr("aria-describedby").await?).is_none();
    assert_that!(is_valid(page, &input).await?).is_false();

    check_validity(page, form).await?;
    page.wait_for_selector(&format!("{container} [aria-describedby]"))
        .await?;
    assert_that!(referenced_texts(page, &input, "aria-describedby").await?).is_not_empty();
    assert_that!(
        field_of(page, &container)
            .await?
            .attr("data-invalid")
            .await?
    )
    .is_some();
    expect_focused(page, &input).await?;

    page.send_keys_to_active("Devon").await?;
    assert_that!(input.attr("aria-describedby").await?).is_some();
    assert_that!(is_valid(page, &input).await?).is_true();

    page.press_tab().await?;
    page.wait_for_no_selector(&format!("{container} [aria-describedby]"))
        .await?;
    page.wait_for_no_selector(&format!("{container} [data-invalid]"))
        .await
}

/// "supports customizing validation errors".
async fn customized_validation_errors(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#tf-custom-error input").await?;
    check_validity(page, "tf-custom-error").await?;
    wait_for_referenced_texts(page, &input, "aria-describedby", "Please enter a name").await?;
    page.wait_for_focus_on(&input, "the custom error field's input")
        .await?;
    page.send_keys_to_active("Devon").await?;
    assert_that!(is_valid(page, &input).await?).is_true();
    page.press_tab().await?;
    page.wait_for_no_selector("#tf-custom-error input[aria-describedby]")
        .await
}

/// "should not render the field error div if no error is provided and isInvalid is true".
async fn invalid_without_message_renders_no_error(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#tf-invalid-without-message input").await?;
    assert_that!(input.attr("aria-invalid").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(input.attr("data-invalid").await?).is_some();
    assert_that!(input.attr("aria-describedby").await?).is_none();
    Ok(())
}

/// "should render the id attribute only on the input element" / "should link an id on the
/// input to the label htmlFor".
async fn id_goes_on_the_input(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#tf-id input").await?;
    assert_that!(input.attr("id").await?).is_equal_to(Some("name".to_owned()));
    assert_that!(field_of(page, "#tf-id").await?.attr("id").await?).is_none();
    let label = page.css("#tf-id label").await?;
    assert_that!(label.attr("for").await?).is_equal_to(Some("name".to_owned()));
    Ok(())
}

async fn form_attribute(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#tf-form-attribute input").await?;
    assert_that!(input.attr("form").await?).is_equal_to(Some("test".to_owned()));
    Ok(())
}

/// Form: "supports server validation errors".
async fn server_validation_errors(page: &Page<'_>) -> Result<(), Report> {
    let input = page.css("#form-server input").await?;
    assert_that!(input.attr("aria-describedby").await?).is_none();

    for _ in 0..2 {
        // Submitting twice doesn't clear the server error.
        page.click_element_with_id("form-server-submit").await?;
        check_validity(page, "form-server").await?;
        wait_for_referenced_texts(page, &input, "aria-describedby", "Invalid name.").await?;
        assert_that!(is_valid(page, &input).await?).is_false();
        expect_focused(page, &input).await?;
    }

    input.clear().await?;
    page.send_keys_to_active("Devon").await?;
    page.press_tab().await?;
    page.wait_for_no_selector("#form-server [aria-describedby]")
        .await?;
    assert_that!(is_valid(page, &input).await?).is_true();
    Ok(())
}

/// Form: `Native` by default, `Aria` sets `novalidate`, fields override the form's behavior.
async fn form_validation_behavior(page: &Page<'_>) -> Result<(), Report> {
    for (form, novalidate, native) in [
        ("form-native", false, true),
        ("form-aria", true, false),
        ("form-native-field-aria", false, false),
        ("form-aria-field-native", true, true),
    ] {
        let form_element = page.element(form).await?;
        assert_that!(form_element.attr("novalidate").await?.is_some())
            .with_detail_message(format!("novalidate of #{form}"))
            .is_equal_to(novalidate);
        let input = page.css(&format!("#{form} input")).await?;
        assert_that!(input.attr("required").await?.is_some())
            .with_detail_message(format!("required in #{form}"))
            .is_equal_to(native);
        assert_that!(input.attr("aria-required").await?.is_some())
            .with_detail_message(format!("aria-required in #{form}"))
            .is_equal_to(!native);
    }
    Ok(())
}
