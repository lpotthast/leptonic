use leptonic::components::prelude::*;
use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::{
    app::AppLayoutContext,
    nav::{NavEntry, NavSection, nav},
    sheet::{Sheet, SheetSide},
};

/// Layout of all documentation pages: the navigation next to the routed page. Small screens hide the sidebar; the
/// navigation opens as a menu [`Sheet`] from the app bar instead.
#[component]
pub fn DocLayout() -> impl IntoView {
    let app_layout = expect_context::<AppLayoutContext>();

    view! {
        <div id="book-doc-layout">
            <aside id="book-doc-sidebar">
                <DocNav/>
            </aside>

            <Sheet
                is_open=Signal::derive(move || app_layout.is_small.get() && app_layout.doc_menu_open.get())
                on_close=move |()| app_layout.doc_menu_open.set(false)
                label="Documentation"
                side=SheetSide::Left
            >
                <DocNav/>
            </Sheet>

            <Outlet/>
        </div>
    }
}

#[component]
fn DocNav() -> impl IntoView {
    view! {
        <nav class="book-doc-nav" aria-label="Documentation">
            {nav().sections.iter().map(|section| view! { <SidebarSection section/> }).collect_view()}
        </nav>
    }
}

#[component]
fn SidebarSection(section: &'static NavSection) -> impl IntoView {
    let header = view! {
        <Icon icon=section.icon/>
        {section.title}
    };

    view! {
        <div class="drawer-section">
            {match &section.overview {
                Some(overview) => view! {
                    <Link href=overview.clone() exact=true classes="section-header">{header}</Link>
                }.into_any(),
                None => view! { <div class="section-header">{header}</div> }.into_any(),
            }}
            <ul>
                {section
                    .entries
                    .iter()
                    .map(|entry| view! { <li><SidebarEntry entry badge=section.badges/></li> })
                    .collect_view()}
            </ul>
        </div>
    }
}

#[component]
fn SidebarEntry(entry: &'static NavEntry, badge: bool) -> impl IntoView {
    let badge = badge.then(|| entry.kind.badge()).flatten();
    view! {
        <Link href=entry.href.clone() classes="item">
            {badge.map(|badge| view! { <span class="doc-badge" data-kind=badge>{badge}</span> })}
            {entry.title}
        </Link>
    }
}
