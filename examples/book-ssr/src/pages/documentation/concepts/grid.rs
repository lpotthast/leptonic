use leptos::prelude::*;

use super::demos::grid::GridConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageGridOverview() -> impl IntoView {
    view! {
        <DocPage title="Grid">
            <p>
                "A grid arranges interactive content in rows and cells that users navigate in two dimensions with the arrow "
                "keys, such as a message list with sender, subject and date, or a palette of colors. Rows can be selected "
                "and activated, cells can hold buttons and links. The whole grid is a single tab stop, so a long grid "
                "doesn\u{2019}t trap keyboard users."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Navigate rows and cells in two dimensions, with row selection"</TableCell><TableCell><b>"Grid"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Show a single column of interactive rows"</TableCell>
                        <TableCell><Link href=routes::doc::GridList.materialize()>"Grid List"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show data in columns with headers and sorting"</TableCell>
                        <TableCell><Link href=routes::doc::Table.materialize()>"Table"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Arrange content in responsive columns, without interaction"</TableCell>
                        <TableCell>"A "<Link href=format!("{}#grid-layout", routes::doc::Layout.materialize())>"CSS grid layout"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::grid::Hook.materialize()>"Grid Hooks"</Link></TableCell>
                        <TableCell>
                            "Keyboard navigation, row selection and ARIA attributes for a grid, its rows and cells that you "
                            "render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::grid::Atom.materialize()>"Grid Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled grid, row group, row and cell elements with that behavior, exposing their state as "
                            "data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Build the rows and cells as a collection, then render a "<Code inline=true>"GridRow"</Code>" per row and a "
                    <Code inline=true>"GridCell"</Code>" per cell. The "<Link href=routes::doc::grid::Atom.materialize()>"Grid Atoms"</Link>
                    " show row actions, multiple selection and disabled rows."
                </p>

                <Demo description="Message list grid with single selection" source=include_str!("demos/grid.rs") source_open=true>
                    <GridConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A grid follows the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/grid/" target=LinkTarget::Blank>"grid pattern"</Link>":"
                </p>

                <ul>
                    <li>
                        <Code inline=true>"role=\"grid\""</Code>" on the container, "<Code inline=true>"role=\"row\""</Code>
                        " on rows and "<Code inline=true>"role=\"gridcell\""</Code>" on cells."
                    </li>
                    <li>
                        <Code inline=true>"aria-selected"</Code>" on selectable rows, and "<Code inline=true>"aria-multiselectable"</Code>
                        " on the grid when several rows can be selected."
                    </li>
                    <li>"A roving tabindex: only the focused row or cell is in the tab order."</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="ArrowUp / ArrowDown">"Move between rows."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">
                        "Move into a row\u{2019}s cells and between them, or between a cell\u{2019}s interactive elements "
                        "(mirrored in right-to-left languages)."
                    </KeyRow>
                    <KeyRow keys="Home / End">"With a row focused: first or last row. With a cell focused: first or last cell of the row."</KeyRow>
                    <KeyRow keys="Control + Home / Control + End">
                        "First or last cell of the grid ("<Keys keys="Meta + Home"/>" / "<Keys keys="Meta + End"/>" on macOS)."
                    </KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Move by the visible height."</KeyRow>
                    <KeyRow keys="Space">"Select or deselect the focused row."</KeyRow>
                    <KeyRow keys="Shift + ArrowUp / Shift + ArrowDown">"Extend the selection (multiple selection)."</KeyRow>
                    <KeyRow keys="Enter">"Activate the row or cell; without an action, select the row."</KeyRow>
                    <KeyRow keys="Control + A">"Select all rows (multiple selection; "<Keys keys="Meta + A"/>" on macOS)."</KeyRow>
                    <KeyRow keys="Escape">"Clear the selection."</KeyRow>
                    <KeyRow keys="Any character">"Focus the next row whose text starts with the typed text."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::grid::Hook.materialize()>"Grid Hooks"</Link></li>
                <li><Link href=routes::doc::grid::Atom.materialize()>"Grid Atoms"</Link></li>
                <li><Link href=routes::doc::GridList.materialize()>"Grid List"</Link></li>
                <li><Link href=routes::doc::Table.materialize()>"Table"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
