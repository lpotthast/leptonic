use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::grid::GridConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageGridOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="grid" class="anchor">
                "Grid"
                <AnchorLink href="#grid" description="Direct link to article header"/>
            </h1>

            <p>
                "Grids organize content into rows and columns. "
                "Leptonic's component-layer Grid is a responsive layout grid with breakpoint support "
                "(xs, sm, md, lg, xl). The hook-layer grid ("<Code inline=true>"use_grid"</Code>
                ") is an interactive ARIA grid with keyboard navigation and selection \u{2014} "
                "used for data grids, not layout."
            </p>

            <p>
                "Leptonic provides grids at three abstraction levels. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer. "
                "The layout grid (component) and the interactive ARIA grid (hook) serve different purposes."
            </p>

            <h2 id="when-to-use" class="anchor">
                "When to Use"
                <AnchorLink href="#when-to-use" description="Direct link to section: When to Use"/>
            </h2>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"If you want to\u{2026}"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Use"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>"Create a responsive column layout"</TableCell>
                            <TableCell><b>"Grid"</b>" component (with Row/Col)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Build an interactive data grid with selection"</TableCell>
                            <TableCell><b>"Grid"</b>" hook (use_grid + use_grid_cell)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Display tabular data with headers"</TableCell>
                            <TableCell>"Table"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="dive-deeper" class="anchor">
                "Dive Deeper"
                <AnchorLink href="#dive-deeper" description="Direct link to section: Dive Deeper"/>
            </h2>

            <p>
                "Not sure which layer to pick? Read the "
                <Link href=routes::doc::Architecture.materialize()>"architecture guide"</Link>
                ". Otherwise, pick a layer:"
            </p>

            <ul>
                <li><Link href=routes::doc::grid::Hook.materialize()>"Hook: use_grid"</Link></li>
                <li><Link href=routes::doc::grid::Atom.materialize()>"Atom: Grid"</Link></li>
                <li><Link href=routes::doc::grid::Component.materialize()>"Component: Grid"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use a layout grid (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    <Grid gap=em(1.0)>
                        <Row>
                            <Col xs=6 sm=4>"Column A"</Col>
                            <Col xs=6 sm=8>"Column B"</Col>
                        </Row>
                    </Grid>
                "#)}
            </Code>

            <DemoShell description="Responsive grid layout" source=include_str!("demos/grid.rs")>
                <GridConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "The interactive grid hook implements the WAI-ARIA Grid pattern. "
                "The layout Grid component has no ARIA semantics \u{2014} it is purely CSS layout."
            </p>

            <h3>"ARIA attributes (interactive grid)"</h3>

            <ul>
                <li><Code inline=true>"role=\"grid\""</Code>" on the container, "
                    <Code inline=true>"role=\"row\""</Code>" on rows, "
                    <Code inline=true>"role=\"gridcell\""</Code>" on cells"</li>
                <li><Code inline=true>"aria-multiselectable"</Code>" \u{2014} when multiple selection is enabled"</li>
                <li>"Roving tabindex for focus management"</li>
            </ul>

            <h3>"Keyboard interaction (interactive grid)"</h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Key"</TableHeaderCell>
                            <TableHeaderCell>"Action"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"Arrow keys"</Code></TableCell>
                            <TableCell>"Navigate between cells"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Home / End"</Code></TableCell>
                            <TableCell>"Jump to first / last item"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Space"</Code></TableCell>
                            <TableCell>"Toggle selection"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Ctrl+A"</Code></TableCell>
                            <TableCell>"Select all (multi-select mode)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Escape"</Code></TableCell>
                            <TableCell>"Clear selection"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Grid", link: "#grid" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}
