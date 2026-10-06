use leptonic::{components::prelude::*, hooks::InputType};
use leptos::prelude::*;

const MESSAGE_MAX_LENGTH: u32 = 500;

#[component]
pub fn TextFieldDescriptionDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);

    view! {
        <div class="demo-form">
            <TextField
                label="Email"
                description="We send the receipt to this address."
                input_type=InputType::Email
                auto_complete="email"
                placeholder="ferris@example.com"
                is_disabled=disabled
                is_read_only=read_only
            />
            // `multiline` renders a `<textarea>`.
            <TextField
                label="Message"
                description="Optional, up to 500 characters."
                multiline=true
                max_length=MESSAGE_MAX_LENGTH
                is_disabled=disabled
                is_read_only=read_only
            />
        </div>

        <div class="demo-toggle-settings">
            <Checkbox state=disabled>"Disabled"</Checkbox>
            <Checkbox state=read_only>"Read-only"</Checkbox>
        </div>
    }
}
