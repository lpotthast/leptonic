use leptos::*;

#[component]
pub fn Box(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-box">
            { children(cx) }
        </div>
    }
}
