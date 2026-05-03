use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn TextFieldBasicDemo() -> impl IntoView {
    let (text_value, set_text_value) = signal(String::new());

    let UseLabelReturn {
        label_props: username_label_props,
        field_props: username_field_props,
    } = use_label(UseLabelInput {
        id: None,
        label_element_type: Some(LabelElementType::Label),
    });

    view! {
        <label
            {..username_label_props.into_attrs()}
            style="display: block; font-weight: 500; margin-bottom: 0.25em;"
        >
            "Username"
        </label>
        <input
            type="text"
            {..username_field_props.into_attrs()}
            placeholder="Enter username"
            prop:value=move || text_value.get()
            on:input=move |ev| set_text_value.set(event_target_value(&ev))
            maxlength="20"
            style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px; width: 250px;"
        />
        <p style="margin: 0.25em 0 0 0; font-size: 0.85em; color: #666;">
            "Choose a unique username."
        </p>
        <p style="margin: 0.5em 0 0 0; font-size: 0.85em;">
            "Value: " { move || text_value.get() } " (" { move || text_value.get().len() } "/20)"
        </p>
    }
}
