use leptonic::{
    atoms::{
        field::{Description, FieldError, Label},
        input::Input,
        number_field::{
            NumberField, NumberFieldDecrementButton, NumberFieldGroup, NumberFieldIncrementButton,
        },
    },
    components::prelude::Checkbox,
};
use leptos::prelude::*;

#[component]
pub fn NumberFieldAtomDemo() -> impl IntoView {
    // The field's value type is the signal's: `i8`. Without `min_value` and `max_value`, the
    // type's bounds apply: stepping stops at -128 and 127.
    let temperature = RwSignal::new(Some(21_i8));
    let disabled = RwSignal::new(false);

    view! {
        <NumberField state=temperature is_disabled=disabled classes="demo-field">
            <Label classes="demo-field-label">"Temperature (\u{b0}C)"</Label>
            <NumberFieldGroup classes="demo-number-field-group">
                <NumberFieldDecrementButton classes="demo-stepper-btn">"\u{2212}"</NumberFieldDecrementButton>
                <Input classes=["demo-input", "demo-text-input", "demo-atom-input", "demo-number-input"]/>
                <NumberFieldIncrementButton classes="demo-stepper-btn">"+"</NumberFieldIncrementButton>
            </NumberFieldGroup>
            <Description classes="demo-field-description">"Home and End jump to the limits of an i8."</Description>
            <FieldError classes="demo-field-error"/>
        </NumberField>

        <p class="demo-status">
            {move || temperature.get().map_or_else(|| "No temperature".to_owned(), |t| format!("Temperature: {t} \u{b0}C"))}
        </p>

        <div class="demo-toggle-settings">
            <Checkbox state=disabled>"Disabled"</Checkbox>
        </div>
    }
}
