use std::sync::Arc;

use leptonic::{
    IntoAttrs,
    hooks::form::{
        InputType, TextFieldElement, UseTextFieldInput, UseTextFieldReturn, UseTextFieldStateInput,
        ValidationBehavior, use_text_field, use_text_field_state,
    },
};
use leptos::prelude::*;

/// A text field with label, description and validation ("at least 3 characters"), in a form.
/// The value starts as "Ada" and is mirrored in `#test-tf-value`. "Clear" empties it
/// programmatically; "Reset" resets the form.
#[component]
pub fn PageHookTextField() -> impl IntoView {
    let state = use_text_field_state(UseTextFieldStateInput {
        default_value: "Ada".to_owned(),
        ..UseTextFieldStateInput::default()
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
        has_label: true.into(),
        name: Some("name".to_owned()),
        validate: Some(Arc::new(|value: &String| {
            if value.chars().count() < 3 {
                Err(vec!["At least 3 characters.".to_owned()])
            } else {
                Ok(())
            }
        })),
        state,
        id: None,
        element: TextFieldElement::Input,
        input_type: Signal::stored(InputType::Text),
        is_disabled: Signal::stored(false),
        is_read_only: Signal::stored(false),
        is_required: Signal::stored(false),
        is_invalid: Signal::stored(false),
        validation_behavior: ValidationBehavior::default(),
        validation: None,
        form: None,
        placeholder: MaybeProp::default(),
        pattern: None,
        min_length: None,
        max_length: None,
        auto_complete: None,
        auto_capitalize: None,
        auto_correct: None,
        spell_check: None,
        input_mode: Signal::stored(None),
        enter_key_hint: None,
        auto_focus: false,
        exclude_from_tab_order: false,
        label_id: None,
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        aria_describedby: None,
        aria_errormessage: None,
        aria_activedescendant: Signal::stored(None),
        aria_autocomplete: None,
        aria_haspopup: None,
        aria_controls: Signal::stored(None),
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        on_key_down: None,
        on_key_up: None,
        shortcuts: None,
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
