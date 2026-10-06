use leptonic::components::prelude::*;
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
                "Leptonic\u{2019}s switches are native "<Code inline=true>"<input type=\"checkbox\" role=\"switch\">"</Code>
                " elements inside a "<Code inline=true>"<label>"</Code>", visually hidden behind a drawn track. They still "
                "submit, reset and validate like checkboxes."
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
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::switch::Hook.materialize()>"Switch hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_switch"</Code>" (and the underlying "<Code inline=true>"use_toggle"</Code>
                            "): switch semantics, form integration and validation for an input and label you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::switch::Atom.materialize()>"Switch atom"</Link></TableCell>
                        <TableCell>"An unstyled "<Code inline=true>"Switch"</Code>" you draw with your children, styled through data attributes."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::switch::Component.materialize()>"Switch component"</Link></TableCell>
                        <TableCell>"A themed switch with sizes, a sliding or stationary variant and optional on/off icons."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"Bind the component to a signal with "<Code inline=true>"state"</Code>" and pass the label as children:"</p>
                <Demo description="Wi-Fi switch bound to a signal, showing its state" source=include_str!("demos/switch.rs") source_open=true>
                    <SwitchConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Switches follow the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/switch/" target=LinkTarget::_Blank>"Switch pattern"</LinkExt>
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
        </DocPage>
    }
}
