use leptos::*;

#[component]
pub fn Skeleton(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-skeleton">
            { children(cx) }
        </div>
    }
}
