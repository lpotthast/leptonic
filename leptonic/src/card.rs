use leptos::*;

#[component]
pub fn Card(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <leptonic-card>
            { children(cx) }
        </leptonic-card>
    }
}
