use leptos::prelude::*;

use super::demos::switch::SwitchConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageSwitchOverview() -> impl IntoView {
    view! {
        <DocPage title="Switch">
            <p>
                "A switch turns a setting on or off, with immediate effect \u{2014} like a light switch. Checkboxes, in "
                "contrast, are usually part of a form that is submitted later."
            </p>
            <p>
                "Leptonic\u{2019}s switches are native checkbox inputs with the switch role, inside their label and visually "
                "hidden behind a drawn track. They still submit, reset and validate like checkboxes."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Turn a setting on or off with immediate effect"</TableCell><TableCell><b>"Switch"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Set a form option that is submitted later"</TableCell>
                        <TableCell><Link href=routes::doc::Checkbox.materialize()>"Checkbox"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Toggle a mode in a toolbar, like bold text"</TableCell>
                        <TableCell><Link href=routes::doc::ToggleButton.materialize()>"Toggle Button"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Trigger a one-time action"</TableCell>
                        <TableCell><Link href=routes::doc::Button.materialize()>"Button"</Link></TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "Let the visual metaphor guide your choice: a switch looks like a physical switch and implies an instant "
                    "effect. A checkbox looks like a form field and implies that the choice is saved or submitted later."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::switch::Hook.materialize()>"Switch Hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_switch"</Code>" (and the underlying "<Code inline=true>"use_toggle"</Code>
                            "): switch semantics, form integration and validation for an input and label you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::switch::Atom.materialize()>"Switch Atom"</Link></TableCell>
                        <TableCell>"An unstyled "<Code inline=true>"Switch"</Code>" you draw with your children, styled through data attributes."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Pass the "<Code inline=true>"Switch"</Code>" atom an "<Code inline=true>"RwSignal"</Code>" as "
                    <Code inline=true>"is_selected"</Code>" and "<Code inline=true>"set_selected"</Code>", and the label as "
                    "children. The track and thumb are your own markup, styled through the atom\u{2019}s data attributes "
                    "(the CSS is on the "<Link href=format!("{}#styling", routes::doc::switch::Atom.materialize())>"Switch Atom"</Link>
                    " page):"
                </p>
                <Demo description="Wi-Fi switch kept in a signal, showing its state, with a disabled toggle" source=include_str!("demos/switch.rs") source_open=true>
                    <SwitchConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Switches follow the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/switch/" target=LinkTarget::Blank>"Switch pattern"</Link>
                    "."
                </p>
                <ul>
                    <li>
                        "The input has "<Code inline=true>"role=\"switch\""</Code>"; screen readers announce it as on or off "
                        "from its checked state."
                    </li>
                    <li>
                        "The surrounding "<Code inline=true>"<label>"</Code>" names the switch. A switch without label text "
                        "needs an "<Code inline=true>"aria_label"</Code>"; a text next to the switch, outside of it, doesn\u{2019}t name it."
                    </li>
                    <li>
                        "The input is visually hidden but focusable: the drawn track shows the focus ring from "
                        <Code inline=true>"data-focus-visible"</Code>"."
                    </li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the switch."</KeyRow>
                    <KeyRow keys="Space">"Turns the switch on or off."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::switch::Hook.materialize()>"Switch Hooks"</Link></li>
                <li><Link href=routes::doc::switch::Atom.materialize()>"Switch Atom"</Link></li>
                <li><Link href=routes::doc::Checkbox.materialize()>"Checkbox"</Link></li>
                <li><Link href=routes::doc::ToggleButton.materialize()>"Toggle Button"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
