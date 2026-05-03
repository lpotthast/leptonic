use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LinkPressedDemo() -> impl IntoView {
    let pressed_link = use_link(UseLinkInput {
        href: Some("#pressed-state".to_string()),
        target: None,
        rel: vec![],
        is_disabled: Signal::default(),
        element_type: LinkElementType::default(),
        aria_current: None,
        on_press: None,
        on_press_start: None,
        on_press_end: None,
    });
    let pressed_link_is_pressed = pressed_link.is_pressed;
    let (link_props, link_styles) = pressed_link.props.into_inner();

    view! {
        <div>
            <a
                {..link_props.into_attrs()}
                style=link_styles
                    .add("color", "var(--brand-color)")
                    .add("transition", "transform 100ms")
                    .add("transform", move || if pressed_link_is_pressed.get() { "scale(0.95)" } else { "scale(1)" })
            >
                "Press and hold me"
            </a>
            <span style="margin-left: 0.5em; font-size: 0.875em; opacity: 0.7;">
                {move || if pressed_link_is_pressed.get() { "(pressed!)" } else { "" }}
            </span>
        </div>
    }
}
