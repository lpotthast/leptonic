use leptonic::{
    atoms::prelude::{Link, LinkRel},
    hooks::LinkTarget,
};
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn LinkExternalAtomDemo() -> impl IntoView {
    view! {
        // `LinkTarget::Blank` opens a new tab (and adds `rel="noopener"`); `NoFollow` asks search engines not to
        // follow the link.
        <Link
            href="https://github.com/lpotthast/leptonic"
            target=LinkTarget::Blank
            rel=vec![LinkRel::NoFollow]
            classes="demo-link-atom"
        >
            "Leptonic on GitHub"
            // Decorative: hidden from assistive technology.
            <span class="demo-link-icon" aria-hidden="true"><Icon icon=icondata::BsBoxArrowUpRight/></span>
        </Link>
        <p class="demo-caption">"Opens in a new tab."</p>
    }
}
