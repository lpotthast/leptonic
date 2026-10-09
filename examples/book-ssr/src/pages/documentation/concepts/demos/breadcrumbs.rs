use leptonic::atoms::{
    breadcrumbs::{Breadcrumb, Breadcrumbs},
    link::Link,
};
use leptos::prelude::*;

#[component]
pub fn BreadcrumbsConceptDemo() -> impl IntoView {
    view! {
        <nav aria-label="Breadcrumbs">
            <Breadcrumbs classes="demo-breadcrumbs">
                <Breadcrumb classes="demo-breadcrumb">
                    <Link href="/doc/overview" classes="demo-breadcrumb-link">"Docs"</Link>
                </Breadcrumb>
                <Breadcrumb classes="demo-breadcrumb">
                    <Link href="/doc/navigation" classes="demo-breadcrumb-link">"Navigation"</Link>
                </Breadcrumb>
                // The current page: announced as such, and not a working link.
                <Breadcrumb is_current=true classes="demo-breadcrumb">
                    <Link href="/doc/breadcrumbs" classes="demo-breadcrumb-link">"Breadcrumbs"</Link>
                </Breadcrumb>
            </Breadcrumbs>
        </nav>
    }
}
