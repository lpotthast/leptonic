use leptos::prelude::*;

use super::demos::listbox::ListboxConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageListboxOverview() -> impl IntoView {
    view! {
        <DocPage title="Listbox">
            <p>
                "Listboxes present a visible list of options to select one or more from. "
                "They support single and multiple selection, keyboard navigation, and type-ahead. "
                "Listboxes are also the foundation of higher-level controls like "
                <Link href=routes::doc::Select.materialize()>"Select"</Link>" and "
                <Link href=routes::doc::Combobox.materialize()>"Combobox"</Link>", which show one in a popover."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Present a visible list of selectable items"</TableCell><TableCell><b>"Listbox"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Present thousands of options"</TableCell>
                        <TableCell>
                            "A listbox in a "<Link href=routes::doc::collection_state::Virtualizer.materialize()>"Virtualizer"</Link>
                            ", which renders only the options in view"
                        </TableCell>
                    </TableRow>
                    <TableRow><TableCell>"Choose from a dropdown"</TableCell><TableCell><Link href=routes::doc::Select.materialize()>"Select"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Toggle independent boolean options"</TableCell><TableCell><Link href=routes::doc::Checkbox.materialize()>"Checkbox"</Link>" group"</TableCell></TableRow>
                    <TableRow><TableCell>"Choose one from a visible set"</TableCell><TableCell><Link href=routes::doc::Radio.materialize()>"Radio"</Link>" group"</TableCell></TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Listboxes exist as hooks and as headless atoms. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::listbox::Hook.materialize()>"Listbox Hooks"</Link></TableCell>
                        <TableCell>
                            "Selection, keyboard navigation and ARIA attributes for a listbox, its options and sections in "
                            "markup you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::listbox::Atom.materialize()>"Listbox Atoms"</Link></TableCell>
                        <TableCell>
                            "An unstyled listbox, options and sections with that behavior, plus label and description slots "
                            "for options. You style them through their data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Describe the options as a collection (each with a key and a text for type-ahead) and render them in a "
                    <Code inline=true>"ListBox"</Code>". The "<Link href=routes::doc::listbox::Atom.materialize()>"Listbox Atoms"</Link>
                    " page adds sections, option descriptions and disabled options."
                </p>

                <Demo description="Fruit listbox with multiple selection" source=include_str!("demos/listbox.rs") source_open=true>
                    <ListboxConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Leptonic listboxes follow the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/listbox/" target=LinkTarget::Blank>"Listbox pattern"</Link>
                    "."
                </p>

                <ul>
                    <li>
                        "The list has "<Code inline=true>"role=\"listbox\""</Code>", each option "<Code inline=true>"role=\"option\""</Code>
                        " with "<Code inline=true>"aria-selected"</Code>", and each section "<Code inline=true>"role=\"group\""</Code>
                        ", labelled by its heading."
                    </li>
                    <li>
                        "With multiple selection, the list has "<Code inline=true>"aria-multiselectable"</Code>". Disabled "
                        "options have "<Code inline=true>"aria-disabled"</Code>" and are skipped by the keyboard."
                    </li>
                    <li>"The listbox is a single tab stop; the arrow keys move the focus between its options."</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="ArrowDown / ArrowUp">"Move focus to the next or previous option (left and right in a horizontal listbox)."</KeyRow>
                    <KeyRow keys="Home / End">"Move focus to the first or last option."</KeyRow>
                    <KeyRow keys="PageDown / PageUp">"Move focus by the visible height."</KeyRow>
                    <KeyRow keys="Space">"Select or deselect the focused option."</KeyRow>
                    <KeyRow keys="Shift + ArrowDown / Shift + ArrowUp">"Extend the selection (multiple selection)."</KeyRow>
                    <KeyRow keys="Control + ArrowDown / Control + ArrowUp">
                        "With the "<Code inline=true>"Replace"</Code>" selection behavior: move focus without selecting ("
                        <Keys keys="Option"/>" instead of "<Keys keys="Control"/>" on macOS)."
                    </KeyRow>
                    <KeyRow keys="Enter">"Perform the option\u{2019}s action; without one, select it."</KeyRow>
                    <KeyRow keys="Control + A">"Select all options (multiple selection; "<Keys keys="Meta + A"/>" on macOS)."</KeyRow>
                    <KeyRow keys="Escape">"Clear the selection."</KeyRow>
                    <KeyRow keys="Any character">"Focus the next option whose text starts with the typed text."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::listbox::Hook.materialize()>"Listbox Hooks"</Link></li>
                <li><Link href=routes::doc::listbox::Atom.materialize()>"Listbox Atoms"</Link></li>
                <li><Link href=routes::doc::Select.materialize()>"Select"</Link></li>
                <li><Link href=routes::doc::GridList.materialize()>"Grid List"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
                <li><Link href=routes::doc::collection_state::Virtualizer.materialize()>"Virtualizer"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
