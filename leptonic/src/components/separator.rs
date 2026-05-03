use leptos::prelude::*;
use leptos_classes::Classes;
use leptos_styles::Styles;

// TODO: Allow more styles / variants

#[component]
pub fn Separator(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    view! { <hr class=classes.add("leptonic-separator").add("solid") style=styles /> }
}
