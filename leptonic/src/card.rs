use leptos::*;

#[component]
pub fn Card(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-card">
            { children(cx) }
        </div>
    }
}
