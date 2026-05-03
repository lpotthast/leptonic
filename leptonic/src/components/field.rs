use leptos::prelude::*;
use leptos_classes::Classes;
use leptos_styles::Styles;

#[component]
pub fn Field(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <div class=classes.add("leptonic-field") style=styles>{children()}</div> }
}

#[component]
pub fn FieldLabel(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <div class=classes.add("leptonic-field-label") style=styles>{children()}</div> }
}
