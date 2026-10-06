use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, pages::documentation::atoms::demos::combobox::ComboBoxAtomDemo, routes};

#[component]
pub fn PageComboboxOverview() -> impl IntoView {
    view! {
        <DocPage title="Combobox">
            <p>
                "A combobox combines a text input with a popover of suggestions. Typing narrows the suggestions down, "
                "and the user picks one with the mouse or the keyboard. It suits long lists, such as countries or "
                "contacts, where scrolling through all options would be slow but the user knows what to type."
            </p>
            <p>
                "While the user moves through the suggestions, focus stays in the input: the highlighted option is "
                "announced to screen readers through "<Code inline=true>"aria-activedescendant"</Code>
                ", and the user can keep typing at any time."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Pick from many options by typing part of their name"</TableCell><TableCell><b>"Combobox"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Pick from a short list without typing"</TableCell><TableCell><Link href=routes::doc::Select.materialize()>"Select"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Keep the options visible all the time"</TableCell><TableCell><Link href=routes::doc::Listbox.materialize()>"Listbox"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Enter free-form text without suggestions"</TableCell><TableCell><Link href=routes::doc::TextField.materialize()>"Text field"</Link></TableCell></TableRow>
                </DocTable>

                <p>
                    "A combobox can also accept text that matches no option ("<Code inline=true>"allows_custom_value"</Code>
                    "), for inputs where the suggestions are only a help."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Comboboxes exist as hooks and as atoms. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::combobox::Hook.materialize()>"use_combobox_state, use_combobox"</Link></TableCell>
                        <TableCell>
                            "State, filtering, keyboard interaction and ARIA attributes for an input, a button and a listbox "
                            "you render and position yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::combobox::Atom.materialize()>"Combobox atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled label, input, button and popover parts with that behavior. The popover is positioned "
                            "for you; you style the parts through classes and data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Build a collection of options, pass it to "<Code inline=true>"ComboBox"</Code>" with a filter, and "
                    "compose the parts:"
                </p>

                <Demo
                    description="Country combobox built from the atoms, with a disabled option"
                    source=include_str!("../atoms/demos/combobox.rs")
                    source_open=true
                >
                    <ComboBoxAtomDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Comboboxes follow the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/combobox/" target=LinkTarget::_Blank>
                        "Combobox pattern"
                    </LinkExt>
                    " with a listbox popup."
                </p>

                <ul>
                    <li>
                        "The input has "<Code inline=true>"role=\"combobox\""</Code>", "<Code inline=true>"aria-autocomplete=\"list\""</Code>
                        " and "<Code inline=true>"aria-expanded"</Code>"; while the popover is open, "<Code inline=true>"aria-controls"</Code>
                        " points to the listbox and "<Code inline=true>"aria-activedescendant"</Code>" to the focused option."
                    </li>
                    <li>
                        "The button is labelled \u{201c}Show suggestions\u{201d} together with the combobox label. It is not in "
                        "the tab order, as the keyboard opens the popover from the input."
                    </li>
                    <li>"While the popover is open, the rest of the page is hidden from assistive technology."</li>
                    <li>"Disabled options are marked with "<Code inline=true>"aria-disabled"</Code>" and skipped by the arrow keys."</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="ArrowDown / ArrowUp">
                        "Open the popover and focus the selected option (or the first or last one); when open, focus the "
                        "next or previous option."
                    </KeyRow>
                    <KeyRow keys="Home / End">"When open: focus the first or last option."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">"Return to the text: remove the focus from the options."</KeyRow>
                    <KeyRow keys="Enter">"Select the focused option and close the popover."</KeyRow>
                    <KeyRow keys="Escape">"Close the popover and restore the selected option\u{2019}s text."</KeyRow>
                    <KeyRow keys="Tab">"Select the focused option and move focus to the next element."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
