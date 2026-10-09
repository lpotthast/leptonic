use std::{collections::HashMap, sync::Arc};

use leptonic::{
    I18nProvider, Locale, NumberFormatOptions, NumberStyle,
    atoms::{
        field::{Description, FieldError, Label},
        form::Form,
        input::Input,
        number_field::{
            NumberField, NumberFieldDecrementButton, NumberFieldGroup, NumberFieldIncrementButton,
        },
    },
    hooks::form::{CommitBehavior, ValidationBehavior, ValidationResult},
};
use leptos::{ev::SubmitEvent, prelude::*};

/// The group, decrement button, input and increment button of a number field.
#[component]
fn Steppers() -> impl IntoView {
    view! {
        <NumberFieldGroup>
            <NumberFieldDecrementButton>"-"</NumberFieldDecrementButton>
            <Input />
            <NumberFieldIncrementButton>"+"</NumberFieldIncrementButton>
        </NumberFieldGroup>
    }
}

fn currency() -> NumberFormatOptions {
    NumberFormatOptions {
        style: NumberStyle::Currency,
        currency: Some("USD".to_owned()),
        ..NumberFormatOptions::default()
    }
}

/// NumberField atoms, as react-aria-components' `NumberField` tests render them. Each case sits
/// in its own element with an id; `#nf-changes` lists the `on_change` values of `#nf-keys`.
#[component]
pub fn PageAtomNumberField() -> impl IntoView {
    let changes = RwSignal::new(Vec::<String>::new());
    let record = move |value: Option<i32>| {
        changes.update(|c| c.push(value.map_or_else(|| "empty".to_owned(), |v| v.to_string())));
    };
    let wheel_changes = RwSignal::new(Vec::<String>::new());
    let record_wheel = move |value: Option<i32>| {
        wheel_changes
            .update(|c| c.push(value.map_or_else(|| "empty".to_owned(), |v| v.to_string())));
    };
    let submits = RwSignal::new(0_u32);
    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        submits.update(|n| *n += 1);
    };
    let german = "de-DE".parse::<Locale>().expect("a locale");
    let server_errors = Signal::stored(HashMap::from([(
        "testNumber".to_owned(),
        vec!["This field has an error.".to_owned()],
    )]));

    view! {
        <div id="test-page-atom-number-field">
            <h1>"NumberField"</h1>

            <div id="nf-slots">
                <NumberField
                    default_value=1024_i32
                    min_value=0
                    is_invalid=true
                    attr:data-foo="bar"
                >
                    <Label>"Width"</Label>
                    <Steppers />
                    <Description>"Description"</Description>
                    <FieldError>"Error"</FieldError>
                </NumberField>
            </div>

            <div id="nf-keys">
                <NumberField default_value=1024_i32 min_value=0 on_change=record>
                    <Label>"Width"</Label>
                    <Steppers />
                </NumberField>
            </div>
            <div>"Changes: " <span id="nf-changes">{move || changes.get().join(" ")}</span></div>

            <div id="nf-read-only">
                <NumberField default_value=1_i32 is_read_only=true>
                    <Label>"Width"</Label>
                    <Steppers />
                </NumberField>
            </div>

            // The fields outside the form they belong to.
            <form id="nf-form"></form>
            <div id="nf-form-value">
                <NumberField default_value=25_i32 name="test" form="nf-form" format_options=currency()>
                    <Label>"Price"</Label>
                    <Steppers />
                </NumberField>
            </div>
            <div id="nf-form-value-disabled">
                <NumberField default_value=25_i32 name="test" form="nf-form" is_disabled=true>
                    <Label>"Price"</Label>
                    <Steppers />
                </NumberField>
            </div>

            <form id="nf-native">
                <NumberField<i32> is_required=true>
                    <Label>"Width"</Label>
                    <Steppers />
                    <FieldError />
                </NumberField<i32>>
            </form>

            <div id="nf-no-grouping">
                <NumberField<i32> format_options=NumberFormatOptions {
                    use_grouping: false,
                    ..NumberFormatOptions::default()
                }>
                    <Label>"Width"</Label>
                    <Steppers />
                </NumberField<i32>>
            </div>

            <div id="nf-no-grouping-de">
                <I18nProvider locale=german>
                    <NumberField<i32> format_options=NumberFormatOptions {
                        use_grouping: false,
                        ..NumberFormatOptions::default()
                    }>
                        <Label>"Breite"</Label>
                        <Steppers />
                    </NumberField<i32>>
                </I18nProvider>
            </div>

            // The scroll wheel steps while the field has focus.
            <div id="nf-wheel">
                <NumberField default_value=0_i32 on_change=record_wheel>
                    <Label>"Wheel"</Label>
                    <Steppers />
                </NumberField>
            </div>
            <div>"Wheel changes: " <span id="nf-wheel-changes">{move || wheel_changes.get().join(" ")}</span></div>

            // A value without a setter rejects every change (react-aria: a controlled `value`).
            <div id="nf-rejecting">
                <NumberField value=Some(200)>
                    <Label>"Width"</Label>
                    <Steppers />
                </NumberField>
            </div>

            <Form attr:id="nf-server-form" validation_errors=server_errors>
                <NumberField name="testNumber" default_value=5_i32>
                    <Label>"Test Number"</Label>
                    <Steppers />
                    <FieldError />
                </NumberField>
            </Form>

            // Validate mode in a form: values out of range or off step are kept and invalid, and
            // Enter submits once the value is valid.
            <form id="nf-validate" on:submit=on_submit>
                <NumberField
                    is_required=true
                    default_value=20_i32
                    min_value=10
                    step=10
                    max_value=50
                    commit_behavior=CommitBehavior::Validate
                >
                    <Label>"Width"</Label>
                    <Steppers />
                    <FieldError />
                </NumberField>
                <button type="submit">"Submit"</button>
            </form>
            <div>"Submits: " <span id="nf-validate-submits">{submits}</span></div>

            <NumberFieldValidation />

            // Typed values.
            <div id="nf-u64">
                <NumberField default_value=u64::MAX - 1>
                    <Label>"Big"</Label>
                    <Steppers />
                </NumberField>
            </div>
            <div id="nf-u8">
                <NumberField<u8>>
                    <Label>"Unsigned"</Label>
                    <Steppers />
                </NumberField<u8>>
            </div>
        </div>
    }
}

/// Number fields with form reset, validators, server errors and custom messages, with native and
/// ARIA validation (react-spectrum's `NumberField.test.js` validation cases).
#[component]
fn NumberFieldValidation() -> impl IntoView {
    let bound = RwSignal::new(Some(10_i32));
    let server_errors = RwSignal::new(HashMap::<String, Vec<String>>::new());
    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        server_errors.set(HashMap::from([(
            "value".to_owned(),
            vec!["Invalid value.".to_owned()],
        )]));
    };
    view! {
        <form id="nfv-reset">
            <NumberField value=bound set_value=bound>
                <Label>"Value"</Label>
                <Steppers />
            </NumberField>
            <input type="reset" id="nfv-reset-button" />
        </form>
        <Form attr:id="nfv-validate">
            <NumberField
                default_value=2_i32
                step=2
                validate={Arc::new(|v: &Option<i32>| {
                    if *v == Some(4) { Ok(()) } else { Err(vec!["Invalid value".to_owned()]) }
                })}
            >
                <Label>"Value"</Label>
                <Steppers />
                <FieldError />
            </NumberField>
        </Form>
        <Form attr:id="nfv-server" validation_errors=server_errors on:submit=on_submit>
            <NumberField<i32> name="value">
                <Label>"Value"</Label>
                <Steppers />
                <FieldError />
            </NumberField<i32>>
            <button id="nfv-server-submit" type="submit">"Submit"</button>
        </Form>
        <Form attr:id="nfv-custom">
            <NumberField<i32> is_required=true>
                <Label>"Value"</Label>
                <Steppers />
                <FieldError message=Arc::new(|result: &ValidationResult| {
                    result
                        .validation_details
                        .value_missing
                        .then(|| "Please enter a value".to_owned())
                }) />
            </NumberField<i32>>
        </Form>
        <div id="nfv-aria-validate">
            <NumberField
                default_value=2_i32
                validation_behavior=ValidationBehavior::Aria
                validate={Arc::new(|v: &Option<i32>| {
                    if *v == Some(2) { Err(vec!["Invalid value".to_owned()]) } else { Ok(()) }
                })}
            >
                <Label>"Value"</Label>
                <Steppers />
                <FieldError />
            </NumberField>
        </div>
        <Form
            attr:id="nfv-aria-server"
            validation_behavior=ValidationBehavior::Aria
            validation_errors=Signal::stored(HashMap::from([(
                "value".to_owned(),
                vec!["Invalid value".to_owned()],
            )]))
        >
            <NumberField<i32> name="value">
                <Label>"Value"</Label>
                <Steppers />
                <FieldError />
            </NumberField<i32>>
        </Form>
    }
}
