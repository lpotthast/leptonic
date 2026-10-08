// Upstream: react-aria-components/test/Checkbox.test.js @ 99e6102368
// Upstream: react-aria-components/test/CheckboxGroup.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions, css},
    polling::wait_for,
};

/// Behavior of the checkbox hooks (through the `Checkbox` and `CheckboxGroup` atoms): a native
/// checkbox inside a label, toggled by pressing the label or with Space; hover, focus ring,
/// indeterminate, disabled, read-only, invalid and required states; a checkbox bound to a
/// signal; groups (shared name, description, disabled and read-only groups, native required
/// validation). Spec: react-aria-components `Checkbox.test.js`, `CheckboxGroup.test.js`.
pub struct CheckboxTests {}

#[async_trait]
impl BrowserTest<str> for CheckboxTests {
    fn name(&self) -> Cow<'_, str> {
        "checkbox_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/checkbox").await?;
        cases!(
            selected_state(&page),
            keyboard_and_focus_ring(&page),
            virtual_label_click(&page),
            hover(&page),
            indeterminate_state(&page),
            disabled_state(&page),
            read_only_state(&page),
            invalid_state(&page),
            required_state(&page),
            bound_state(&page),
            bound_read_only_and_on_change(&page),
            group(&page),
            group_disabled_and_read_only(&page),
            group_validation(&page),
        );
        Ok(())
    }
}

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

/// Pressing the label toggles the checkbox: `data-selected` on the label, the native `checked`
/// state, and the state the atom reports.
async fn selected_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Basic").await?;
    let input = input(&label).await?;
    let value = basic_value(page).await?;
    assert_that!(input.attr("type").await?)
        .get_some()
        .is_equal_to("checkbox");
    assert_that!(label.attr("data-selected").await?).is_none();
    assert_that!(input.is_selected().await?).is_false();

    label.click().await?;
    label.wait_for_attr("data-selected", Some("true")).await?;
    value.wait_for_inner_text("true").await?;
    assert_that!(input.is_selected().await?).is_true();

    label.click().await?;
    label.wait_for_attr("data-selected", None).await?;
    value.wait_for_inner_text("false").await?;
    assert_that!(input.is_selected().await?).is_false();
    Ok(())
}

/// Tab focuses the checkbox with a focus ring; Space toggles it.
async fn keyboard_and_focus_ring(page: &Page<'_>) -> Result<(), Report> {
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

/// A virtual click on the label toggles, as with a native label.
async fn virtual_label_click(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Basic").await?;
    let value = basic_value(page).await?;
    label.virtual_click().await?;
    value.wait_for_inner_text("true").await?;
    label.virtual_click().await?;
    value.wait_for_inner_text("false").await?;
    Ok(())
}

/// The pointer over the label sets `data-hovered`; leaving clears it.
async fn hover(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Basic").await?;
    assert_that!(label.attr("data-hovered").await?).is_none();
    label.hover().await?;
    label.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("#test-cb-before").await?.hover().await?;
    label.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

async fn indeterminate_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Indeterminate").await?;
    let input = input(&label).await?;
    assert_that!(label.attr("data-indeterminate").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(input.prop("indeterminate").await?)
        .get_some()
        .is_equal_to("true");
    Ok(())
}

/// A disabled checkbox ignores presses.
async fn disabled_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Disabled").await?;
    let input = input(&label).await?;
    assert_that!(label.attr("data-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(input.is_enabled().await?).is_false();
    label.click().await?;
    let value = page.element("#test-cb-disabled-value").await?;
    value.inner_text_stays("false").await?;
    assert_that!(label.attr("data-selected").await?).is_none();
    Ok(())
}

/// A read-only checkbox keeps its state; the DOM follows the state, not the rejected press.
async fn read_only_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Read only").await?;
    let input = input(&label).await?;
    assert_that!(label.attr("data-readonly").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(input.attr("aria-readonly").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(input.is_selected().await?).is_true();
    label.click().await?;
    let value = page.element("#test-cb-readonly-value").await?;
    value.inner_text_stays("true").await?;
    assert_that!(input.is_selected().await?).is_true();
    assert_that!(label.attr("data-selected").await?)
        .get_some()
        .is_equal_to("true");
    Ok(())
}

async fn invalid_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Invalid").await?;
    let input = input(&label).await?;
    assert_that!(label.attr("data-invalid").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(input.attr("aria-invalid").await?)
        .get_some()
        .is_equal_to("true");
    Ok(())
}

/// Native validation uses `required`, ARIA validation `aria-required`. The native error shows
/// once the form is validated, and clears when checked.
async fn required_state(page: &Page<'_>) -> Result<(), Report> {
    let native = label(page, "Required native").await?;
    let native_input = input(&native).await?;
    assert_that!(native.attr("data-required").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(native_input.attr("required").await?).is_some();
    assert_that!(native_input.attr("aria-required").await?).is_none();

    let aria = label(page, "Required aria").await?;
    let aria_input = input(&aria).await?;
    assert_that!(aria_input.attr("required").await?).is_none();
    assert_that!(aria_input.attr("aria-required").await?)
        .get_some()
        .is_equal_to("true");

    assert_that!(native.attr("data-invalid").await?).is_none();
    let form = page.element("#test-cb-required-form").await?;
    assert_that!(form.check_validity().await?).is_false();
    native.wait_for_attr("data-invalid", Some("true")).await?;
    native.click().await?;
    native.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// A checkbox bound to a signal follows changes from outside and writes the signal.
async fn bound_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Bound").await?;
    let input = input(&label).await?;
    let flip = page.element("#test-cb-bound-flip").await?;
    flip.click().await?;
    label.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(input.is_selected().await?).is_true();
    label.click().await?;
    label.wait_for_attr("data-selected", None).await?;
    flip.click().await?;
    label.wait_for_attr("data-selected", Some("true")).await?;
    Ok(())
}

/// A bound checkbox stays read-only (by press and by Space), and reports changes to
/// `on_change` (react-aria: `useToggleState` gets `isReadOnly` and `onChange` either way).
async fn bound_read_only_and_on_change(page: &Page<'_>) -> Result<(), Report> {
    let read_only = label(page, "Bound read only").await?;
    let input = input(&read_only).await?;
    read_only.click().await?;
    input.focus().await?;
    page.send_keys(Key::Space).await?;
    read_only.attr_stays("data-selected", None).await?;
    assert_that!(input.is_selected().await?).is_false();

    label(page, "Bound reported").await?.click().await?;
    let reported = page.element("#test-cb-reported").await?;
    reported.wait_for_inner_text("true").await?;
    Ok(())
}

/// A group: labelled and described, its checkboxes share the group's name and submit their keys;
/// the state lists the checked keys; a disabled checkbox in an enabled group ignores presses.
async fn group(page: &Page<'_>) -> Result<(), Report> {
    let group = page.element("[role=group]").await?;
    assert_that!(group.referenced_text("aria-labelledby").await?).is_equal_to("Pets");
    assert_that!(group.referenced_text("aria-describedby").await?).is_equal_to("Pick your pets.");

    let dogs = label(page, "Dogs").await?;
    let cats = label(page, "Cats").await?;
    let dragons = label(page, "Dragons").await?;
    let dogs_input = input(&dogs).await?;
    let cats_input = input(&cats).await?;
    assert_that!(dogs_input.attr("name").await?)
        .get_some()
        .is_equal_to("pets");
    assert_that!(cats_input.attr("name").await?)
        .get_some()
        .is_equal_to("pets");
    assert_that!(dogs_input.value().await?)
        .get_some()
        .is_equal_to("dogs");
    assert_that!(dogs_input.referenced_text("aria-describedby").await?)
        .is_equal_to("Pick your pets.");

    let value = page.element("#test-cb-group-value").await?;
    dogs.click().await?;
    value.wait_for_inner_text("dogs").await?;
    cats.click().await?;
    value.wait_for_inner_text("cats,dogs").await?;
    dogs.click().await?;
    value.wait_for_inner_text("cats").await?;
    dogs.wait_for_attr("data-selected", None).await?;
    assert_that!(cats.attr("data-selected").await?)
        .get_some()
        .is_equal_to("true");

    assert_that!(dragons.attr("data-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(input(&dragons).await?.is_enabled().await?).is_false();
    dragons.click().await?;
    value.inner_text_stays("cats").await?;
    Ok(())
}

/// A disabled group disables its checkboxes; a read-only group keeps their states.
async fn group_disabled_and_read_only(page: &Page<'_>) -> Result<(), Report> {
    let disabled_group = page
        .element("[role=group][aria-label='Disabled group']")
        .await?;
    assert_that!(disabled_group.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(disabled_group.attr("data-disabled").await?)
        .get_some()
        .is_equal_to("true");
    let disabled = label(page, "Disabled group A").await?;
    assert_that!(disabled.attr("data-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(input(&disabled).await?.is_enabled().await?).is_false();

    let read_only_group = page
        .element("[role=group][aria-label='Read-only group']")
        .await?;
    assert_that!(read_only_group.attr("data-readonly").await?)
        .get_some()
        .is_equal_to("true");
    let a = label(page, "Read-only group A").await?;
    let b = label(page, "Read-only group B").await?;
    assert_that!(input(&b).await?.attr("aria-readonly").await?)
        .get_some()
        .is_equal_to("true");
    b.click().await?;
    a.click().await?;
    a.attr_stays("data-selected", Some("true")).await?;
    assert_that!(b.attr("data-selected").await?).is_none();
    assert_that!(input(&a).await?.is_selected().await?).is_true();
    assert_that!(input(&b).await?.is_selected().await?).is_false();
    Ok(())
}

/// A required group (native validation): every checkbox is `required` while none is checked;
/// validating the form marks the group invalid and adds the browser's validation message to its
/// description; checking one clears both.
async fn group_validation(page: &Page<'_>) -> Result<(), Report> {
    let form = page.element("#test-cb-group-form").await?;
    let group = form.element("[role=group]").await?;
    let a = label(page, "Required group A").await?;
    let b = label(page, "Required group B").await?;
    for label in [&a, &b] {
        assert_that!(input(label).await?.attr("required").await?).is_some();
        assert_that!(label.attr("data-invalid").await?).is_none();
    }
    assert_that!(group.attr("data-invalid").await?).is_none();
    let description = group.referenced_text("aria-describedby").await?;

    assert_that!(form.check_validity().await?).is_false();
    group.wait_for_attr("data-invalid", Some("true")).await?;
    for label in [&a, &b] {
        label.wait_for_attr("data-invalid", Some("true")).await?;
    }
    let message = input(&a)
        .await?
        .prop("validationMessage")
        .await?
        .unwrap_or_default();
    assert_that!(&message).is_not_blank();
    let invalid_description = [description.as_str(), message.as_str()]
        .into_iter()
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    wait_for("the group's description")
        .observing(|| group.referenced_text("aria-describedby"))
        .to_be_equal_to(invalid_description)
        .await?;

    a.click().await?;
    group.wait_for_attr("data-invalid", None).await?;
    for label in [&a, &b] {
        label.wait_for_attr("data-invalid", None).await?;
        assert_that!(input(label).await?.attr("required").await?).is_none();
    }
    wait_for("the group's description")
        .observing(|| group.referenced_text("aria-describedby"))
        .to_be_equal_to(description.clone())
        .await?;
    Ok(())
}
