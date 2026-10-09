use leptonic::atoms::{
    checkbox::{CheckboxButton, CheckboxField},
    switch,
};
use leptos::prelude::*;

#[component]
pub fn SwitchAtomDemo() -> impl IntoView {
    let wifi = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);

    view! {
        // The field holds the state; its button is a `<label>` around a hidden input, whose children draw the track.
        <switch::SwitchField is_selected=wifi set_selected=wifi is_disabled=disabled is_read_only=read_only>
            <switch::SwitchButton classes="demo-switch">
                <span class="demo-switch-track" aria-hidden="true">
                    <span class="demo-switch-thumb"></span>
                </span>
                "Wi-Fi"
            </switch::SwitchButton>
        </switch::SwitchField>

        <p class="demo-status">{move || if wifi.get() { "Wi-Fi is on." } else { "Wi-Fi is off." }}</p>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
            <CheckboxField is_selected=read_only set_selected=read_only>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Read-only"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
