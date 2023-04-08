use leptos::*;

#[component]
pub fn Fade(cx: Scope, inn: Signal<bool>, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-fade" data-in=move || inn.get()>
            { children(cx) }
        </div>
    }
}
