// Upstream: react-aria-components/test/NumberField.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use super::test_text_field_atom::{
    check_validity, field_of, is_valid, referenced_texts, wait_for_referenced_texts,
};
use crate::pages::{BaseActions, Page};

/// The NumberField atoms: slots, states, form value, validation, keyboard, typing, paste,
/// commit behavior and typed values.
pub struct NumberFieldAtomTests {}

#[async_trait]
impl BrowserTest<str> for NumberFieldAtomTests {
    fn name(&self) -> Cow<'_, str> {
        "number_field_atom_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/number-field").await?;

        provides_slots(&page).await?;
        hover_and_focus_visible_state(&page).await?;
        read_only_state(&page).await?;
        form_value(&page).await?;
        validation_errors(&page).await?;
        arrow_keys(&page).await?;
        programmatic_clicks_on_steppers(&page).await?;
        deleting_the_first_digit_before_a_group_separator(&page).await?;
        typing_and_enter_commit(&page).await?;
        no_grouping_characters_without_grouping(&page).await?;
        pasting_into_a_format(&page).await?;
        rejected_values_keep_the_text(&page).await?;
        server_errors_survive_an_unchanged_blur(&page).await?;
        validate_commit_behavior(&page).await?;
        typed_values(&page).await?;

        Ok(())
    }
}

async fn input(page: &Page<'_>, container: &str) -> Result<WebElement, Report> {
    page.css(&format!("#{container} input:not([type=hidden])"))
        .await
}

async fn value(input: &WebElement) -> Result<String, Report> {
    Ok(input.prop("value").await?.unwrap_or_default())
}

async fn wait_for_value(page: &Page<'_>, input: &WebElement, expected: &str) -> Result<(), Report> {
    for _ in 0..50 {
        if value(input).await? == expected {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    assert_that!(value(input).await?).is_equal_to(expected.to_owned());
    let _ = page;
    Ok(())
}

/// Focuses `input` from the keyboard: Tab from the enabled element before it (or the heading).
async fn tab_into(page: &Page<'_>, input: &WebElement) -> Result<(), Report> {
    page.driver
        .execute(
            "arguments[0].focus(); arguments[0].blur(); \
             document.querySelector('h1').setAttribute('tabindex', '-1');",
            vec![input.to_json()?],
        )
        .await?;
    page.driver
        .execute(
            "let all = [...document.querySelectorAll('input:not([type=hidden]), button')] \
                 .filter(el => !el.disabled || el === arguments[0]); \
             let i = all.indexOf(arguments[0]); \
             (i > 0 ? all[i - 1] : document.querySelector('h1')).focus();",
            vec![input.to_json()?],
        )
        .await?;
    page.press_tab().await?;
    page.wait_for_focus_on(input, "the input (focus check 1)")
        .await
}

async fn clear(page: &Page<'_>, input: &WebElement) -> Result<(), Report> {
    page.driver
        .execute("arguments[0].select();", vec![input.to_json()?])
        .await?;
    input.send_keys(Key::Backspace).await?;
    Ok(())
}

async fn paste(page: &Page<'_>, input: &WebElement, text: &str) -> Result<(), Report> {
    page.driver
        .execute(
            "let data = new DataTransfer(); data.setData('text/plain', arguments[1]); \
             arguments[0].select(); \
             arguments[0].dispatchEvent(new ClipboardEvent('paste', \
                 {clipboardData: data, bubbles: true, cancelable: true}));",
            vec![input.to_json()?, serde_json::Value::from(text)],
        )
        .await?;
    Ok(())
}

/// "provides slots".
async fn provides_slots(page: &Page<'_>) -> Result<(), Report> {
    let group = page.css("#nf-slots [role=group]").await?;
    assert_that!(group.attr("role").await?).is_equal_to(Some("group".to_owned()));
    assert_that!(field_of(page, "#nf-slots").await?.attr("data-foo").await?)
        .is_equal_to(Some("bar".to_owned()));
    let input = input(page, "nf-slots").await?;
    assert_that!(value(&input).await?).is_equal_to("1,024".to_owned());
    assert_that!(referenced_texts(page, &input, "aria-labelledby").await?)
        .is_equal_to("Width".to_owned());
    wait_for_referenced_texts(page, &input, "aria-describedby", "Description Error").await?;
    let buttons = page.driver.find_all(By::Css("#nf-slots button")).await?;
    assert_that!(buttons[0].attr("aria-label").await?).is_equal_to(Some("Decrease".to_owned()));
    assert_that!(buttons[1].attr("aria-label").await?).is_equal_to(Some("Increase".to_owned()));
    // Named "Decrease Width": by itself and the label.
    let label_id = page
        .css("#nf-slots label")
        .await?
        .attr("id")
        .await?
        .unwrap_or_default();
    let button_id = buttons[0].attr("id").await?.unwrap_or_default();
    assert_that!(buttons[0].attr("aria-labelledby").await?)
        .is_equal_to(Some(format!("{button_id} {label_id}")));
    Ok(())
}

/// "should support hover state" / "should support focus visible state".
async fn hover_and_focus_visible_state(page: &Page<'_>) -> Result<(), Report> {
    let group = page.css("#nf-keys [role=group]").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&group)
        .perform()
        .await?;
    page.wait_for_selector("#nf-keys [role=group][data-hovered]")
        .await?;
    let heading = page.driver.find(By::Css("h1")).await?;
    page.driver
        .action_chain()
        .move_to_element_center(&heading)
        .perform()
        .await?;
    page.wait_for_no_selector("#nf-keys [role=group][data-hovered]")
        .await?;

    let input = input(page, "nf-keys").await?;
    tab_into(page, &input).await?;
    page.wait_for_selector("#nf-keys [role=group][data-focus-visible]")
        .await?;
    page.press_tab().await?;
    page.wait_for_no_selector("#nf-keys [role=group][data-focus-visible]")
        .await
}

async fn read_only_state(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(
        field_of(page, "#nf-slots")
            .await?
            .attr("data-readonly")
            .await?
    )
    .is_none();
    assert_that!(
        field_of(page, "#nf-read-only")
            .await?
            .attr("data-readonly")
            .await?
    )
    .is_some();
    Ok(())
}

/// "should support form value" / "should support disabled when having a form value".
async fn form_value(page: &Page<'_>) -> Result<(), Report> {
    let hidden = page.css("#nf-form-value input[name=test]").await?;
    assert_that!(hidden.prop("value").await?).is_equal_to(Some("25".to_owned()));
    assert_that!(hidden.attr("form").await?).is_equal_to(Some("test".to_owned()));
    assert_that!(value(&input(page, "nf-form-value").await?).await?)
        .is_equal_to("$25.00".to_owned());
    let disabled = page.css("#nf-form-value-disabled input[name=test]").await?;
    assert_that!(disabled.attr("disabled").await?).is_some();
    Ok(())
}

/// "supports validation errors".
async fn validation_errors(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-native").await?;
    assert_that!(input.attr("required").await?).is_some();
    assert_that!(input.attr("aria-required").await?).is_none();
    assert_that!(input.attr("aria-describedby").await?).is_none();
    assert_that!(is_valid(page, &input).await?).is_false();
    assert_that!(
        field_of(page, "#nf-native")
            .await?
            .attr("data-invalid")
            .await?
    )
    .is_none();

    check_validity(page, "nf-native").await?;
    page.wait_for_selector("#nf-native input[aria-describedby]")
        .await?;
    assert_that!(referenced_texts(page, &input, "aria-describedby").await?).is_not_empty();
    assert_that!(
        field_of(page, "#nf-native")
            .await?
            .attr("data-invalid")
            .await?
    )
    .is_some();
    page.wait_for_focus_on(&input, "the input (focus check 2)")
        .await?;

    page.send_keys_to_active("3").await?;
    assert_that!(input.attr("aria-describedby").await?).is_some();
    assert_that!(is_valid(page, &input).await?).is_true();
    page.press_tab().await?;
    page.wait_for_no_selector("#nf-native input[aria-describedby]")
        .await?;
    page.wait_for_no_selector("#nf-native [data-invalid]").await
}

/// "should support repeat keydown events when holding an arrow key".
async fn arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-keys").await?;
    tab_into(page, &input).await?;
    for _ in 0..3 {
        page.send_keys_to_active(Key::Up).await?;
    }
    wait_for_value(page, &input, "1,027").await?;
    page.wait_for_text("nf-changes", "1025 1026 1027").await?;
    for _ in 0..3 {
        page.send_keys_to_active(Key::Down).await?;
    }
    wait_for_value(page, &input, "1,024").await
}

/// "should trigger onChange via programmatic click() on stepper buttons".
async fn programmatic_clicks_on_steppers(page: &Page<'_>) -> Result<(), Report> {
    let buttons = page.driver.find_all(By::Css("#nf-keys button")).await?;
    let before = page.read_text_of("nf-changes").await?;
    page.driver
        .execute("arguments[0].click();", vec![buttons[1].to_json()?])
        .await?;
    page.wait_for_text("nf-changes", &format!("{before} 1025"))
        .await?;
    page.driver
        .execute("arguments[0].click();", vec![buttons[0].to_json()?])
        .await?;
    page.wait_for_text("nf-changes", &format!("{before} 1025 1024"))
        .await
}

/// "should allow you to delete the first digit in a number if it is followed by a group
/// separator".
async fn deleting_the_first_digit_before_a_group_separator(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-keys").await?;
    tab_into(page, &input).await?;
    page.driver
        .execute(
            "arguments[0].setSelectionRange(1, 1);",
            vec![input.to_json()?],
        )
        .await?;
    page.send_keys_to_active(Key::Backspace).await?;
    wait_for_value(page, &input, ",024").await?;
    page.send_keys_to_active(Key::Enter).await?;
    wait_for_value(page, &input, "24").await
}

/// "supports onChange".
async fn typing_and_enter_commit(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-keys").await?;
    tab_into(page, &input).await?;
    clear(page, &input).await?;
    page.send_keys_to_active("1024").await?;
    page.send_keys_to_active(Key::Enter).await?;
    wait_for_value(page, &input, "1,024").await?;
    let changes = page.read_text_of("nf-changes").await?;
    assert_that!(changes.ends_with("1024")).is_true();
    Ok(())
}

/// "should not type the grouping characters when useGrouping is false".
async fn no_grouping_characters_without_grouping(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-no-grouping").await?;
    tab_into(page, &input).await?;
    page.send_keys_to_active("102,4").await?;
    wait_for_value(page, &input, "1024").await?;
    clear(page, &input).await?;
    paste(page, &input, "1,024").await?;
    page.press_tab().await?;
    // Unparsable pasted text keeps the previous (empty) value.
    wait_for_value(page, &input, "").await
}

/// "should support pasting into a format".
async fn pasting_into_a_format(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-currency").await?;
    tab_into(page, &input).await?;
    clear(page, &input).await?;
    paste(page, &input, "1,024").await?;
    wait_for_value(page, &input, "$1,024.00").await
}

/// "should not change the input value if the new value is not accepted".
async fn rejected_values_keep_the_text(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-rejecting").await?;
    tab_into(page, &input).await?;
    clear(page, &input).await?;
    paste(page, &input, "1024").await?;
    wait_for_value(page, &input, "200").await?;
    page.send_keys_to_active(Key::Enter).await?;
    wait_for_value(page, &input, "200").await
}

/// "should not reset validation errors on blur when value has not changed".
async fn server_errors_survive_an_unchanged_blur(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-server-form").await?;
    page.wait_for_selector("#nf-server-form > div[data-invalid]")
        .await?;
    wait_for_referenced_texts(page, &input, "aria-describedby", "This field has an error.").await?;
    page.driver
        .execute(
            "arguments[0].focus(); arguments[0].blur();",
            vec![input.to_json()?],
        )
        .await?;
    assert_that!(
        field_of(page, "#nf-server-form")
            .await?
            .attr("data-invalid")
            .await?
    )
    .is_some();
    assert_that!(referenced_texts(page, &input, "aria-describedby").await?)
        .is_equal_to("This field has an error.".to_owned());
    Ok(())
}

/// "should not change the edited input value when value snapping is disabled".
async fn validate_commit_behavior(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-validate").await?;
    assert_that!(is_valid(page, &input).await?).is_true();

    // Over max.
    tab_into(page, &input).await?;
    clear(page, &input).await?;
    page.send_keys_to_active("1024").await?;
    page.press_tab().await?;
    wait_for_value(page, &input, "1,024").await?;
    page.wait_for_selector("#nf-validate > div[data-invalid]")
        .await?;
    assert_that!(input.attr("aria-invalid").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(is_valid(page, &input).await?).is_false();
    assert_that!(referenced_texts(page, &input, "aria-describedby").await?).is_not_empty();
    check_validity(page, "nf-validate").await?;
    page.wait_for_focus_on(&input, "the input (focus check 3)")
        .await?;

    for (typed, valid) in [("30", true), ("2", false), ("31", false)] {
        clear(page, &input).await?;
        page.send_keys_to_active(typed).await?;
        page.press_tab().await?;
        wait_for_value(page, &input, typed).await?;
        if valid {
            page.wait_for_no_selector("#nf-validate input[aria-describedby]")
                .await?;
        } else {
            page.wait_for_selector("#nf-validate input[aria-describedby]")
                .await?;
        }
        assert_that!(is_valid(page, &input).await?)
            .with_detail_message(format!("validity of {typed}"))
            .is_equal_to(valid);
        tab_into(page, &input).await?;
    }

    // Required.
    clear(page, &input).await?;
    page.press_tab().await?;
    wait_for_value(page, &input, "").await?;
    assert_that!(is_valid(page, &input).await?).is_false();
    page.wait_for_selector("#nf-validate input[aria-describedby]")
        .await
}

/// Integers beyond `f64` stay exact; unsigned fields reject a minus sign.
async fn typed_values(page: &Page<'_>) -> Result<(), Report> {
    let big = input(page, "nf-u64").await?;
    let increment = page.css("#nf-u64 button[aria-label=Increase]").await?;
    increment.click().await?;
    wait_for_value(page, &big, "18,446,744,073,709,551,615").await?;
    page.wait_for_selector("#nf-u64 button[aria-label=Increase][disabled]")
        .await?;

    let unsigned = input(page, "nf-u8").await?;
    tab_into(page, &unsigned).await?;
    page.send_keys_to_active("-5").await?;
    wait_for_value(page, &unsigned, "5").await
}
