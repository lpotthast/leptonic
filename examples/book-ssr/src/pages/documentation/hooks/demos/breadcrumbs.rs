use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

use crate::routes;

/// The breadcrumb trail of this page.
#[component]
pub fn BreadcrumbsDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    let UseBreadcrumbsReturn { nav_props, .. } = use_breadcrumbs(UseBreadcrumbsInput {
        label: Some("Breadcrumbs".to_string()),
    });

    let docs = use_breadcrumb_item(UseBreadcrumbItemInput {
        href: Some(routes::doc::Overview.materialize()),
        is_disabled: disabled.into(),
        ..Default::default()
    });
    let navigation = use_breadcrumb_item(UseBreadcrumbItemInput {
        href: Some(routes::doc::Navigation.materialize()),
        is_disabled: disabled.into(),
        ..Default::default()
    });
    // The current page: no `href`, `aria-current="page"`.
    let current = use_breadcrumb_item(UseBreadcrumbItemInput {
        is_current: true,
        ..Default::default()
    });

    view! {
        <nav {..nav_props.into_attrs()} class="demo-navigation-breadcrumbs">
            <ol>
                <li>
                    <a {..docs.link_props.into_attrs()} class="demo-navigation-breadcrumb">"Docs"</a>
                    <span class="demo-navigation-separator" aria-hidden="true">"/"</span>
                </li>
                <li>
                    <a {..navigation.link_props.into_attrs()} class="demo-navigation-breadcrumb">"Navigation"</a>
                    <span class="demo-navigation-separator" aria-hidden="true">"/"</span>
                </li>
                <li>
                    <a {..current.link_props.into_attrs()} class="demo-navigation-breadcrumb">"use_breadcrumbs"</a>
                </li>
            </ol>
        </nav>

        <Checkbox state=disabled>"Disable links"</Checkbox>
    }
}
