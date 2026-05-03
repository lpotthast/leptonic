use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn BreadcrumbsDemo() -> impl IntoView {
    let UseBreadcrumbsReturn { nav_props, .. } = use_breadcrumbs(UseBreadcrumbsInput {
        label: Some("Navigation".to_string()),
    });

    let home_item = use_breadcrumb_item(UseBreadcrumbItemInput {
        href: Some("#".to_string()),
        is_current: false,
        on_press: Some(Callback::new(|()| {})),
        ..Default::default()
    });

    let products_item = use_breadcrumb_item(UseBreadcrumbItemInput {
        href: Some("#".to_string()),
        is_current: false,
        on_press: Some(Callback::new(|()| {})),
        ..Default::default()
    });

    let current_item = use_breadcrumb_item(UseBreadcrumbItemInput {
        href: None,
        is_current: true,
        ..Default::default()
    });

    view! {
        <nav {..nav_props.into_attrs()} style="margin: 1em 0;">
            <ol style="display: flex; list-style: none; padding: 0; margin: 0; gap: 0.5em;">
                <li>
                    <a {..home_item.link_props.into_attrs()} style="color: var(--brand-color); text-decoration: none;">
                        "Home"
                    </a>
                    <span style="margin-left: 0.5em;">"/"</span>
                </li>
                <li>
                    <a {..products_item.link_props.into_attrs()} style="color: var(--brand-color); text-decoration: none;">
                        "Products"
                    </a>
                    <span style="margin-left: 0.5em;">"/"</span>
                </li>
                <li>
                    <span {..current_item.link_props.into_attrs()} style="color: inherit;">
                        "Widget Pro"
                    </span>
                </li>
            </ol>
        </nav>
    }
}
