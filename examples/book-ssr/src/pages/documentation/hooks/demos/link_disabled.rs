use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LinkDisabledDemo() -> impl IntoView {
    let (is_disabled, set_is_disabled) = signal(false);
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

        <label class="demo-checkbox-label">
            <input
                type="checkbox"
                prop:checked=is_disabled
                on:change=move |e| set_is_disabled.set(event_target_checked(&e))
            />
            "Disable link"
        </label>
    }
}
