use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LabelInvalidDemo() -> impl IntoView {
    let UseFieldReturn {
        label_props: error_label_props,
        field_props: error_field_props,
        description_props: error_description_props,
        error_message_props,
    } = use_field(UseFieldInput {
        id: None,
        label: Some("Password".into()),
        description: Some("Minimum 8 characters.".into()),
        error_message: Some("Password is too short.".into()),
        validation_state: ValidationState::Invalid,
        is_required: false,
        is_disabled: false,
        is_read_only: false,
    });

    view! {
        <label
            id=error_label_props.id.clone()
            for=error_label_props.html_for.clone()
            style="display: block; margin-bottom: 0.25em; font-weight: 500;"
        >
            "Password"
        </label>
        <input
            type="password"
            {..error_field_props.into_attrs()}
            value="short"
            style="padding: 0.5em; border: 2px solid #dc3545; border-radius: 4px; width: 250px;"
        />
        <p
            id=error_description_props.id.clone()
            style="margin: 0.25em 0 0 0; font-size: 0.85em; color: #666;"
        >
            "Minimum 8 characters."
        </p>
        <p
            id=error_message_props.id.clone()
            role=error_message_props.role
            aria-live=error_message_props.aria_live
            style="margin: 0.25em 0 0 0; font-size: 0.85em; color: #dc3545; font-weight: 500;"
        >
            "Password is too short."
        </p>
    }
}
