use leptos::*;

#[component]
pub fn Slide(cx: Scope, inn: Signal<bool>, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-slide" data-in=move || inn.get()>
            { children(cx) }
        </div>
    }
}
