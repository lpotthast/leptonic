use leptonic::atoms::checkbox::{CheckboxButton, CheckboxField};
use leptos::prelude::*;

#[component]
pub fn CheckboxConceptDemo() -> impl IntoView {
    let subscribed = RwSignal::new(false);
    let disabled = RwSignal::new(false);

    view! {
        // The field holds the state; its button is a `<label>` around a hidden input, whose children draw the box.
        <CheckboxField is_selected=subscribed set_selected=subscribed is_disabled=disabled>
            <CheckboxButton classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Subscribe to the newsletter"
            </CheckboxButton>
        </CheckboxField>
        <p class="demo-status">{move || if subscribed.get() { "Subscribed." } else { "Not subscribed." }}</p>
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
