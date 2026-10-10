// Upstream: react-aria-components/test/NumberField.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/numberfield/NumberField.test.js @ 99e6102368
//! The NumberField atoms: slots, states, form value, validation, keyboard, typing, paste,
//! commit behavior and typed values.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Modifier, Page, SyntheticEvent};

const PATH: &str = "/atoms/number-field";

/// The `NumberField` in the fixture section matching `section`.
async fn field_in(page: &Page<'_>, section: &str) -> Result<WebElement, Report> {
    page.element(format!("{section} .leptonic-NumberField"))
        .await
}

/// The visible input of the number field in `#container`.
async fn input(page: &Page<'_>, container: &str) -> Result<WebElement, Report> {
    page.element(format!(
        "#{container} .leptonic-NumberField input:not([type=hidden])"
    ))
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
async fn clear(page: &Page<'_>, input: &WebElement) -> Result<(), Report> {
    input
        .type_keys(page.primary_modifier().await? + "a")
        .await?;
    input.type_keys(Key::Backspace).await?;
    Ok(())
}

/// Pastes `text` over the input's whole text (a `paste` event with clipboard data, which
/// WebDriver can't send).
async fn paste(page: &Page<'_>, input: &WebElement, text: &str) -> Result<(), Report> {
    page.low_level()
        .eval::<()>(
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
    assert_that!(|| async {
        let message = input.prop("validationMessage").await?.unwrap_or_default();
        Ok::<_, Report>((message, input.accessible_description().await?))
    })
    .eventually_ok()
    .satisfies(|observed| {
        observed.derive(|(message, _)| message).is_not_empty();
        observed
            .derive(|(_, description)| description)
            .is_equal_to(&observed.actual().0);
    })
    .await;
    Ok(())
}

/// The field renders a group, an input named by its label and described by description and error,
/// and stepper buttons named by themselves and the label ("provides slots").
#[browser_test]
pub async fn provides_slots(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element("#nf-slots [role=group]").await?;
    assert_that!(group)
        .has_attribute("role")
        .await
        .is_equal_to("group");
    assert_that!(field_in(page, "#nf-slots").await?)
        .has_attribute("data-foo")
        .await
        .is_equal_to("bar");
    let input = input(page, "nf-slots").await?;
    assert_that!(input)
        .property("value")
        .await
        .some()
        .is_equal_to("1,024");
    assert_that!(input)
        .accessible_name()
        .await
        .is_equal_to("Width");
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Description Error"))
        .await;
    let buttons = page.elements("#nf-slots button").await?;
    assert_that!(&buttons).has_length(2);
    let (decrease, increase) = (&buttons[0], &buttons[1]);
    assert_that!(decrease)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Decrease");
    assert_that!(increase)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Increase");
    // Named "Decrease Width": by itself and the label.
    let label_id = page
        .element("#nf-slots label")
        .await?
        .id()
        .await?
        .unwrap_or_default();
    let button_id = decrease.id().await?.unwrap_or_default();
    assert_that!(decrease)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to(format!("{button_id} {label_id}"));
    Ok(())
}

/// The group has `data-hovered` while hovered and `data-focus-visible` while the input has keyboard
/// focus ("should support hover state", "should support focus visible state").
#[browser_test]
pub async fn hover_and_focus_visible_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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

/// A stepper button focused while the user uses the keyboard has `data-focus-visible`, as its
/// documentation says.
#[browser_test]
pub async fn steppers_show_focus_visible(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nf-keys").await?;
    tab_into(page, &input).await?;
    let increment = page
        .element("#nf-keys .leptonic-NumberFieldIncrementButton")
        .await?;
    assert_that!(increment)
        .attribute("data-focus-visible")
        .await
        .is_none();
    increment.focus().await?;
    increment
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    Ok(())
}

/// A read-only field has `data-readonly`, a plain one hasn't ("should support read-only state").
#[browser_test]
pub async fn read_only_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let plain = field_in(page, "#nf-slots").await?;
    assert_that!(plain)
        .attribute("data-readonly")
        .await
        .is_none();
    let read_only = field_in(page, "#nf-read-only").await?;
    assert_that!(read_only).has_attribute("data-readonly").await;
    Ok(())
}

/// A named field submits its raw value through a hidden input tied to its form, which is disabled
/// with the field ("should support form value", "should support disabled when having a form
/// value").
#[browser_test]
pub async fn form_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let hidden = page.element("#nf-form-value input[name=test]").await?;
    assert_that!(hidden)
        .property("value")
        .await
        .some()
        .is_equal_to("25");
    assert_that!(hidden)
        .has_attribute("form")
        .await
        .is_equal_to("nf-form");
    assert_that!(input(page, "nf-form-value").await?)
        .property("value")
        .await
        .some()
        .is_equal_to("$25.00");
    let disabled = page
        .element("#nf-form-value-disabled input[name=test]")
        .await?;
    assert_that!(disabled).has_attribute("disabled").await;
    Ok(())
}

/// An empty required field shows the native error after a validity check and focuses the input,
/// and the error stays while typing until the value is committed ("supports validation errors").
#[browser_test]
pub async fn validation_errors(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nf-native").await?;
    let field = field_in(page, "#nf-native").await?;
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
    assert_that!(field)
        .attribute("data-invalid")
        .await
        .is_none();

    assert_that!(page.element("#nf-native").await?.check_validity().await?).is_false();
    wait_for_error(&input).await?;
    field.wait_for_attr("data-invalid", Some("true")).await?;
    page.wait_for_focus(&input).await?;

    // The error stays while typing, and goes once the value is committed.
    let described = assert_that!(input)
        .has_attribute("aria-describedby")
        .await
        .actual()
        .clone();
    page.send_keys("3").await?;
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

/// Every ArrowUp and ArrowDown steps the value once and reports each change ("should support
/// repeat keydown events when holding an arrow key").
#[browser_test]
pub async fn arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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

/// A script `click()` on the steppers increments and decrements the value ("should trigger
/// onChange via programmatic click() on stepper buttons").
#[browser_test]
pub async fn programmatic_clicks_on_steppers(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let decrease = page.element("#nf-keys button[aria-label=Decrease]").await?;
    let increase = page.element("#nf-keys button[aria-label=Increase]").await?;
    let changes = page.element("#nf-changes").await?;
    increase.virtual_click().await?;
    changes.wait_for_inner_text("1025").await?;
    decrease.virtual_click().await?;
    changes.wait_for_inner_text("1025 1024").await?;
    Ok(())
}

/// Backspace deletes the first digit before a group separator, and Enter commits the rest ("should
/// allow you to delete the first digit in a number if it is followed by a group separator").
#[browser_test]
pub async fn deleting_the_first_digit_before_a_group_separator(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nf-keys").await?;
    tab_into(page, &input).await?;
    // The caret after the first digit.
    page.low_level()
        .eval::<()>(
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

/// A typed number committed with Enter is formatted and reported as one change ("supports
/// onChange").
#[browser_test]
pub async fn typing_and_enter_commit(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nf-keys").await?;
    tab_into(page, &input).await?;
    clear(page, &input).await?;
    page.send_keys("2048").await?;
    page.send_keys(Key::Enter).await?;
    input.wait_for_prop("value", "2,048").await?;
    page.element("#nf-changes")
        .await?
        .wait_for_inner_text("2048")
        .await?;
    Ok(())
}

/// Without grouping, a typed group separator is dropped and pasted text with one is rejected
/// ("should not type the grouping characters when useGrouping is false").
#[browser_test]
pub async fn no_grouping_characters_without_grouping(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nf-no-grouping").await?;
    tab_into(page, &input).await?;
    page.send_keys("102,4").await?;
    input.wait_for_prop("value", "1024").await?;
    clear(page, &input).await?;
    paste(page, &input, "1,024").await?;
    page.send_keys(Key::Tab).await?;
    // Unparsable pasted text keeps the previous (empty) value.
    input.wait_for_prop("value", "").await?;
    Ok(())
}

/// Without grouping in German, a typed `.` group separator is dropped and pasted text with one is
/// rejected ("should not type the grouping characters when useGrouping is false and in German
/// locale").
#[browser_test]
pub async fn no_grouping_characters_in_german(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nf-no-grouping-de").await?;
    tab_into(page, &input).await?;
    page.send_keys("102.4").await?;
    input.wait_for_prop("value", "1024").await?;
    clear(page, &input).await?;
    paste(page, &input, "1.024").await?;
    page.send_keys(Key::Tab).await?;
    // Unparsable pasted text keeps the previous (empty) value.
    input.wait_for_prop("value", "").await?;
    Ok(())
}

/// The scroll wheel steps the value only while the input is focused, and a zoom (Ctrl+wheel)
/// doesn't step it ("cannot scroll to step when not focused", "increment value when scrolling
/// upwards", "decrement value when scrolling downwards", "should not fire increment or decrement
/// if it is a zoom event").
#[browser_test]
pub async fn scroll_wheel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nf-wheel").await?;
    let changes = page.element("#nf-wheel-changes").await?;
    let up = || SyntheticEvent::wheel().delta_y(-10.0);
    let down = || SyntheticEvent::wheel().delta_y(10.0);
    page.blur_focused().await?;
    input.dispatch(up()).await?;
    changes
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;

    tab_into(page, &input).await?;
    input.dispatch(up()).await?;
    changes.wait_for_inner_text("-1").await?;
    input.wait_for_prop("value", "-1").await?;
    input.dispatch(down()).await?;
    changes.wait_for_inner_text("-1 0").await?;
    // A zoom (the Ctrl key: a pinch): no step.
    input
        .dispatch(down().modifiers(&[Modifier::Control]))
        .await?;
    input.dispatch(up().modifiers(&[Modifier::Control])).await?;
    changes
        .inner_text_stays("-1 0", std::time::Duration::from_millis(100))
        .await?;
    // The wheel doesn't blur the input.
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// A plain number pasted into a currency field is formatted as currency ("should support pasting
/// into a format").
#[browser_test]
pub async fn pasting_into_a_format(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nf-form-value").await?;
    tab_into(page, &input).await?;
    clear(page, &input).await?;
    paste(page, &input, "1,024").await?;
    input.wait_for_prop("value", "$1,024.00").await?;
    Ok(())
}

/// A field whose value the app doesn't accept keeps showing its value after a paste and Enter
/// ("should not change the input value if the new value is not accepted").
#[browser_test]
pub async fn rejected_values_keep_the_text(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nf-rejecting").await?;
    tab_into(page, &input).await?;
    clear(page, &input).await?;
    paste(page, &input, "1024").await?;
    input.wait_for_prop("value", "200").await?;
    page.send_keys(Key::Enter).await?;
    input
        .prop_stays("value", "200", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A server validation error stays when the input is focused and blurred without a change ("should
/// not reset validation errors on blur when value has not changed").
#[browser_test]
pub async fn server_errors_survive_an_unchanged_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nf-server-form").await?;
    let field = field_in(page, "#nf-server-form").await?;
    field.wait_for_attr("data-invalid", Some("true")).await?;
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("This field has an error."))
        .await;
    input.focus().await?;
    page.blur_focused().await?;
    field
        .attr_stays(
            "data-invalid",
            Some("true"),
            std::time::Duration::from_millis(100),
        )
        .await?;
    assert_that!(input)
        .accessible_description()
        .await
        .is_equal_to("This field has an error.");
    Ok(())
}

/// With `CommitBehavior::Validate`, committed values out of range, off step or empty are kept as
/// typed (announced) and shown as invalid, and a validity check of the form focuses the field;
/// valid ones clear the error ("should not change the edited input value when value snapping is
/// disabled").
#[browser_test]
pub async fn validate_commit_behavior(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nf-validate").await?;
    let form = page.element("#nf-validate").await?;
    assert_that!(input.is_valid().await?).is_true();

    // Over max.
    tab_into(page, &input).await?;
    clear(page, &input).await?;
    page.send_keys("1024").await?;
    page.send_keys(Key::Tab).await?;
    input.wait_for_prop("value", "1,024").await?;
    // The announcer creates its live regions only when the first announcement is made.
    let assertive = page
        .element("[data-live-announcer] [aria-live=assertive]")
        .await?;
    assert_that!(|| assertive.inner_text())
        .eventually_ok()
        .satisfies(|text| {
            text.derive_owned(|log| log.lines().any(|line| line == "1,024"))
                .is_true();
        })
        .await;
    field_in(page, "#nf-validate")
        .await?
        .wait_for_attr("data-invalid", Some("true"))
        .await?;
    wait_for_error(&input).await?;
    assert_that!(input)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(form.check_validity().await?).is_false();
    page.wait_for_focus(&input).await?;

    // Valid, under min, off step: typed values are announced; invalid ones focus the field on a
    // validity check.
    for (typed, valid) in [("30", true), ("2", false), ("31", false)] {
        clear(page, &input).await?;
        page.send_keys(typed).await?;
        assert_that!(|| assertive.inner_text())
            .eventually_ok()
            .satisfies(|text| {
                text.derive_owned(|log| log.lines().any(|line| line == typed))
                    .is_true();
            })
            .await;
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
        if valid {
            tab_into(page, &input).await?;
        } else {
            assert_that!(form.check_validity().await?).is_false();
            page.wait_for_focus(&input).await?;
        }
    }

    // Required.
    clear(page, &input).await?;
    page.send_keys(Key::Tab).await?;
    input.wait_for_prop("value", "").await?;
    assert_that!(input.is_valid().await?).is_false();
    wait_for_error(&input).await?;

    // Valid again.
    tab_into(page, &input).await?;
    page.send_keys("30").await?;
    page.send_keys(Key::Tab).await?;
    input.wait_for_prop("value", "30").await?;
    input.wait_for_attr("aria-describedby", None).await?;
    assert_that!(input.is_valid().await?).is_true();
    Ok(())
}

/// With `CommitBehavior::Validate` and native validation, Enter doesn't submit the form with an
/// out-of-range value but does once the value is corrected.
#[browser_test]
pub async fn validate_commit_behavior_and_enter_submit(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nf-validate").await?;
    tab_into(page, &input).await?;
    clear(page, &input).await?;
    page.send_keys("1024").await?;
    page.send_keys(Key::Enter).await?;
    wait_for_error(&input).await?;
    assert_that!(input.is_valid().await?).is_false();
    page.element("#nf-validate-submits")
        .await?
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;

    clear(page, &input).await?;
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

/// A `u64` field steps exactly up to `u64::MAX` and then disables its increment button, while a
/// `u8` field drops a typed minus sign and clamps larger values to 255.
#[browser_test]
pub async fn typed_values(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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
    clear(page, &unsigned).await?;
    page.send_keys("300").await?;
    page.send_keys(Key::Enter).await?;
    unsigned.wait_for_prop("value", "255").await?;
    clear(page, &unsigned).await?;
    page.send_keys("1000").await?;
    page.send_keys(Key::Up).await?;
    unsigned.wait_for_prop("value", "255").await?;
    Ok(())
}

/// Resetting the form restores a bound field's initial value through its setter ("supports form
/// reset").
#[browser_test]
pub async fn form_reset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nfv-reset").await?;
    assert_that!(input)
        .property("value")
        .await
        .some()
        .is_equal_to("10");
    tab_into(page, &input).await?;
    clear(page, &input).await?;
    page.send_keys("100").await?;
    input.wait_for_prop("value", "100").await?;
    page.blur_focused().await?;
    page.element("#nfv-reset-button").await?.click().await?;
    input.wait_for_prop("value", "10").await?;
    Ok(())
}

/// Pressing a stepper commits the value, so the native required error of an empty field goes
/// ("commits validation changes when pressing increment/decrement buttons").
#[browser_test]
pub async fn steppers_commit_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nf-native").await?;
    assert_that!(page.element("#nf-native").await?.check_validity().await?).is_false();
    wait_for_error(&input).await?;
    page.wait_for_focus(&input).await?;
    page.element("#nf-native .leptonic-NumberFieldDecrementButton")
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "0").await?;
    input.wait_for_attr("aria-describedby", None).await?;
    assert_that!(input.is_valid().await?).is_true();
    Ok(())
}

/// Focusing and leaving an empty required field without a change shows no error; typing a value
/// keeps the error until the value is committed ("only commits on blur if the value changed").
#[browser_test]
pub async fn only_commits_on_blur_if_the_value_changed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nf-native").await?;
    tab_into(page, &input).await?;
    page.send_keys(Key::Tab).await?;
    input
        .attr_stays(
            "aria-describedby",
            None,
            std::time::Duration::from_millis(100),
        )
        .await?;
    assert_that!(page.element("#nf-native").await?.check_validity().await?).is_false();
    wait_for_error(&input).await?;
    page.wait_for_focus(&input).await?;
    let described = input.attr("aria-describedby").await?;
    page.send_keys("4").await?;
    input
        .attr_stays(
            "aria-describedby",
            described.as_deref(),
            std::time::Duration::from_millis(100),
        )
        .await?;
    assert_that!(input.is_valid().await?).is_true();
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

/// A native validator's error shows after a validity check, which focuses the field; it stays
/// while typing and goes once a valid value is committed ("supports validate function", native).
#[browser_test]
pub async fn native_validate_function(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nfv-validate").await?;
    assert_that!(input)
        .attribute("aria-describedby")
        .await
        .is_none();
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(
        page.element("#nfv-validate")
            .await?
            .check_validity()
            .await?
    )
    .is_false();
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Invalid value"))
        .await;
    page.wait_for_focus(&input).await?;
    clear(page, &input).await?;
    page.send_keys("3").await?;
    assert_that!(input).has_attribute("aria-describedby").await;
    assert_that!(input.is_valid().await?).is_false();
    // 3 commits as 4 (snapped to the step of 2 from 0, rounding up).
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    assert_that!(input.is_valid().await?).is_true();
    Ok(())
}

/// A server error for the field's name shows after submitting and makes the input invalid until a
/// new value is committed ("supports server validation", native).
#[browser_test]
pub async fn native_server_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nfv-server").await?;
    assert_that!(input)
        .attribute("aria-describedby")
        .await
        .is_none();
    page.element("#nfv-server-submit").await?.click().await?;
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Invalid value."))
        .await;
    assert_that!(input.is_valid().await?).is_false();
    tab_into(page, &input).await?;
    page.send_keys("4").await?;
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    assert_that!(input.is_valid().await?).is_true();
    Ok(())
}

/// A `FieldError` message chosen by the validation details replaces the browser's message
/// ("supports customizing native error messages").
#[browser_test]
pub async fn custom_native_error_message(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nfv-custom").await?;
    assert_that!(input)
        .attribute("aria-describedby")
        .await
        .is_none();
    assert_that!(page.element("#nfv-custom").await?.check_validity().await?).is_false();
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Please enter a value"))
        .await;
    Ok(())
}

/// With ARIA validation, a validator's error shows at once (`aria-invalid`, natively valid) and
/// goes once a valid value is committed ("supports validate function", aria).
#[browser_test]
pub async fn aria_validate_function(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nfv-aria-validate").await?;
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Invalid value"))
        .await;
    assert_that!(input)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    assert_that!(input.is_valid().await?).is_true();
    tab_into(page, &input).await?;
    clear(page, &input).await?;
    page.send_keys("4").await?;
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    input.wait_for_attr("aria-invalid", None).await?;
    Ok(())
}

/// With ARIA validation, a server error for the field's name shows at once and goes once a new
/// value is committed ("supports server validation", aria).
#[browser_test]
pub async fn aria_server_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input(page, "nfv-aria-server").await?;
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Invalid value"))
        .await;
    assert_that!(input)
        .has_attribute("aria-invalid")
        .await
        .is_equal_to("true");
    tab_into(page, &input).await?;
    page.send_keys("4").await?;
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    input.wait_for_attr("aria-invalid", None).await?;
    Ok(())
}
