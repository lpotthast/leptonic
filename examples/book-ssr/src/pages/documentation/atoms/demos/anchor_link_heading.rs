use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn AnchorLinkHeadingDemo() -> impl IntoView {
    view! {
        <div class="demo-navigation-frame">
            <h3 id="my-section-anchor">
                "My Section"
                // Without children, the link renders a single `#`.
                <AnchorLink href="#my-section-anchor" description="Direct link to section: My Section"/>
            </h3>
        </div>
    }
}
