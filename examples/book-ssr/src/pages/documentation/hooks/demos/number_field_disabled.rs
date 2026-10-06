use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

#[component]
pub fn NumberFieldDisabledDemo() -> impl IntoView {
    let disabled = RwSignal::new(true);

    // The field reads `is_disabled` from its state.
    let state = use_number_field_state(UseNumberFieldStateInput {
        default_value: Some(42_u16),
        is_disabled: disabled.into(),
        ..UseNumberFieldStateInput::default()
    });

    let field = use_number_field(UseNumberFieldInput {
        has_label: true,
        ..UseNumberFieldInput::new(state)
    });

    let (decrement_attrs, decrement_styles) = use_button(field.decrement_button).props.into_parts();
    let (increment_attrs, increment_styles) = use_button(field.increment_button).props.into_parts();

    view! {
        <div class="demo-field" {..field.group_props.into_attrs()}>
            <label class="demo-field-label" {..field.label_props.into_attrs()}>"Seats"</label>
            <div class="demo-inline-controls demo-no-margin">
                <button class="demo-stepper-btn" {..decrement_attrs} style=decrement_styles>"\u{2212}"</button>
                <input class="demo-input demo-text-input demo-number-input" {..field.input_props.into_attrs()}/>
                <button class="demo-stepper-btn" {..increment_attrs} style=increment_styles>"+"</button>
            </div>
        </div>
        <Checkbox state=disabled>"Disabled"</Checkbox>
    }
}
