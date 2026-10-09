use leptonic::atoms::{
    checkbox::{CheckboxButton, CheckboxField},
    toggle_button::ToggleButton,
};
use leptos::prelude::*;

#[component]
pub fn ToggleButtonAtomDemo() -> impl IntoView {
    let muted = RwSignal::new(false);
    let disabled = RwSignal::new(false);

    view! {
        <ToggleButton is_selected=muted set_selected=muted is_disabled=disabled classes="demo-atom-toggle-button">"Mute"</ToggleButton>
        <p class="demo-status">{move || if muted.get() { "Muted." } else { "Not muted." }}</p>
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
