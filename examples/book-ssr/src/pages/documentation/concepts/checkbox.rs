use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::checkbox::CheckboxConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageCheckboxOverview() -> impl IntoView {
    view! {
        <DocPage title="Checkbox">
            <p>
                "Checkboxes let users turn individual options on or off. They are the standard control for binary choices "
                "in forms \u{2014} agreeing to terms, enabling features, or selecting items from a list. A checkbox can also be "
                "indeterminate, when it summarizes child checkboxes of which only some are checked."
            </p>
            <p>
                "Leptonic\u{2019}s checkboxes are native "<Code inline=true>"<input type=\"checkbox\">"</Code>" elements inside a "
                <Code inline=true>"<label>"</Code>", so they submit with forms, reset with them and take part in validation. "
                "Checkbox groups select a set of values with a shared label, description and error message."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Turn one or more independent options on or off"</TableCell><TableCell><b>"Checkbox"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Switch a setting that takes effect immediately"</TableCell>
                        <TableCell><Link href=routes::doc::Switch.materialize()>"Switch"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Choose exactly one option from a set"</TableCell>
                        <TableCell><Link href=routes::doc::Radio.materialize()>"Radio"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Toggle a formatting or view option in a toolbar"</TableCell>
                        <TableCell><Link href=routes::doc::ToggleButton.materialize()>"Toggle Button"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Choose from a long list in a dropdown"</TableCell>
                        <TableCell><Link href=routes::doc::Select.materialize()>"Select"</Link></TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "Checkboxes fit form settings that are submitted later. If a change should take effect immediately, "
                    "like a \u{201c}Dark mode\u{201d} setting, use a switch instead."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::checkbox::Hook.materialize()>"Checkbox hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_checkbox"</Code>", "<Code inline=true>"use_checkbox_group"</Code>" and their state "
                            "hooks: behavior, validation and form integration for inputs and labels you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::checkbox::Atom.materialize()>"Checkbox atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"Checkbox"</Code>" and "<Code inline=true>"CheckboxGroup"</Code>
                            " with their parts, styled through data attributes."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::checkbox::Component.materialize()>"Checkbox component"</Link></TableCell>
                        <TableCell>"Themed "<Code inline=true>"Checkbox"</Code>" and "<Code inline=true>"CheckboxGroup"</Code>" with icons, label and description."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Bind the component to a signal with "<Code inline=true>"state"</Code>" and pass the label as children; "
                    "pressing the label toggles the checkbox too."
                </p>
                <Demo
                    description="Newsletter checkbox bound to a signal, with its state shown below"
                    source=include_str!("demos/checkbox.rs")
                    source_open=true
                >
                    <CheckboxConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Checkboxes follow the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/checkbox/" target=LinkTarget::_Blank>
                        "Checkbox pattern"
                    </LinkExt>"."
                </p>
                <ul>
                    <li>
                        "The native input brings the checkbox role, the checked state and keyboard handling. Indeterminate "
                        "checkboxes set the input\u{2019}s "<Code inline=true>"indeterminate"</Code>" property, which screen "
                        "readers announce as \u{201c}mixed\u{201d}."
                    </li>
                    <li>
                        "The surrounding "<Code inline=true>"<label>"</Code>" names the checkbox. Give a checkbox without label "
                        "text an "<Code inline=true>"aria_label"</Code>"."
                    </li>
                    <li>
                        <Code inline=true>"aria-invalid"</Code>", "<Code inline=true>"aria-required"</Code>" and "
                        <Code inline=true>"aria-readonly"</Code>" reflect the validation and settings; descriptions and error "
                        "messages are linked with "<Code inline=true>"aria-describedby"</Code>"."
                    </li>
                    <li>
                        "A checkbox group is a "<Code inline=true>"role=\"group\""</Code>" labelled by its label. Its "
                        "description and error message describe each of its checkboxes."
                    </li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the next checkbox."</KeyRow>
                    <KeyRow keys="Space">"Toggles the focused checkbox."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
