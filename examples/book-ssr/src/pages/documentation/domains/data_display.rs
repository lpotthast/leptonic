use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, toc::Toc},
    routes,
};

#[component]
pub fn PageDataDisplay() -> impl IntoView {
    view! {
        <Article>
            <h1 id="data-display" class="anchor">
                "Data Display"
                <AnchorLink href="#data-display" description="Direct link to article header"/>
            </h1>

            <p>
                "Components for presenting structured data in grids, tables, and trees."
            </p>

            <h2 id="overview" class="anchor">
                "Overview"
                <AnchorLink href="#overview" description="Direct link to section: Overview"/>
            </h2>

            <h3 id="concepts" class="anchor">
                "Concepts"
                <AnchorLink href="#concepts" description="Direct link to section: Concepts"/>
            </h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Name"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                            <TableHeaderCell>"When to Use"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Grid.materialize()>"Grid"</Link></TableCell>
                            <TableCell>"Two-dimensional layout with keyboard navigation"</TableCell>
                            <TableCell>"Spatial data or interactive 2D grids (calendars, dashboards)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Table.materialize()>"Table"</Link></TableCell>
                            <TableCell>"Tabular data with sorting and selection"</TableCell>
                            <TableCell>"Row-oriented data with columns (records, lists, reports)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="standalone" class="anchor">
                "Standalone"
                <AnchorLink href="#standalone" description="Direct link to section: Standalone"/>
            </h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Name"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Link href=routes::doc::hooks::UseTree.materialize()>"use_tree"</Link></TableCell>
                            <TableCell>"Hierarchical data structure with expand/collapse"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Data Display", link: "#data-display" },
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Concepts", link: "#concepts" },
                Toc::Leaf { title: "Standalone", link: "#standalone" },
            ]
        }/>
    }
}
