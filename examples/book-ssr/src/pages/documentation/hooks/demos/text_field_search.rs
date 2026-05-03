use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn TextFieldSearchDemo() -> impl IntoView {
    let (search_value, set_search_value) = signal(String::new());

    let UseLabelReturn {
        label_props: search_label_props,
        field_props: search_field_props,
    } = use_label(UseLabelInput {
        id: None,
        label_element_type: Some(LabelElementType::Label),
    });

    view! {
        <label
            {..search_label_props.into_attrs()}
            style="display: block; font-weight: 500; margin-bottom: 0.25em;"
        >
            "Search"
        </label>
        <div style="display: flex; gap: 0.5em;">
            <input
                type="search"
                {..search_field_props.into_attrs()}
                placeholder="Search..."
                prop:value=move || search_value.get()
                on:input=move |ev| set_search_value.set(event_target_value(&ev))
                style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px; width: 250px;"
            />
            <button
                on:click=move |_| set_search_value.set(String::new())
                style="padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer;"
            >
                "Clear"
            </button>
        </div>
        <p style="margin: 0.5em 0 0 0; font-size: 0.85em;">
            "Press Enter to submit, Escape to clear"
        </p>
    }
}
