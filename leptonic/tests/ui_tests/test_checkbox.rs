// Upstream: react-aria-components/test/Checkbox.test.js @ 99e6102368
// Upstream: react-aria-components/test/CheckboxGroup.test.js @ 99e6102368
//! Behavior of the checkbox hooks (through the `Checkbox` and `CheckboxGroup` atoms): a native
//! checkbox inside a label, toggled by pressing the label or with Space; hover, focus ring,
//! indeterminate, disabled, read-only, invalid and required states; a checkbox bound to a
//! signal; groups (shared name, description, disabled and read-only groups, native required
//! validation). Spec: react-aria-components `Checkbox.test.js`, `CheckboxGroup.test.js`.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, KeyKind, Page, SyntheticEvent, css};

const PATH: &str = "/atoms/checkbox";

/// The `<label>` of the checkbox with the text `text`.
async fn label(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(css("label").text(text)).await
}

/// The native checkbox inside `label`.
async fn input(label: &WebElement) -> Result<WebElement, Report> {
    label.element("input").await
}

/// The fixture's output of the "Basic" checkbox's state (`true`/`false`).
async fn basic_value(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-cb-basic-value").await
}

/// Pressing the label toggles the checkbox: the label's `data-selected`, the input's `checked`
/// state and the reported value follow ("should support selected state").
#[browser_test]
pub async fn selected_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Basic").await?;
    let input = input(&label).await?;
    let value = basic_value(page).await?;
    assert_that!(input)
        .has_attribute("type")
        .await
        .is_equal_to("checkbox");
    assert_that!(label)
        .attribute("data-selected")
        .await
        .is_none();
    assert_that!(input).selected().await.is_false();

    label.click().await?;
    label.wait_for_attr("data-selected", Some("true")).await?;
    value.wait_for_inner_text("true").await?;
    assert_that!(input).selected().await.is_true();

    label.click().await?;
    label.wait_for_attr("data-selected", None).await?;
    value.wait_for_inner_text("false").await?;
    assert_that!(input).selected().await.is_false();
    Ok(())
}

/// Tab focuses the checkbox with a focus ring, Space toggles it, and the ring goes when the focus
/// leaves ("should support focus ring").
#[browser_test]
pub async fn keyboard_and_focus_ring(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Basic").await?;
    let input = input(&label).await?;
    let value = basic_value(page).await?;
    page.element("#test-cb-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&input).await?;
    label.wait_for_attr("data-focused", Some("true")).await?;
    label
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;

    page.send_keys(Key::Space).await?;
    value.wait_for_inner_text("true").await?;
    page.send_keys(Key::Space).await?;
    value.wait_for_inner_text("false").await?;

    page.send_keys(Key::Tab).await?;
    label.wait_for_attr("data-focused", None).await?;
    label.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// A virtual click on the label toggles the checkbox, as with a native label.
#[browser_test]
pub async fn virtual_label_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Basic").await?;
    let value = basic_value(page).await?;
    label.virtual_click().await?;
    value.wait_for_inner_text("true").await?;
    label.virtual_click().await?;
    value.wait_for_inner_text("false").await?;
    Ok(())
}

/// Hovering the label sets `data-hovered`, and leaving it clears it ("should support hover").
#[browser_test]
pub async fn hover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Basic").await?;
    assert_that!(label)
        .attribute("data-hovered")
        .await
        .is_none();
    label.hover().await?;
    label.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("#test-cb-before").await?.hover().await?;
    label.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Holding the pointer down on the label sets `data-pressed` until it is released ("should support
/// press state").
#[browser_test]
pub async fn press_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Basic").await?;
    assert_that!(label)
        .attribute("data-pressed")
        .await
        .is_none();
    let held = label.press_and_hold().await?;
    label.wait_for_attr("data-pressed", Some("true")).await?;
    held.release().await?;
    label.wait_for_attr("data-pressed", None).await?;
    basic_value(page).await?.wait_for_inner_text("true").await?;
    Ok(())
}

/// Holding Space on the focused checkbox sets `data-pressed` until the key is released, which
/// toggles it ("should support press state with keyboard").
#[browser_test]
pub async fn press_state_with_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Basic").await?;
    let input = input(&label).await?;
    input.focus().await?;
    input
        .dispatch(SyntheticEvent::keyboard(KeyKind::Down, " "))
        .await?;
    label.wait_for_attr("data-pressed", Some("true")).await?;
    input
        .dispatch(SyntheticEvent::keyboard(KeyKind::Up, " "))
        .await?;
    label.wait_for_attr("data-pressed", None).await?;
    Ok(())
}

/// An indeterminate checkbox has `data-indeterminate` on its label and its input's `indeterminate`
/// property set ("should support indeterminate state").
#[browser_test]
pub async fn indeterminate_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Indeterminate").await?;
    let input = input(&label).await?;
    assert_that!(label)
        .has_attribute("data-indeterminate")
        .await
        .is_equal_to("true");
    assert_that!(input)
        .property("indeterminate")
        .await
        .some()
        .is_equal_to("true");
    Ok(())
}

/// A disabled checkbox marks its label `data-disabled`, disables its input and ignores presses
/// ("should support disabled state").
#[browser_test]
pub async fn disabled_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Disabled").await?;
    let input = input(&label).await?;
    assert_that!(label)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    assert_that!(input).enabled().await.is_false();
    label.click().await?;
    let value = page.element("#test-cb-disabled-value").await?;
    value
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(label)
        .attribute("data-selected")
        .await
        .is_none();
    Ok(())
}

/// A read-only checkbox is `aria-readonly` and stays checked when pressed, in its state and in the
/// DOM ("should support read only state").
#[browser_test]
pub async fn read_only_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Read only").await?;
    let input = input(&label).await?;
    assert_that!(label)
        .has_attribute("data-readonly")
        .await
        .is_equal_to("true");
    assert_that!(input)
        .has_attribute("aria-readonly")
        .await
        .is_equal_to("true");
    assert_that!(input).selected().await.is_true();
    label.click().await?;
    let value = page.element("#test-cb-readonly-value").await?;
    value
        .inner_text_stays("true", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(input).selected().await.is_true();
    assert_that!(label)
        .has_attribute("data-selected")
        .await
        .is_equal_to("true");
    Ok(())
}

/// An invalid checkbox has `data-invalid` on its label and `aria-invalid` on its input ("should
/// support invalid state").
#[browser_test]
pub async fn invalid_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Invalid").await?;
    let input = input(&label).await?;
    assert_that!(label)
        .has_attribute("data-invalid")
        .await
        .is_equal_to("true");
    assert_that!(input)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    Ok(())
}

/// A required checkbox is `required` with native validation and `aria-required` with ARIA
/// validation; validating the form marks it invalid until it's checked ("should support required
/// state").
#[browser_test]
pub async fn required_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let native = label(page, "Required native").await?;
    let native_input = input(&native).await?;
    assert_that!(native)
        .has_attribute("data-required")
        .await
        .is_equal_to("true");
    assert_that!(native_input).has_attribute("required").await;
    assert_that!(native_input)
        .attribute("aria-required")
        .await
        .is_none();

    let aria = label(page, "Required aria").await?;
    let aria_input = input(&aria).await?;
    assert_that!(aria_input)
        .attribute("required")
        .await
        .is_none();
    assert_that!(aria_input)
        .has_attribute("aria-required")
        .await
        .is_equal_to("true");

    assert_that!(native)
        .attribute("data-invalid")
        .await
        .is_none();
    let form = page.element("#test-cb-required-form").await?;
    assert_that!(form.check_validity().await?).is_false();
    native.wait_for_attr("data-invalid", Some("true")).await?;
    native.click().await?;
    native.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// A checkbox bound to a signal follows changes from outside and writes the signal.
#[browser_test]
pub async fn bound_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page, "Bound").await?;
    let input = input(&label).await?;
    let flip = page.element("#test-cb-bound-flip").await?;
    flip.click().await?;
    label.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(input).selected().await.is_true();
    label.click().await?;
    label.wait_for_attr("data-selected", None).await?;
    flip.click().await?;
    label.wait_for_attr("data-selected", Some("true")).await?;
    Ok(())
}

/// A bound read-only checkbox ignores presses and Space, and a bound checkbox reports changes to
/// `on_change`.
#[browser_test]
pub async fn bound_read_only_and_on_change(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let read_only = label(page, "Bound read only").await?;
    let input = input(&read_only).await?;
    read_only.click().await?;
    input.focus().await?;
    page.send_keys(Key::Space).await?;
    read_only
        .attr_stays("data-selected", None, std::time::Duration::from_millis(100))
        .await?;
    assert_that!(input).selected().await.is_false();

    label(page, "Bound reported").await?.click().await?;
    let reported = page.element("#test-cb-reported").await?;
    reported.wait_for_inner_text("true").await?;
    Ok(())
}

/// A labelled group gives its checkboxes its name and description, and its value lists the checked
/// keys; a disabled checkbox in it ignores presses ("should support selected state", "should
/// support disabled state on checkbox", "supports help text").
#[browser_test]
pub async fn group(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page
        .element(css("[role=group]").has(css("label").text("Dogs")))
        .await?;
    assert_that!(group)
        .accessible_name()
        .await
        .is_equal_to("Pets");
    assert_that!(group)
        .accessible_description()
        .await
        .is_equal_to("Pick your pets.");

    let dogs = label(page, "Dogs").await?;
    let cats = label(page, "Cats").await?;
    let dragons = label(page, "Dragons").await?;
    let dogs_input = input(&dogs).await?;
    let cats_input = input(&cats).await?;
    assert_that!(dogs_input)
        .has_attribute("name")
        .await
        .is_equal_to("pets");
    assert_that!(cats_input)
        .has_attribute("name")
        .await
        .is_equal_to("pets");
    assert_that!(dogs_input)
        .property("value")
        .await
        .some()
        .is_equal_to("dogs");
    assert_that!(dogs_input)
        .accessible_description()
        .await
        .is_equal_to("Pick your pets.");

    let value = page.element("#test-cb-group-value").await?;
    dogs.click().await?;
    value.wait_for_inner_text("dogs").await?;
    cats.click().await?;
    value.wait_for_inner_text("cats,dogs").await?;
    dogs.click().await?;
    value.wait_for_inner_text("cats").await?;
    dogs.wait_for_attr("data-selected", None).await?;
    assert_that!(cats)
        .has_attribute("data-selected")
        .await
        .is_equal_to("true");

    assert_that!(dragons)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    assert_that!(input(&dragons).await?)
        .enabled()
        .await
        .is_false();
    dragons.click().await?;
    value
        .inner_text_stays("cats", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A disabled group disables its checkboxes, and pressing the checkboxes of a read-only group keeps
/// their states ("should support disabled state on group", "should support read only state").
#[browser_test]
pub async fn group_disabled_and_read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disabled_group = page
        .element("[role=group][aria-label='Disabled group']")
        .await?;
    assert_that!(disabled_group)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    assert_that!(disabled_group)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    let disabled = label(page, "Disabled group A").await?;
    assert_that!(disabled)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    assert_that!(input(&disabled).await?)
        .enabled()
        .await
        .is_false();

    let read_only_group = page
        .element("[role=group][aria-label='Read-only group']")
        .await?;
    assert_that!(read_only_group)
        .has_attribute("data-readonly")
        .await
        .is_equal_to("true");
    let a = label(page, "Read-only group A").await?;
    let b = label(page, "Read-only group B").await?;
    assert_that!(input(&b).await?)
        .has_attribute("aria-readonly")
        .await
        .is_equal_to("true");
    b.click().await?;
    a.click().await?;
    a.attr_stays(
        "data-selected",
        Some("true"),
        std::time::Duration::from_millis(100),
    )
    .await?;
    assert_that!(b).attribute("data-selected").await.is_none();
    assert_that!(input(&a).await?).selected().await.is_true();
    assert_that!(input(&b).await?).selected().await.is_false();
    Ok(())
}

/// Validating a form with a required group and no checked checkbox marks the group and its
/// checkboxes invalid and adds the browser's message to the group's description; checking one
/// clears both ("should support validation errors").
#[browser_test]
pub async fn group_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let form = page.element("#test-cb-group-form").await?;
    let group = form.element("[role=group]").await?;
    let a = label(page, "Required group A").await?;
    let b = label(page, "Required group B").await?;
    for label in [&a, &b] {
        assert_that!(input(label).await?)
            .has_attribute("required")
            .await;
        assert_that!(label)
            .attribute("data-invalid")
            .await
            .is_none();
    }
    assert_that!(group)
        .attribute("data-invalid")
        .await
        .is_none();
    let description = group.accessible_description().await?;

    assert_that!(form.check_validity().await?).is_false();
    group.wait_for_attr("data-invalid", Some("true")).await?;
    for label in [&a, &b] {
        label.wait_for_attr("data-invalid", Some("true")).await?;
    }
    let message = assert_that!(input(&a).await?)
        .property("validationMessage")
        .await
        .some()
        .is_not_blank()
        .actual()
        .clone();
    let invalid_description = [description.as_str(), message.as_str()]
        .into_iter()
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    assert_that!(|| group.accessible_description())
        .eventually_ok()
        .matches(eq(invalid_description))
        .await;

    a.click().await?;
    group.wait_for_attr("data-invalid", None).await?;
    for label in [&a, &b] {
        label.wait_for_attr("data-invalid", None).await?;
        assert_that!(input(label).await?)
            .attribute("required")
            .await
            .is_none();
    }
    assert_that!(|| group.accessible_description())
        .eventually_ok()
        .matches(eq(description.clone()))
        .await;
    Ok(())
}

/// The group and the inputs of the checkboxes with the texts `texts` in the section `section`.
async fn validation_section(
    page: &Page<'_>,
    section: &str,
    texts: &[&str],
) -> Result<(WebElement, Vec<WebElement>), Report> {
    let group = page.element(format!("#{section} [role=group]")).await?;
    let mut inputs = Vec::new();
    for text in texts {
        inputs.push(input(&label(page, text).await?).await?);
    }
    Ok((group, inputs))
}

/// Waits until `group` is described by exactly `description` (its error), or by nothing.
async fn wait_for_group_error(group: &WebElement, description: &str) -> Result<(), Report> {
    assert_that!(|| group.accessible_description())
        .eventually_ok()
        .matches(eq(description.to_owned()))
        .await;
    Ok(())
}

/// A native group validator makes every checkbox invalid until all three are checked; a validity
/// check shows its error and focuses the first checkbox ("supports group level validate function",
/// native).
#[browser_test]
pub async fn native_group_validate_function(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let names = [
        "Native group terms",
        "Native group cookies",
        "Native group privacy",
    ];
    let (group, inputs) = validation_section(page, "gv-native-group", &names).await?;
    assert_that!(group)
        .attribute("aria-describedby")
        .await
        .is_none();
    for input in &inputs {
        assert_that!(input).attribute("required").await.is_none();
        assert_that!(input.is_valid().await?).is_false();
    }
    let form = page.element("#gv-native-group").await?;
    assert_that!(form.check_validity().await?).is_false();
    wait_for_group_error(&group, "You must accept all terms").await?;
    page.wait_for_focus(&inputs[0]).await?;

    for (index, name) in names.iter().enumerate() {
        label(page, name).await?.click().await?;
        let all = index == names.len() - 1;
        for input in &inputs {
            assert_that!(|| input.is_valid())
                .eventually_ok()
                .matches(eq(all))
                .await;
        }
    }
    group.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

/// The native validators of two checkboxes in a group make only those invalid; the group shows
/// both errors after a validity check and focuses the first, and drops each error once its
/// checkbox is checked ("supports checkbox level validate function", native).
#[browser_test]
pub async fn native_checkbox_validate_function(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let names = [
        "Native items terms",
        "Native items cookies",
        "Native items privacy",
    ];
    let (group, inputs) = validation_section(page, "gv-native-items", &names).await?;
    assert_that!(group)
        .attribute("aria-describedby")
        .await
        .is_none();
    assert_that!(inputs[0].is_valid().await?).is_false();
    assert_that!(inputs[1].is_valid().await?).is_false();
    assert_that!(inputs[2].is_valid().await?).is_true();

    let form = page.element("#gv-native-items").await?;
    assert_that!(form.check_validity().await?).is_false();
    wait_for_group_error(
        &group,
        "You must accept the terms. You must accept the cookies.",
    )
    .await?;
    page.wait_for_focus(&inputs[0]).await?;

    label(page, names[0]).await?.click().await?;
    wait_for_group_error(&group, "You must accept the cookies.").await?;
    assert_that!(inputs[0].is_valid().await?).is_true();
    assert_that!(inputs[1].is_valid().await?).is_false();
    label(page, names[1]).await?.click().await?;
    group.wait_for_attr("aria-describedby", None).await?;
    assert_that!(inputs[1].is_valid().await?).is_true();
    Ok(())
}

/// A server error for the group's name shows after submitting and makes its checkboxes invalid
/// until one is checked ("supports server validation", native).
#[browser_test]
pub async fn native_group_server_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let names = ["Server terms A", "Server terms B"];
    let (group, inputs) = validation_section(page, "gv-native-server", &names).await?;
    assert_that!(group)
        .attribute("aria-describedby")
        .await
        .is_none();
    page.element("#gv-native-server-submit")
        .await?
        .click()
        .await?;
    wait_for_group_error(&group, "You must accept the terms.").await?;
    for input in &inputs {
        assert_that!(input.is_valid().await?).is_false();
    }
    label(page, names[0]).await?.click().await?;
    group.wait_for_attr("aria-describedby", None).await?;
    for input in &inputs {
        assert_that!(input.is_valid().await?).is_true();
    }
    Ok(())
}

/// A `FieldError` message chosen by the validation details replaces the browser's message
/// ("supports customizing native error messages").
#[browser_test]
pub async fn custom_native_error_message(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (group, _) =
        validation_section(page, "gv-custom-message", &["Custom message terms"]).await?;
    assert_that!(group)
        .attribute("aria-describedby")
        .await
        .is_none();
    let form = page.element("#gv-custom-message").await?;
    assert_that!(form.check_validity().await?).is_false();
    wait_for_group_error(&group, "Please select at least one item").await?;
    Ok(())
}

/// With ARIA validation, a group validator shows its error at once and marks every checkbox
/// `aria-invalid` (natively valid) until all three are checked ("supports group level validate
/// function", aria).
#[browser_test]
pub async fn aria_group_validate_function(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let names = [
        "Aria group terms",
        "Aria group cookies",
        "Aria group privacy",
    ];
    let (group, inputs) = validation_section(page, "gv-aria-group", &names).await?;
    wait_for_group_error(&group, "You must accept all terms").await?;
    for input in &inputs {
        assert_that!(input)
            .has_attribute("aria-invalid")
            .await
            .is_equal_to("true");
        assert_that!(input.is_valid().await?).is_true();
    }
    for name in &names[..2] {
        label(page, name).await?.click().await?;
    }
    group
        .attr_stays(
            "aria-describedby",
            group.attr("aria-describedby").await?.as_deref(),
            std::time::Duration::from_millis(100),
        )
        .await?;
    label(page, names[2]).await?.click().await?;
    group.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

/// With ARIA validation, the validators of two checkboxes mark those `aria-invalid` and show both
/// errors on the group, each until its checkbox is checked ("supports checkbox level validate
/// function", aria).
#[browser_test]
pub async fn aria_checkbox_validate_function(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let names = [
        "Aria items terms",
        "Aria items cookies",
        "Aria items privacy",
    ];
    let (group, inputs) = validation_section(page, "gv-aria-items", &names).await?;
    wait_for_group_error(
        &group,
        "You must accept the terms. You must accept the cookies.",
    )
    .await?;
    assert_that!(inputs[0])
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    assert_that!(inputs[1])
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    for input in &inputs {
        assert_that!(input.is_valid().await?).is_true();
    }
    label(page, names[0]).await?.click().await?;
    wait_for_group_error(&group, "You must accept the cookies.").await?;
    label(page, names[1]).await?.click().await?;
    group.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

/// With ARIA validation, a server error for the group's name shows at once and marks its checkbox
/// `aria-invalid` until it is checked ("supports server validation", aria).
#[browser_test]
pub async fn aria_group_server_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (group, inputs) =
        validation_section(page, "gv-aria-server", &["Aria server terms A"]).await?;
    wait_for_group_error(&group, "You must accept the terms").await?;
    assert_that!(inputs[0])
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    label(page, "Aria server terms A").await?.click().await?;
    group.wait_for_attr("aria-describedby", None).await?;
    inputs[0].wait_for_attr("aria-invalid", None).await?;
    Ok(())
}
