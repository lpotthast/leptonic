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
            <a
                {..link_props.into_attrs()}
                style=link_styles
                    .add("color", move || if is_disabled.get() { "#999" } else { "var(--brand-color)" })
                    .add("cursor", move || if is_disabled.get() { "not-allowed" } else { "pointer" })
            >
                "This link can be disabled"
            </a>
        </div>

        <label style="display: flex; align-items: center; gap: 0.5em; cursor: pointer; margin-top: 0.5em;">
            <input
                type="checkbox"
                prop:checked=is_disabled
                on:change=move |e| set_is_disabled.set(event_target_checked(&e))
            />
            "Disable link"
        </label>
    }
}
