// Upstream: react-aria-components/test/TextField.test.js @ 99e6102368
// Upstream: react-aria-components/test/Form.test.js @ 99e6102368
//! The TextField, Input, TextArea, Label, Description, FieldError and Form atoms.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/atoms/text-field";

/// The `TextField` in the fixture section matching `section`.
async fn field_in(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.element(format!("{section} .leptonic-TextField")).await
}

/// A field with an input shows its value and data attributes, is labelled by its label and
/// described by its description and error message ("provides slots").
#[browser_test]
pub async fn provides_slots_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    provides_slots(page, "input").await
}

/// A field with a text area shows its value and data attributes, is labelled by its label and
/// described by its description and error message ("provides slots").
#[browser_test]
pub async fn provides_slots_textarea(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    provides_slots(page, "textarea").await
}

/// "provides slots": the value, `type` (inputs only), data attributes on the field, the label
/// and the description and error message referenced by the input.
async fn provides_slots(page: &Page<'_>, element: &str) -> Result<(), Report> {
    let container = format!("#tf-slots-{element}");
    let input = page.element(format!("{container} {element}")).await?;
    assert_that!(input)
        .property("value")
        .await
        .some()
        .is_equal_to("test");
    assert_that!(input)
        .attribute("type")
        .await
        .derive_owned(|value| value.as_deref())
        .with_detail_message(format!("the type of the {element}"))
        .is_equal_to((element == "input").then_some("text"));
    assert_that!(field_in(page, &container).await?)
        .has_attribute("data-foo")
        .await
        .is_equal_to("bar");

    assert_that!(input)
        .accessible_name()
        .await
        .is_equal_to("Test");
    let label = page.element(format!("{container} label")).await?;
    let id = input.id().await?;
    assert_that!(label).attribute("for").await.is_equal_to(id);
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Description Error"))
        .await;
    Ok(())
}

/// Hovering the input sets `data-hovered` on it, leaving it removes it ("should support hover
/// state").
#[browser_test]
pub async fn hover_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tf-slots-input input").await?;
    assert_that!(input)
        .attribute("data-hovered")
        .await
        .is_none();
    input.hover().await?;
    input.wait_for_attr("data-hovered", Some("true")).await?;
    let heading = page.element("h1").await?;
    heading.hover().await?;
    input.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Tabbing onto the input sets `data-focused` and `data-focus-visible` on it, tabbing away removes
/// them ("should support focus visible state").
#[browser_test]
pub async fn focus_visible_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tf-slots-input input").await?;
    assert_that!(input)
        .attribute("data-focus-visible")
        .await
        .is_none();
    // Tab from the element before the input.
    let heading = page.element("h1").await?;
    page.low_level()
        .eval::<()>(
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

/// A read-only field has `data-readonly`, a required one `data-required`, a plain one neither
/// ("should support read-only state", "should support required state").
#[browser_test]
pub async fn read_only_and_required_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let plain = field_in(page, "#tf-slots-input").await?;
    assert_that!(plain)
        .attribute("data-readonly")
        .await
        .is_none();
    assert_that!(plain)
        .attribute("data-required")
        .await
        .is_none();
    let read_only = field_in(page, "#tf-read-only").await?;
    assert_that!(read_only).has_attribute("data-readonly").await;
    let required = field_in(page, "#tf-required").await?;
    assert_that!(required).has_attribute("data-required").await;
    Ok(())
}

/// With native validation, a field with an input shows the browser's message once its form is
/// checked, keeps it while typing and drops it on commit ("supports validation errors").
#[browser_test]
pub async fn native_validation_errors_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    native_validation_errors(page, "tf-native-input").await
}

/// With native validation, a field with a text area shows the browser's message once its form is
/// checked, keeps it while typing and drops it on commit ("supports validation errors").
#[browser_test]
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

    assert_that!(page.element(&container).await?.check_validity().await?).is_false();
    let message = assert_that!(input)
        .property("validationMessage")
        .await
        .some()
        .is_not_blank()
        .actual()
        .clone();
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq(message.as_str()))
        .await;
    field.wait_for_attr("data-invalid", Some("true")).await?;
    page.wait_for_focus(&input).await?;

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

/// A field's custom error message shows once its form is checked and goes once a valid value is
/// committed ("supports customizing validation errors").
#[browser_test]
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
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Please enter a name"))
        .await;
    page.wait_for_focus(&input).await?;
    page.send_keys("Devon").await?;
    assert_that!(input.is_valid().await?).is_true();
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

/// An invalid field without an error message marks its input invalid but describes it by nothing
/// ("should not render the field error div if no error is provided and isInvalid is true").
#[browser_test]
pub async fn invalid_without_message_renders_no_error(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tf-invalid-without-message input").await?;
    assert_that!(input)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    assert_that!(input).has_attribute("data-invalid").await;
    assert_that!(input)
        .attribute("aria-describedby")
        .await
        .is_none();
    Ok(())
}

/// The field's `id` goes on the input only, and the label's `for` points to it ("should render the
/// id attribute only on the input element", "should link an id on the input to the label htmlFor").
#[browser_test]
pub async fn id_goes_on_the_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tf-id input").await?;
    assert_that!(input)
        .has_attribute("id")
        .await
        .is_equal_to("name");
    assert_that!(field_in(page, "#tf-id").await?)
        .attribute("id")
        .await
        .is_none();
    let label = page.element("#tf-id label").await?;
    assert_that!(label)
        .has_attribute("for")
        .await
        .is_equal_to("name");
    Ok(())
}

/// The field's `form` prop becomes the input's `form` attribute ("should support form prop").
#[browser_test]
pub async fn form_attribute(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tf-form-attribute input").await?;
    assert_that!(input)
        .has_attribute("form")
        .await
        .is_equal_to("tf-form");
    Ok(())
}

/// A form's server errors show on every submit until the user commits a new value, and show again
/// when the server returns them again ("supports server validation errors").
#[browser_test]
pub async fn server_validation_errors(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#form-server input").await?;
    assert_that!(input)
        .attribute("aria-describedby")
        .await
        .is_none();

    for _ in 0..2 {
        // Submitting twice doesn't clear the server error.
        page.element("#form-server-submit").await?.click().await?;
        assert_that!(page.element("#form-server").await?.check_validity().await?).is_false();
        assert_that!(|| input.accessible_description())
            .eventually_ok()
            .matches(eq("Invalid name."))
            .await;
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
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Invalid name."))
        .await;
    assert_that!(input.is_valid().await?).is_false();
    Ok(())
}

/// Text that a bound value rejects or rewrites (e.g. to uppercase) shows in the input as the bound
/// value holds it.
#[browser_test]
pub async fn bound_values_keep_the_dom_in_sync(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let fixed = page.element("#tf-rejecting input").await?;
    fixed.focus().await?;
    page.send_keys("x").await?;
    fixed
        .prop_stays("value", "fixed", std::time::Duration::from_millis(100))
        .await?;

    let upper = page.element("#tf-uppercase input").await?;
    upper.focus().await?;
    page.send_keys("ab").await?;
    upper.wait_for_prop("value", "AB").await?;
    Ok(())
}

/// A form validates natively by default and sets `novalidate` with `Aria`, and a field's own
/// validation behavior overrides its form's ("should use validationBehavior="native" by default",
/// "supports validationBehavior="aria"").
#[browser_test]
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
        assert_that!(form_element)
            .attribute("novalidate")
            .await
            .derive_owned(|value| value.as_deref())
            .with_detail_message(format!("novalidate of #{form}"))
            .is_equal_to(novalidate.then_some("true"));
        let input = form_element.element("input").await?;
        assert_that!(input)
            .attribute("required")
            .await
            .derive_owned(|value| value.as_deref())
            .with_detail_message(format!("required in #{form}"))
            .is_equal_to(native.then_some("true"));
        assert_that!(input)
            .attribute("aria-required")
            .await
            .derive_owned(|value| value.as_deref())
            .with_detail_message(format!("aria-required in #{form}"))
            .is_equal_to((!native).then_some("true"));
    }
    Ok(())
}

/// A native validator's error shows after a validity check, which focuses the field; the input
/// is valid as soon as the typed value passes, and the error goes once the value is committed
/// ("$Name supports validate function", native).
#[browser_test]
pub async fn native_validate_function(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tfv-validate input").await?;
    assert_that!(input)
        .attribute("aria-describedby")
        .await
        .is_none();
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(
        page.element("#tfv-validate")
            .await?
            .check_validity()
            .await?
    )
    .is_false();
    page.wait_for_focus(&input).await?;
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Invalid name"))
        .await;
    page.send_keys("Devon").await?;
    assert_that!(|| input.is_valid())
        .eventually_ok()
        .matches(eq(true))
        .await;
    assert_that!(input).has_attribute("aria-describedby").await;
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

/// A form that cancels its fields' `invalid` events keeps the focus where it is on a failed
/// validity check ("$Name does not auto focus invalid input if default is prevented").
#[browser_test]
pub async fn no_auto_focus_if_invalid_is_prevented(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tfv-prevented input").await?;
    assert_that!(
        page.element("#tfv-prevented")
            .await?
            .check_validity()
            .await?
    )
    .is_false();
    // The focus would move in a microtask after the `invalid` event.
    page.settle().await?;
    assert_that!(input).focused().await.is_false();
    Ok(())
}

/// With ARIA validation, a validator's error shows at once (`aria-invalid`, natively valid) and
/// goes as soon as the typed value passes ("$Name supports validate function", aria).
#[browser_test]
pub async fn aria_validate_function(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tfv-aria-validate input").await?;
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Invalid name"))
        .await;
    assert_that!(input)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    assert_that!(input.is_valid().await?).is_true();
    input.click().await?;
    page.send_keys("Devon").await?;
    input.wait_for_attr("aria-describedby", None).await?;
    input.wait_for_attr("aria-invalid", None).await?;
    Ok(())
}

/// With ARIA validation, a server error for the field's name shows at once and goes once a new
/// value is committed ("$Name supports server validation", aria).
#[browser_test]
pub async fn aria_server_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tfv-aria-server input").await?;
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Invalid name"))
        .await;
    assert_that!(input)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    input.click().await?;
    page.send_keys("Devon").await?;
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    input.wait_for_attr("aria-invalid", None).await?;
    Ok(())
}

/// A disabled field disables its input and marks it and the field `data-disabled`; typing
/// changes nothing ("$Name should support isDisabled").
#[browser_test]
pub async fn disabled_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#tf-disabled input").await?;
    assert_that!(input).enabled().await.is_false();
    assert_that!(input)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    assert_that!(field_in(page, "#tf-disabled").await?)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    Ok(())
}

/// An `Input` outside a field renders a plain input with its default class: typing changes its
/// value, and it tracks its own hover and focus (react-aria-components' `Input` without an
/// `InputContext`).
#[browser_test]
pub async fn standalone_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("input[aria-label='Standalone']").await?;
    assert_that!(input)
        .attribute("class")
        .await
        .is_equal_to(Some("leptonic-Input".to_owned()));

    assert_that!(input)
        .attribute("data-hovered")
        .await
        .is_none();
    input.hover().await?;
    input.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("h1").await?.hover().await?;
    input.wait_for_attr("data-hovered", None).await?;

    // A click focuses it without a visible focus ring.
    input.click().await?;
    page.wait_for_focus(&input).await?;
    input.wait_for_attr("data-focused", Some("true")).await?;
    assert_that!(input)
        .attribute("data-focus-visible")
        .await
        .is_none();
    page.send_keys("hello").await?;
    input.wait_for_prop("value", "hello").await?;
    page.element("#tf-standalone output")
        .await?
        .wait_for_inner_text("hello")
        .await?;

    // Leaving and coming back with the keyboard shows it.
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("data-focused", None).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&input).await?;
    input
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    Ok(())
}

/// A disabled, invalid `Input` outside a field is disabled and `aria-invalid`, and shows both in
/// its data attributes.
#[browser_test]
pub async fn standalone_input_disabled_and_invalid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page
        .element("input[aria-label='Standalone disabled']")
        .await?;
    assert_that!(input).enabled().await.is_false();
    assert_that!(input)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    assert_that!(input)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    assert_that!(input)
        .has_attribute("data-invalid")
        .await
        .is_equal_to("true");
    Ok(())
}

/// A `TextArea` outside a field renders a plain textarea with its default class, which takes
/// typed text.
#[browser_test]
pub async fn standalone_textarea(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let textarea = page
        .element("textarea[aria-label='Standalone notes']")
        .await?;
    assert_that!(textarea)
        .attribute("class")
        .await
        .is_equal_to(Some("leptonic-TextArea".to_owned()));
    textarea.click().await?;
    page.wait_for_focus(&textarea).await?;
    textarea.wait_for_attr("data-focused", Some("true")).await?;
    page.send_keys("notes").await?;
    textarea.wait_for_prop("value", "notes").await?;
    Ok(())
}
