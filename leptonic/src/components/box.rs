use leptos::prelude::*;

#[component]
pub fn Box(
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-box>
            { children() }
        </leptonic-box>
    }
}
