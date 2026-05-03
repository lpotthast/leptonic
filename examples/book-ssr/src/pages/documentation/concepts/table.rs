use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::table::TableConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageTableOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="table" class="anchor">
                "Table"
                <AnchorLink href="#table" description="Direct link to article header"/>
            </h1>

            <p>
                "Tables display structured data in rows and columns with optional selection, "
                "sorting, and keyboard navigation. They are appropriate when users need to scan, "
                "compare, or interact with tabular data."
            </p>

            <p>
                "Leptonic provides tables at two abstraction levels. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer."
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
                            <TableCell>"Display structured data in rows and columns"</TableCell>
                            <TableCell><b>"Table"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Create a responsive layout grid"</TableCell>
                            <TableCell>"Grid (component)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Display a scrollable list of selectable items"</TableCell>
                            <TableCell>"Listbox"</TableCell>
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
                <li><Link href=routes::doc::table::Hook.materialize()>"Hook: use_table"</Link></li>
                <li><Link href=routes::doc::table::Component.materialize()>"Component: Table"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use a table (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    <TableContainer>
                        <Table bordered=true hoverable=true>
                            <TableHeader>
                                <TableRow>
                                    <TableHeaderCell>"Name"</TableHeaderCell>
                                    <TableHeaderCell>"Value"</TableHeaderCell>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                <TableRow>
                                    <TableCell>"Alpha"</TableCell>
                                    <TableCell>"100"</TableCell>
                                </TableRow>
                                <TableRow>
                                    <TableCell>"Beta"</TableCell>
                                    <TableCell>"200"</TableCell>
                                </TableRow>
                            </TableBody>
                        </Table>
                    </TableContainer>
                "#)}
            </Code>

            <DemoShell description="Data table with rows and columns" source=include_str!("demos/table.rs")>
                <TableConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic tables follow the WAI-ARIA Table/Grid pattern."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"grid\""</Code>" on the table, "
                    <Code inline=true>"role=\"row\""</Code>" on rows, "
                    <Code inline=true>"role=\"gridcell\""</Code>" / "<Code inline=true>"role=\"columnheader\""</Code>" on cells"</li>
                <li><Code inline=true>"aria-rowindex"</Code>" (1-based), "<Code inline=true>"aria-colindex"</Code>" (1-based)"</li>
                <li><Code inline=true>"aria-selected"</Code>" \u{2014} on rows/cells when selection is enabled"</li>
            </ul>

            <h3>"Keyboard interaction"</h3>

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
                            <TableCell><Code inline=true>"Arrow Up / Down"</Code></TableCell>
                            <TableCell>"Navigate between rows"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Arrow Left / Right"</Code></TableCell>
                            <TableCell>"Navigate between cells"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Space"</Code></TableCell>
                            <TableCell>"Toggle row selection"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Enter"</Code></TableCell>
                            <TableCell>"Trigger row action"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Table", link: "#table" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}
