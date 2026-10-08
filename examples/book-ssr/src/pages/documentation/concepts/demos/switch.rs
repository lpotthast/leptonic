use leptonic::atoms::{checkbox::{CheckboxButton, CheckboxField}, switch::{SwitchButton, SwitchField}};
use leptos::prelude::*;

#[component]
pub fn SwitchConceptDemo() -> impl IntoView {
    let wifi = RwSignal::new(true);
    let disabled = RwSignal::new(false);

    view! {
        // The field holds the state; its button is a `<label>` around a hidden input, whose children draw the track.
        <SwitchField is_selected=wifi set_selected=wifi is_disabled=disabled>
            <SwitchButton classes="demo-switch">
                <span class="demo-switch-track" aria-hidden="true"><span class="demo-switch-thumb"></span></span>
                "Wi-Fi"
            </SwitchButton>
        </SwitchField>
        <p class="demo-status">{move || if wifi.get() { "Wi-Fi is on." } else { "Wi-Fi is off." }}</p>
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
