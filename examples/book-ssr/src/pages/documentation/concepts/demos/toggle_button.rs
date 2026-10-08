use leptonic::atoms::{checkbox::{CheckboxButton, CheckboxField}, toggle_button::ToggleButton};
use leptos::prelude::*;

#[component]
pub fn ToggleButtonConceptDemo() -> impl IntoView {
    let starred = RwSignal::new(false);
    let disabled = RwSignal::new(false);

    view! {
        <ToggleButton is_selected=starred set_selected=starred is_disabled=disabled classes="demo-atom-toggle-button">
            "Star"
        </ToggleButton>
        <p class="demo-status">{move || if starred.get() { "Starred." } else { "Not starred." }}</p>
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
