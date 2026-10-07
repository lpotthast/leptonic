use leptos::prelude::*;

use super::demos::disclosure::DisclosureConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageDisclosureOverview() -> impl IntoView {
    view! {
        <DocPage title="Disclosure">
            <p>
                "A disclosure lets users expand and collapse a section of content under a trigger button. It reduces "
                "visual clutter by hiding details until the user asks for them, such as advanced settings or the answers "
                "of a FAQ. Several disclosures form an accordion, which optionally lets only one of them be expanded at a "
                "time."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Show or hide supplementary content in place"</TableCell><TableCell><b>"Disclosure"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Keep only one section expanded at a time (accordion)"</TableCell>
                        <TableCell>
                            <b>"Disclosure"</b>" in a group: the "
                            <Link href=format!("{}#disclosuregroup", routes::doc::disclosure::Atom.materialize())>"DisclosureGroup"</Link>
                            " atom"
                        </TableCell>
                    </TableRow>
                    <TableRow><TableCell>"Switch between parallel content panels"</TableCell><TableCell><Link href=routes::doc::Tabs.materialize()>"Tabs"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Show content in a blocking overlay"</TableCell><TableCell><Link href=routes::doc::Modal.materialize()>"Modal"</Link></TableCell></TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Disclosures exist at all three layers. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::disclosure::Hook.materialize()>"Disclosure Hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_disclosure"</Code>" and its state hooks: ARIA attributes and keyboard handling "
                            "for a trigger button and the panel it controls, alone or in a group."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::disclosure::Atom.materialize()>"Disclosure Atoms"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"Disclosure"</Code>", "<Code inline=true>"DisclosureTrigger"</Code>" (around a "
                            "Button atom), "<Code inline=true>"DisclosurePanel"</Code>" and "<Code inline=true>"DisclosureGroup"</Code>
                            ": the complete behavior, unstyled and styled through data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"The "<Code inline=true>"Collapsible"</Code>" component is the quickest way to a disclosure:"</p>

                <Demo description="A collapsible with a header and a body" source=include_str!("demos/disclosure.rs") source_open=true>
                    <DisclosureConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "All layers follow the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/disclosure/" target=LinkTarget::Blank>"Disclosure pattern"</Link>
                    ":"
                </p>

                <ul>
                    <li><Code inline=true>"aria-expanded"</Code>" on the trigger tells whether the panel is shown."</li>
                    <li><Code inline=true>"aria-controls"</Code>" on the trigger points to the panel."</li>
                    <li>
                        "The panel is labelled by the trigger ("<Code inline=true>"aria-labelledby"</Code>"). Collapsed, it is "
                        <Code inline=true>"hidden=\"until-found\""</Code>": hidden, but the browser\u{2019}s find in page still "
                        "finds its content and expands the disclosure."
                    </li>
                </ul>

                <p>
                    "When a disclosure is a section of the page, put its trigger in a heading, so that heading navigation "
                    "finds it."
                </p>

                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the trigger."</KeyRow>
                    <KeyRow keys="Enter / Space">"Expands or collapses the panel."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::disclosure::Hook.materialize()>"Disclosure Hooks"</Link></li>
                <li><Link href=routes::doc::disclosure::Atom.materialize()>"Disclosure Atoms"</Link></li>
                <li><Link href=routes::doc::Tabs.materialize()>"Tabs"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
