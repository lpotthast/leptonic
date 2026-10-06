use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LabelInvalidDemo() -> impl IntoView {
    let UseFieldReturn {
        label_props,
        field_props,
        description_props,
        error_message_props,
        ..
    } = use_field(UseFieldInput {
        has_label: true,
        ..UseFieldInput::default()
    });

    // The field is always invalid in this demo, so the error message is always rendered. In a
    // real form, render it only while the value is invalid: its id is referenced only while the
    // element exists.
    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..label_props.into_attrs()}>"Password"</label>
            <input
                type="password"
                value="short"
                aria-invalid="true"
                class="demo-input demo-text-input"
                {..field_props.into_attrs()}
            />
            <p class="demo-field-description" {..description_props.into_attrs()}>"Minimum 8 characters."</p>
            <p class="demo-field-error" {..error_message_props.into_attrs()}>"Password is too short."</p>
        </div>
    }
}
