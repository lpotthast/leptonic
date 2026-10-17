use leptonic::hooks::form::{LabelElementType, UseLabelInput, UseLabelReturn, use_label};
use leptos::prelude::*;

#[component]
pub fn LabelSpanDemo() -> impl IntoView {
    let UseLabelReturn { label, field } = use_label(UseLabelInput {
        has_label: true.into(),
        label_element_type: LabelElementType::Span,
        ..UseLabelInput::default()
    });

    // A `<span>` can't label a `<div>` natively, so the field references it with `aria-labelledby`.
    view! {
        <div class="demo-field">
            <span class="demo-field-label" {..label.attrs()}>"Notes"</span>
            <div
                role="textbox"
                aria-multiline="true"
                contenteditable="true"
                class="demo-input demo-text-input"
                {..field.attrs()}
            ></div>
        </div>
    }
}
