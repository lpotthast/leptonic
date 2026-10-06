use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::radio::RadioConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageRadioOverview() -> impl IntoView {
    view! {
        <DocPage title="Radio">
            <p>
                "Radio buttons present a set of mutually exclusive options \u{2014} selecting one deselects all others. "
                "They show all choices at once, which helps users compare options before committing."
            </p>
            <p>
                "In leptonic, radios always live in a radio group, which holds the selected value. Each radio is a native "
                <Code inline=true>"<input type=\"radio\">"</Code>" inside a "<Code inline=true>"<label>"</Code>
                ", so the group submits with forms and takes part in validation."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Choose exactly one of 2\u{2013}5 visible options"</TableCell><TableCell><b>"Radio"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Choose one option from a longer list in a dropdown"</TableCell>
                        <TableCell><Link href=routes::doc::Select.materialize()>"Select"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Turn several independent options on or off"</TableCell>
                        <TableCell><Link href=routes::doc::Checkbox.materialize()>"Checkbox"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick one of a few compact options in a toolbar, like a text alignment"</TableCell>
                        <TableCell><Link href=routes::doc::ToggleButton.materialize()>"Toggle Button"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Switch between content panels"</TableCell>
                        <TableCell><Link href=routes::doc::Tabs.materialize()>"Tabs"</Link></TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "Radio buttons work best when the options are few enough to show inline, typically two to five. "
                    "For longer lists, a select saves space."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::radio::Hook.materialize()>"Radio hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_radio_group"</Code>", "<Code inline=true>"use_radio"</Code>" and the group state: "
                            "group semantics, arrow-key navigation and validation for inputs and labels you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::radio::Atom.materialize()>"Radio atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"RadioGroup"</Code>" and "<Code inline=true>"Radio"</Code>
                            " with label, description and error message parts, styled through data attributes."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::radio::Component.materialize()>"Radio component"</Link></TableCell>
                        <TableCell>"Themed "<Code inline=true>"RadioGroup"</Code>" and "<Code inline=true>"Radio"</Code>" with label and description."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A "<Code inline=true>"RadioGroup"</Code>" with a label holds the selected value; each "
                    <Code inline=true>"Radio"</Code>" has a "<Code inline=true>"value"</Code>" and its label as children."
                </p>
                <Demo
                    description="Radio group with two labeled shipping options and the current selection"
                    source=include_str!("demos/radio.rs")
                    source_open=true
                >
                    <RadioConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Radio groups follow the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/radio/" target=LinkTarget::_Blank>
                        "Radio Group pattern"
                    </LinkExt>"."
                </p>
                <ul>
                    <li>
                        "The group is a "<Code inline=true>"role=\"radiogroup\""</Code>" labelled by its label, with "
                        <Code inline=true>"aria-orientation"</Code>", "<Code inline=true>"aria-required"</Code>", "
                        <Code inline=true>"aria-readonly"</Code>", "<Code inline=true>"aria-invalid"</Code>" and "
                        <Code inline=true>"aria-disabled"</Code>"."
                    </li>
                    <li>
                        "Each radio is a native input sharing the group\u{2019}s "<Code inline=true>"name"</Code>
                        ", named by its "<Code inline=true>"<label>"</Code>". The group\u{2019}s description and error message "
                        "describe each radio."
                    </li>
                    <li>
                        "The group is a single tab stop: "<Keys keys="Tab"/>" moves to the selected radio (while none is selected, "
                        "to the first or, with "<Keys keys="Shift + Tab"/>", the last), and the arrow keys move the selection."
                    </li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus into and out of the group."</KeyRow>
                    <KeyRow keys="ArrowDown / ArrowRight">"Selects the next radio, wrapping around at the end."</KeyRow>
                    <KeyRow keys="ArrowUp / ArrowLeft">"Selects the previous radio, wrapping around at the start."</KeyRow>
                    <KeyRow keys="Space">"Selects the focused radio."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
