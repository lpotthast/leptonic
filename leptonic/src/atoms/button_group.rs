use leptos::prelude::*;

#[component]
pub fn ButtonGroup(
    children: Children,
) -> impl IntoView {
    // TODO: Manage focus through something like `useFocusContainer`?

    view! {
        <leptonic-btn-group>
            { children() }
        </leptonic-btn-group>
    }
}
