use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

#[component]
pub fn IconDemo() -> impl IntoView {
    view! {
        <div class="demo-control-row">
            <Icon icon=icondata::BsFolderFill classes="demo-icon-large"/>
            <Icon icon=icondata::BsFolder classes="demo-icon-large"/>
        </div>
    }
}
