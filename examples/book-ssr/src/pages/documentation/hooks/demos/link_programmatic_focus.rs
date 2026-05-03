use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LinkProgrammaticFocusDemo() -> impl IntoView {
    let focus_link = use_link(UseLinkInput {
        href: Some("#focus-handle".to_string()),
        target: None,
        rel: vec![],
        is_disabled: Signal::default(),
        element_type: LinkElementType::default(),
        aria_current: None,
        on_press: None,
        on_press_start: None,
        on_press_end: None,
    });
    let focus_handle = focus_link.focus_handle;
    let (link_props, link_styles) = focus_link.props.into_inner();

    view! {
        <div>
            <a {..link_props.into_attrs()} style=link_styles.add("color", "var(--brand-color)")>
                "Target link"
            </a>
        </div>
        <button on:click=move |_| focus_handle.focus() style="margin-top: 0.5em;">
            "Focus the link above"
        </button>
    }
}
