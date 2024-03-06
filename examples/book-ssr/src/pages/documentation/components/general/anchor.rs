use leptonic::components::prelude::*;
use leptos::*;

use crate::pages::documentation::article::Article;

#[component]
pub fn PageAnchor() -> impl IntoView {
    view! {
        <Article>
            <H1>"Anchor"</H1>

            <H3 id="#test-heading">
                "Heading followed by an AnchorLink"
                <AnchorLink href="#test-heading" description="The title describes this anchor." />
            </H3>
        </Article>
    }
}
