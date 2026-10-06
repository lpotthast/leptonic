use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn AnchorLinkComponentDemo() -> impl IntoView {
    view! {
        <h3 id="anchor-link-component-demo-heading">
            "Returns"
            // Without children, the link shows a `#`: name it.
            <AnchorLink href="#anchor-link-component-demo-heading" aria_label="Link to this section: Returns"/>
        </h3>
        <p>
            "Return anything within 30 days. "
            <AnchorLink href="#anchor-link-component-demo-heading">"Link to these terms"</AnchorLink>
        </p>
    }
}
