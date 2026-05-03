use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

#[component]
pub fn ToggleIconsDemo() -> impl IntoView {
    let (state, set_state) = signal(false);

    view! {
        <Toggle state=state set_state=set_state icons=ToggleIcons {
            on: icondata::BsFolderFill,
            off: icondata::BsFolder,
        }/>
    }
}
