use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageOverview() -> impl IntoView {
    view! {
        <DocPage title="Overview">
            <p>
                "Leptonic is an accessible UI library for the "
                <Link href="https://leptos.dev/" target=LinkTarget::Blank>"Leptos"</Link>" web framework."
            </p>

            <p>
                "It comes in two layers. "<b>"Hooks"</b>", ported from "
                <Link href="https://react-spectrum.adobe.com/react-aria/" target=LinkTarget::Blank>"react-aria"</Link>
                ", implement interaction and accessibility: pressing, hovering, keyboard navigation, focus management, "
                "selection, overlays and the ARIA patterns of menus, listboxes, sliders and more. "<b>"Atoms"</b>
                " are unstyled Leptos components that apply hooks to one element each: buttons, text fields, selects, "
                "sliders, date pickers, menus, dialogs, toasts, tabs, tables and more. They bring no styles: you style "
                "them with your own CSS, through their default classes and the data attributes of their state, or start "
                "from leptonic\u{2019}s optional atom theme. "
                <Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                " explains the layers and helps you pick the one that fits your project; "
                <Link href=routes::doc::Themes.materialize()>"Themes"</Link>" shows how to style atoms."
            </p>

            <p>
                "Get started with the "<Link href=routes::doc::Installation.materialize()>"Installation"</Link>
                " instructions, or browse the concepts in the sidebar. Every page is also available as Markdown: append "
                <Code inline=true>".md"</Code>" to its URL, or start at the "
                <Link href="/doc/llm-index.md" rel=vec![LinkRel::External]>"index of all pages"</Link>"."
            </p>

            <Section title="How the Documentation Is Organized">
                <p>"The sidebar has three parts; the first holds the Getting started and Guides groups:"</p>
                <ul>
                    <li>
                        <b>"Getting started"</b>": this overview, the "
                        <Link href=routes::doc::Installation.materialize()>"installation"</Link>" and the "
                        <Link href=routes::doc::Changelog.materialize()>"changelog"</Link>"."
                    </li>
                    <li>
                        <b>"Guides"</b>": topics every other page builds on, such as the layers, "
                        <Link href=routes::doc::EventPropagation.materialize()>"event propagation"</Link>", "
                        <Link href=routes::doc::Themes.materialize()>"themes"</Link>", "
                        <Link href=routes::doc::Forms.materialize()>"forms and validation"</Link>" and "
                        <Link href=routes::doc::Accessibility.materialize()>"accessibility"</Link>"."
                    </li>
                    <li>
                        <b>"Concepts"</b>": the UI elements your app places on its pages (a button, a select, a table), "
                        "in groups by purpose such as "<Link href=routes::doc::Fields.materialize()>"Fields"</Link>" or "
                        <Link href=routes::doc::Overlays.materialize()>"Overlays"</Link>". A concept leptonic implements at "
                        "both layers has an overview, which explains it and helps you choose a layer, and a tab per layer "
                        "(Hook and Atom; in the plural when the layer has several pieces) with the full reference, including "
                        "how to style the atoms. Start with the overview\u{2019}s Quick Start. The markers "<b>"H A"</b>
                        " next to a concept in the sidebar show which layers it has; a missing layer is dimmed. A concept "
                        "with one layer has a single page. Parts without behavior, such as cards or stacks, are CSS recipes "
                        "on their group\u{2019}s overview."
                    </li>
                    <li>
                        <b>"Building blocks"</b>": the hooks, atoms and utilities that give an element one behavior many "
                        "concepts share, such as "<Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>
                        " or "<Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>". They are grouped "
                        "into areas ("<Link href=routes::doc::Interactions.materialize()>"Interactions"</Link>", "
                        <Link href=routes::doc::Focus.materialize()>"Focus"</Link>", \u{2026}), listed by name with a badge "
                        "of their kind. Use them when you build an element of your own."
                    </li>
                </ul>
                <p>"Most groups and areas have an overview page that compares their members and helps you choose between them."</p>
            </Section>

            <Section title="Help and Contributing">
                <ul>
                    <li>
                        "Ask questions in the Leptos "
                        <Link href="https://discord.gg/x8NhWWYTV2" target=LinkTarget::Blank>"Discord"</Link>" server."
                    </li>
                    <li>
                        "Report bugs and missing features in the "
                        <Link href="https://github.com/lpotthast/leptonic/issues" target=LinkTarget::Blank>"issues"</Link>
                        ". Code contributions are welcome."
                    </li>
                    <li>
                        "Compare your setup with the "
                        <Link href="https://github.com/lpotthast/leptonic/tree/main/examples/book-ssr" target=LinkTarget::Blank>
                            "source of this book"
                        </Link>", which is built with leptonic."
                    </li>
                </ul>
            </Section>
        </DocPage>
    }
}
