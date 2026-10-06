use leptonic::components::prelude::*;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::{
    app::{MAIN_ID, SITE_DESCRIPTION},
    nav::{NavGroup, PartKind, nav},
    routes,
};

#[component]
pub fn PageWelcome() -> impl IntoView {
    view! {
        <Title text="Leptonic \u{2013} accessible UI building blocks for Leptos"/>
        <Meta name="description" content=SITE_DESCRIPTION/>

        <main id=MAIN_ID class="book-welcome" tabindex="-1">
            <div class="book-welcome-intro">
                <h1 class="book-welcome-title">"Leptonic"</h1>
                <p class="book-welcome-tagline">
                    "Accessible UI building blocks for Leptos: hooks, atoms and themed components."
                </p>
                <div class="book-welcome-actions">
                    <LinkButton href=routes::doc::Installation.materialize() size=ButtonSize::Big>
                        "Get started"
                    </LinkButton>
                    <LinkButton
                        href=routes::doc::Overview.materialize()
                        size=ButtonSize::Big
                        variant=ButtonVariant::Outlined
                        color=ButtonColor::Secondary
                    >
                        "Read the overview"
                    </LinkButton>
                </div>
            </div>

            // The two questions the documentation is organized by, and the way in for newcomers. Generated from the
            // navigation, so that they always list its groups.
            <ul class="book-welcome-cards">
                <WelcomeCard
                    title="New to leptonic?"
                    href=routes::doc::Installation.materialize()
                    links=guide_links()
                >
                    "Install it, then read the guides every page builds on."
                </WelcomeCard>
                <WelcomeCard
                    title="Which UI element do I need?"
                    href=first_page(PartKind::Concepts)
                    links=group_links(PartKind::Concepts)
                >
                    "Concepts: every UI element, from buttons to tables, with its hooks, atoms and styled component."
                </WelcomeCard>
                <WelcomeCard
                    title="How do I give my own element a behavior?"
                    href=first_page(PartKind::BuildingBlocks)
                    links=group_links(PartKind::BuildingBlocks)
                >
                    "Building blocks: press, hover and focus handling, overlays, drag and drop and more, for any element."
                </WelcomeCard>
            </ul>

            <ul class="book-welcome-cards">
                <WelcomeCard title="Accessible" href=routes::doc::Accessibility.materialize() links=Vec::new()>
                    "Keyboard, pointer, touch and screen reader interaction, ported from react-aria\u{2019}s battle-tested hooks."
                </WelcomeCard>
                <WelcomeCard title="Three layers" href=routes::doc::Architecture.materialize() links=Vec::new()>
                    "Use the hooks for full control, unstyled atoms for your own design system, or themed components."
                </WelcomeCard>
                <WelcomeCard title="Rust all the way" href=routes::doc::Ssr.materialize() links=Vec::new()>
                    "Typed APIs, server-side rendering and hydration, and internationalization without a JS runtime."
                </WelcomeCard>
            </ul>
        </main>
    }
}

/// The groups of the sidebar part `kind`.
fn part_groups(kind: PartKind) -> Vec<&'static NavGroup> {
    nav()
        .parts
        .iter()
        .filter(|part| part.kind == kind)
        .flat_map(|part| &part.groups)
        .collect()
}

/// Where a group leads: its overview, or its first page.
fn group_href(group: &NavGroup) -> Option<String> {
    group
        .overview
        .clone()
        .or_else(|| group.entries.first().map(|entry| entry.href.clone()))
}

/// Links to the groups of the sidebar part `kind`.
fn group_links(kind: PartKind) -> Vec<(&'static str, String)> {
    part_groups(kind)
        .into_iter()
        .filter_map(|group| group_href(group).map(|href| (group.title, href)))
        .collect()
}

/// Links to the getting-started pages and the guides.
fn guide_links() -> Vec<(&'static str, String)> {
    part_groups(PartKind::Guides)
        .into_iter()
        .flat_map(|group| &group.entries)
        .map(|entry| (entry.title, entry.href.clone()))
        .collect()
}

/// The first page of the sidebar part `kind`.
fn first_page(kind: PartKind) -> String {
    part_groups(kind)
        .into_iter()
        .find_map(group_href)
        .unwrap_or_else(|| routes::doc::Overview.materialize())
}

/// A card of the welcome page: a linked title, a sentence, and further links (title and target).
#[component]
fn WelcomeCard(
    title: &'static str,
    href: String,
    links: Vec<(&'static str, String)>,
    children: Children,
) -> impl IntoView {
    let links = (!links.is_empty()).then(|| {
        view! {
            <ul class="book-welcome-links">
                {links
                    .into_iter()
                    .map(|(title, href)| view! { <li><Link href>{title}</Link></li> })
                    .collect_view()}
            </ul>
        }
    });
    view! {
        <li class="book-welcome-card">
            <h2><Link href>{title}</Link></h2>
            <p>{children()}</p>
            {links}
        </li>
    }
}
