use leptos::prelude::*;
use leptos_classes::Classes;
use leptos_styles::Styles;

#[component]
pub fn Card(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <div class=classes.add("leptonic-card") style=styles>{children()}</div> }
}
