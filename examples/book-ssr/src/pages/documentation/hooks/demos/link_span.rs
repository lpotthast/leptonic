use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LinkSpanDemo() -> impl IntoView {
    let span_link = use_link(UseLinkInput {
        href: None,
        target: None,
        rel: vec![],
        is_disabled: Signal::default(),
        element_type: LinkElementType::Span,
        aria_current: None,
        on_press: Some(Callback::new(|_| {
            leptos::logging::log!("Span link pressed!");
        })),
        on_press_start: None,
        on_press_end: None,
    });

    let (link_props, link_styles) = span_link.props.into_inner();

    view! {
        <div>
            <strong>"Span as Link: "</strong>
            <span
                {..link_props.into_attrs()}
                style=link_styles
                    .add("color", "var(--brand-color)")
                    .add("cursor", "pointer")
                    .add("text-decoration", "underline")
            >
                "Click or press Enter (check console)"
            </span>
        </div>
    }
}
