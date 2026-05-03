use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn LabelBasicDemo() -> impl IntoView {
    let UseLabelReturn {
        label_props,
        field_props,
    } = use_label(UseLabelInput {
        id: None,
        label_element_type: Some(LabelElementType::Label),
    });

    view! {
        <div style="margin-bottom: 0.5em;">
            <label
                {..label_props.into_attrs()}
                style="display: block; margin-bottom: 0.25em; font-weight: 500;"
            >
                "Username"
            </label>
            <input
                type="text"
                {..field_props.into_attrs()}
                style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px; width: 200px;"
            />
        </div>
    }
}
