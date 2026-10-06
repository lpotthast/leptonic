use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, pages::documentation::atoms::demos::listbox::ListBoxAtomDemo, routes};

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
                    <TableRow><TableCell>"Choose from a dropdown"</TableCell><TableCell><Link href=routes::doc::Select.materialize()>"Select"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Toggle independent boolean options"</TableCell><TableCell><Link href=routes::doc::Checkbox.materialize()>"Checkbox group"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Choose one from a visible set"</TableCell><TableCell><Link href=routes::doc::Radio.materialize()>"Radio group"</Link></TableCell></TableRow>
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
                        <TableCell><Link href=routes::doc::listbox::Hook.materialize()>"use_listbox"</Link></TableCell>
                        <TableCell>
                            "Behavior and ARIA attributes for a listbox ("<Code inline=true>"use_listbox"</Code>"), its options ("
                            <Code inline=true>"use_option"</Code>") and sections ("<Code inline=true>"use_listbox_section"</Code>
                            ") in markup you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::listbox::Atom.materialize()>"ListBox atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"ListBox"</Code>", "<Code inline=true>"ListBoxItem"</Code>" and "
                            <Code inline=true>"ListBoxSection"</Code>" components with that behavior, plus label and description "
                            "slots for options. You style them through classes and data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The quickest way to a listbox is the headless "<Code inline=true>"ListBox"</Code>" atom. "
                    "Describe the options as a collection (each with a key and a text for type-ahead, optionally in "
                    "sections), then render one "<Code inline=true>"ListBoxItem"</Code>" per option, in the same order. "
                    "The "<Link href=routes::doc::listbox::Atom.materialize()>"ListBox atoms"</Link>" page documents all "
                    "parts; the "<Link href=routes::doc::listbox::Hook.materialize()>"listbox hooks"</Link>
                    " build one from your own markup."
                </p>

                <Demo
                    description="Pizza topping listbox with sections, multiple selection, option descriptions and a disabled option"
                    source=include_str!("../atoms/demos/listbox.rs")
                    source_open=true
                >
                    <ListBoxAtomDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Leptonic listboxes follow the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/listbox/" target=LinkTarget::_Blank>"Listbox pattern"</LinkExt>
                    "."
                </p>

                <ul>
                    <li>
                        <Code inline=true>"role=\"listbox\""</Code>" on the container, "<Code inline=true>"role=\"option\""</Code>
                        " on each option, "<Code inline=true>"role=\"group\""</Code>" on sections"
                    </li>
                    <li><Code inline=true>"aria-selected"</Code>" \u{2014} reflects the selection state of an option"</li>
                    <li><Code inline=true>"aria-multiselectable"</Code>" \u{2014} present in multiple selection mode"</li>
                    <li><Code inline=true>"aria-disabled"</Code>" \u{2014} marks disabled options"</li>
                    <li>"Roving tabindex: the listbox is a single tab stop, arrow keys move focus between options"</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="ArrowDown / ArrowUp">"Move focus between options."</KeyRow>
                    <KeyRow keys="Home / End">"Move focus to the first or last option."</KeyRow>
                    <KeyRow keys="Space">"Toggle the selection of the focused option."</KeyRow>
                    <KeyRow keys="Control + A / Command + A">"Select all options (multiple selection)."</KeyRow>
                    <KeyRow keys="Escape">"Clear the selection."</KeyRow>
                    <KeyRow keys="Any character">"Type-ahead: focus the next option whose text starts with the typed text."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
