use leptonic::hooks::*;
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
        has_label: true,
        ..UseNumberFieldInput::new(state)
    });

    let (decrement_attrs, decrement_styles) = use_button(field.decrement_button).props.into_parts();
    let (increment_attrs, increment_styles) = use_button(field.increment_button).props.into_parts();

    view! {
        <div class="demo-field" {..field.group_props.into_attrs()}>
            <label class="demo-field-label" {..field.label_props.into_attrs()}>"Opacity (0.0\u{2013}1.0, step 0.1)"</label>
            <div class="demo-inline-controls demo-no-margin">
                <button class="demo-stepper-btn" {..decrement_attrs} style=decrement_styles>"\u{2212}"</button>
                <input class="demo-input demo-text-input demo-number-input" {..field.input_props.into_attrs()}/>
                <button class="demo-stepper-btn" {..increment_attrs} style=increment_styles>"+"</button>
            </div>
        </div>
        <p>
            "Value: "
            {move || state.value().get().map_or_else(|| "(empty)".to_owned(), |value| value.to_string())}
        </p>
    }
}
