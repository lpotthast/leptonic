use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

#[component]
pub fn SwitchStationaryDemo() -> impl IntoView {
    let (locked, set_locked) = signal(true);

    view! {
        <Switch
            state=(locked, set_locked)
            variant=SwitchVariant::Stationary
            icons=SwitchIcons { off: icondata::BsUnlock, on: icondata::BsLock }
        >
            "Lock the layout"
        </Switch>
        <p class="demo-status">{move || if locked.get() { "Locked" } else { "Unlocked" }}</p>
    }
}
