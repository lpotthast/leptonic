use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SwitchBasicDemo() -> impl IntoView {
    let airplane_mode = RwSignal::new(false);

    view! {
        <Switch is_selected=airplane_mode set_selected=airplane_mode>"Airplane mode"</Switch>
        <p class="demo-status">{move || if airplane_mode.get() { "Airplane mode is on." } else { "Airplane mode is off." }}</p>
    }
}
