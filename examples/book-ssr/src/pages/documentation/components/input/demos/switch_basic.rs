use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SwitchBasicDemo() -> impl IntoView {
    let (airplane_mode, set_airplane_mode) = signal(false);

    view! {
        <Switch state=(airplane_mode, set_airplane_mode)>"Airplane mode"</Switch>
        <p class="demo-status">{move || if airplane_mode.get() { "Airplane mode is on." } else { "Airplane mode is off." }}</p>
    }
}
