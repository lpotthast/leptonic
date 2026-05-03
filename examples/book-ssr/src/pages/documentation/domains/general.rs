use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, toc::Toc},
    routes,
};

#[component]
pub fn PageGeneral() -> impl IntoView {
    view! {
        <Article>
            <h1 id="general" class="anchor">
                "General"
                <AnchorLink href="#general" description="Direct link to article header"/>
            </h1>

            <p>
                "Foundational components and utilities used across the library \u{2014} typography, icons, and callback helpers. "
                "These are building blocks that don\u{2019}t belong to a specific interaction domain. "
                "They typically exist at the component layer only."
            </p>

            <h2 id="overview" class="anchor">
                "Overview"
                <AnchorLink href="#overview" description="Direct link to section: Overview"/>
            </h2>

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
                            <TableCell><Link href=routes::doc::components::Typography.materialize()>"Typography"</Link></TableCell>
                            <TableCell>"Text styling and semantic elements"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::components::Icon.materialize()>"Icon"</Link></TableCell>
                            <TableCell>"SVG icon rendering"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::components::Callback.materialize()>"Callback"</Link></TableCell>
                            <TableCell>"Callback type utilities"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="relationships" class="anchor">
                "Relationships"
                <AnchorLink href="#relationships" description="Direct link to section: Relationships"/>
            </h2>

            <ul>
                <li><strong>"Typography"</strong>" provides text elements ("<code>"H1"</code>"\u{2013}"<code>"H6"</code>", "<code>"P"</code>", "<code>"Code"</code>") used on every documentation page and in most applications."</li>
                <li><strong>"Icon"</strong>" renders SVG icons that can be placed inside buttons, links, and other interactive elements."</li>
                <li><strong>"Callback"</strong>" provides the "<code>"Callback<T>"</code>" type used throughout leptonic\u{2019}s hook and component APIs."</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "General", link: "#general" },
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Relationships", link: "#relationships" },
            ]
        }/>
    }
}
