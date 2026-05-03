use leptos::prelude::*;
use leptos_classes::Classes;
use leptos_styles::Styles;

#[component]
pub fn Grow(
    inn: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=classes.add("leptonic-grow") style=styles data-in=move || inn.get()>
            {children()}
        </div>
    }
}
