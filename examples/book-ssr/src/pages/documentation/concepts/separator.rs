use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::separator::SeparatorConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageSeparatorOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="separator" class="anchor">
                "Separator"
                <AnchorLink href="#separator" description="Direct link to article header"/>
            </h1>

            <p>
                "Separators are visual dividers that create clear boundaries between sections of content. "
                "They carry semantic meaning \u{2014} assistive technology announces them as thematic breaks, "
                "distinguishing them from purely decorative borders or spacing."
            </p>

            <p>
                "Leptonic provides separators at two abstraction levels. "
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
                            <TableCell>"Mark a semantic boundary between content sections"</TableCell>
                            <TableCell><b>"Separator"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Add visual spacing without semantic meaning"</TableCell>
                            <TableCell>"CSS margin / padding"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Add a decorative line"</TableCell>
                            <TableCell>"CSS border"</TableCell>
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
                <li><Link href=routes::doc::separator::Hook.materialize()>"Hook: use_separator"</Link></li>
                <li><Link href=routes::doc::separator::Component.materialize()>"Component: Separator"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use a separator (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    <p>"Content above"</p>
                    <Separator />
                    <p>"Content below"</p>
                "#)}
            </Code>

            <DemoShell description="Horizontal separator" source=include_str!("demos/separator.rs")>
                <SeparatorConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic separators follow the WAI-ARIA Separator pattern. "
                "Separators are not interactive."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"separator\""</Code>" \u{2014} on non-"<Code inline=true>"<hr>"</Code>" elements (native "<Code inline=true>"<hr>"</Code>" has implicit role)"</li>
                <li><Code inline=true>"aria-orientation"</Code>" \u{2014} \"horizontal\" (default) or \"vertical\""</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Separator", link: "#separator" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}
