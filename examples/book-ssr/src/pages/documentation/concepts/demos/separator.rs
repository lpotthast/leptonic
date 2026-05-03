use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SeparatorConceptDemo() -> impl IntoView {
    view! {
        <p>"Content above"</p>
        <Separator />
        <p>"Content below"</p>
    }
}
