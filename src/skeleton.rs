use leptos::*;

#[component]
pub fn Skeleton(cx: Scope, #[prop(into, optional)] height: Option<String>, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-skeleton" height=height>
            { children(cx) }
        </div>
    }
}
