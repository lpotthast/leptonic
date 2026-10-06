use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LinkSpanDemo() -> impl IntoView {
    let (presses, set_presses) = signal(0);

    let span_link = use_link(UseLinkInput {
        href: None,
        target: None,
        rel: vec![],
        is_disabled: Signal::default(),
        // Adds `role="link"`, so assistive technology announces the span as a link.
        element_type: LinkElementType::Span,
        aria_current: None,
        on_press: Some(Callback::new(move |_| set_presses.update(|n| *n += 1))),
        on_press_start: None,
        on_press_end: None,
    });

    let (link_attrs, link_styles) = span_link.props.into_parts();

    view! {
        <div>
            <strong>"Span as link: "</strong>
            <span {..link_attrs} class="demo-link demo-link-underlined" style=link_styles>
                "Click me or focus me and press Enter"
            </span>
        </div>
        <p class="demo-overlays-caption">{move || format!("Pressed {} times", presses.get())}</p>
    }
}
