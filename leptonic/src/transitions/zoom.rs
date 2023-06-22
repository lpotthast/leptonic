use leptos::*;

#[component]
pub fn Zoom(cx: Scope, inn: Signal<bool>, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-zoom" data-in=move || inn.get()>
            { children(cx) }
        </div>
    }
}
