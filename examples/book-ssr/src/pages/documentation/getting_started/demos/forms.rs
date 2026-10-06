use std::{collections::HashMap, sync::Arc};

use leptonic::{
    components::prelude::{Button, ButtonVariant},
    hooks::{
        FormValidationContext, InputType, IntoAttrs, UseTextFieldInput, UseTextFieldReturn,
        UseTextFieldStateInput, ValidateFn, ValidationBehavior, use_text_field,
        use_text_field_state,
    },
};
use leptos::{context::Provider, prelude::*};

#[component]
pub fn FormsDemo() -> impl IntoView {
    // Errors reported by the server, by field name.
    let server_errors = RwSignal::new(HashMap::<String, Vec<String>>::new());
    let submitted = RwSignal::new(false);

    let simulate_server_error = move |_| {
        server_errors.set(HashMap::from([(
            "username".to_owned(),
            vec!["This username is taken.".to_owned()],
        )]));
    };

    view! {
        <Provider value=FormValidationContext { errors: server_errors.into() }>
            <form
                class="demo-form"
                on:submit=move |e| {
                    e.prevent_default();
                    submitted.set(true);
                }
                on:reset=move |_| {
                    server_errors.set(HashMap::new());
                    submitted.set(false);
                }
            >
                // Shows errors while you type.
                <Field
                    label="Username"
                    name="username"
                    behavior=ValidationBehavior::Aria
                    input_type=InputType::Text
                    validate=Arc::new(|value: &String| {
                        if value.chars().count() >= 3 { Ok(()) } else { Err(vec!["At least 3 characters.".to_owned()]) }
                    })
                />
                // Shows errors when the form is submitted, using the browser's constraint validation.
                <Field
                    label="Email"
                    name="email"
                    behavior=ValidationBehavior::Native
                    input_type=InputType::Email
                    validate=Arc::new(|value: &String| {
                        if value.ends_with(".example") { Err(vec!["Use a real domain.".to_owned()]) } else { Ok(()) }
                    })
                />
                <div class="demo-flex-center-row">
                    <button class="demo-btn" type="submit">"Submit"</button>
                    <button class="demo-btn" type="reset">"Reset"</button>
                    <Button variant=ButtonVariant::Outlined on_press=simulate_server_error>"Simulate server error"</Button>
                </div>
            </form>
        </Provider>
        <p class="demo-caption">{move || if submitted.get() { "Submitted." } else { "Not submitted yet." }}</p>
    }
}

/// A required text field with a label and an error message.
#[component]
fn Field(
    label: &'static str,
    name: &'static str,
    behavior: ValidationBehavior,
    input_type: InputType,
    validate: ValidateFn<String>,
) -> impl IntoView {
    let state = use_text_field_state(UseTextFieldStateInput::default());
    let UseTextFieldReturn {
        label_props,
        input_props,
        error_message_props,
        is_invalid,
        validation_errors,
        ..
    } = use_text_field(UseTextFieldInput {
        has_label: true,
        name: Some(name.to_owned()),
        input_type: input_type.into(),
        is_required: true.into(),
        validate: Some(validate),
        validation_behavior: behavior,
        ..UseTextFieldInput::new(state)
    });

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..label_props.into_attrs()}>{label}</label>
            <input class="demo-input demo-text-input" {..input_props.into_attrs()}/>
            <Show when=move || is_invalid.get()>
                <p class="demo-field-error" {..error_message_props.clone().into_attrs()}>
                    {move || validation_errors.get().join(" ")}
                </p>
            </Show>
        </div>
    }
}
