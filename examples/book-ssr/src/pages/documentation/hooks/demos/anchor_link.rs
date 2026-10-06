use leptonic::{ScrollBehavior, components::prelude::*, hooks::*};
use leptos::prelude::*;

#[component]
pub fn AnchorLinkDemo() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);

    let UseAnchorLinkReturn { props, .. } = use_anchor_link(UseAnchorLinkInput {
        href: Href::from("#my-anchor-element"),
        scroll_behavior: Some(ScrollBehavior::Smooth),
        is_disabled: disabled.into(),
        element_type: LinkElementType::Anchor,
        // The link only shows "#", so describe where it leads.
        description: Some(Oco::Borrowed("Jump to the anchor target")),
        on_press: None,
        on_press_start: None,
        on_press_end: None,
    });
    let (link_attrs, link_styles) = props.into_parts();

    view! {
        <a {..link_attrs} style=link_styles class="leptonic-anchor-link">"#"</a>

        <Checkbox state=(disabled, set_disabled) classes="demo-form-row">"Disabled"</Checkbox>

        <div id="my-anchor-element" class="demo-anchor-target">
            "This is the anchor target element."
        </div>
    }
}
