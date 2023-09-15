use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageAnchor() -> impl IntoView {
    view! {
        <H1>"Anchor"</H1>

        <H3 id="#test-heading">
            "Test Heading"
            <Anchor href="#test-heading" title="The title describes this anchor." />
        </H3>
    }
}
