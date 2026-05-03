use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, toc::Toc},
    routes,
};

#[component]
pub fn PageNavigation() -> impl IntoView {
    view! {
        <Article>
            <h1 id="navigation" class="anchor">
                "Navigation"
                <AnchorLink href="#navigation" description="Direct link to article header"/>
            </h1>

            <p>
                "Components for moving between views and locations \u{2014} links, menus, and breadcrumb trails."
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
                            <TableCell><Link href=routes::doc::Link.materialize()>"Link"</Link></TableCell>
                            <TableCell>"Navigate to a URL or anchor"</TableCell>
                            <TableCell>"Inline navigation to another page or section"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Menu.materialize()>"Menu"</Link></TableCell>
                            <TableCell>"Action menu with keyboard navigation"</TableCell>
                            <TableCell>"Grouping multiple related actions behind a trigger"</TableCell>
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
                            <TableCell><Link href=routes::doc::hooks::UseBreadcrumbs.materialize()>"use_breadcrumbs"</Link></TableCell>
                            <TableCell>"Hierarchical navigation trail"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            // ── Relationships ───────────────────────────────────────

            <h2 id="relationships" class="anchor">
                "Relationships"
                <AnchorLink href="#relationships" description="Direct link to section: Relationships"/>
            </h2>

            <p>"How navigation components compose together:"</p>

            <ul>
                <li><strong>"Breadcrumbs"</strong>" show the user\u{2019}s position within a hierarchy, with each level being a Link."</li>
                <li><strong>"Links"</strong>" handle direct page-to-page navigation. They can appear standalone, inside menus, or as breadcrumb segments."</li>
                <li><strong>"Menus"</strong>" group multiple actions (or navigation links) behind a single trigger, keeping the interface uncluttered."</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Navigation", link: "#navigation" },
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Concepts", link: "#concepts" },
                Toc::Leaf { title: "Standalone", link: "#standalone" },
                Toc::Leaf { title: "Relationships", link: "#relationships" },
            ]
        }/>
    }
}
