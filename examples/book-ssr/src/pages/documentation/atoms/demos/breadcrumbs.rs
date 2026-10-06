use leptonic::{
    atoms::prelude::{Breadcrumb, Breadcrumbs, Link},
    components::prelude::Checkbox,
};
use leptos::prelude::*;

/// The trail of this page. The last item is the current page: it can't be followed.
#[component]
pub fn BreadcrumbsAtomDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    view! {
        // The separators are drawn in CSS, hidden from assistive technology.
        <nav aria-label="Breadcrumbs">
            <Breadcrumbs is_disabled=disabled classes="demo-breadcrumbs">
                <Breadcrumb classes="demo-breadcrumb">
                    <Link href="/doc/overview" classes="demo-breadcrumb-link">"Docs"</Link>
                </Breadcrumb>
                <Breadcrumb classes="demo-breadcrumb">
                    <Link href="/doc/navigation" classes="demo-breadcrumb-link">"Navigation"</Link>
                </Breadcrumb>
                <Breadcrumb classes="demo-breadcrumb">
                    <Link href="/doc/breadcrumbs" classes="demo-breadcrumb-link">"Breadcrumbs"</Link>
                </Breadcrumb>
                <Breadcrumb is_current=true classes="demo-breadcrumb">
                    <Link href="/doc/breadcrumbs/atom" classes="demo-breadcrumb-link">"Atoms"</Link>
                </Breadcrumb>
            </Breadcrumbs>
        </nav>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
