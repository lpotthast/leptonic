use leptonic::atoms::{
    checkbox::{CheckboxButton, CheckboxField},
    field::{Description, FieldError, Label},
    input::Input,
    number_field::{
        NumberField, NumberFieldDecrementButton, NumberFieldGroup, NumberFieldIncrementButton,
    },
};
use leptos::prelude::*;

#[component]
pub fn NumberFieldAtomDemo() -> impl IntoView {
    // The field's value type is the signal's: `i8`. Without `min_value` and `max_value`, the
    // type's bounds apply: stepping stops at -128 and 127.
    let temperature = RwSignal::new(Some(21_i8));
    let disabled = RwSignal::new(false);

    view! {
        <NumberField value=temperature set_value=temperature is_disabled=disabled classes="demo-field">
            <Label classes="demo-field-label">"Temperature (\u{b0}C)"</Label>
            // The group draws the border and the focus ring; the input inside it stays plain.
            <NumberFieldGroup classes="demo-number-field-group">
                <NumberFieldDecrementButton classes="demo-number-field-stepper">
                    <span aria-hidden="true">"\u{2212}"</span>
                </NumberFieldDecrementButton>
                <Input classes="demo-number-field-input"/>
                <NumberFieldIncrementButton classes="demo-number-field-stepper">
                    <span aria-hidden="true">"+"</span>
                </NumberFieldIncrementButton>
            </NumberFieldGroup>
            <Description classes="demo-field-description">"Home and End jump to the limits of an i8."</Description>
            <FieldError classes="demo-field-error"/>
        </NumberField>

        <p class="demo-status">
            {move || temperature.get().map_or_else(|| "No temperature.".to_owned(), |t| format!("Temperature: {t}\u{a0}\u{b0}C"))}
        </p>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
