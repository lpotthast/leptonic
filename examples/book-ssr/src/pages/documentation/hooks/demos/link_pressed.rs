use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LinkPressedDemo() -> impl IntoView {
    let link = use_link(UseLinkInput {
        // A link that runs code instead of navigating, so holding it doesn't leave the page.
        element_type: LinkElementType::Other,
        ..UseLinkInput::default()
    });
    let is_pressed = link.is_pressed;
    let (link_attrs, link_styles) = link.props.into_parts();

    view! {
        // The hook's `is_pressed`, rendered as an attribute for the styles.
        <p>
            <span
                {..link_attrs}
                class="demo-link demo-link-underlined demo-pressable"
                data-pressed=move || is_pressed.get().then_some("true")
                style=link_styles
            >
                "Press and hold me"
            </span>
        </p>
        <p class="demo-status">{move || if is_pressed.get() { "Pressed" } else { "Not pressed" }}</p>
    }
}
