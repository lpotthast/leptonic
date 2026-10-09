use indoc::indoc;
use leptos::prelude::*;

use super::demos::breadcrumbs::BreadcrumbsAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomBreadcrumbs() -> impl IntoView {
    view! {
        <DocPage title="Breadcrumbs Atoms">
            <p>
                "The unstyled "<AnchorLink href="#breadcrumbs">"Breadcrumbs"</AnchorLink>" and "
                <AnchorLink href="#breadcrumb">"Breadcrumb"</AnchorLink>" render the trail of links from the top level "
                "down to the current page. See the "<Link href=routes::doc::Breadcrumbs.materialize()>"Breadcrumbs overview"</Link>
                " for concept guidance."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Hook"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Breadcrumbs"</Code></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-breadcrumbs", routes::doc::breadcrumbs::Hook.materialize())>"use_breadcrumbs"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Breadcrumb"</Code></TableCell>
                        <TableCell>
                            "None of its own: it hands the "<Link href=format!("{}#link", routes::doc::link::Atom.materialize())>"Link"</Link>
                            " inside it what "
                            <Link href=format!("{}#use-breadcrumb-item", routes::doc::breadcrumbs::Hook.materialize())>"use_breadcrumb_item"</Link>
                            " would give it: the current item is a disabled link with "<Code inline=true>"aria-current=\"page\""</Code>
                            ", the others are current only on their exact route."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::{breadcrumbs::{Breadcrumb, Breadcrumbs}, link::Link}};

                        view! {
                            <nav aria-label="Breadcrumbs">
                                <Breadcrumbs>
                                    <Breadcrumb><Link href="/">"Home"</Link></Breadcrumb>
                                    <Breadcrumb is_current=true><Link href="/docs">"Docs"</Link></Breadcrumb>
                                </Breadcrumbs>
                            </nav>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "The trail of this page, styled through the "<AnchorLink href="#data-attributes">"data attributes"</AnchorLink>
                    ". \u{201c}Disabled\u{201d} disables the whole trail with the "<Code inline=true>"is_disabled"</Code>" of "
                    <Code inline=true>"Breadcrumbs"</Code>"."
                </p>
                <Demo description="The breadcrumb trail of this page, with a Disabled checkbox" source=include_str!("demos/breadcrumbs.rs")>
                    <BreadcrumbsAtomDemo/>
                </Demo>
            </Section>

            <Section title="Breadcrumbs">
                <p>"An "<Code inline=true>"<ol>"</Code>" named \u{201c}Breadcrumbs\u{201d} by default. Wrap it in a "<Code inline=true>"<nav>"</Code>" landmark."</p>
                <Section title="Props" id="breadcrumbs-props">
                    <ApiTable kind=ApiKind::Props of="atoms::breadcrumbs::Breadcrumbs">
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="\"Breadcrumbs\"">"Names the list."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether all items are disabled."</ApiRow>
                        <ApiRow name="on_action" ty="Option<Callback<Key>>" default="None">
                            "Called with the "<Code inline=true>"id"</Code>" of a pressed item."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the list."</ApiRow>
                        <ApiRow name="children" ty="Children">"The "<Code inline=true>"Breadcrumb"</Code>"s. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Breadcrumb">
                <p>
                    "An "<Code inline=true>"<li>"</Code>" around a "<Code inline=true>"Link"</Code>". Mark the current page\u{2019}s "
                    "item (the last) "<Code inline=true>"is_current"</Code>": an item can\u{2019}t tell while it renders whether it "
                    "is the last one."
                </p>
                <Section title="Props" id="breadcrumb-props">
                    <ApiTable kind=ApiKind::Props of="atoms::breadcrumbs::Breadcrumb">
                        <ApiRow name="key" ty="Option<Key>" default="generated">"The item\u{2019}s key for "<Code inline=true>"on_action"</Code>"."</ApiRow>
                        <ApiRow name="is_current" ty="Signal<bool>" default="false">"Whether the item is the current page."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the item."</ApiRow>
                        <ApiRow name="children" ty="Children">"The item\u{2019}s "<Code inline=true>"Link"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-current" ty="true">"On the current "<Code inline=true>"Breadcrumb"</Code>"."</ApiRow>
                    <ApiRow name="data-disabled" ty="true">
                        "On disabled "<Code inline=true>"Breadcrumbs"</Code>", and on disabled or current items."
                    </ApiRow>
                </ApiTable>
                <p>
                    "The "<Code inline=true>"Link"</Code>" inside each item has the "
                    <Link href=format!("{}#data-attributes", routes::doc::link::Atom.materialize())>"Link\u{2019}s data attributes"</Link>
                    "; the current item\u{2019}s link is disabled too."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. "<Code inline=true>"Breadcrumbs"</Code>" renders the class "
                    <Code inline=true>"leptonic-Breadcrumbs"</Code>" on its list, "<Code inline=true>"Breadcrumb"</Code>" the "
                    "class "<Code inline=true>"leptonic-Breadcrumb"</Code>" on each item, each followed by the "
                    <Code inline=true>"classes"</Code>" you pass; the "<Code inline=true>"Link"</Code>" inside an item has "
                    "its own class. Draw the separators with CSS, hidden from assistive technology by an empty alternative "
                    "text. Style the current item after the disabled ones, as it is disabled as well. The book\u{2019}s "
                    "demos use these rules:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-breadcrumbs { display: flex; flex-wrap: wrap; gap: 0.5em; margin: 0; padding: 0; list-style: none; }
                        .my-breadcrumb:not(:last-child)::after { content: "/" / ""; margin-left: 0.5em; color: var(--muted); }
                        .my-breadcrumb-link { color: var(--accent); text-decoration: none; }
                        .my-breadcrumb-link[data-hovered] { text-decoration: underline; }
                        .my-breadcrumb-link[data-disabled] { color: var(--muted); cursor: not-allowed; }
                        .my-breadcrumb[data-current] .my-breadcrumb-link { color: inherit; font-weight: 600; cursor: default; }
                        .my-breadcrumb-link[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; }
                    "#)}
                </Code>
                <p>
                    "Leptonic also ships an optional atom theme that styles the default classes, for apps that don\u{2019}t "
                    "want to start from scratch: "<Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Breadcrumbs.materialize()>"Breadcrumbs overview"</Link></li>
                <li><Link href=routes::doc::breadcrumbs::Hook.materialize()>"Breadcrumbs Hooks"</Link></li>
                <li><Link href=routes::doc::link::Atom.materialize()>"Link Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
