use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn NumberFieldDisabledDemo() -> impl IntoView {
    let disabled_state = use_number_field_state(UseNumberFieldStateInput {
        default_value: Some(42.0),
        ..Default::default()
    });

    let disabled_field = use_number_field(UseNumberFieldInput {
        state: disabled_state,
        is_disabled: Signal::derive(|| true),
        is_read_only: Signal::derive(|| false),
        label: Some("Disabled".to_string()),
        ..Default::default()
    });

    let (decrement_attrs, decrement_styles) = use_button(disabled_field.decrement_button)
        .props
        .into_parts();
    let (increment_attrs, increment_styles) = use_button(disabled_field.increment_button)
        .props
        .into_parts();

    view! {
        <div {..disabled_field.group_props.into_attrs()}>
            <label
                {..disabled_field.label_props.into_attrs()}
                style="display: block; font-weight: 500; margin-bottom: 0.25em;"
            >
                "Disabled Field"
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
                    prop:value=disabled_field.display_value
                    {..disabled_field.input_props.into_attrs()}
                    style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px; width: 80px; text-align: center; opacity: 0.5;"
                />
                <button
                    {..increment_attrs}
                    style=increment_styles
                    class="demo-stepper-btn"
                >
                    "+"
                </button>
            </div>
        </div>
    }
}
