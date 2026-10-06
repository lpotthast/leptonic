use std::sync::Arc;

use leptonic::{
    atoms::{
        field::{Description, FieldError, Label},
        input::{Input, TextArea},
        text_field::TextField,
    },
    components::prelude::Checkbox,
    hooks::{TextFieldState, ValidationBehavior},
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
                state=TextFieldState::from(name)
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
                <Input classes=["demo-input", "demo-text-input", "demo-atom-input"]/>
                <FieldError classes="demo-field-error"/>
            </TextField>

            <TextField
                state=TextFieldState::from(bio)
                max_length=BIO_MAX
                placeholder="A few words about you"
                is_disabled=disabled
                classes="demo-field"
            >
                <Label classes="demo-field-label">"Bio"</Label>
                <TextArea classes=["demo-input", "demo-text-input", "demo-atom-input"]/>
                <Description classes="demo-field-description">
                    {move || format!("{} of {BIO_MAX} characters", bio.with(|bio| bio.chars().count()))}
                </Description>
            </TextField>
        </div>

        <p class="demo-status">{move || format!("Hello, {}!", name.get())}</p>

        <div class="demo-toggle-settings">
            <Checkbox state=disabled>"Disabled"</Checkbox>
        </div>
    }
}
