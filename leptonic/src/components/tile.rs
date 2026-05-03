use leptos::prelude::*;
use leptos_classes::Classes;
use leptos_styles::Styles;

#[component]
pub fn Tile(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <div class=classes.add("leptonic-tile") style=styles>{children()}</div> }
}
