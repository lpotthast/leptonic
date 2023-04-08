use leptos::*;

#[component]
pub fn Fade(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-fade">
            { children(cx) }
        </div>
    }
}
