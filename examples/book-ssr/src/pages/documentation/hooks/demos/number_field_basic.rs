use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn NumberFieldBasicDemo() -> impl IntoView {
    let basic_state = use_number_field_state(UseNumberFieldStateInput {
        default_value: Some(50.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        step: 1.0,
        ..Default::default()
    });

    let basic_field = use_number_field(UseNumberFieldInput {
        state: basic_state,
        is_disabled: Signal::derive(|| false),
        is_read_only: Signal::derive(|| false),
        label: Some("Quantity".to_string()),
        min_value: Some(0.0),
        max_value: Some(100.0),
        step: 1.0,
        ..Default::default()
    });

    let (decrement_attrs, decrement_styles) =
        use_button(basic_field.decrement_button).props.into_parts();
    let (increment_attrs, increment_styles) =
        use_button(basic_field.increment_button).props.into_parts();

    view! {
        <div {..basic_field.group_props.into_attrs()}>
            <label
                {..basic_field.label_props.into_attrs()}
                style="display: block; font-weight: 500; margin-bottom: 0.25em;"
            >
                "Quantity (0\u{2013}100)"
            </label>
            <div style="display: flex; gap: 0.25em; align-items: center;">
                <button
                    {..decrement_attrs}
                    style=decrement_styles
                    class="demo-stepper-btn"
                >
                    "\u{2212}"
                </button>
                <input
                    prop:value=basic_field.display_value
                    {..basic_field.input_props.into_attrs()}
                    style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px; width: 80px; text-align: center;"
                />
                <button
                    {..increment_attrs}
                    style=increment_styles
                    class="demo-stepper-btn"
                >
                    "+"
                </button>
            </div>
            <p style="margin: 0.5em 0 0 0; font-size: 0.85em;">
                "Value: " { move || basic_state.number_value.get().map_or("(empty)".to_string(), |v| format!("{v}")) }
            </p>
        </div>
    }
}
