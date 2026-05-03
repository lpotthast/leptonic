use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn AnchorLinkSimpleDemo() -> impl IntoView {
    view! {
        <AnchorLink href="#my-section-anchor">
            "My Section"
        </AnchorLink>
    }
}
