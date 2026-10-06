use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

#[component]
pub fn SwitchIconsDemo() -> impl IntoView {
    let night_mode = RwSignal::new(false);

    view! {
        <Switch
            is_selected=night_mode set_selected=night_mode
            icons=SwitchIcons { off: icondata::BsSun, on: icondata::BsMoon }
        >
            "Night mode"
        </Switch>
        <p class="demo-status">{move || if night_mode.get() { "Night mode is on." } else { "Night mode is off." }}</p>
    }
}
