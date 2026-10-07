use leptonic::{components::prelude::Checkbox, hooks::*};
use leptos::prelude::*;

#[component]
pub fn NumberFieldBasicDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    // The value type is the field's: here `u32`. Range, step and the disabled state live on the state.
    let state = use_number_field_state(UseNumberFieldStateInput {
        default_value: Some(50_u32),
        max_value: Signal::stored(Some(100)),
        is_disabled: disabled.into(),
        ..UseNumberFieldStateInput::default()
    });

    let field = use_number_field(UseNumberFieldInput {
        has_label: true.into(),
        state,
        id: None,
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        aria_describedby: None,
        is_required: Signal::stored(false),
        placeholder: MaybeProp::default(),
        auto_focus: false,
        is_wheel_disabled: false,
        increment_aria_label: MaybeProp::default(),
        decrement_aria_label: MaybeProp::default(),
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        on_key_down: None,
        on_key_up: None,
    });

    // The stepper buttons come as `UseButtonInput`s: render them with `use_button`. They are named
    // "Decrease Quantity" and "Increase Quantity".
    let (decrement_attrs, decrement_styles) = use_button(field.decrement_button).props.into_parts();
    let (increment_attrs, increment_styles) = use_button(field.increment_button).props.into_parts();

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..field.label_props.into_attrs()}>"Quantity (0\u{2013}100)"</label>
            // The group wraps the input and its stepper buttons.
            <div class="demo-input-row" {..field.group_props.into_attrs()}>
                <button class="demo-btn" {..decrement_attrs} style=decrement_styles>
                    <span aria-hidden="true">"\u{2212}"</span>
                </button>
                <input class="demo-input demo-text-input demo-number-input" {..field.input_props.into_attrs()}/>
                <button class="demo-btn" {..increment_attrs} style=increment_styles>
                    <span aria-hidden="true">"+"</span>
                </button>
            </div>
        </div>

        <p class="demo-status">
            {move || state.value().get().map_or_else(|| "No quantity.".to_owned(), |value| format!("Quantity: {value}"))}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
