use leptonic::atoms::{checkbox::Checkbox, switch};
use leptos::prelude::*;

#[component]
pub fn SwitchAtomDemo() -> impl IntoView {
    let wifi = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);

    view! {
        // The atom renders a `<label>` around a visually hidden input; the children draw the track.
        <switch::Switch is_selected=wifi set_selected=wifi is_disabled=disabled is_read_only=read_only classes="demo-switch">
            <span class="demo-switch-track" aria-hidden="true">
                <span class="demo-switch-thumb"></span>
            </span>
            "Wi-Fi"
        </switch::Switch>

        <p class="demo-status">{move || if wifi.get() { "Wi-Fi is on." } else { "Wi-Fi is off." }}</p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
            <Checkbox is_selected=read_only set_selected=read_only classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Read-only"
            </Checkbox>
        </div>
    }
}
