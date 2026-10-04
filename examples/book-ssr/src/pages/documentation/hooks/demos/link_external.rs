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
            <a {..link_props.into_attrs()} class="demo-link" style=link_styles>
                "Visit Leptos"
                <span class="demo-link-icon">{"\u{2197}"}</span>
            </a>
            <span class="demo-hint">"(opens in new tab)"</span>
        </div>
    }
}
