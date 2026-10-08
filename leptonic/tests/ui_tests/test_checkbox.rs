// Upstream: react-aria-components/test/Checkbox.test.js @ 99e6102368
// Upstream: react-aria-components/test/CheckboxGroup.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

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

    async fn run(
        &self,
        driver: &browser_test::thirtyfour::WebDriver,
        base_url: &str,
    ) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/checkbox").await?;

        selected_state(&page).await?;
        keyboard_and_focus_ring(&page).await?;
        virtual_label_click(&page).await?;
        hover(&page).await?;
        indeterminate_state(&page).await?;
        disabled_state(&page).await?;
        read_only_state(&page).await?;
        invalid_state(&page).await?;
        required_state(&page).await?;
        bound_state(&page).await?;
        bound_read_only_and_on_change(&page).await?;
        group(&page).await?;
        group_disabled_and_read_only(&page).await?;
        group_validation(&page).await?;

        Ok(())
    }
}

/// The `<label>` of the checkbox with the visible text `text`.
async fn label(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    Ok(page
        .driver
        .find(By::XPath(format!("//label[normalize-space(.)='{text}']")))
        .await?)
}

async fn input(label: &WebElement) -> Result<WebElement, Report> {
    Ok(label.find(By::Css("input")).await?)
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn checked(input: &WebElement) -> Result<bool, Report> {
    Ok(input.prop("checked").await?.as_deref() == Some("true"))
}

async fn selected_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Basic").await?;
    let input = input(&label).await?;
    assert_that!(attr(&input, "type").await?).is_equal_to(Some("checkbox".to_owned()));
    assert_that!(attr(&label, "data-selected").await?).is_none();
    assert_that!(checked(&input).await?).is_false();

    label.click().await?;
    page.wait_for_attr(&label, "data-selected", Some("true"))
        .await?;
    page.wait_for_text("test-cb-basic-value", "true").await?;
    assert_that!(checked(&input).await?).is_true();

    label.click().await?;
    page.wait_for_attr(&label, "data-selected", None).await?;
    page.wait_for_text("test-cb-basic-value", "false").await?;
    assert_that!(checked(&input).await?).is_false();
    Ok(())
}

async fn keyboard_and_focus_ring(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Basic").await?;
    let input = input(&label).await?;
    page.click_element_with_id("test-cb-before").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&input, "the basic checkbox").await?;
    page.wait_for_attr(&label, "data-focused", Some("true"))
        .await?;
    page.wait_for_attr(&label, "data-focus-visible", Some("true"))
        .await?;

    page.send_keys_to_active(Key::Space).await?;
    page.wait_for_text("test-cb-basic-value", "true").await?;
    page.send_keys_to_active(Key::Space).await?;
    page.wait_for_text("test-cb-basic-value", "false").await?;

    page.press_tab().await?;
    page.wait_for_attr(&label, "data-focused", None).await?;
    page.wait_for_attr(&label, "data-focus-visible", None).await
}

async fn hover(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Basic").await?;
    assert_that!(attr(&label, "data-hovered").await?).is_none();
    page.driver
        .action_chain()
        .move_to_element_center(&label)
        .perform()
        .await?;
    page.wait_for_attr(&label, "data-hovered", Some("true"))
        .await?;
    let other = page.element("test-cb-before").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&other)
        .perform()
        .await?;
    page.wait_for_attr(&label, "data-hovered", None).await
}

async fn indeterminate_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Indeterminate").await?;
    let input = input(&label).await?;
    assert_that!(attr(&label, "data-indeterminate").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(input.prop("indeterminate").await?).is_equal_to(Some("true".to_owned()));
    Ok(())
}

async fn disabled_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Disabled").await?;
    let input = input(&label).await?;
    assert_that!(attr(&label, "data-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&input, "disabled").await?).is_some();
    label.click().await?;
    stays!(
        "the text of #test-cb-disabled-value",
        "false".to_owned(),
        page.read_text_of("test-cb-disabled-value").await?
    );
    assert_that!(attr(&label, "data-selected").await?).is_none();
    Ok(())
}

async fn read_only_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Read only").await?;
    let input = input(&label).await?;
    assert_that!(attr(&label, "data-readonly").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&input, "aria-readonly").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(checked(&input).await?).is_true();
    label.click().await?;
    stays!(
        "the text of #test-cb-readonly-value",
        "true".to_owned(),
        page.read_text_of("test-cb-readonly-value").await?
    );
    // The DOM follows the state, not the rejected click.
    assert_that!(checked(&input).await?).is_true();
    assert_that!(attr(&label, "data-selected").await?).is_equal_to(Some("true".to_owned()));
    Ok(())
}

async fn invalid_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Invalid").await?;
    let input = input(&label).await?;
    assert_that!(attr(&label, "data-invalid").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&input, "aria-invalid").await?).is_equal_to(Some("true".to_owned()));
    Ok(())
}

async fn required_state(page: &Page<'_>) -> Result<(), Report> {
    // Native validation: the `required` attribute, no `aria-required`.
    let native = label(page, "Required native").await?;
    let native_input = input(&native).await?;
    assert_that!(attr(&native, "data-required").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&native_input, "required").await?).is_some();
    assert_that!(attr(&native_input, "aria-required").await?).is_none();
    // ARIA validation: `aria-required`, no `required`.
    let aria = label(page, "Required aria").await?;
    let aria_input = input(&aria).await?;
    assert_that!(attr(&aria_input, "required").await?).is_none();
    assert_that!(attr(&aria_input, "aria-required").await?).is_equal_to(Some("true".to_owned()));

    // Native validation shows once the form is validated, and clears when checked.
    assert_that!(attr(&native, "data-invalid").await?).is_none();
    page.driver
        .execute(
            "document.getElementById('test-cb-required-form').checkValidity()",
            Vec::new(),
        )
        .await?;
    page.wait_for_attr(&native, "data-invalid", Some("true"))
        .await?;
    native.click().await?;
    page.wait_for_attr(&native, "data-invalid", None).await
}

/// A checkbox bound to a signal follows changes from outside and writes the signal.
async fn bound_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Bound").await?;
    let input = input(&label).await?;
    page.click_element_with_id("test-cb-bound-flip").await?;
    page.wait_for_attr(&label, "data-selected", Some("true"))
        .await?;
    assert_that!(checked(&input).await?).is_true();
    label.click().await?;
    page.wait_for_attr(&label, "data-selected", None).await?;
    page.click_element_with_id("test-cb-bound-flip").await?;
    page.wait_for_attr(&label, "data-selected", Some("true"))
        .await
}

/// A bound checkbox stays read-only (by press and by Space), and reports changes to
/// `on_change` (react-aria: `useToggleState` gets `isReadOnly` and `onChange` either way).
async fn bound_read_only_and_on_change(page: &Page<'_>) -> Result<(), Report> {
    let read_only = label(page, "Bound read only").await?;
    let input = input(&read_only).await?;
    read_only.click().await?;
    input.focus().await?;
    page.send_keys_to_active(Key::Space).await?;
    stays!(
        "data-selected of the read-only checkbox",
        None,
        attr(&read_only, "data-selected").await?
    );
    assert_that!(checked(&input).await?).is_false();

    label(page, "Bound reported").await?.click().await?;
    page.wait_for_text("test-cb-reported", "true").await
}

async fn group(page: &Page<'_>) -> Result<(), Report> {
    let group = page.css("[role=group]").await?;
    let group_label = page
        .driver
        .find(By::XPath("//span[normalize-space(.)='Pets']"))
        .await?;
    let label_id = attr(&group_label, "id").await?;
    assert_that!(attr(&group, "aria-labelledby").await?).is_equal_to(label_id);

    let dogs = label(page, "Dogs").await?;
    let cats = label(page, "Cats").await?;
    let dragons = label(page, "Dragons").await?;
    let dogs_input = input(&dogs).await?;
    let cats_input = input(&cats).await?;
    // The group's name (none is generated), the key as value.
    assert_that!(attr(&dogs_input, "name").await?).is_equal_to(Some("pets".to_owned()));
    assert_that!(attr(&cats_input, "name").await?).is_equal_to(Some("pets".to_owned()));
    assert_that!(attr(&dogs_input, "value").await?).is_equal_to(Some("dogs".to_owned()));
    // The group's description describes the group and each checkbox.
    let description = page
        .driver
        .find(By::XPath("//*[normalize-space(.)='Pick your pets.']"))
        .await?;
    let description_id = attr(&description, "id").await?.unwrap_or_default();
    assert_that!(attr(&group, "aria-describedby").await?.unwrap_or_default())
        .contains(description_id.as_str());
    assert_that!(
        attr(&dogs_input, "aria-describedby")
            .await?
            .unwrap_or_default()
    )
    .contains(description_id.as_str());

    dogs.click().await?;
    page.wait_for_text("test-cb-group-value", "dogs").await?;
    cats.click().await?;
    page.wait_for_text("test-cb-group-value", "cats,dogs")
        .await?;
    dogs.click().await?;
    page.wait_for_text("test-cb-group-value", "cats").await?;
    page.wait_for_attr(&dogs, "data-selected", None).await?;
    assert_that!(attr(&cats, "data-selected").await?).is_equal_to(Some("true".to_owned()));

    // A disabled checkbox in an enabled group.
    assert_that!(attr(&dragons, "data-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&input(&dragons).await?, "disabled").await?).is_some();
    dragons.click().await?;
    stays!(
        "the text of #test-cb-group-value",
        "cats".to_owned(),
        page.read_text_of("test-cb-group-value").await?
    );
    Ok(())
}

async fn group_disabled_and_read_only(page: &Page<'_>) -> Result<(), Report> {
    let disabled_group = page
        .css("[role=group][aria-label='Disabled group']")
        .await?;
    assert_that!(attr(&disabled_group, "aria-disabled").await?)
        .is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&disabled_group, "data-disabled").await?)
        .is_equal_to(Some("true".to_owned()));
    let disabled = label(page, "Disabled group A").await?;
    assert_that!(attr(&disabled, "data-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&input(&disabled).await?, "disabled").await?).is_some();

    let read_only_group = page
        .css("[role=group][aria-label='Read-only group']")
        .await?;
    assert_that!(attr(&read_only_group, "data-readonly").await?)
        .is_equal_to(Some("true".to_owned()));
    let a = label(page, "Read-only group A").await?;
    let b = label(page, "Read-only group B").await?;
    assert_that!(attr(&input(&b).await?, "aria-readonly").await?)
        .is_equal_to(Some("true".to_owned()));
    b.click().await?;
    a.click().await?;
    stays!(
        "data-selected of A",
        Some("true".to_owned()),
        attr(&a, "data-selected").await?
    );
    assert_that!(attr(&b, "data-selected").await?).is_none();
    assert_that!(checked(&input(&a).await?).await?).is_true();
    assert_that!(checked(&input(&b).await?).await?).is_false();
    Ok(())
}

/// A required group (native validation): every checkbox is `required` while none is checked;
/// validating the form marks the group invalid and shows the error, checking one clears it.
async fn group_validation(page: &Page<'_>) -> Result<(), Report> {
    let form = page.element("test-cb-group-form").await?;
    let group = form.find(By::Css("[role=group]")).await?;
    let a = label(page, "Required group A").await?;
    let b = label(page, "Required group B").await?;
    for label in [&a, &b] {
        assert_that!(attr(&input(label).await?, "required").await?).is_some();
        assert_that!(attr(label, "data-invalid").await?).is_none();
    }
    assert_that!(attr(&group, "data-invalid").await?).is_none();
    let described = attr(&group, "aria-describedby").await?;

    page.driver
        .execute(
            "document.getElementById('test-cb-group-form').checkValidity()",
            Vec::new(),
        )
        .await?;
    page.wait_for_attr(&group, "data-invalid", Some("true"))
        .await?;
    for label in [&a, &b] {
        page.wait_for_attr(label, "data-invalid", Some("true"))
            .await?;
    }
    let error_id = attr(&group, "aria-describedby").await?.unwrap_or_default();
    assert_that!(Some(error_id.clone())).is_not_equal_to(described.clone());
    let error = page
        .element(error_id.split(' ').next_back().unwrap_or_default())
        .await?;
    assert_that!(error.text().await?.is_empty()).is_false();

    a.click().await?;
    page.wait_for_attr(&group, "data-invalid", None).await?;
    for label in [&a, &b] {
        page.wait_for_attr(label, "data-invalid", None).await?;
        assert_that!(attr(&input(label).await?, "required").await?).is_none();
    }
    page.wait_for_attr(&group, "aria-describedby", described.as_deref())
        .await
}

/// A virtual click on the label toggles, as with a native label.
async fn virtual_label_click(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Basic").await?;
    virtual_click(page, &label).await?;
    page.wait_for_text("test-cb-basic-value", "true").await?;
    virtual_click(page, &label).await?;
    page.wait_for_text("test-cb-basic-value", "false").await
}

/// `element.click()` from script: a virtual click, as assistive technology sends.
async fn virtual_click(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .execute("arguments[0].click()", vec![element.to_json()?])
        .await?;
    Ok(())
}
