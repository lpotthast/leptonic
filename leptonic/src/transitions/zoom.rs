use leptos::*;

#[component]
pub fn Zoom(inn: Signal<bool>, children: Children) -> impl IntoView {
    view! {
        <div class="leptonic-zoom" data-in=move || inn.get()>
            { children() }
        </div>
    }
}
