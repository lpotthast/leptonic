use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn NumberFieldBasicDemo() -> impl IntoView {
    // The value type is the field's: here `u32`.
    let state = use_number_field_state(UseNumberFieldStateInput {
        default_value: Some(50_u32),
        max_value: Signal::stored(Some(100)),
        ..UseNumberFieldStateInput::default()
    });

    let field = use_number_field(UseNumberFieldInput {
        has_label: true,
        ..UseNumberFieldInput::new(state)
    });

    // The stepper buttons come as `UseButtonInput`s: render them with `use_button`.
    let (decrement_attrs, decrement_styles) = use_button(field.decrement_button).props.into_parts();
    let (increment_attrs, increment_styles) = use_button(field.increment_button).props.into_parts();

    view! {
        <div class="demo-field" {..field.group_props.into_attrs()}>
            <label class="demo-field-label" {..field.label_props.into_attrs()}>"Quantity (0\u{2013}100)"</label>
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
