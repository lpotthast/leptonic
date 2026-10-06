use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

#[component]
pub fn LinkDisabledDemo() -> impl IntoView {
    let is_disabled = RwSignal::new(false);
    let disabled_link = use_link(UseLinkInput {
        href: Some("#".to_string()),
        target: None,
        rel: vec![],
        is_disabled: is_disabled.into(),
        element_type: LinkElementType::default(),
        aria_current: None,
        on_press: None,
        on_press_start: None,
        on_press_end: None,
    });

    let (link_props, link_styles) = disabled_link.props.into_inner();

    view! {
        <div>
            <strong>"Disabled Link: "</strong>
            // `.demo-link` greys itself out via `[aria-disabled="true"]`, which the hook sets.
            <a {..link_props.into_attrs()} class="demo-link" style=link_styles>
                "This link can be disabled"
            </a>
        </div>

        <Checkbox state=is_disabled>"Disable link"</Checkbox>
    }
}
