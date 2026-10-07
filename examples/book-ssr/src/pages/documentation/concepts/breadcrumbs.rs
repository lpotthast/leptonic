use leptos::prelude::*;

use super::demos::breadcrumbs::BreadcrumbsConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageBreadcrumbsOverview() -> impl IntoView {
    view! {
        <DocPage title="Breadcrumbs">
            <p>
                "Breadcrumbs show the trail of links from the start page to the current page, so users see where they are and "
                "can go back up the hierarchy. The last item is the current page; it is marked as such for screen readers "
                "and is not a link."
            </p>
            <p>
                "Each item is a "<Link href=routes::doc::Link.materialize()>"link"</Link>", with all its behavior: client-side "
                "routing, disabling and press events. Separators between the items are decoration and stay hidden from "
                "assistive technology."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Show where a page sits in a hierarchy of pages and lead back up"</TableCell>
                        <TableCell><b>"Breadcrumbs"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Link to a single other page"</TableCell>
                        <TableCell><Link href=routes::doc::Link.materialize()>"Link"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Switch between views of the same page"</TableCell>
                        <TableCell><Link href=routes::doc::Tabs.materialize()>"Tabs"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Offer a list of actions or destinations behind a button"</TableCell>
                        <TableCell><Link href=routes::doc::Menu.materialize()>"Menu"</Link></TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "Breadcrumbs add to the main navigation, they don\u{2019}t replace it. They pay off from two levels "
                    "below the start page; in a flat site, they only repeat the page title."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::breadcrumbs::Hook.materialize()>"Breadcrumbs Hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_breadcrumbs"</Code>" names the trail; "
                            <Code inline=true>"use_breadcrumb_item"</Code>" makes each item a link and the current one a "
                            "disabled link with "<Code inline=true>"aria-current"</Code>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::breadcrumbs::Atom.materialize()>"Breadcrumbs Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"Breadcrumbs"</Code>" (an "<Code inline=true>"<ol>"</Code>") and "
                            <Code inline=true>"Breadcrumb"</Code>" (an "<Code inline=true>"<li>"</Code>" around a "
                            <Link href=format!("{}#link", routes::doc::link::Atom.materialize())>"Link"</Link>"), styled through data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Wrap the atoms in a "<Code inline=true>"<nav>"</Code>", put a "<Code inline=true>"Link"</Code>" into each "
                    <Code inline=true>"Breadcrumb"</Code>" and mark the last one "<Code inline=true>"is_current"</Code>
                    ". The trail of this page; the separators are drawn with CSS:"
                </p>
                <Demo
                    description="Breadcrumb trail of three links built from the atoms, the last one the current page"
                    source=include_str!("demos/breadcrumbs.rs")
                    source_open=true
                >
                    <BreadcrumbsConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Breadcrumbs follow the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/breadcrumb/" target=LinkTarget::Blank>"Breadcrumb pattern"</Link>":"
                </p>
                <ul>
                    <li>
                        "The trail is an ordered list, named \u{201c}Breadcrumbs\u{201d} by default ("
                        <Code inline=true>"aria_label"</Code>"), inside a "<Code inline=true>"<nav>"</Code>" landmark that "
                        "you render around it. Name the landmark too ("<Code inline=true>"aria-label=\"Breadcrumbs\""</Code>
                        "), so that it stands apart from the page\u{2019}s other navigations."
                    </li>
                    <li>
                        "The current item is a disabled link with "<Code inline=true>"aria-current=\"page\""</Code>
                        " and no "<Code inline=true>"href"</Code>", so it is announced as the current page and skipped when "
                        "tabbing."
                    </li>
                    <li>
                        "Disabled items get "<Code inline=true>"aria-disabled=\"true\""</Code>" and no "
                        <Code inline=true>"href"</Code>"."
                    </li>
                    <li>
                        "Hide separators from assistive technology: with "<Code inline=true>"aria-hidden"</Code>" on separator "
                        "elements, or with an empty alternative text in CSS ("<Code inline=true>"content: \"/\" / \"\""</Code>")."
                    </li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab / Shift + Tab">"Moves focus to the next or previous link of the trail."</KeyRow>
                    <KeyRow keys="Enter">"Follows the focused link."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::breadcrumbs::Hook.materialize()>"Breadcrumbs Hooks"</Link></li>
                <li><Link href=routes::doc::breadcrumbs::Atom.materialize()>"Breadcrumbs Atoms"</Link></li>
                <li><Link href=routes::doc::Link.materialize()>"Link"</Link></li>
                <li><Link href=routes::doc::Navigation.materialize()>"Navigation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
