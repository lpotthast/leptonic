// Upstream: react-aria/test/utils/useFormReset.test.tsx @ 99e6102368
// Upstream: react-aria/test/checkbox/useCheckboxGroup.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/Checkbox.test.js @ 99e6102368
// Upstream: react-aria-components/test/RadioGroup.test.js @ 99e6102368
// Upstream: react-aria-components/test/Switch.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/radio/Radio.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions, css, xpath},
    polling::wait_for,
};

/// Checkboxes, checkbox groups, radio groups and switches in forms: form reset restores their
/// defaults (not when the reset is canceled), Enter submits their form, right-to-left arrow keys,
/// realtime re-validation of a checkbox group, and the `CheckboxField`/`SwitchField`/`RadioField`
/// atoms.
pub struct FormsTests {}

#[async_trait]
impl BrowserTest<str> for FormsTests {
    fn name(&self) -> Cow<'_, str> {
        "forms_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/forms").await?;
        cases!(
            form_reset(&page),
            canceled_form_reset(&page),
            implicit_submission_with_enter(&page),
            right_to_left_arrow_keys(&page),
            checkbox_group_realtime_validation(&page),
            field_atoms(&page),
        );
        Ok(())
    }
}

/// The `<label>` with the visible text `text`.
async fn label(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(css("label").text(text)).await
}

/// The native control inside `label`.
async fn input(label: &WebElement) -> Result<WebElement, Report> {
    label.element("input").await
}

/// Waits until the control of `label` is (un)checked, in the DOM and in its state
/// (`data-selected`).
async fn wait_for_checked(label: &WebElement, expected: bool) -> Result<(), Report> {
    label
        .wait_for_attr("data-selected", expected.then_some("true"))
        .await?;
    let text = label.inner_text().await?;
    assert_that!(input(label).await?.is_selected().await?)
        .with_detail_message(format!("checked: {text}"))
        .is_equal_to(expected);
    Ok(())
}

/// The values the form `#fm-reset` submits for each of its controls: Terms, Pets, Size, Wi-Fi.
async fn reset_form_values(page: &Page<'_>) -> Result<Vec<Vec<String>>, Report> {
    let form = page.element("#fm-reset").await?;
    let mut values = Vec::new();
    for name in ["terms", "pets", "size", "wifi"] {
        values.push(form.form_values(name).await?);
    }
    Ok(values)
}

/// "should call onReset on reset" (react-aria `useFormReset`), for each control.
async fn form_reset(page: &Page<'_>) -> Result<(), Report> {
    let initial = [vec![], vec!["cats"], vec!["m"], vec!["on"]];
    assert_that!(reset_form_values(page).await?).contains_exactly(&initial);

    // Change every control.
    for text in ["Terms", "Dogs", "Cats", "Small", "Wi-Fi"] {
        label(page, text).await?.click().await?;
    }
    wait_for_checked(&label(page, "Terms").await?, true).await?;
    wait_for_checked(&label(page, "Dogs").await?, true).await?;
    wait_for_checked(&label(page, "Cats").await?, false).await?;
    wait_for_checked(&label(page, "Small").await?, true).await?;
    wait_for_checked(&label(page, "Medium").await?, false).await?;
    wait_for_checked(&label(page, "Wi-Fi").await?, false).await?;
    assert_that!(reset_form_values(page).await?).contains_exactly([
        vec!["on"],
        vec!["dogs"],
        vec!["s"],
        vec![],
    ]);

    page.element("#fm-reset-button").await?.click().await?;
    wait_for_checked(&label(page, "Terms").await?, false).await?;
    wait_for_checked(&label(page, "Dogs").await?, false).await?;
    wait_for_checked(&label(page, "Cats").await?, true).await?;
    wait_for_checked(&label(page, "Small").await?, false).await?;
    wait_for_checked(&label(page, "Medium").await?, true).await?;
    wait_for_checked(&label(page, "Wi-Fi").await?, true).await?;
    assert_that!(reset_form_values(page).await?).contains_exactly(&initial);
    Ok(())
}

/// "should not call onReset if reset is cancelled" (react-aria `useFormReset`).
async fn canceled_form_reset(page: &Page<'_>) -> Result<(), Report> {
    let kept = label(page, "Kept").await?;
    let kept_switch = label(page, "Kept switch").await?;
    kept.click().await?;
    kept_switch.click().await?;
    wait_for_checked(&kept, true).await?;
    wait_for_checked(&kept_switch, true).await?;

    page.element("#fm-reset-canceled-button")
        .await?
        .click()
        .await?;
    // Nothing changes.
    kept.attr_stays("data-selected", Some("true")).await?;
    kept_switch
        .attr_stays("data-selected", Some("true"))
        .await?;
    wait_for_checked(&kept, true).await?;
    wait_for_checked(&kept_switch, true).await?;
    Ok(())
}

/// "should support implicit form submission from a focused checkbox/radio/switch on Enter".
async fn implicit_submission_with_enter(page: &Page<'_>) -> Result<(), Report> {
    for (submits, text) in [
        ("1", "Submit checkbox"),
        ("2", "Submit radio"),
        ("3", "Submit switch"),
    ] {
        // As upstream: click the control (focusing it), then press Enter.
        let label = label(page, text).await?;
        let input = input(&label).await?;
        label.click().await?;
        page.wait_for_focus(&input).await?;
        let before = input.is_selected().await?;
        page.send_keys(Key::Enter).await?;
        page.element("#fm-submits")
            .await?
            .wait_for_inner_text(submits)
            .await?;
        // Enter doesn't toggle.
        assert_that!(input.is_selected().await?)
            .with_detail_message(text)
            .is_equal_to(before);
    }
    Ok(())
}

/// Right to left: in a horizontal group ArrowRight selects the previous radio and ArrowLeft the
/// next; a vertical group keeps them (react-spectrum `Radio.test.js`, "rtl + horizontal" and
/// "rtl + vertical").
async fn right_to_left_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    for (group, right_selects, left_selects) in
        [("RTL horizontal", "C", "A"), ("RTL vertical", "B", "A")]
    {
        let first = label(page, &format!("{group} A")).await?;
        let first_input = input(&first).await?;
        first_input.focus().await?;
        page.wait_for_focus(&first_input).await?;

        page.send_keys(Key::Right).await?;
        let selected = label(page, &format!("{group} {right_selects}")).await?;
        wait_for_checked(&selected, true).await?;
        let selected_input = input(&selected).await?;
        page.wait_for_focus(&selected_input).await?;

        page.send_keys(Key::Left).await?;
        let selected = label(page, &format!("{group} {left_selects}")).await?;
        wait_for_checked(&selected, true).await?;
        let selected_input = input(&selected).await?;
        page.wait_for_focus(&selected_input).await?;
    }
    Ok(())
}

/// "should re-validate in realtime" (react-aria `useCheckboxGroup`): checking a checkbox of a
/// required group makes it valid, unchecking it invalid again.
async fn checkbox_group_realtime_validation(page: &Page<'_>) -> Result<(), Report> {
    let group = page
        .element(xpath(
            "//*[@role='group'][.//span[normalize-space(.)='Favorite pet']]",
        ))
        .await?;
    let dragons = label(page, "Realtime dragons").await?;

    dragons.click().await?;
    wait_for_checked(&dragons, true).await?;
    group.wait_for_attr("data-invalid", None).await?;

    dragons.click().await?;
    wait_for_checked(&dragons, false).await?;
    group.wait_for_attr("data-invalid", Some("true")).await?;

    dragons.click().await?;
    wait_for_checked(&dragons, true).await?;
    group.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// Waits until the input's description (the texts `aria-describedby` refers to) is `expected`.
async fn wait_for_description(input: &WebElement, expected: &str) -> Result<(), Report> {
    wait_for("the description")
        .observing(|| input.referenced_text("aria-describedby"))
        .to_be_equal_to(expected)
        .await?;
    Ok(())
}

/// react-aria-components' `CheckboxField` tests ("should render a checkbox with default class",
/// "should support DOM props", "supports help text", "should support required state"), and
/// `SwitchField`/`RadioField` with descriptions.
async fn field_atoms(page: &Page<'_>) -> Result<(), Report> {
    let checkbox = label(page, "Field checkbox").await?;
    let field = checkbox.parent().await?;
    assert_that!(field.tag_name().await?).is_equal_to("div");
    assert_that!(field.class_name().await?)
        .get_some()
        .contains("leptonic-CheckboxField");
    assert_that!(checkbox.class_name().await?)
        .get_some()
        .contains("leptonic-CheckboxButton");
    assert_that!(field.attr("data-foo").await?)
        .get_some()
        .is_equal_to("bar");
    assert_that!(field.attr("data-required").await?)
        .get_some()
        .is_equal_to("true");
    let checkbox_input = input(&checkbox).await?;
    wait_for_description(&checkbox_input, "Checkbox help").await?;

    let switch = label(page, "Field switch").await?;
    let switch_field = switch.parent().await?;
    assert_that!(switch_field.class_name().await?)
        .get_some()
        .contains("leptonic-SwitchField");
    let switch_input = input(&switch).await?;
    wait_for_description(&switch_input, "Switch help").await?;

    let radio = label(page, "Field radio A").await?;
    let radio_field = radio.parent().await?;
    assert_that!(radio_field.class_name().await?)
        .get_some()
        .contains("leptonic-RadioField");
    let radio_input = input(&radio).await?;
    wait_for_description(&radio_input, "Radio A help").await?;

    // In a checkbox group: the checkbox's description, and the group's.
    let in_group = input(&label(page, "Field group X").await?).await?;
    wait_for_description(&in_group, "X help").await?;

    // Required: submitting shows the errors, checking clears them.
    page.element("#fm-fields").await?.submit().await?;
    field.wait_for_attr("data-invalid", Some("true")).await?;
    switch_field
        .wait_for_attr("data-invalid", Some("true"))
        .await?;
    radio_field
        .wait_for_attr("data-invalid", Some("true"))
        .await?;
    // The help text, then the browser's validation message.
    for (input, help) in [
        (&checkbox_input, "Checkbox help"),
        (&switch_input, "Switch help"),
    ] {
        let description = input.referenced_text("aria-describedby").await?;
        assert_that!(&description).starts_with(format!("{help} "));
        assert_that!(description.len()).is_greater_than(help.len() + 1);
    }
    checkbox.click().await?;
    switch.click().await?;
    radio.click().await?;
    field.wait_for_attr("data-invalid", None).await?;
    switch_field.wait_for_attr("data-invalid", None).await?;
    radio_field.wait_for_attr("data-invalid", None).await?;
    wait_for_description(&checkbox_input, "Checkbox help").await?;
    wait_for_description(&switch_input, "Switch help").await?;
    Ok(())
}
