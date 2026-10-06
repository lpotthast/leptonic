use leptonic::{components::prelude::*, hooks::LinkTarget};
use leptos::prelude::*;

#[component]
pub fn LinkConceptDemo() -> impl IntoView {
    view! {
        // `Link` is for pages of your app, `LinkExt` for other sites.
        <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkTarget::_Blank>
            "Leptonic on GitHub"
        </LinkExt>
    }
}
