use leptos::prelude::*;

#[component]
pub fn Field(children: Children) -> impl IntoView {
    view! {
        <leptonic-field>
            { children() }
        </leptonic-field>
    }
}

#[component]
pub fn FieldLabel(children: Children) -> impl IntoView {
    view! {
        <leptonic-field-label>
            { children() }
        </leptonic-field-label>
    }
}
