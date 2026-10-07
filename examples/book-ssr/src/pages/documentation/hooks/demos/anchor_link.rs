use leptonic::utils::scroll_behavior::ScrollBehavior;
use leptonic::{atoms::checkbox::Checkbox, hooks::*};
use leptos::prelude::*;

#[component]
pub fn AnchorLinkDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    // Scrolls smoothly by default.
    let link = use_anchor_link(UseAnchorLinkInput {
        link: UseLinkInput {
            is_disabled: disabled.into(),
            // The link only shows "#", so name where it leads.
            aria_label: "Jump to the returns policy".into(),
            ..UseLinkInput::default()
        },
        href: "#use-anchor-link-demo-target".into(),
        scroll_behavior: Some(ScrollBehavior::default()),
    });
    let (link_attrs, link_styles) = link.props.into_parts();

    view! {
        <p>
            "Returns "
            <a {..link_attrs} class="demo-link" style=link_styles>"#"</a>
        </p>
        <p>"Orders ship within two working days, in recyclable packaging."</p>
        <p id="use-anchor-link-demo-target" class="demo-anchor-target">"Return anything within 30 days."</p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}
