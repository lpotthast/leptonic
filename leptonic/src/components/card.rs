use leptos::prelude::*;

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! { <div class="leptonic-card">{children()}</div> }
}
