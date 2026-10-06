use std::sync::Arc;

use leptonic::{
    components::prelude::*,
    hooks::{InputType, ValidationBehavior},
};
use leptos::prelude::*;

#[component]
pub fn TextFieldValidationDemo() -> impl IntoView {
    view! {
        <div class="demo-form">
            // `Aria` shows errors as you type.
            <TextField
                label="Username"
                description="3 to 16 letters, digits or underscores."
                is_required=true
                validation_behavior=ValidationBehavior::Aria
                validate=Arc::new(|username: &String| {
                    let mut errors = Vec::new();
                    if !(3..=16).contains(&username.chars().count()) {
                        errors.push("Use 3 to 16 characters.".to_owned());
                    }
                    if !username.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                        errors.push("Use only letters, digits and underscores.".to_owned());
                    }
                    if errors.is_empty() { Ok(()) } else { Err(errors) }
                })
            />
            // Without a `validation_behavior` (and outside a form), errors appear once you leave the changed field.
            <TextField
                label="Website"
                description="Checked by the browser when you leave the field."
                input_type=InputType::Url
                placeholder="https://example.com"
            />
        </div>
    }
}
