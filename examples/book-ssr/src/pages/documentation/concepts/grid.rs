use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::grid::GridConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageGridOverview() -> impl IntoView {
    view! {
        <DocPage title="Grid">
            <p>
                "Grids organize content into rows and columns. "
                "Leptonic\u{2019}s component-layer Grid is a responsive layout grid with breakpoint support "
                "(xs, sm, md, lg, xl). The hook-layer grid ("<Code inline=true>"use_grid"</Code>
                ") is an interactive ARIA grid with keyboard navigation and selection \u{2014} "
                "used for data grids, not layout."
            </p>

            <p>"The layout grid (component) and the interactive ARIA grid (hook) serve different purposes."</p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Create a responsive column layout"</TableCell><TableCell><b>"Grid"</b>" component (with Row/Col)"</TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Build an interactive data grid with selection"</TableCell>
                        <TableCell><b>"Grid"</b>" hook ("<Code inline=true>"use_grid"</Code>" + "<Code inline=true>"use_grid_cell"</Code>")"</TableCell>
                    </TableRow>
                    <TableRow><TableCell>"Display tabular data with headers"</TableCell><TableCell><Link href=routes::doc::Table.materialize()>"Table"</Link></TableCell></TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Grids exist at all three layers. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::grid::Hook.materialize()>"use_grid"</Link></TableCell>
                        <TableCell>"An interactive ARIA grid with keyboard navigation and selection, rendered by you."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::grid::Atom.materialize()>"Grid atoms"</Link></TableCell>
                        <TableCell>"Unstyled grid and grid list components exposing their state as data attributes."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::grid::Component.materialize()>"Grid component"</Link></TableCell>
                        <TableCell>"A responsive layout grid with rows and columns."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"The simplest way to use a layout grid (component layer):"</p>

                <Demo description="Responsive grid layout with two columns" source=include_str!("demos/grid.rs") source_open=true>
                    <GridConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The interactive grid hook implements the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/grid/" target=LinkTarget::_Blank>"Grid pattern"</LinkExt>
                    ". The layout Grid component has no ARIA semantics \u{2014} it is purely CSS layout."
                </p>

                <ul>
                    <li>
                        <Code inline=true>"role=\"grid\""</Code>" on the container, "
                        <Code inline=true>"role=\"row\""</Code>" on rows, "
                        <Code inline=true>"role=\"gridcell\""</Code>" on cells"
                    </li>
                    <li><Code inline=true>"aria-multiselectable"</Code>" \u{2014} when multiple selection is enabled"</li>
                    <li>"Roving tabindex for focus management"</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="ArrowUp / ArrowDown / ArrowLeft / ArrowRight">"Navigate between cells."</KeyRow>
                    <KeyRow keys="Home / End">"Jump to the first / last item."</KeyRow>
                    <KeyRow keys="Space">"Toggle selection."</KeyRow>
                    <KeyRow keys="Control + A">"Select all (multi-select mode)."</KeyRow>
                    <KeyRow keys="Escape">"Clear selection."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
