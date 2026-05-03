use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn NumberFieldFractionalDemo() -> impl IntoView {
    let frac_state = use_number_field_state(UseNumberFieldStateInput {
        default_value: Some(0.0),
        min_value: Some(0.0),
        max_value: Some(1.0),
        step: 0.1,
        ..Default::default()
    });

    let frac_field = use_number_field(UseNumberFieldInput {
        state: frac_state,
        is_disabled: Signal::derive(|| false),
        is_read_only: Signal::derive(|| false),
        label: Some("Opacity".to_string()),
        min_value: Some(0.0),
        max_value: Some(1.0),
        step: 0.1,
        ..Default::default()
    });

    view! {
        <div {..frac_field.group_props.into_attrs()}>
            <label
                {..frac_field.label_props.into_attrs()}
                style="display: block; font-weight: 500; margin-bottom: 0.25em;"
            >
                "Opacity (0.0\u{2013}1.0, step 0.1)"
            </label>
            <div style="display: flex; gap: 0.25em; align-items: center;">
                <button
                    {..frac_field.decrement_button_props.into_attrs()}
                    style="padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer;"
                >
                    "\u{2212}"
                </button>
                <input
                    prop:value=frac_field.display_value
                    {..frac_field.input_props.into_attrs()}
                    style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px; width: 80px; text-align: center;"
                />
                <button
                    {..frac_field.increment_button_props.into_attrs()}
                    style="padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer;"
                >
                    "+"
                </button>
            </div>
        </div>
    }
}
