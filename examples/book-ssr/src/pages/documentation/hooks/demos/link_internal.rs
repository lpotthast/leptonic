use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LinkInternalDemo() -> impl IntoView {
    let internal_link = use_link(UseLinkInput {
        href: Some("#demo".to_string()),
        target: None,
        rel: vec![],
        is_disabled: Signal::default(),
        element_type: LinkElementType::default(),
        aria_current: None,
        on_press: None,
        on_press_start: None,
        on_press_end: None,
    });

    let (link_props, link_styles) = internal_link.props.into_inner();

    view! {
        <div>
            <strong>"Internal Link: "</strong>
            <a {..link_props.into_attrs()} style=link_styles.add("color", "var(--brand-color)")>
                "Jump to demo section"
            </a>
        </div>
    }
}
