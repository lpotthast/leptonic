use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LabelValidDemo() -> impl IntoView {
    let UseFieldReturn {
        label_props,
        field_props,
        description_props,
        ..
    } = use_field(UseFieldInput {
        has_label: true,
        ..UseFieldInput::default()
    });

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..label_props.into_attrs()}>"Email address"</label>
            <input type="email" class="demo-input demo-text-input" {..field_props.into_attrs()}/>
            <p class="demo-field-description" {..description_props.into_attrs()}>
                "We\u{2019}ll never share your email."
            </p>
        </div>
    }
}
