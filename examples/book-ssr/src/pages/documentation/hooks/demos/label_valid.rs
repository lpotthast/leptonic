use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LabelValidDemo() -> impl IntoView {
    let UseFieldReturn {
        label_props: field_label_props,
        field_props: field_input_props,
        description_props,
        error_message_props: _,
    } = use_field(UseFieldInput {
        id: None,
        label: Some("Email Address".into()),
        description: Some("We'll never share your email.".into()),
        error_message: None,
        validation_state: ValidationState::Valid,
        is_required: false,
        is_disabled: false,
        is_read_only: false,
    });

    view! {
        <label
            id=field_label_props.id.clone()
            for=field_label_props.html_for.clone()
            style="display: block; margin-bottom: 0.25em; font-weight: 500;"
        >
            "Email Address"
        </label>
        <input
            type="email"
            id=field_input_props.id.clone()
            aria-labelledby=field_input_props.aria_labelledby.clone()
            aria-describedby=field_input_props.aria_describedby.clone()
            aria-invalid=field_input_props.aria_invalid
            aria-required=field_input_props.aria_required
            style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px; width: 250px;"
        />
        <p
            id=description_props.id.clone()
            style="margin: 0.25em 0 0 0; font-size: 0.85em; color: #666;"
        >
            "We'll never share your email."
        </p>
    }
}
