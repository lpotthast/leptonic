use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SwitchConceptDemo() -> impl IntoView {
    let wifi = RwSignal::new(true);
    let disabled = RwSignal::new(false);

    view! {
        <Switch is_selected=wifi set_selected=wifi is_disabled=disabled>"Wi-Fi"</Switch>
        <p class="demo-status">{move || if wifi.get() { "Wi-Fi is on." } else { "Wi-Fi is off." }}</p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
