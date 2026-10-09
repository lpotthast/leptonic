use leptonic::hooks::link::{UseLinkInput, use_link};
use leptos::prelude::*;

#[component]
pub fn LinkInternalDemo() -> impl IntoView {
    let link = use_link(UseLinkInput {
        href: Signal::stored(Some("/doc/link".to_owned())),
        ..UseLinkInput::default()
    });
    let (link_attrs, link_styles) = link.props.into_parts();

    view! {
        <p>
            "Back to the "
            <a {..link_attrs} class="demo-link" style=link_styles>"Link overview"</a>
            "."
        </p>
    }
}
