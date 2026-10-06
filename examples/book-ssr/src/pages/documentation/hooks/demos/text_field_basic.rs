use std::sync::Arc;

use leptonic::{
    components::prelude::*,
    hooks::{
        IntoAttrs, UseTextFieldInput, UseTextFieldReturn, UseTextFieldStateInput, use_text_field,
        use_text_field_state,
    },
};
use leptos::prelude::*;

#[component]
pub fn TextFieldBasicDemo() -> impl IntoView {
    // The hook owns the value; read it from `username.value`, change it with `username.set_value`.
    let username = use_text_field_state(UseTextFieldStateInput::default());
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
        has_label: true,
        is_disabled: disabled.into(),
        is_required: true.into(),
        placeholder: "Enter a username".into(),
        max_length: Some(20),
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
            // The error message is referenced by the input only while it is rendered.
            <Show when=move || is_invalid.get()>
                <p class="demo-field-error" {..error_message_props.clone().into_attrs()}>
                    {move || validation_errors.get().join(" ")}
                </p>
            </Show>
        </div>

        <p>
            "Value: \u{201c}"{move || username.value.get()}"\u{201d} ("
            {move || username.value.get().chars().count()}"/20)"
        </p>

        <div class="demo-flex-center-row">
            <Button on_press=move |_| username.set_value(String::new())>"Clear"</Button>
            <Checkbox state=disabled>"Disabled"</Checkbox>
        </div>
    }
}
