// Upstream: react-aria/test/utils/useFormReset.test.tsx @ 99e6102368
// Upstream: react-aria/test/checkbox/useCheckboxGroup.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/Checkbox.test.js @ 99e6102368
// Upstream: react-aria-components/test/RadioGroup.test.js @ 99e6102368
// Upstream: react-aria-components/test/Switch.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/radio/Radio.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

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

        form_reset(&page).await?;
        canceled_form_reset(&page).await?;
        implicit_submission_with_enter(&page).await?;
        right_to_left_arrow_keys(&page).await?;
        checkbox_group_realtime_validation(&page).await?;
        field_atoms(&page).await?;

        Ok(())
    }
}

/// The `<label>` with the visible text `text`.
async fn label(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    Ok(page
        .driver
        .find(By::XPath(format!("//label[normalize-space(.)='{text}']")))
        .await?)
}

async fn input(label: &WebElement) -> Result<WebElement, Report> {
    Ok(label.find(By::Css("input")).await?)
}

async fn checked(label: &WebElement) -> Result<bool, Report> {
    Ok(input(label).await?.prop("checked").await?.as_deref() == Some("true"))
}

/// Waits until the control of `label` is (un)checked, in the DOM and in its state
/// (`data-selected`).
async fn wait_for_checked(
    page: &Page<'_>,
    label: &WebElement,
    expected: bool,
) -> Result<(), Report> {
    page.wait_for_attr(label, "data-selected", expected.then_some("true"))
        .await?;
    assert_that!(checked(label).await?)
        .with_detail_message(format!("checked: {}", label.text().await?))
        .is_equal_to(expected);
    Ok(())
}

/// The form's data as `name=value` pairs joined by `&`.
async fn form_data(page: &Page<'_>, form: &str) -> Result<String, Report> {
    let data = page
        .driver
        .execute(
            "return new URLSearchParams(new FormData(document.getElementById(arguments[0]))) \
                 .toString();",
            vec![serde_json::Value::from(form)],
        )
        .await?;
    Ok(data.json().as_str().unwrap_or_default().to_owned())
}

/// "should call onReset on reset" (react-aria `useFormReset`), for each control.
async fn form_reset(page: &Page<'_>) -> Result<(), Report> {
    let initial = "pets=cats&size=m&wifi=on";
    assert_that!(form_data(page, "fm-reset").await?).is_equal_to(initial.to_owned());

    // Change every control.
    for text in ["Terms", "Dogs", "Cats", "Small", "Wi-Fi"] {
        label(page, text).await?.click().await?;
    }
    wait_for_checked(page, &label(page, "Terms").await?, true).await?;
    wait_for_checked(page, &label(page, "Dogs").await?, true).await?;
    wait_for_checked(page, &label(page, "Cats").await?, false).await?;
    wait_for_checked(page, &label(page, "Small").await?, true).await?;
    wait_for_checked(page, &label(page, "Medium").await?, false).await?;
    wait_for_checked(page, &label(page, "Wi-Fi").await?, false).await?;
    assert_that!(form_data(page, "fm-reset").await?)
        .is_equal_to("terms=on&pets=dogs&size=s".to_owned());

    page.click_element_with_id("fm-reset-button").await?;
    wait_for_checked(page, &label(page, "Terms").await?, false).await?;
    wait_for_checked(page, &label(page, "Dogs").await?, false).await?;
    wait_for_checked(page, &label(page, "Cats").await?, true).await?;
    wait_for_checked(page, &label(page, "Small").await?, false).await?;
    wait_for_checked(page, &label(page, "Medium").await?, true).await?;
    wait_for_checked(page, &label(page, "Wi-Fi").await?, true).await?;
    assert_that!(form_data(page, "fm-reset").await?).is_equal_to(initial.to_owned());
    Ok(())
}

/// "should not call onReset if reset is cancelled" (react-aria `useFormReset`).
async fn canceled_form_reset(page: &Page<'_>) -> Result<(), Report> {
    let kept = label(page, "Kept").await?;
    let kept_switch = label(page, "Kept switch").await?;
    kept.click().await?;
    kept_switch.click().await?;
    wait_for_checked(page, &kept, true).await?;
    wait_for_checked(page, &kept_switch, true).await?;

    page.click_element_with_id("fm-reset-canceled-button")
        .await?;
    // Settle, then check that nothing changed.
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    wait_for_checked(page, &kept, true).await?;
    wait_for_checked(page, &kept_switch, true).await
}

/// "should support implicit form submission from a focused checkbox/radio/switch on Enter".
async fn implicit_submission_with_enter(page: &Page<'_>) -> Result<(), Report> {
    for (submits, text) in [
        ("1", "Submit checkbox"),
        ("2", "Submit radio"),
        ("3", "Submit switch"),
    ] {
        let input = input(&label(page, text).await?).await?;
        input.focus().await?;
        page.wait_for_focus_on(&input, text).await?;
        page.send_keys_to_active(Key::Enter).await?;
        page.wait_for_text("fm-submits", submits).await?;
        // Enter doesn't toggle.
        assert_that!(input.prop("checked").await?.as_deref()).is_equal_to(Some("false"));
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
        page.wait_for_focus_on(&first_input, group).await?;

        page.send_keys_to_active(Key::Right).await?;
        let selected = label(page, &format!("{group} {right_selects}")).await?;
        wait_for_checked(page, &selected, true).await?;
        page.wait_for_focus_on(&input(&selected).await?, "the radio ArrowRight selected")
            .await?;

        page.send_keys_to_active(Key::Left).await?;
        let selected = label(page, &format!("{group} {left_selects}")).await?;
        wait_for_checked(page, &selected, true).await?;
        page.wait_for_focus_on(&input(&selected).await?, "the radio ArrowLeft selected")
            .await?;
    }
    Ok(())
}

/// "should re-validate in realtime" (react-aria `useCheckboxGroup`): checking a checkbox of a
/// required group makes it valid, unchecking it invalid again.
async fn checkbox_group_realtime_validation(page: &Page<'_>) -> Result<(), Report> {
    let group = page
        .driver
        .find(By::XPath(
            "//*[@role='group'][.//span[normalize-space(.)='Favorite pet']]",
        ))
        .await?;
    let dragons = label(page, "Realtime dragons").await?;

    dragons.click().await?;
    wait_for_checked(page, &dragons, true).await?;
    page.wait_for_attr(&group, "data-invalid", None).await?;

    dragons.click().await?;
    wait_for_checked(page, &dragons, false).await?;
    page.wait_for_attr(&group, "data-invalid", Some("true"))
        .await?;

    dragons.click().await?;
    wait_for_checked(page, &dragons, true).await?;
    page.wait_for_attr(&group, "data-invalid", None).await
}

/// The texts of the elements the input's `aria-describedby` refers to.
async fn descriptions(page: &Page<'_>, input: &WebElement) -> Result<Vec<String>, Report> {
    let texts = page
        .driver
        .execute(
            "return (arguments[0].getAttribute('aria-describedby') ?? '').split(' ') \
                 .filter(Boolean).map(id => document.getElementById(id)?.textContent ?? '?');",
            vec![input.to_json()?],
        )
        .await?;
    Ok(texts.convert()?)
}

/// Waits until the input is described by `expected` (in this order).
async fn wait_for_descriptions(
    page: &Page<'_>,
    input: &WebElement,
    expected: &[&str],
) -> Result<(), Report> {
    for _ in 0..50 {
        if descriptions(page, input).await? == expected {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    assert_that!(descriptions(page, input).await?).is_equal_to(
        expected
            .iter()
            .map(|text| (*text).to_owned())
            .collect::<Vec<_>>(),
    );
    Ok(())
}

/// react-aria-components' `CheckboxField` tests ("should render a checkbox with default class",
/// "should support DOM props", "supports help text", "should support required state"), and
/// `SwitchField`/`RadioField` with descriptions.
async fn field_atoms(page: &Page<'_>) -> Result<(), Report> {
    let checkbox = label(page, "Field checkbox").await?;
    let field = checkbox.find(By::XPath("..")).await?;
    assert_that!(field.tag_name().await?).is_equal_to("div".to_owned());
    assert_that!(field.attr("class").await?.unwrap_or_default()).contains("leptonic-CheckboxField");
    assert_that!(checkbox.attr("class").await?.unwrap_or_default())
        .contains("leptonic-CheckboxButton");
    assert_that!(field.attr("data-foo").await?).is_equal_to(Some("bar".to_owned()));
    assert_that!(field.attr("data-required").await?).is_equal_to(Some("true".to_owned()));
    let checkbox_input = input(&checkbox).await?;
    wait_for_descriptions(page, &checkbox_input, &["Checkbox help"]).await?;

    let switch = label(page, "Field switch").await?;
    let switch_field = switch.find(By::XPath("..")).await?;
    assert_that!(switch_field.attr("class").await?.unwrap_or_default())
        .contains("leptonic-SwitchField");
    let switch_input = input(&switch).await?;
    wait_for_descriptions(page, &switch_input, &["Switch help"]).await?;

    let radio = label(page, "Field radio A").await?;
    let radio_field = radio.find(By::XPath("..")).await?;
    assert_that!(radio_field.attr("class").await?.unwrap_or_default())
        .contains("leptonic-RadioField");
    wait_for_descriptions(page, &input(&radio).await?, &["Radio A help"]).await?;

    // In a checkbox group: the checkbox's description, and the group's.
    let in_group = label(page, "Field group X").await?;
    wait_for_descriptions(page, &input(&in_group).await?, &["X help"]).await?;

    // Required: submitting shows the errors, checking clears them.
    page.driver
        .execute(
            "document.getElementById('fm-fields').requestSubmit()",
            Vec::new(),
        )
        .await?;
    page.wait_for_attr(&field, "data-invalid", Some("true"))
        .await?;
    page.wait_for_attr(&switch_field, "data-invalid", Some("true"))
        .await?;
    page.wait_for_attr(&radio_field, "data-invalid", Some("true"))
        .await?;
    for input in [&checkbox_input, &switch_input] {
        let texts = descriptions(page, input).await?;
        assert_that!(texts.len()).is_equal_to(2);
        assert_that!(texts[1].is_empty()).is_false();
    }
    checkbox.click().await?;
    switch.click().await?;
    radio.click().await?;
    page.wait_for_attr(&field, "data-invalid", None).await?;
    page.wait_for_attr(&switch_field, "data-invalid", None)
        .await?;
    page.wait_for_attr(&radio_field, "data-invalid", None)
        .await?;
    wait_for_descriptions(page, &checkbox_input, &["Checkbox help"]).await?;
    wait_for_descriptions(page, &switch_input, &["Switch help"]).await
}
