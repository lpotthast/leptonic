use leptonic::prelude::icondata;
use leptonic::{components::prelude::*, hooks::LinkTarget};
use leptos::prelude::*;

#[component]
pub fn LinkExternalDemo() -> impl IntoView {
    view! {
        <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkTarget::_Blank>
            // An icon-only link needs an accessible name.
            <Icon icon=icondata::BsGithub classes="demo-navigation-icon-large" aria_label="Leptonic on GitHub"/>
        </LinkExt>
    }
}
