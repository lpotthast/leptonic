use leptos::*;

#[component]
pub fn Grow(inn: Signal<bool>, children: Children) -> impl IntoView {
    view! {
        <div class="leptonic-grow" data-in=move || inn.get()>
            { children() }
        </div>
    }
}
