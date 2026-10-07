use leptonic::atoms::{
    checkbox::Checkbox,
    field::{Description, Label},
    input::Input,
    number_field::{
        NumberField, NumberFieldDecrementButton, NumberFieldGroup, NumberFieldIncrementButton,
    },
};
use leptos::prelude::*;

#[component]
pub fn NumberFieldConceptDemo() -> impl IntoView {
    // The field's number type is the signal's: `u8`. The value is `None` while the field is empty.
    let tickets = RwSignal::new(Some(2_u8));
    let disabled = RwSignal::new(false);

    view! {
        <NumberField value=tickets set_value=tickets min_value=1_u8 max_value=10_u8 is_disabled=disabled classes="demo-field">
            <Label classes="demo-field-label">"Tickets"</Label>
            // The group draws the border and the focus ring around the input and the steppers.
            <NumberFieldGroup classes="demo-number-field-group">
                <NumberFieldDecrementButton classes="demo-number-field-stepper">
                    <span aria-hidden="true">"\u{2212}"</span>
                </NumberFieldDecrementButton>
                <Input classes="demo-number-field-input"/>
                <NumberFieldIncrementButton classes="demo-number-field-stepper">
                    <span aria-hidden="true">"+"</span>
                </NumberFieldIncrementButton>
            </NumberFieldGroup>
            <Description classes="demo-field-description">"1 to 10 tickets per order."</Description>
        </NumberField>

        <p class="demo-status">
            {move || match tickets.get() {
                None => "No tickets selected.".to_owned(),
                Some(1) => "1 ticket selected.".to_owned(),
                Some(tickets) => format!("{tickets} tickets selected."),
            }}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}
