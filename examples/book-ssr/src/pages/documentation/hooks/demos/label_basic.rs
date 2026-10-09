use leptonic::{
    IntoAttrs,
    hooks::form::{UseLabelInput, UseLabelReturn, use_label},
};
use leptos::prelude::*;

#[component]
pub fn LabelBasicDemo() -> impl IntoView {
    let UseLabelReturn {
        label_props,
        field_props,
    } = use_label(UseLabelInput {
        has_label: true.into(),
        ..UseLabelInput::default()
    });

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..label_props.into_attrs()}>"Username"</label>
            <input type="text" class="demo-input demo-text-input" {..field_props.into_attrs()}/>
        </div>
    }
}
