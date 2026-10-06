use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SwitchConceptDemo() -> impl IntoView {
    let (wifi, set_wifi) = signal(true);

    view! {
        <Switch state=(wifi, set_wifi)>"Wi-Fi"</Switch>
        <p class="demo-status">{move || if wifi.get() { "Wi-Fi is on." } else { "Wi-Fi is off." }}</p>
    }
}
