use leptos::*;

#[component]
pub fn Card(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="crud-card">
            { children(cx) }
        </div>
    }
}
