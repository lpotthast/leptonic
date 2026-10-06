use std::sync::Arc;

use leptonic::{
    components::prelude::*,
    hooks::{
        IntoAttrs, UseTextFieldInput, UseTextFieldReturn, UseTextFieldStateInput, use_text_field,
        use_text_field_state,
    },
};
use leptos::prelude::*;

const MAX_LENGTH: u32 = 20;

#[component]
pub fn TextFieldBasicDemo() -> impl IntoView {
    // The hook owns the value; read it from `username.value`, change it with `username.set_value`.
    let username = use_text_field_state(UseTextFieldStateInput {
        default_value: "ferris".to_owned(),
        ..UseTextFieldStateInput::default()
    });
    let disabled = RwSignal::new(false);

    let UseTextFieldReturn {
        label_props,
        input_props,
        description_props,
        error_message_props,
        is_invalid,
        validation_errors,
        ..
    } = use_text_field(UseTextFieldInput {
        has_label: true.into(),
        is_disabled: disabled.into(),
        is_required: true.into(),
        placeholder: "Enter a username".into(),
        max_length: Some(MAX_LENGTH),
        validate: Some(Arc::new(|value: &String| {
            if value.chars().count() >= 3 && value.chars().all(char::is_alphanumeric) {
                Ok(())
            } else {
                Err(vec!["Use at least 3 letters or digits.".to_owned()])
            }
        })),
        ..UseTextFieldInput::new(username)
    });

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..label_props.into_attrs()}>"Username"</label>
            <input class="demo-input demo-text-input" {..input_props.into_attrs()}/>
            <p class="demo-field-description" {..description_props.into_attrs()}>"3 to 20 letters or digits."</p>
            // The input references the error message only while it is rendered.
            <Show when=move || is_invalid.get()>
                <p class="demo-field-error" {..error_message_props.clone().into_attrs()}>
                    {move || validation_errors.get().join(" ")}
                </p>
            </Show>
        </div>

        <p class="demo-status">
            {move || username.value.with(|value| {
                format!("Value: \u{201c}{value}\u{201d} ({} of {MAX_LENGTH} characters)", value.chars().count())
            })}
        </p>

        <div class="demo-controls">
            <Button on_press=move |_| username.set_value(String::new())>"Clear"</Button>
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
