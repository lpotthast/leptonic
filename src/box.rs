use leptos::*;

#[component]
pub fn Box(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="box">
            { children(cx) }
        </div>
    }
}
