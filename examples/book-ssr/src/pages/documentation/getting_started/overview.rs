use leptonic::{components::prelude::*, hooks::LinkTarget};
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageOverview() -> impl IntoView {
    view! {
        <DocPage title="Overview">
            <p>
                "Leptonic is an accessible component library for the "
                <LinkExt href="https://leptos.dev/" target=LinkTarget::_Blank>"Leptos"</LinkExt>" web framework."
            </p>

            <p>
                "It comes in three layers. "<b>"Hooks"</b>", ported from "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/" target=LinkTarget::_Blank>"react-aria"</LinkExt>
                ", implement interaction and accessibility: pressing, hovering, keyboard navigation, focus management, "
                "selection, overlays and the ARIA semantics of widgets like menus, listboxes and sliders. "<b>"Atoms"</b>
                " wrap hooks into unstyled single-element components. "<b>"Components"</b>" are themed, ready-made UI: "
                "buttons, inputs, selects, sliders, date pickers, a rich text editor, modals, toasts, tabs, tables and more. "
                "Read "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>" to pick the layer "
                "that fits your project."
            </p>

            <p>
                "Get started with the "<Link href=routes::doc::Installation.materialize()>"Installation"</Link>
                " instructions, or browse the concepts in the sidebar. Every page is also available as Markdown: append "
                <Code inline=true>".md"</Code>" to its URL, or start at the "<a href="/doc/llm-index.md">"index"</a>"."
            </p>

            <Section title="Need help?">
                <ul>
                    <li>
                        "Ask in the Leptos "
                        <LinkExt href="https://discord.gg/x8NhWWYTV2" target=LinkTarget::_Blank>"Discord"</LinkExt>" server."
                    </li>
                    <li>
                        "If you think you found a bug, open an "
                        <LinkExt href="https://github.com/lpotthast/leptonic/issues" target=LinkTarget::_Blank>"issue"</LinkExt>"."
                    </li>
                    <li>
                        "Compare your setup with the "
                        <LinkExt href="https://github.com/lpotthast/leptonic/tree/main/examples/book-ssr" target=LinkTarget::_Blank>
                            "source of this book"
                        </LinkExt>", which is built with leptonic."
                    </li>
                </ul>
            </Section>

            <Section title="Contribute">
                <p>
                    "Missing a component or a feature, or found a bug? Tell us in the Discord server or in the "
                    <LinkExt href="https://github.com/lpotthast/leptonic/issues" target=LinkTarget::_Blank>"issues"</LinkExt>
                    ". Code contributions are always welcome."
                </p>
            </Section>
        </DocPage>
    }
}
