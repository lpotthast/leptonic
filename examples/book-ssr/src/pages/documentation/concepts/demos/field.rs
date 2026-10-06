use std::sync::Arc;

use leptonic::{
    atoms::{
        field::{Description, FieldError, Label},
        input::Input,
        text_field::TextField,
    },
    hooks::ValidationBehavior,
};
use leptos::prelude::*;

#[component]
pub fn FieldConceptDemo() -> impl IntoView {
    let username = RwSignal::new("ferris".to_owned());

    view! {
        <TextField
            value=username
            set_value=username
            validate=Arc::new(|name: &String| {
                if name.trim().chars().count() < 3 {
                    Err(vec!["Use at least three characters.".to_owned()])
                } else {
                    Ok(())
                }
            })
            // Show errors while typing (outside a `Form`, they appear when the value is committed).
            validation_behavior=ValidationBehavior::Aria
            classes="demo-field"
        >
            // A `<label>` pointing at the input.
            <Label classes="demo-field-label">"Username"</Label>
            <Input classes="demo-atom-input"/>
            // Referenced by the input (`aria-describedby`) while rendered.
            <Description classes="demo-field-description">"Shown next to your posts."</Description>
            // Rendered only while the field is invalid, showing the validation errors.
            <FieldError classes="demo-field-error"/>
        </TextField>

        <p class="demo-status">
            {move || username.with(|name| if name.is_empty() { "No username entered.".to_owned() } else { format!("Username: {name}") })}
        </p>
    }
}
