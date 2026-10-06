use leptonic::{atoms::prelude as atoms, components::prelude::*, prelude::icondata};
use leptos::prelude::*;
use leptos_router::{components::Outlet, hooks::use_location};

use crate::{
    app::AppLayoutContext,
    nav::{Layer, NavEntry, NavGroup, NavPart, PartKind, nav},
    sheet::{Sheet, SheetSide},
};

/// Layout of all documentation pages: the navigation next to the routed page. Small screens hide the sidebar; the
/// navigation opens as a menu [`Sheet`] from the app bar instead.
///
/// The routed page renders the `<main>` (and its table of contents next to it), see
/// [`DocPage`](crate::kit::DocPage).
#[component]
pub fn DocLayout() -> impl IntoView {
    let app_layout = expect_context::<AppLayoutContext>();

    view! {
        <div id="book-doc-layout">
            <div id="book-doc-sidebar">
                <DocNav/>
            </div>

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
            {nav().parts.iter().map(|part| view! { <SidebarPart part/> }).collect_view()}
        </nav>
    }
}

/// A part of the sidebar. Parts with a title ("Concepts", "Building blocks") start with it as a heading, so that
/// screen reader users can jump between them.
#[component]
fn SidebarPart(part: &'static NavPart) -> impl IntoView {
    view! {
        <div class="book-nav-part">
            {part.title.map(|title| view! { <h2 class="book-nav-part-title">{title}</h2> })}
            {part.groups.iter().map(|group| view! { <SidebarGroup group part=part.kind/> }).collect_view()}
        </div>
    }
}

/// A group of the sidebar. Groups with entries collapse; the guides start expanded, every other group while it
/// contains the current page.
#[component]
fn SidebarGroup(group: &'static NavGroup, part: PartKind) -> impl IntoView {
    let title = move || {
        view! {
            <Icon icon=group.icon/>
            {group.title}
        }
    };
    let overview_link = move |classes: &'static str| {
        group.overview.clone().map(|overview| {
            view! { <Link href=overview current_match=CurrentMatch::Exact classes=classes>{title()}</Link> }
        })
    };

    if group.entries.is_empty() {
        return view! {
            <div class="book-nav-group">
                {match overview_link("book-nav-group-header") {
                    Some(link) => link.into_any(),
                    None => view! { <div class="book-nav-group-header">{title()}</div> }.into_any(),
                }}
            </div>
        }
        .into_any();
    }

    let location = use_location();
    let expanded = RwSignal::new(
        part == PartKind::Guides
            || location
                .pathname
                .with_untracked(|path| group.contains(path)),
    );
    // Opens the group of the page navigated to. Groups the user expanded stay expanded.
    Effect::new(move |_| {
        if location.pathname.with(|path| group.contains(path)) {
            expanded.set(true);
        }
    });

    // Built inside the `Disclosure`: the trigger takes its context.
    let header = move || match overview_link("book-nav-group-title") {
        // The toggle sits next to the overview link, so it needs a name of its own ("Fields pages", next to the link
        // "Fields").
        Some(link) => view! {
            <atoms::DisclosureTrigger>
                <atoms::Button classes="book-nav-group-toggle" attr:aria-label=format!("{} pages", group.title)>
                    <Icon icon=icondata::BsChevronRight classes="book-nav-group-chevron"/>
                </atoms::Button>
            </atoms::DisclosureTrigger>
            {link}
        }
        .into_any(),
        None => view! {
            <atoms::DisclosureTrigger>
                <atoms::Button classes=["book-nav-group-toggle", "book-nav-group-title"]>
                    <Icon icon=icondata::BsChevronRight classes="book-nav-group-chevron"/>
                    {title()}
                </atoms::Button>
            </atoms::DisclosureTrigger>
        }
        .into_any(),
    };

    view! {
        <atoms::Disclosure is_expanded=expanded set_expanded=expanded classes="book-nav-group">
            <div class="book-nav-group-header">{header()}</div>
            <atoms::DisclosurePanel>
                <ul>
                    {group
                        .entries
                        .iter()
                        .map(|entry| view! { <li><SidebarEntry entry part/></li> })
                        .collect_view()}
                </ul>
            </atoms::DisclosurePanel>
        </atoms::Disclosure>
    }
    .into_any()
}

/// An entry of the sidebar. Concepts show the layers they are documented at; building blocks show their kind.
#[component]
fn SidebarEntry(entry: &'static NavEntry, part: PartKind) -> impl IntoView {
    let badge = (part == PartKind::BuildingBlocks)
        .then(|| entry.kind.badge())
        .flatten();
    let marks = part == PartKind::Concepts;
    view! {
        <Link href=entry.href.clone() classes="book-nav-item">
            {badge.map(|badge| view! { <span class="book-badge" data-kind=badge>{badge}</span> })}
            <span>{entry.title}</span>
            {marks.then(|| view! { <LayerMarks entry/> })}
        </Link>
    }
}

/// One letter per layer (H, A, C): filled for the layers the concept is documented at, plain for the others.
/// Decorative: the concept's page names its layers.
#[component]
fn LayerMarks(entry: &'static NavEntry) -> impl IntoView {
    let layers = entry.layers();
    let title = layers
        .iter()
        .map(|layer| layer.name())
        .collect::<Vec<_>>()
        .join(", ");
    let marks = Layer::ALL
        .into_iter()
        .map(|layer| {
            let present = layers.contains(&layer);
            view! { <span data-layer=layer.name() data-present=present.to_string()>{layer.letter()}</span> }
        })
        .collect_view();
    view! { <span class="book-layer-marks" aria-hidden="true" title=title>{marks}</span> }
}
