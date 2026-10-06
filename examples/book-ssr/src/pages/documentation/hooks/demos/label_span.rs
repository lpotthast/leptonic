use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LabelSpanDemo() -> impl IntoView {
    let UseLabelReturn {
        label_props,
        field_props,
    } = use_label(UseLabelInput {
        has_label: true,
        label_element_type: LabelElementType::Span,
        ..UseLabelInput::default()
    });

    // A `<span>` can't label a `<div>` natively, so the field references it with `aria-labelledby`.
    view! {
        <div class="demo-field">
            <span class="demo-field-label" {..label_props.into_attrs()}>"Notes"</span>
            <div
                role="textbox"
                contenteditable="true"
                class="demo-input demo-text-input"
                {..field_props.into_attrs()}
            ></div>
        </div>
    }
}
