use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::table::TableConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageTableOverview() -> impl IntoView {
    view! {
        <DocPage title="Table">
            <p>
                "A table shows data in rows and columns, with column headers naming each column. Users scan and compare "
                "it row by row, sort it by a column, and select rows to act on them."
            </p>
            <p>
                "An interactive table is one tab stop: the arrow keys move between its rows, cells and column headers, "
                "so keyboard users can reach every cell without tabbing through the whole table. Leptonic builds its "
                "tables on the "<Link href=routes::doc::Grid.materialize()>"grid"</Link>" and adds what makes a table: "
                "column headers (optionally grouped), row headers labelling each row, sortable columns and selection "
                "checkboxes."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Show records with the same fields, to sort, compare and select"</TableCell><TableCell><b>"Table"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Show static tabular data without interaction"</TableCell>
                        <TableCell><Link href=routes::doc::table::Component.materialize()>"Table component"</Link>" (styled HTML table)"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Navigate a two-dimensional arrangement without column headers (a calendar, a palette)"</TableCell>
                        <TableCell><Link href=routes::doc::grid::Hook.materialize()>"Grid"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a list of selectable items with one column"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Listbox.materialize()>"Listbox"</Link>", or a "
                            <Link href=routes::doc::grid::Atom.materialize()>"grid list"</Link>" if items contain buttons or links"
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Tables exist at all three layers. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::table::Hook.materialize()>"Table hooks"</Link></TableCell>
                        <TableCell>
                            "Keyboard navigation, sorting, row selection, column resizing and ARIA semantics for a table "
                            "whose every element you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::table::Atom.materialize()>"Table atoms"</Link></TableCell>
                        <TableCell>
                            "The same behavior as unstyled components: "<Code inline=true>"Table"</Code>" renders the column "
                            "headers and selection checkboxes, you render the rows and cells and style them through data "
                            "attributes. A "<Code inline=true>"ResizableTableContainer"</Code>" around the table makes its "
                            "columns resizable."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::table::Component.materialize()>"Table component"</Link></TableCell>
                        <TableCell>
                            "A themed, static HTML table. It is not built on the table hooks yet: it has no keyboard "
                            "navigation, sorting or selection."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The table atoms are the quickest way to an interactive table. Describe the columns and rows as a "
                    <Code inline=true>"TableCollection"</Code>", render a "<Code inline=true>"TableRow"</Code>" per row, and "
                    "sort the data when the user presses a column header:"
                </p>

                <Demo description="Sortable planet table with single selection, built from the table atoms" source=include_str!("demos/table.rs") source_open=true>
                    <TableConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Interactive tables follow the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/grid/" target=LinkTarget::_Blank>"Grid pattern"</LinkExt>
                    " for data grids:"
                </p>

                <ul>
                    <li>
                        <Code inline=true>"role=\"grid\""</Code>" on the table, "<Code inline=true>"role=\"rowgroup\""</Code>
                        " on its header and body, "<Code inline=true>"role=\"row\""</Code>" on rows"
                    </li>
                    <li>
                        <Code inline=true>"role=\"columnheader\""</Code>" on column headers, with "<Code inline=true>"aria-colindex"</Code>
                        ", and "<Code inline=true>"aria-sort"</Code>" on sortable ones"
                    </li>
                    <li>
                        <Code inline=true>"role=\"rowheader\""</Code>" on the cells labelling their row (the row references them "
                        "with "<Code inline=true>"aria-labelledby"</Code>"), "<Code inline=true>"role=\"gridcell\""</Code>
                        " on the others"
                    </li>
                    <li>
                        <Code inline=true>"aria-selected"</Code>" on rows and "<Code inline=true>"aria-multiselectable"</Code>
                        " on the table when rows can be selected; selection checkboxes are labelled \u{201c}Select\u{201d} and "
                        "\u{201c}Select All\u{201d}"
                    </li>
                    <li>"One tab stop with roving focus: the table remembers the focused row or cell"</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="ArrowUp / ArrowDown">"Move between rows; up from the first row reaches the column headers."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">"Move into a row\u{2019}s cells and between them, or between column headers."</KeyRow>
                    <KeyRow keys="Home / End">"First or last row, or first or last cell of the row."</KeyRow>
                    <KeyRow keys="Space">"Toggle the selection of the focused row."</KeyRow>
                    <KeyRow keys="Enter / Space">"On a sortable column header: sort by it (again to reverse the order)."</KeyRow>
                    <KeyRow keys="Enter">"Run the row\u{2019}s action; without one, toggle its selection."</KeyRow>
                    <KeyRow keys="Control + A">"Select all rows (multiple selection)."</KeyRow>
                    <KeyRow keys="Escape">"Clear the selection."</KeyRow>
                </KeyboardTable>

                <p>
                    "The sort is not announced to screen reader users yet; they hear the new "<Code inline=true>"aria-sort"</Code>
                    " state when they move to the column header."
                </p>
            </Section>
        </DocPage>
    }
}
