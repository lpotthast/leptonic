use leptos::prelude::*;

use super::demos::grid_list::GridListConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageGridListOverview() -> impl IntoView {
    view! {
        <DocPage title="Grid List">
            <p>
                "A grid list shows a list of items, like a listbox, but each row may contain interactive elements such as "
                "buttons, checkboxes or links: a file list with a menu per file, or a gallery of cards. Users move between "
                "the rows with the arrow keys, and into a row\u{2019}s elements with the left and right arrow keys. Rows can "
                "be selected and activated."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Show a list of rows with interactive content"</TableCell><TableCell><b>"Grid List"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Let users pick options from a list of plain text items"</TableCell>
                        <TableCell><Link href=routes::doc::Listbox.materialize()>"Listbox"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Navigate rows with several cells in two dimensions"</TableCell>
                        <TableCell><Link href=routes::doc::Grid.materialize()>"Grid"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show removable tags in a row"</TableCell>
                        <TableCell><Link href=routes::doc::TagGroup.materialize()>"Tag Group"</Link>" (a horizontal grid list)"</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::grid_list::Hook.materialize()>"Grid List Hooks"</Link></TableCell>
                        <TableCell>
                            "Keyboard navigation, selection and ARIA attributes for a list, its rows and sections that you "
                            "render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::grid_list::Atom.materialize()>"Grid List Atoms"</Link></TableCell>
                        <TableCell>
                            "An unstyled list and rows with that behavior, exposing their state as data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Build the items from your data with "<Code inline=true>"use_list_collection"</Code>" and render a "
                    <Code inline=true>"GridListItem"</Code>" per item. The "<Link href=routes::doc::grid_list::Atom.materialize()>"Grid List Atoms"</Link>
                    " show multiple selection, disabled rows and buttons inside rows."
                </p>

                <Demo description="File list with single selection and an open action" source=include_str!("demos/grid_list.rs") source_open=true>
                    <GridListConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A grid list follows the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/grid/" target=LinkTarget::Blank>"grid pattern"</Link>
                    " with a single column: the list is a "<Code inline=true>"grid"</Code>", each row a "
                    <Code inline=true>"row"</Code>" holding one "<Code inline=true>"gridcell"</Code>", selectable rows carry "
                    <Code inline=true>"aria-selected"</Code>". The list is a single tab stop."
                </p>

                <KeyboardTable>
                    <KeyRow keys="ArrowUp / ArrowDown">"Move between rows (in a card layout, to the row in the same column)."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">
                        "Move between a row and its interactive elements; in a card layout, between rows. In a horizontal "
                        "list, to the neighboring column (see "
                        <Link href=format!("{}#layout-and-orientation", routes::doc::grid_list::Atom.materialize())>"Layout and Orientation"</Link>")."
                    </KeyRow>
                    <KeyRow keys="Home / End">"First or last row."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Move by the visible height."</KeyRow>
                    <KeyRow keys="Space">"Select or deselect the focused row."</KeyRow>
                    <KeyRow keys="Shift + ArrowUp / Shift + ArrowDown">"Extend the selection (multiple selection)."</KeyRow>
                    <KeyRow keys="Enter">"Activate the row; without an action, select it."</KeyRow>
                    <KeyRow keys="Control + A">"Select all rows (multiple selection; "<Keys keys="Meta + A"/>" on macOS)."</KeyRow>
                    <KeyRow keys="Escape">"Clear the selection."</KeyRow>
                    <KeyRow keys="Any character">"Focus the next row whose text starts with the typed text."</KeyRow>
                    <KeyRow keys="Tab">"Leave the list; with the "<Code inline=true>"Tab"</Code>" navigation behavior, move between the focused row\u{2019}s elements first."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::grid_list::Hook.materialize()>"Grid List Hooks"</Link></li>
                <li><Link href=routes::doc::grid_list::Atom.materialize()>"Grid List Atoms"</Link></li>
                <li><Link href=routes::doc::Listbox.materialize()>"Listbox"</Link></li>
                <li><Link href=routes::doc::Grid.materialize()>"Grid"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
