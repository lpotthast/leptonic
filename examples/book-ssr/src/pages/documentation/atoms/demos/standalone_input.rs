use leptonic::atoms::{
    checkbox::{CheckboxButton, CheckboxField},
    input::Input,
};
use leptos::prelude::*;

/// Whether `tag` is a valid tag: letters, digits and dashes.
fn is_valid_tag(tag: &str) -> bool {
    tag.chars().all(|c| c.is_alphanumeric() || c == '-')
}

#[component]
pub fn StandaloneInputDemo() -> impl IntoView {
    let tag = RwSignal::new(String::new());
    let disabled = RwSignal::new(false);
    let is_invalid = Signal::derive(move || !tag.with(|tag| is_valid_tag(tag)));

    view! {
        // No field around it: the input is labelled and read like any `<input>`.
        <Input
            attr:aria-label="New tag"
            attr:aria-describedby="standalone-tag-status"
            attr:placeholder="New tag"
            on:input=move |e| tag.set(event_target_value(&e))
            is_invalid=is_invalid
            is_disabled=disabled
            classes="demo-atom-input"
        />

        <p id="standalone-tag-status" class="demo-status">
            {move || {
                if is_invalid.get() {
                    "Use letters, digits and dashes only.".to_owned()
                } else {
                    tag.with(|tag| if tag.is_empty() { "No tag yet.".to_owned() } else { format!("Tag: #{tag}") })
                }
            }}
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
