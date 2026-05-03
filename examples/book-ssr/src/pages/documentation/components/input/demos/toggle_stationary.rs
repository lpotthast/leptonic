use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

#[component]
pub fn ToggleStationaryDemo() -> impl IntoView {
    let (state, set_state) = signal(false);

    view! {
        <Toggle state=state set_state=set_state variant=ToggleVariant::Stationary icons=ToggleIcons {
            on: icondata::BsFolderFill,
            off: icondata::BsFolder,
        }/>
    }
}
