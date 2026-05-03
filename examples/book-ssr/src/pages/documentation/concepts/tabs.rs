use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::tabs::TabsConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageTabsOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="tabs" class="anchor">
                "Tabs"
                <AnchorLink href="#tabs" description="Direct link to article header"/>
            </h1>

            <p>
                "Tabs organize content into panels, showing one panel at a time. "
                "The user switches panels by selecting a tab from a horizontal (or vertical) tab list. "
                "Tabs are ideal when content is parallel in structure \u{2014} "
                "settings categories, data views, or step-by-step sections."
            </p>

            <p>
                "Leptonic provides tabs at two abstraction levels. "
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
                            <TableCell>"Switch between parallel content panels"</TableCell>
                            <TableCell><b>"Tabs"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Expand/collapse independent sections"</TableCell>
                            <TableCell>"Collapsible"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Navigate between pages"</TableCell>
                            <TableCell>"Router / Link"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Choose a value from options"</TableCell>
                            <TableCell>"Select / Radio"</TableCell>
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
                <li><Link href=routes::doc::tabs::Hook.materialize()>"Hook: use_tabs"</Link></li>
                <li><Link href=routes::doc::tabs::Component.materialize()>"Component: Tabs"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use tabs (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    <Tabs>
                        <Tab name="tab-1" label=|| "First">"Content of first tab"</Tab>
                        <Tab name="tab-2" label=|| "Second">"Content of second tab"</Tab>
                        <Tab name="tab-3" label=|| "Third">"Content of third tab"</Tab>
                    </Tabs>
                "#)}
            </Code>

            <DemoShell description="Tabbed content panels" source=include_str!("demos/tabs.rs")>
                <TabsConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic tabs follow the WAI-ARIA Tabs pattern."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"tablist\""</Code>" on the tab bar, "
                    <Code inline=true>"role=\"tab\""</Code>" on each tab, "
                    <Code inline=true>"role=\"tabpanel\""</Code>" on each panel"</li>
                <li><Code inline=true>"aria-selected"</Code>" \u{2014} \"true\" on the active tab"</li>
                <li><Code inline=true>"aria-controls"</Code>" / "<Code inline=true>"aria-labelledby"</Code>" \u{2014} connect tabs to panels"</li>
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
                            <TableCell><Code inline=true>"Arrow Right / Down"</Code></TableCell>
                            <TableCell>"Focus next tab"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Arrow Left / Up"</Code></TableCell>
                            <TableCell>"Focus previous tab"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Home"</Code></TableCell>
                            <TableCell>"Focus first tab"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"End"</Code></TableCell>
                            <TableCell>"Focus last tab"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Tab"</Code></TableCell>
                            <TableCell>"Move focus from tab bar into panel content"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Tabs", link: "#tabs" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}
