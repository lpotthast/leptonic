use leptonic::{
    atoms::button::Button,
    hooks::link::{UseLinkInput, use_link},
};
use leptos::prelude::*;

#[component]
pub fn LinkProgrammaticFocusDemo() -> impl IntoView {
    let link = use_link(UseLinkInput {
        href: Signal::stored(Some("/doc/link".to_owned())),
        ..UseLinkInput::default()
    });
    let focus_handle = link.focus_handle;
    let is_focused = link.is_focused;
    let (link_attrs, link_styles) = link.props.into_parts();

    view! {
        <p><a {..link_attrs} class="demo-link" style=link_styles>"Link overview"</a></p>
        <Button on_press=move |_| focus_handle.focus() classes="demo-btn">"Focus the link"</Button>
        <p class="demo-status">{move || if is_focused.get() { "The link has focus." } else { "The link doesn\u{2019}t have focus." }}</p>
    }
}
