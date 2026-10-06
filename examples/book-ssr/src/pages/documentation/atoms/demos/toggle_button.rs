use leptonic::{atoms::toggle_button::ToggleButton, components::prelude::Checkbox};
use leptos::prelude::*;

#[component]
pub fn ToggleButtonAtomDemo() -> impl IntoView {
    let muted = RwSignal::new(false);
    let disabled = RwSignal::new(false);

    view! {
        <ToggleButton state=muted is_disabled=disabled classes="demo-toggle-button">"Mute"</ToggleButton>
        <p class="demo-status">{move || if muted.get() { "Muted" } else { "Not muted" }}</p>
        <div class="demo-toggle-settings">
            <Checkbox state=disabled>"Disabled"</Checkbox>
        </div>
    }
}
