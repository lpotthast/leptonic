use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageAnchor(cx: Scope) -> impl IntoView {
    view! { cx,
        <H2>"Anchor"</H2>

        <Typography variant=TypographyVariant::H3 id="#test-heading">
            "Test Heading"
            <Anchor href="#test-heading" title="Direct link back to the page we are currently on." />
        </Typography>
    }
}
