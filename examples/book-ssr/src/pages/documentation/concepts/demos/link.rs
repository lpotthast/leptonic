use leptonic::{atoms::prelude::Link, hooks::LinkTarget};
use leptos::prelude::*;

#[component]
pub fn LinkConceptDemo() -> impl IntoView {
    view! {
        // A link to another site, opening in a new tab.
        <Link href="https://github.com/lpotthast/leptonic" target=LinkTarget::Blank classes="demo-link-atom">
            "Leptonic on GitHub"
        </Link>
    }
}
