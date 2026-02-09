use leptos::prelude::*;

#[component]
pub fn Field(children: Children) -> impl IntoView {
    view! { <div class="leptonic-field">{children()}</div> }
}

#[component]
pub fn FieldLabel(children: Children) -> impl IntoView {
    view! { <div class="leptonic-field-label">{children()}</div> }
}
