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
                "radio input inside its label, so the group submits with forms and takes part in validation."
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
                        <TableCell><Link href=routes::doc::radio::Hook.materialize()>"Radio Hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_radio_group"</Code>", "<Code inline=true>"use_radio"</Code>" and the group state: "
                            "group semantics, arrow-key navigation and validation for inputs and labels you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::radio::Atom.materialize()>"Radio Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"RadioGroup"</Code>" and "<Code inline=true>"Radio"</Code>
                            " with label, description and error message parts, styled through data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A "<Code inline=true>"RadioGroup"</Code>" with a label holds the selected key; pass it an "
                    <Code inline=true>"RwSignal"</Code>" as "<Code inline=true>"value"</Code>" and "<Code inline=true>"set_value"</Code>
                    " to keep the selection in your app. Each "<Code inline=true>"Radio"</Code>" has a "
                    <Code inline=true>"value"</Code>" and its label as children."
                </p>
                <Demo
                    description="Radio group with two labeled shipping options, the current selection and a disabled toggle"
                    source=include_str!("demos/radio.rs")
                    source_open=true
                >
                    <RadioConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Radio groups follow the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/radio/" target=LinkTarget::Blank>
                        "Radio Group pattern"
                    </Link>"."
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
                    <KeyRow keys="ArrowDown / ArrowRight">
                        "Focuses and selects the next radio, wrapping around at the end. Disabled radios are skipped."
                    </KeyRow>
                    <KeyRow keys="ArrowUp / ArrowLeft">"Focuses and selects the previous radio, wrapping around at the start."</KeyRow>
                    <KeyRow keys="Space">"Selects the focused radio."</KeyRow>
                </KeyboardTable>
                <p>
                    "Both arrow key pairs work in either orientation. In right-to-left locales, "<Keys keys="ArrowRight"/>
                    " moves backwards in horizontal groups."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::radio::Hook.materialize()>"Radio Hooks"</Link></li>
                <li><Link href=routes::doc::radio::Atom.materialize()>"Radio Atoms"</Link></li>
                <li><Link href=routes::doc::Checkbox.materialize()>"Checkbox"</Link></li>
                <li><Link href=routes::doc::Select.materialize()>"Select"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
