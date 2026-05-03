use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, toc::Toc},
    routes,
};

#[component]
pub fn PageLayoutCategory() -> impl IntoView {
    view! {
        <Article>
            <h1 id="layout" class="anchor">
                "Layout"
                <AnchorLink href="#layout" description="Direct link to article header"/>
            </h1>

            <p>
                "Components for structuring and organizing page content \u{2014} collapsible sections, dividers, tabbed panels, and more."
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
                            <TableCell><Link href=routes::doc::Collapsible.materialize()>"Collapsible"</Link></TableCell>
                            <TableCell>"Expandable content sections with disclosure"</TableCell>
                            <TableCell>"Hide secondary content until the user expands it"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Separator.materialize()>"Separator"</Link></TableCell>
                            <TableCell>"Visual divider between content sections"</TableCell>
                            <TableCell>"Visually separate groups of related content"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Tabs.materialize()>"Tabs"</Link></TableCell>
                            <TableCell>"Tabbed panel navigation"</TableCell>
                            <TableCell>"Switch between multiple views in the same space"</TableCell>
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
                            <TableCell><Link href=routes::doc::components::AppBar.materialize()>"App Bar"</Link></TableCell>
                            <TableCell>"Top-level application navigation bar"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::components::Drawer.materialize()>"Drawer"</Link></TableCell>
                            <TableCell>"Slide-out side panel"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::components::Skeleton.materialize()>"Skeleton"</Link></TableCell>
                            <TableCell>"Loading placeholder shapes"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::components::Stack.materialize()>"Stack"</Link></TableCell>
                            <TableCell>"Vertical or horizontal element stacking"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::hooks::UseToolbar.materialize()>"use_toolbar"</Link></TableCell>
                            <TableCell>"Toolbar with keyboard navigation"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            // ── Relationships ───────────────────────────────────────

            <h2 id="relationships" class="anchor">
                "Relationships"
                <AnchorLink href="#relationships" description="Direct link to section: Relationships"/>
            </h2>

            <p>"How layout components relate to each other:"</p>

            <ul>
                <li><strong>"Tabs vs Collapsible"</strong>" \u{2014} Tabs switch between mutually exclusive panels; Collapsible sections can be independently expanded or collapsed."</li>
                <li><strong>"Stack vs Grid"</strong>" \u{2014} Stack arranges items along a single axis (vertical or horizontal); Grid provides two-dimensional layout with keyboard navigation."</li>
                <li><strong>"Drawer vs App Bar"</strong>" \u{2014} App Bar provides persistent top-level navigation; Drawer slides in from the side for secondary navigation or settings."</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Layout", link: "#layout" },
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Concepts", link: "#concepts" },
                Toc::Leaf { title: "Standalone", link: "#standalone" },
                Toc::Leaf { title: "Relationships", link: "#relationships" },
            ]
        }/>
    }
}
