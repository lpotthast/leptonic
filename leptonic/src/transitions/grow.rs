use leptos::*;

#[component]
pub fn Grow(cx: Scope, inn: Signal<bool>, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-grow" data-in=move || inn.get()>
            { children(cx) }
        </div>
    }
}
