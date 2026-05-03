use leptonic::{components::prelude::*, hooks::LinkTarget};
use leptos::prelude::*;

#[component]
pub fn LinkRelDemo() -> impl IntoView {
    view! {
        <LinkExt
            href="https://example.com"
            target=LinkTarget::_Blank
            rel=vec![LinkRel::NoFollow, LinkRel::NoReferrer]
        >
            "Link with nofollow and noreferrer"
        </LinkExt>
    }
}
