use leptonic::{
    atoms::field::{Description, FieldContext, FieldError, FieldLabelProps, Label, TextElement},
    hooks::{IntoAttrs, UseFieldInput, UseFieldReturn, ValidityStateSnapshot, use_field},
};
use leptos::{context::Provider, prelude::*};

fn validate(username: &str) -> Vec<String> {
    let mut errors = Vec::new();
    if !(3..=16).contains(&username.chars().count()) {
        errors.push("Use 3 to 16 characters.".to_owned());
    }
    if !username.chars().all(char::is_alphanumeric) {
        errors.push("Use only letters and digits.".to_owned());
    }
    errors
}

/// A text field built from `use_field`. It provides a `FieldContext`, so the `Label`,
/// `Description` and `FieldError` atoms inside it label and describe its input.
#[component]
pub fn FieldCustomDemo() -> impl IntoView {
    let username = RwSignal::new("leptos".to_owned());
    let validation_errors = Signal::derive(move || validate(&username.get()));
    let is_invalid = Signal::derive(move || !validation_errors.get().is_empty());

    let UseFieldReturn {
        label_props,
        field_props,
        description_props,
        error_message_props,
        ..
    } = use_field(UseFieldInput {
        has_label: true,
        ..UseFieldInput::default()
    });

    let field = FieldContext {
        // A `<label for=..>`: the input is a native form control.
        label: FieldLabelProps::label(label_props),
        description: description_props,
        error_message: error_message_props,
        is_invalid,
        validation_errors,
        // The validation is custom: no native constraint applies.
        validation_details: Signal::derive(move || ValidityStateSnapshot {
            custom_error: is_invalid.get(),
            valid: !is_invalid.get(),
            ..ValidityStateSnapshot::default()
        }),
    };

    view! {
        <Provider value=field>
            <div class="demo-field">
                <Label classes="demo-field-label">"Username"</Label>
                <input
                    type="text"
                    class="demo-input demo-text-input"
                    aria-invalid=move || is_invalid.get().then_some("true")
                    prop:value=username
                    on:input=move |ev| username.set(event_target_value(&ev))
                    {..field_props.into_attrs()}
                />
                <Description classes="demo-field-description">"Shown on your public profile."</Description>
                // Custom children replace the joined errors; `Div` allows block content.
                <FieldError element=TextElement::Div classes="demo-field-error">
                    <ul class="demo-field-error-list">
                        {move || validation_errors.get().into_iter().map(|error| view! { <li>{error}</li> }).collect_view()}
                    </ul>
                </FieldError>
            </div>
        </Provider>
    }
}
