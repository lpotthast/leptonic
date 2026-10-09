use leptonic::{
    IntoAttrs,
    hooks::{
        button::use_button,
        form::{
            UseNumberFieldInput, UseNumberFieldStateInput, use_number_field, use_number_field_state,
        },
    },
};
use leptos::prelude::*;

#[component]
pub fn NumberFieldFractionalDemo() -> impl IntoView {
    let state = use_number_field_state(UseNumberFieldStateInput {
        default_value: Some(0.0_f64),
        min_value: Signal::stored(Some(0.0)),
        max_value: Signal::stored(Some(1.0)),
        step: Signal::stored(Some(0.1)),
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

    let (decrement_attrs, decrement_styles) = use_button(field.decrement_button).props.into_parts();
    let (increment_attrs, increment_styles) = use_button(field.increment_button).props.into_parts();

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..field.label_props.into_attrs()}>"Opacity (0.0\u{2013}1.0, step 0.1)"</label>
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

        // The value is exact: 0.1 + 0.1 + 0.1 is 0.3.
        <p class="demo-status">
            {move || state.value().get().map_or_else(|| "No opacity.".to_owned(), |value| format!("Opacity: {value}"))}
        </p>
    }
}
