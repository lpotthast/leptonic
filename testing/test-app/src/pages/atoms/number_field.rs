use std::collections::HashMap;

use leptonic::{
    atoms::{
        field::{Description, FieldError, Label},
        form::Form,
        input::Input,
        number_field::{
            NumberField, NumberFieldDecrementButton, NumberFieldGroup, NumberFieldIncrementButton,
        },
    },
    hooks::CommitBehavior,
    utils::{
        i18n::{I18nProvider, Locale},
        number_formatter::{NumberFormatOptions, NumberStyle},
    },
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

            <div id="nf-form-value">
                <NumberField default_value=25_i32 name="test" form="test" format_options=currency()>
                    <Label>"Price"</Label>
                    <Steppers />
                </NumberField>
            </div>
            <div id="nf-form-value-disabled">
                <NumberField default_value=25_i32 name="test" form="test" is_disabled=true>
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

            <div id="nf-currency">
                <NumberField default_value=200_i32 format_options=currency()>
                    <Label>"Price"</Label>
                    <Steppers />
                </NumberField>
            </div>

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

            <form id="nf-validate">
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
            </form>

            // Validate mode in a form: Enter submits once the value is valid.
            <form id="nf-validate-submit" on:submit=on_submit>
                <NumberField
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
