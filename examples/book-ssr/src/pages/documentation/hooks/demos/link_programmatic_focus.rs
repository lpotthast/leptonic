use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

#[component]
pub fn LinkProgrammaticFocusDemo() -> impl IntoView {
    let focus_link = use_link(UseLinkInput {
        href: Some("#programmatic-focus".to_string()),
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
            <a {..link_props.into_attrs()} class="demo-link" style=link_styles>
                "Target link"
            </a>
        </div>
        <Button on_press=move |_| focus_handle.focus() classes="demo-mt-half">"Focus the link above"</Button>
    }
}
