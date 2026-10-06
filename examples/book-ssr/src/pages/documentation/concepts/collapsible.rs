use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::collapsible::CollapsibleConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageCollapsibleOverview() -> impl IntoView {
    view! {
        <DocPage title="Collapsible">
            <p>
                "Collapsibles let users expand and collapse sections of content. "
                "They reduce visual clutter by hiding details until the user asks for them. "
                "Multiple collapsibles can be grouped with "<Code inline=true>"Collapsibles"</Code>
                ", which optionally enforces accordion behavior (opening one closes the others)."
            </p>

            <p>
                "The underlying WAI-ARIA pattern is called \u{201c}Disclosure\u{201d}, hence the hook name "
                <Code inline=true>"use_disclosure"</Code>"."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Show/hide supplementary content in place"</TableCell><TableCell><b>"Collapsible"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Enforce only one section open at a time (accordion)"</TableCell>
                        <TableCell><Code inline=true>"Collapsibles"</Code>" with "<Code inline=true>"OnOpen::CloseOthers"</Code></TableCell>
                    </TableRow>
                    <TableRow><TableCell>"Switch between parallel content panels"</TableCell><TableCell><Link href=routes::doc::Tabs.materialize()>"Tabs"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Show content in a blocking overlay"</TableCell><TableCell><Link href=routes::doc::Modal.materialize()>"Modal"</Link></TableCell></TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Collapsibles exist as a hook and as a component. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::collapsible::Hook.materialize()>"use_disclosure"</Link></TableCell>
                        <TableCell>"ARIA attributes and keyboard handling for a trigger button and the panel it controls."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::collapsible::Component.materialize()>"Collapsible component"</Link></TableCell>
                        <TableCell>
                            "A themed collapsible with header and body slots, groupable with "
                            <Code inline=true>"Collapsibles"</Code>"."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"The component is the quickest way to a collapsible:"</p>

                <Demo description="Collapsible with a header and a body" source=include_str!("demos/collapsible.rs") source_open=true>
                    <CollapsibleConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    <Link href=routes::doc::collapsible::Hook.materialize()>"use_disclosure"</Link>" follows the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/disclosure/" target=LinkTarget::_Blank>"Disclosure pattern"</LinkExt>
                    ":"
                </p>

                <ul>
                    <li><Code inline=true>"aria-expanded"</Code>" on the trigger tells whether the panel is shown."</li>
                    <li><Code inline=true>"aria-controls"</Code>" on the trigger points to the panel."</li>
                    <li>
                        "The panel is labelled by the trigger ("<Code inline=true>"aria-labelledby"</Code>") and marked "
                        <Code inline=true>"aria-hidden"</Code>" while collapsed."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the trigger."</KeyRow>
                    <KeyRow keys="Enter / Space">"Expands or collapses the panel."</KeyRow>
                </KeyboardTable>

                <p>
                    "The "<Code inline=true>"Collapsible"</Code>" component is not built on the hook yet: its header reacts to clicks "
                    "only, can\u{2019}t be focused with the keyboard and doesn\u{2019}t announce its state. Use the hook where "
                    "keyboard and screen reader support matter."
                </p>
            </Section>
        </DocPage>
    }
}
