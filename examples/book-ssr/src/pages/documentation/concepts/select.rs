use leptos::prelude::*;

use super::demos::select::SelectConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageSelectOverview() -> impl IntoView {
    view! {
        <DocPage title="Select">
            <p>
                "A select is a button showing the chosen option that opens a list of options in a popover. It is the "
                "control of choice when there are too many options for radio buttons, but the user still picks from a "
                "fixed list, and the options should take no space until they are needed."
            </p>
            <p>
                "A select is a form field: it has a label, can be required and validated, and submits its value with "
                "a form through a hidden native element."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Choose one or several options from a list that opens on demand"</TableCell><TableCell><b>"Select"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Find an option in a long list by typing part of its name"</TableCell><TableCell><Link href=routes::doc::Combobox.materialize()>"Combobox"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Choose one of two to five options that stay visible"</TableCell><TableCell><Link href=routes::doc::Radio.materialize()>"Radio"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Show the options without a popover"</TableCell><TableCell><Link href=routes::doc::Listbox.materialize()>"Listbox"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Run an action rather than set a value"</TableCell><TableCell><Link href=routes::doc::Menu.materialize()>"Menu"</Link></TableCell></TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Selects exist at all three layers. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::select::Hook.materialize()>"Select Hooks"</Link></TableCell>
                        <TableCell>
                            "State, keyboard interaction and ARIA attributes for a trigger, a listbox popover and a hidden "
                            "native select that you render and position yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::select::Atom.materialize()>"Select Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled trigger, value, popover and hidden form element with that behavior. The options are a "
                            "listbox, so they can have sections and descriptions; you style the parts through data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The themed component takes the options as values of your own type. Its "<Code inline=true>"label"</Code>
                    " names the select for everyone, including screen reader users:"
                </p>

                <Demo description="Coffee size select with a label and a disabled toggle" source=include_str!("demos/select.rs") source_open=true>
                    <SelectConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The trigger is a button that opens a listbox popover, following the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/listbox/" target=LinkTarget::Blank>"Listbox pattern"</Link>
                    " once open."
                </p>

                <ul>
                    <li>
                        "The trigger has "<Code inline=true>"aria-haspopup=\"listbox\""</Code>" and "
                        <Code inline=true>"aria-expanded"</Code>". It is labelled by the label and the selected value, so "
                        "screen readers announce both."
                    </li>
                    <li>
                        "The popover holds a "<Code inline=true>"listbox"</Code>" of "<Code inline=true>"option"</Code>
                        "s with "<Code inline=true>"aria-selected"</Code>". It is modal: focus stays inside, and the rest "
                        "of the page is hidden from assistive technology until it closes."
                    </li>
                    <li>
                        "A visually hidden native "<Code inline=true>"<select>"</Code>" mirrors the value, so the select "
                        "takes part in forms and browser autofill."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Enter / Space / ArrowDown">"On the trigger: open the popover and focus the selected (or first) option."</KeyRow>
                    <KeyRow keys="ArrowUp">"On the trigger: open the popover and focus the selected (or last) option."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">"On the trigger, with single selection: select the previous or next option without opening the popover."</KeyRow>
                    <KeyRow keys="Any character">"On the trigger, with single selection: select the next option starting with the typed text. In the popover: focus it."</KeyRow>
                    <KeyRow keys="ArrowDown / ArrowUp">"In the popover: move between options."</KeyRow>
                    <KeyRow keys="Home / End">"In the popover: focus the first or last option."</KeyRow>
                    <KeyRow keys="Enter / Space">"In the popover: select the focused option; with single selection, also close the popover."</KeyRow>
                    <KeyRow keys="Escape">"Close the popover and return focus to the trigger."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::select::Hook.materialize()>"Select Hooks"</Link></li>
                <li><Link href=routes::doc::select::Atom.materialize()>"Select Atoms"</Link></li>
                <li><Link href=routes::doc::Combobox.materialize()>"Combobox"</Link></li>
                <li><Link href=routes::doc::Listbox.materialize()>"Listbox"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
