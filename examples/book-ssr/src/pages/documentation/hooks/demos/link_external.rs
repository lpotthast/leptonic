use leptonic::hooks::link::{LinkRel, LinkTarget, UseLinkInput, use_link};
use leptos::prelude::*;

#[component]
pub fn LinkExternalDemo() -> impl IntoView {
    let link = use_link(UseLinkInput {
        href: Signal::stored(Some("https://leptos.dev".to_owned())),
        target: LinkTarget::Blank.into(),
        // `NoOpener` is added for `LinkTarget::Blank`.
        rel: vec![LinkRel::NoReferrer],
        ..UseLinkInput::default()
    });
    let (link_attrs, link_styles) = link.props.into_parts();

    view! {
        <p>
            <a {..link_attrs} class="demo-link" style=link_styles>
                "Visit Leptos"
                <span class="demo-link-icon" aria-hidden="true">"\u{2197}"</span>
            </a>
        </p>
        <p class="demo-caption">"Opens in a new tab."</p>
    }
}
