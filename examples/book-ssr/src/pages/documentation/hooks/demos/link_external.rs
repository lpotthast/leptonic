use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LinkExternalDemo() -> impl IntoView {
    let external_link = use_link(UseLinkInput {
        href: Some("https://leptos.dev".to_string()),
        target: Some(LinkTarget::_Blank),
        rel: vec![LinkRel::NoOpener, LinkRel::NoReferrer],
        is_disabled: Signal::default(),
        element_type: LinkElementType::default(),
        aria_current: None,
        on_press: None,
        on_press_start: None,
        on_press_end: None,
    });

    let (link_props, link_styles) = external_link.props.into_inner();

    view! {
        <div>
            <strong>"External Link: "</strong>
            <a {..link_props.into_attrs()} style=link_styles.add("color", "var(--brand-color)")>
                "Visit Leptos"
                <span style="margin-left: 0.25em;">{"\u{2197}"}</span>
            </a>
            <span style="margin-left: 0.5em; font-size: 0.875em; opacity: 0.7;">
                "(opens in new tab)"
            </span>
        </div>
    }
}
