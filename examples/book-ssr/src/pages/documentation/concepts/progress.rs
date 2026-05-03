use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::progress::ProgressConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageProgressOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="progress" class="anchor">
                "Progress"
                <AnchorLink href="#progress" description="Direct link to article header"/>
            </h1>

            <p>
                "Progress bars communicate how far along a process has advanced. "
                "They support both determinate mode (specific percentage) and "
                "indeterminate mode (activity without a known endpoint). "
                "Use them for file uploads, form submissions, or any operation with observable progress."
            </p>

            <p>
                "Leptonic provides progress bars at two abstraction levels. "
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
                            <TableCell>"Show progress toward a known total"</TableCell>
                            <TableCell><b>"ProgressBar"</b>" (determinate)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Indicate activity without a known endpoint"</TableCell>
                            <TableCell><b>"ProgressBar"</b>" (indeterminate, "<Code inline=true>"progress=None"</Code>")"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Placeholder while content loads"</TableCell>
                            <TableCell>"Skeleton"</TableCell>
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
                <li><Link href=routes::doc::progress::Hook.materialize()>"Hook: use_progress_bar"</Link></li>
                <li><Link href=routes::doc::progress::Component.materialize()>"Component: ProgressBar"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use a progress bar (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    <ProgressBar progress=Signal::derive(move || Some(75.0)) />
                ")}
            </Code>

            <DemoShell description="Progress bar with percentage" source=include_str!("demos/progress.rs")>
                <ProgressConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic progress bars follow the WAI-ARIA Progressbar pattern. "
                "Progress bars are informational and have no keyboard interaction."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"progressbar\""</Code></li>
                <li><Code inline=true>"aria-valuenow"</Code>" \u{2014} current value (absent in indeterminate mode)"</li>
                <li><Code inline=true>"aria-valuemin"</Code>" / "<Code inline=true>"aria-valuemax"</Code>" \u{2014} range bounds"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Progress", link: "#progress" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}
