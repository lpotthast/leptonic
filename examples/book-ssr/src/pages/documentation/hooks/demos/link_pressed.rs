use leptonic::{hooks::*, utils::classes::Classes};
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
                class=Classes::from(["demo-link", "demo-pressable"])
                    .add_reactive("demo-pressed", pressed_link_is_pressed)
                style=link_styles
            >
                "Press and hold me"
            </a>
            <span class="demo-hint">
                {move || if pressed_link_is_pressed.get() { "(pressed!)" } else { "" }}
            </span>
        </div>
    }
}
