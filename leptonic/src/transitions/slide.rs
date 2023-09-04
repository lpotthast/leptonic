use leptos::*;

#[component]
pub fn Slide(inn: Signal<bool>, children: Children) -> impl IntoView {
    view! {
        <div class="leptonic-slide" data-in=move || inn.get()>
            { children() }
        </div>
    }
}
