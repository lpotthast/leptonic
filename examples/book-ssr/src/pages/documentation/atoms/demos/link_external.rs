use leptonic::prelude::icondata;
use leptonic::{components::prelude::*, hooks::LinkTarget};
use leptos::prelude::*;

#[component]
pub fn LinkExternalDemo() -> impl IntoView {
    view! {
        <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkTarget::_Blank>
            <Icon attr:id="github-icon" icon=icondata::BsGithub attr:style="font-size: 3em;"/>
        </LinkExt>
    }
}
