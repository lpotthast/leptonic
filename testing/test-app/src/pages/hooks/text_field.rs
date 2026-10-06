use std::sync::Arc;

use leptonic::hooks::{
    IntoAttrs, UseTextFieldInput, UseTextFieldReturn, UseTextFieldStateInput, use_text_field,
    use_text_field_state,
};
use leptos::prelude::*;

/// A text field with label, description and validation ("at least 3 characters"), in a form.
/// The value starts as "Ada" and is mirrored in `#test-tf-value`. "Clear" empties it
/// programmatically; "Reset" resets the form.
#[component]
pub fn PageHookTextField() -> impl IntoView {
    let state = use_text_field_state(UseTextFieldStateInput {
        default_value: "Ada".to_owned(),
        on_change: None,
    });
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
        name: Some("name".to_owned()),
        validate: Some(Arc::new(|value: &String| {
            if value.chars().count() < 3 {
                Err(vec!["At least 3 characters.".to_owned()])
            } else {
                Ok(())
            }
        })),
        ..UseTextFieldInput::new(state)
    });

    view! {
        <div id="test-page-hook-text-field">
            <h1>"Text field"</h1>
            <form id="test-tf-form">
                <label {..label_props.into_attrs()}>"Name"</label>
                <input {..input_props.into_attrs()} />
                <div {..description_props.into_attrs()}>"Your first name."</div>
                <Show when=move || is_invalid.get()>
                    <div {..error_message_props.clone().into_attrs()}>
                        {move || validation_errors.get().join(" ")}
                    </div>
                </Show>
                <button id="test-tf-reset" type="reset">"Reset"</button>
            </form>
            <button id="test-tf-clear" on:click=move |_| state.set_value(String::new())>
                "Clear"
            </button>
            <div>"Value: " <span id="test-tf-value">{move || state.value.get()}</span></div>
        </div>
    }
}
