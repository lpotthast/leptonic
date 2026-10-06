use leptonic::{
    atoms::prelude::{Link, LinkRel},
    components::prelude::Icon,
    hooks::LinkTarget,
    prelude::icondata,
};
use leptos::prelude::*;

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
            // Decorative: the icon has no label, so assistive technology skips it.
            <Icon icon=icondata::BsBoxArrowUpRight classes="demo-link-icon"/>
        </Link>
        <p class="demo-caption">"Opens in a new tab."</p>
    }
}
