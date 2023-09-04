use leptos::*;

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <leptonic-card>
            { children() }
        </leptonic-card>
    }
}
