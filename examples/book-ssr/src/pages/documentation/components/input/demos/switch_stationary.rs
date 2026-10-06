use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

#[component]
pub fn SwitchStationaryDemo() -> impl IntoView {
    let locked = RwSignal::new(true);

    view! {
        <Switch
            is_selected=locked set_selected=locked
            variant=SwitchVariant::Stationary
            icons=SwitchIcons { off: icondata::BsUnlock, on: icondata::BsLock }
        >
            "Lock the layout"
        </Switch>
        <p class="demo-status">{move || if locked.get() { "The layout is locked." } else { "The layout is unlocked." }}</p>
    }
}
