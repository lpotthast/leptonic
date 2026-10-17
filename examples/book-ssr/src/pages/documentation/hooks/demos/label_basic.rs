use leptonic::hooks::form::{UseLabelInput, UseLabelReturn, use_label};
use leptos::prelude::*;

#[component]
pub fn LabelBasicDemo() -> impl IntoView {
    let UseLabelReturn { label, field } = use_label(UseLabelInput {
        has_label: true.into(),
        ..UseLabelInput::default()
    });

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..label.attrs()}>"Username"</label>
            <input type="text" class="demo-input demo-text-input" {..field.attrs()}/>
        </div>
    }
}
