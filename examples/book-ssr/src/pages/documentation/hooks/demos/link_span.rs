use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LinkSpanDemo() -> impl IntoView {
    let presses = RwSignal::new(0_u32);

    let link = use_link(UseLinkInput {
        // Adds `role="link"` and `tabindex="0"`, so the span is announced and reachable as a link.
        element_type: LinkElementType::Other,
        on_press: Some(Callback::new(move |_| presses.update(|n| *n += 1))),
        ..UseLinkInput::default()
    });
    let (link_attrs, link_styles) = link.props.into_parts();

    view! {
        <p>
            <span {..link_attrs} class="demo-link demo-link-underlined" style=link_styles>
                "Press me, or focus me and press Enter"
            </span>
        </p>
        <p class="demo-status">
            {move || match presses.get() {
                1 => "Pressed 1 time".to_owned(),
                n => format!("Pressed {n} times"),
            }}
        </p>
    }
}
