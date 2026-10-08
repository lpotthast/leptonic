// Upstream: react-aria-components/test/TextField.test.js @ 99e6102368
// Upstream: react-aria-components/test/Form.test.js @ 99e6102368
//! The TextField, Input, TextArea, Label, Description, FieldError and Form atoms.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions},
    polling::wait_for,
};

const PATH: &str = "/atoms/text-field";

/// The `TextField` in the fixture section matching `section`.
async fn field_in(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.element(format!("{section} .leptonic-TextField")).await
}

/// "provides slots" of a field with an input.
pub async fn provides_slots_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    provides_slots(page, "input").await
}

/// "provides slots" of a field with a text area.
pub async fn provides_slots_textarea(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    provides_slots(page, "textarea").await
}

/// "provides slots": the value, `type` (inputs only), data attributes on the field, the label
/// and the description and error message referenced by the input.
async fn provides_slots(page: &Page<'_>, element: &str) -> Result<(), Report> {
    let container = format!("#tf-slots-{element}");
    let input = page.element(format!("{container} {element}")).await?;
    assert_that!(input.value().await?)
        .get_some()
        .is_equal_to("test");
    assert_that!(input.attr("type").await?.as_deref())
        .with_detail_message(format!("the type of the {element}"))
        .is_equal_to((element == "input").then_some("text"));
    assert_that!(field_in(page, &container).await?.attr("data-foo").await?)
        .get_some()
        .is_equal_to("bar");

    assert_that!(input.referenced_text("aria-labelledby").await?).is_equal_to("Test");
    let label = page.element(format!("{container} label")).await?;
    let id = input.id().await?;
    assert_that!(label.attr("for").await?).is_equal_to(id);
    wait_for("the description of the input")
        .observing(|| input.referenced_text("aria-describedby"))
        .to_be_equal_to("Description Error")
        .await?;
    Ok(())
}

pub async fn hover_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tf-slots-input input").await?;
    assert_that!(input.attr("data-hovered").await?).is_none();
    input.hover().await?;
    input.wait_for_attr("data-hovered", Some("true")).await?;
    let heading = page.element("h1").await?;
    heading.hover().await?;
    input.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

pub async fn focus_visible_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tf-slots-input input").await?;
    assert_that!(input.attr("data-focus-visible").await?).is_none();
    // Tab from the element before the input.
    let heading = page.element("h1").await?;
    page.eval::<()>(
        "arguments[0].setAttribute('tabindex', '-1');",
        vec![heading.to_json()?],
    )
    .await?;
    heading.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&input).await?;
    input.wait_for_attr("data-focused", Some("true")).await?;
    input
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

pub async fn read_only_and_required_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let plain = field_in(page, "#tf-slots-input").await?;
    assert_that!(plain.attr("data-readonly").await?).is_none();
    assert_that!(plain.attr("data-required").await?).is_none();
    let read_only = field_in(page, "#tf-read-only").await?;
    assert_that!(read_only.attr("data-readonly").await?).is_some();
    let required = field_in(page, "#tf-required").await?;
    assert_that!(required.attr("data-required").await?).is_some();
    Ok(())
}

/// "supports validation errors" of a field with an input.
pub async fn native_validation_errors_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    native_validation_errors(page, "tf-native-input").await
}

/// "supports validation errors" of a field with a text area.
pub async fn native_validation_errors_textarea(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    native_validation_errors(page, "tf-native-textarea").await
}

/// "supports validation errors": with native validation, the error (the browser's validation
/// message) shows once the form is checked (focusing the input), stays while typing and goes
/// once the value is committed.
async fn native_validation_errors(page: &Page<'_>, form: &str) -> Result<(), Report> {
    let container = format!("#{form}");
    let input = page
        .element(format!("{container} input, {container} textarea"))
        .await?;
    let field = field_in(page, &container).await?;
    assert_that!(input.attr("required").await?).is_some();
    assert_that!(input.attr("aria-required").await?).is_none();
    assert_that!(input.attr("aria-describedby").await?).is_none();
    assert_that!(input.is_valid().await?).is_false();

    assert_that!(page.element(&container).await?.check_validity().await?).is_false();
    let message = input.prop("validationMessage").await?.unwrap_or_default();
    assert_that!(&message).is_not_blank();
    wait_for("the description of the input")
        .observing(|| input.referenced_text("aria-describedby"))
        .to_be_equal_to(message.as_str())
        .await?;
    field.wait_for_attr("data-invalid", Some("true")).await?;
    page.wait_for_focus(&input).await?;

    page.send_keys("Devon").await?;
    let described = input.attr("aria-describedby").await?;
    input
        .attr_stays("aria-describedby", described.as_deref())
        .await?;
    assert_that!(input.is_valid().await?).is_true();

    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    field.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// "supports customizing validation errors".
pub async fn customized_validation_errors(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tf-custom-error input").await?;
    assert_that!(
        page.element("#tf-custom-error")
            .await?
            .check_validity()
            .await?
    )
    .is_false();
    wait_for("the description of the input")
        .observing(|| input.referenced_text("aria-describedby"))
        .to_be_equal_to("Please enter a name")
        .await?;
    page.wait_for_focus(&input).await?;
    page.send_keys("Devon").await?;
    assert_that!(input.is_valid().await?).is_true();
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

/// "should not render the field error div if no error is provided and isInvalid is true".
pub async fn invalid_without_message_renders_no_error(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tf-invalid-without-message input").await?;
    assert_that!(input.attr("aria-invalid").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(input.attr("data-invalid").await?).is_some();
    assert_that!(input.attr("aria-describedby").await?).is_none();
    Ok(())
}

/// "should render the id attribute only on the input element" / "should link an id on the
/// input to the label htmlFor".
pub async fn id_goes_on_the_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tf-id input").await?;
    assert_that!(input.id().await?)
        .get_some()
        .is_equal_to("name");
    assert_that!(field_in(page, "#tf-id").await?.id().await?).is_none();
    let label = page.element("#tf-id label").await?;
    assert_that!(label.attr("for").await?)
        .get_some()
        .is_equal_to("name");
    Ok(())
}

pub async fn form_attribute(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tf-form-attribute input").await?;
    assert_that!(input.attr("form").await?)
        .get_some()
        .is_equal_to("test");
    Ok(())
}

/// Form: "supports server validation errors".
pub async fn server_validation_errors(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#form-server input").await?;
    assert_that!(input.attr("aria-describedby").await?).is_none();

    for _ in 0..2 {
        // Submitting twice doesn't clear the server error.
        page.element("#form-server-submit").await?.click().await?;
        assert_that!(page.element("#form-server").await?.check_validity().await?).is_false();
        wait_for("the description of the input")
            .observing(|| input.referenced_text("aria-describedby"))
            .to_be_equal_to("Invalid name.")
            .await?;
        assert_that!(input.is_valid().await?).is_false();
        page.wait_for_focus(&input).await?;
    }

    input.clear().await?;
    page.send_keys("Devon").await?;
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    assert_that!(input.is_valid().await?).is_true();

    // The server answers with the same errors again: they show again (react-aria resets on
    // every new errors object).
    page.element("#form-server-submit").await?.click().await?;
    wait_for("the description of the input")
        .observing(|| input.referenced_text("aria-describedby"))
        .to_be_equal_to("Invalid name.")
        .await?;
    assert_that!(input.is_valid().await?).is_false();
    Ok(())
}

/// No upstream test (React keeps a controlled input's DOM value in sync by itself): text a bound
/// value rejects or changes shows as the value holds it.
pub async fn bound_values_keep_the_dom_in_sync(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let fixed = page.element("#tf-rejecting input").await?;
    fixed.focus().await?;
    page.send_keys("x").await?;
    fixed.prop_stays("value", "fixed").await?;

    let upper = page.element("#tf-uppercase input").await?;
    upper.focus().await?;
    page.send_keys("ab").await?;
    upper.wait_for_prop("value", "AB").await?;
    Ok(())
}

/// Form: `Native` by default, `Aria` sets `novalidate`, fields override the form's behavior.
pub async fn form_validation_behavior(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    // Boolean attributes read as "true" while present.
    for (form, novalidate, native) in [
        ("form-native", false, true),
        ("form-aria", true, false),
        ("form-native-field-aria", false, false),
        ("form-aria-field-native", true, true),
    ] {
        let form_element = page.element(format!("#{form}")).await?;
        assert_that!(form_element.attr("novalidate").await?.as_deref())
            .with_detail_message(format!("novalidate of #{form}"))
            .is_equal_to(novalidate.then_some("true"));
        let input = form_element.element("input").await?;
        assert_that!(input.attr("required").await?.as_deref())
            .with_detail_message(format!("required in #{form}"))
            .is_equal_to(native.then_some("true"));
        assert_that!(input.attr("aria-required").await?.as_deref())
            .with_detail_message(format!("aria-required in #{form}"))
            .is_equal_to((!native).then_some("true"));
    }
    Ok(())
}
