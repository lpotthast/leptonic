// Upstream: react-aria-components/test/NumberField.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/numberfield/NumberField.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions, SyntheticEvent},
    polling::wait_for,
};

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

        cases!(
            provides_slots(&page),
            hover_and_focus_visible_state(&page),
            read_only_state(&page),
            form_value(&page),
            validation_errors(&page),
            arrow_keys(&page),
            programmatic_clicks_on_steppers(&page),
            deleting_the_first_digit_before_a_group_separator(&page),
            typing_and_enter_commit(&page),
            no_grouping_characters_without_grouping(&page),
            no_grouping_characters_in_german(&page),
            scroll_wheel(&page),
            pasting_into_a_format(&page),
            rejected_values_keep_the_text(&page),
            server_errors_survive_an_unchanged_blur(&page),
            validate_commit_behavior(&page),
            validate_commit_behavior_and_enter_submit(&page),
            typed_values(&page),
        );

        Ok(())
    }
}

/// The `NumberField` in the fixture section matching `section`.
async fn field_in(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.element(format!("{section} .leptonic-NumberField"))
        .await
}

/// The visible input of the number field in `#container`.
async fn input(page: &Page<'_>, container: &str) -> Result<WebElement, Report> {
    page.element(format!("#{container} input:not([type=hidden])"))
        .await
}

/// Focuses `input` from the keyboard (focus visible): Shift+Tab to the element before it, Tab
/// back.
async fn tab_into(page: &Page<'_>, input: &WebElement) -> Result<(), Report> {
    input.focus().await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(input).await?;
    Ok(())
}

/// Selects the input's whole text and deletes it.
async fn clear(input: &WebElement) -> Result<(), Report> {
    input.send_keys(Key::Control + "a").await?;
    input.send_keys(Key::Backspace).await?;
    Ok(())
}

/// Pastes `text` over the input's whole text (a `paste` event with clipboard data, which
/// WebDriver can't send).
async fn paste(page: &Page<'_>, input: &WebElement, text: &str) -> Result<(), Report> {
    page.eval::<()>(
        "const [input, text] = arguments;
         const data = new DataTransfer();
         data.setData('text/plain', text);
         input.select();
         input.dispatchEvent(new ClipboardEvent('paste',
             {clipboardData: data, bubbles: true, cancelable: true}));",
        vec![input.to_json()?, text.into()],
    )
    .await
}

/// Wait until `input` is described by its validation error, the browser's validation message.
async fn wait_for_error(input: &WebElement) -> Result<(), Report> {
    wait_for("the input's validation message and description")
        .observing(|| async {
            let message = input.prop("validationMessage").await?.unwrap_or_default();
            Ok((message, input.referenced_text("aria-describedby").await?))
        })
        .to_be(
            "a message the input is described by",
            |(message, description)| !message.is_empty() && description == message,
        )
        .await?;
    Ok(())
}

/// "provides slots".
async fn provides_slots(page: &Page<'_>) -> Result<(), Report> {
    let group = page.element("#nf-slots [role=group]").await?;
    assert_that!(group.attr("role").await?)
        .get_some()
        .is_equal_to("group");
    assert_that!(field_in(page, "#nf-slots").await?.attr("data-foo").await?)
        .get_some()
        .is_equal_to("bar");
    let input = input(page, "nf-slots").await?;
    assert_that!(input.value().await?)
        .get_some()
        .is_equal_to("1,024");
    assert_that!(input.referenced_text("aria-labelledby").await?).is_equal_to("Width");
    wait_for("the description of the input")
        .observing(|| input.referenced_text("aria-describedby"))
        .to_be_equal_to("Description Error")
        .await?;
    let buttons = page.elements("#nf-slots button").await?;
    assert_that!(&buttons).has_length(2);
    let (decrease, increase) = (&buttons[0], &buttons[1]);
    assert_that!(decrease.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Decrease");
    assert_that!(increase.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Increase");
    // Named "Decrease Width": by itself and the label.
    let label_id = page
        .element("#nf-slots label")
        .await?
        .id()
        .await?
        .unwrap_or_default();
    let button_id = decrease.id().await?.unwrap_or_default();
    assert_that!(decrease.attr("aria-labelledby").await?)
        .get_some()
        .is_equal_to(format!("{button_id} {label_id}"));
    Ok(())
}

/// "should support hover state" / "should support focus visible state".
async fn hover_and_focus_visible_state(page: &Page<'_>) -> Result<(), Report> {
    let group = page.element("#nf-keys [role=group]").await?;
    group.hover().await?;
    group.wait_for_attr("data-hovered", Some("true")).await?;
    let heading = page.element("h1").await?;
    heading.hover().await?;
    group.wait_for_attr("data-hovered", None).await?;

    let input = input(page, "nf-keys").await?;
    tab_into(page, &input).await?;
    group
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.send_keys(Key::Tab).await?;
    group.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

async fn read_only_state(page: &Page<'_>) -> Result<(), Report> {
    let plain = field_in(page, "#nf-slots").await?;
    assert_that!(plain.attr("data-readonly").await?).is_none();
    let read_only = field_in(page, "#nf-read-only").await?;
    assert_that!(read_only.attr("data-readonly").await?).is_some();
    Ok(())
}

/// "should support form value" / "should support disabled when having a form value".
async fn form_value(page: &Page<'_>) -> Result<(), Report> {
    let hidden = page.element("#nf-form-value input[name=test]").await?;
    assert_that!(hidden.value().await?)
        .get_some()
        .is_equal_to("25");
    assert_that!(hidden.attr("form").await?)
        .get_some()
        .is_equal_to("test");
    assert_that!(input(page, "nf-form-value").await?.value().await?)
        .get_some()
        .is_equal_to("$25.00");
    let disabled = page
        .element("#nf-form-value-disabled input[name=test]")
        .await?;
    assert_that!(disabled.attr("disabled").await?).is_some();
    Ok(())
}

/// "supports validation errors".
async fn validation_errors(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-native").await?;
    let field = field_in(page, "#nf-native").await?;
    assert_that!(input.attr("required").await?).is_some();
    assert_that!(input.attr("aria-required").await?).is_none();
    assert_that!(input.attr("aria-describedby").await?).is_none();
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(field.attr("data-invalid").await?).is_none();

    assert_that!(page.element("#nf-native").await?.check_validity().await?).is_false();
    wait_for_error(&input).await?;
    field.wait_for_attr("data-invalid", Some("true")).await?;
    page.wait_for_focus(&input).await?;

    // The error stays while typing, and goes once the value is committed.
    page.send_keys("3").await?;
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

/// "should support repeat keydown events when holding an arrow key".
async fn arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-keys").await?;
    tab_into(page, &input).await?;
    for _ in 0..3 {
        page.send_keys(Key::Up).await?;
    }
    input.wait_for_prop("value", "1,027").await?;
    page.element("#nf-changes")
        .await?
        .wait_for_inner_text("1025 1026 1027")
        .await?;
    for _ in 0..3 {
        page.send_keys(Key::Down).await?;
    }
    input.wait_for_prop("value", "1,024").await?;
    Ok(())
}

/// "should trigger onChange via programmatic click() on stepper buttons".
async fn programmatic_clicks_on_steppers(page: &Page<'_>) -> Result<(), Report> {
    let decrease = page.element("#nf-keys button[aria-label=Decrease]").await?;
    let increase = page.element("#nf-keys button[aria-label=Increase]").await?;
    let changes = page.element("#nf-changes").await?;
    let before = changes.inner_text().await?;
    increase.virtual_click().await?;
    changes
        .wait_for_inner_text(&format!("{before} 1025"))
        .await?;
    decrease.virtual_click().await?;
    changes
        .wait_for_inner_text(&format!("{before} 1025 1024"))
        .await?;
    Ok(())
}

/// "should allow you to delete the first digit in a number if it is followed by a group
/// separator".
async fn deleting_the_first_digit_before_a_group_separator(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-keys").await?;
    tab_into(page, &input).await?;
    // The caret after the first digit.
    page.eval::<()>(
        "arguments[0].setSelectionRange(1, 1);",
        vec![input.to_json()?],
    )
    .await?;
    page.send_keys(Key::Backspace).await?;
    input.wait_for_prop("value", ",024").await?;
    page.send_keys(Key::Enter).await?;
    input.wait_for_prop("value", "24").await?;
    Ok(())
}

/// "supports onChange".
async fn typing_and_enter_commit(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-keys").await?;
    tab_into(page, &input).await?;
    clear(&input).await?;
    page.send_keys("1024").await?;
    page.send_keys(Key::Enter).await?;
    input.wait_for_prop("value", "1,024").await?;
    let changes = page.element("#nf-changes").await?;
    wait_for("the last change")
        .observing(|| async {
            Ok(changes
                .inner_text()
                .await?
                .split_whitespace()
                .last()
                .map(str::to_owned))
        })
        .to_be_equal_to(Some("1024".to_owned()))
        .await?;
    Ok(())
}

/// "should not type the grouping characters when useGrouping is false".
async fn no_grouping_characters_without_grouping(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-no-grouping").await?;
    tab_into(page, &input).await?;
    page.send_keys("102,4").await?;
    input.wait_for_prop("value", "1024").await?;
    clear(&input).await?;
    paste(page, &input, "1,024").await?;
    page.send_keys(Key::Tab).await?;
    // Unparsable pasted text keeps the previous (empty) value.
    input.wait_for_prop("value", "").await?;
    Ok(())
}

/// "should not type the grouping characters when useGrouping is false and in German locale".
async fn no_grouping_characters_in_german(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-no-grouping-de").await?;
    tab_into(page, &input).await?;
    page.send_keys("102.4").await?;
    input.wait_for_prop("value", "1024").await?;
    clear(&input).await?;
    paste(page, &input, "1.024").await?;
    page.send_keys(Key::Tab).await?;
    // Unparsable pasted text keeps the previous (empty) value.
    input.wait_for_prop("value", "").await?;
    Ok(())
}

/// React Spectrum's scroll wheel tests: "cannot scroll to step when not focused", "increment
/// value when scrolling upwards", "decrement value when scrolling downwards", "should not fire
/// increment or decrement if it is a zoom event".
async fn scroll_wheel(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-wheel").await?;
    let changes = page.element("#nf-wheel-changes").await?;
    let up = || SyntheticEvent::wheel().with("deltaY", -10.0);
    let down = || SyntheticEvent::wheel().with("deltaY", 10.0);
    page.blur_focused().await?;
    input.dispatch(up()).await?;
    changes.inner_text_stays("").await?;

    tab_into(page, &input).await?;
    input.dispatch(up()).await?;
    changes.wait_for_inner_text("-1").await?;
    input.wait_for_prop("value", "-1").await?;
    input.dispatch(down()).await?;
    changes.wait_for_inner_text("-1 0").await?;
    // A zoom (the Ctrl key: a pinch): no step.
    input.dispatch(down().with("ctrlKey", true)).await?;
    input.dispatch(up().with("ctrlKey", true)).await?;
    changes.inner_text_stays("-1 0").await?;
    // The wheel doesn't blur the input.
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// "should support pasting into a format".
async fn pasting_into_a_format(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-currency").await?;
    tab_into(page, &input).await?;
    clear(&input).await?;
    paste(page, &input, "1,024").await?;
    input.wait_for_prop("value", "$1,024.00").await?;
    Ok(())
}

/// "should not change the input value if the new value is not accepted".
async fn rejected_values_keep_the_text(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-rejecting").await?;
    tab_into(page, &input).await?;
    clear(&input).await?;
    paste(page, &input, "1024").await?;
    input.wait_for_prop("value", "200").await?;
    page.send_keys(Key::Enter).await?;
    input.prop_stays("value", "200").await?;
    Ok(())
}

/// "should not reset validation errors on blur when value has not changed".
async fn server_errors_survive_an_unchanged_blur(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-server-form").await?;
    let field = field_in(page, "#nf-server-form").await?;
    field.wait_for_attr("data-invalid", Some("true")).await?;
    wait_for("the description of the input")
        .observing(|| input.referenced_text("aria-describedby"))
        .to_be_equal_to("This field has an error.")
        .await?;
    input.focus().await?;
    page.blur_focused().await?;
    field.attr_stays("data-invalid", Some("true")).await?;
    assert_that!(input.referenced_text("aria-describedby").await?)
        .is_equal_to("This field has an error.");
    Ok(())
}

/// "should not change the edited input value when value snapping is disabled".
async fn validate_commit_behavior(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-validate").await?;
    assert_that!(input.is_valid().await?).is_true();

    // Over max.
    tab_into(page, &input).await?;
    clear(&input).await?;
    page.send_keys("1024").await?;
    page.send_keys(Key::Tab).await?;
    input.wait_for_prop("value", "1,024").await?;
    field_in(page, "#nf-validate")
        .await?
        .wait_for_attr("data-invalid", Some("true"))
        .await?;
    wait_for_error(&input).await?;
    assert_that!(input.attr("aria-invalid").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(page.element("#nf-validate").await?.check_validity().await?).is_false();
    page.wait_for_focus(&input).await?;

    for (typed, valid) in [("30", true), ("2", false), ("31", false)] {
        clear(&input).await?;
        page.send_keys(typed).await?;
        page.send_keys(Key::Tab).await?;
        input.wait_for_prop("value", typed).await?;
        if valid {
            input.wait_for_attr("aria-describedby", None).await?;
        } else {
            wait_for_error(&input).await?;
        }
        assert_that!(input.is_valid().await?)
            .with_detail_message(format!("validity of {typed}"))
            .is_equal_to(valid);
        tab_into(page, &input).await?;
    }

    // Required.
    clear(&input).await?;
    page.send_keys(Key::Tab).await?;
    input.wait_for_prop("value", "").await?;
    assert_that!(input.is_valid().await?).is_false();
    wait_for_error(&input).await?;
    Ok(())
}

/// No upstream test: with `CommitBehavior::Validate` and native validation, Enter doesn't submit
/// an out-of-range value, and submits once it is corrected (the range's custom validity is
/// cleared).
async fn validate_commit_behavior_and_enter_submit(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page, "nf-validate-submit").await?;
    tab_into(page, &input).await?;
    clear(&input).await?;
    page.send_keys("1024").await?;
    page.send_keys(Key::Enter).await?;
    wait_for_error(&input).await?;
    assert_that!(input.is_valid().await?).is_false();
    page.element("#nf-validate-submits")
        .await?
        .inner_text_stays("0")
        .await?;

    clear(&input).await?;
    page.send_keys("30").await?;
    page.send_keys(Key::Enter).await?;
    page.element("#nf-validate-submits")
        .await?
        .wait_for_inner_text("1")
        .await?;
    assert_that!(input.is_valid().await?).is_true();
    input.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

/// Integers beyond `f64` stay exact; unsigned fields reject a minus sign.
async fn typed_values(page: &Page<'_>) -> Result<(), Report> {
    let big = input(page, "nf-u64").await?;
    let increment = page.element("#nf-u64 button[aria-label=Increase]").await?;
    increment.click().await?;
    big.wait_for_prop("value", "18,446,744,073,709,551,615")
        .await?;
    // A boolean attribute reads "true" while present.
    increment.wait_for_attr("disabled", Some("true")).await?;

    let unsigned = input(page, "nf-u8").await?;
    tab_into(page, &unsigned).await?;
    page.send_keys("-5").await?;
    unsigned.wait_for_prop("value", "5").await?;

    // A value beyond the type's range clamps to it, and stepping goes on from there.
    clear(&unsigned).await?;
    page.send_keys("300").await?;
    page.send_keys(Key::Enter).await?;
    unsigned.wait_for_prop("value", "255").await?;
    clear(&unsigned).await?;
    page.send_keys("1000").await?;
    page.send_keys(Key::Up).await?;
    unsigned.wait_for_prop("value", "255").await?;
    Ok(())
}
