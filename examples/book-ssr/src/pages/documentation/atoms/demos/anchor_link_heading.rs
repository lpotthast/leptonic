use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn AnchorLinkHeadingDemo() -> impl IntoView {
    view! {
        <div style="
            padding: 1em;
            border-radius: 0.25em;
            border: 0.1em solid var(--typography-code-background-color);
        ">
            <h2 id="my-section-anchor">
                "My Section"
                <AnchorLink href="#my-section-anchor" description="Direct link to section: My Section">#</AnchorLink>
            </h2>
        </div>
    }
}
