use std::sync::Arc;

use leptonic::{
    atoms::{
        checkbox::{CheckboxButton, CheckboxField},
        field::{Description, FieldError, Label},
        input::{Input, TextArea},
        text_field::TextField,
    },
    hooks::form::ValidationBehavior,
};
use leptos::prelude::*;

const BIO_MAX: u32 = 160;

#[component]
pub fn TextFieldAtomDemo() -> impl IntoView {
    let name = RwSignal::new("Ferris".to_owned());
    let bio = RwSignal::new(String::new());
    let disabled = RwSignal::new(false);

    view! {
        <div class="demo-form">
            <TextField
                value=name set_value=name
                is_required=true
                validate=Arc::new(|name: &String| {
                    if name.trim().is_empty() { Err(vec!["Enter a display name.".to_owned()]) } else { Ok(()) }
                })
                // Outside a `Form`, errors appear when the value is committed; `Aria` shows them as you type.
                validation_behavior=ValidationBehavior::Aria
                is_disabled=disabled
                classes="demo-field"
            >
                <Label classes="demo-field-label">"Display name"</Label>
                <Input classes="demo-atom-input"/>
                <FieldError classes="demo-field-error"/>
            </TextField>

            <TextField
                value=bio set_value=bio
                max_length=BIO_MAX
                placeholder="A few words about you"
                is_disabled=disabled
                classes="demo-field"
            >
                <Label classes="demo-field-label">"Bio"</Label>
                <TextArea classes="demo-atom-input"/>
                <Description classes="demo-field-description">
                    {move || format!("{} of {BIO_MAX} characters", bio.with(|bio| bio.chars().count()))}
                </Description>
            </TextField>
        </div>

        <p class="demo-status">
            {move || name.with(|name| {
                let name = name.trim();
                if name.is_empty() { "Hello, stranger!".to_owned() } else { format!("Hello, {name}!") }
            })}
        </p>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
