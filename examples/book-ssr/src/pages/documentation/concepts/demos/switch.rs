use leptonic::atoms::{checkbox::Checkbox, switch::Switch};
use leptos::prelude::*;

#[component]
pub fn SwitchConceptDemo() -> impl IntoView {
    let wifi = RwSignal::new(true);
    let disabled = RwSignal::new(false);

    view! {
        // The atom renders a `<label>` around a visually hidden input; the children draw the track.
        <Switch is_selected=wifi set_selected=wifi is_disabled=disabled classes="demo-switch">
            <span class="demo-switch-track" aria-hidden="true"><span class="demo-switch-thumb"></span></span>
            "Wi-Fi"
        </Switch>
        <p class="demo-status">{move || if wifi.get() { "Wi-Fi is on." } else { "Wi-Fi is off." }}</p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}
