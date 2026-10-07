use leptonic::atoms::separator::Separator;
use leptos::prelude::*;

#[component]
pub fn SeparatorConceptDemo() -> impl IntoView {
    view! {
        <p>"Content above"</p>
        // An <hr>; the class draws the line.
        <Separator classes="demo-separator-line"/>
        <p>"Content below"</p>
    }
}
