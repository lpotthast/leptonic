use leptos::prelude::*;

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! { <leptonic-card>{children()}</leptonic-card> }
}
