use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LabelSpanDemo() -> impl IntoView {
    let UseLabelReturn {
        label_props: span_label_props,
        field_props: span_field_props,
    } = use_label(UseLabelInput {
        id: None,
        label_element_type: Some(LabelElementType::Span),
    });

    view! {
        <span
            {..span_label_props.into_attrs()}
            style="display: block; margin-bottom: 0.25em; font-weight: 500;"
        >
            "Non-native control"
        </span>
        <div
            role="slider"
            tabindex="0"
            {..span_field_props.into_attrs()}
            style="width: 200px; height: 20px; background: #ddd; border-radius: 10px; cursor: pointer;"
        >
            <div style="width: 50%; height: 100%; background: var(--brand-color); border-radius: 10px;"></div>
        </div>
    }
}
