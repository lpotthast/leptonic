use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::collapsible::CollapsibleConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageCollapsibleOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="collapsible" class="anchor">
                "Collapsible"
                <AnchorLink href="#collapsible" description="Direct link to article header"/>
            </h1>

            <p>
                "Collapsibles let users expand and collapse sections of content. "
                "They reduce visual clutter by hiding details until the user asks for them. "
                "Multiple collapsibles can be grouped with "<Code inline=true>"Collapsibles"</Code>
                ", which optionally enforces accordion behavior "
                "(opening one closes the others)."
            </p>

            <p>
                "Leptonic provides collapsibles at two abstraction levels. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer. "
                "The underlying WAI-ARIA pattern is called \"Disclosure\", hence the hook name "
                <Code inline=true>"use_disclosure"</Code>"."
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
                            <TableCell>"Show/hide supplementary content in place"</TableCell>
                            <TableCell><b>"Collapsible"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Enforce only one section open at a time (accordion)"</TableCell>
                            <TableCell><Code inline=true>"Collapsibles"</Code>" with "<Code inline=true>"OnOpen::CloseOthers"</Code></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Switch between parallel content panels"</TableCell>
                            <TableCell>"Tabs"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Show content in a blocking overlay"</TableCell>
                            <TableCell>"Modal"</TableCell>
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
                <li><Link href=routes::doc::collapsible::Hook.materialize()>"Hook: use_disclosure"</Link></li>
                <li><Link href=routes::doc::collapsible::Component.materialize()>"Component: Collapsible"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use a collapsible (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    <Collapsible>
                        <CollapsibleHeader slot>"Click to expand"</CollapsibleHeader>
                        <CollapsibleBody slot>"This content is hidden until expanded."</CollapsibleBody>
                    </Collapsible>
                "#)}
            </Code>

            <DemoShell description="Collapsible content section" source=include_str!("demos/collapsible.rs")>
                <CollapsibleConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic collapsibles follow the WAI-ARIA Disclosure pattern."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"aria-expanded"</Code>" \u{2014} \"true\" or \"false\" on the trigger"</li>
                <li><Code inline=true>"aria-controls"</Code>" \u{2014} points to the content panel"</li>
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
                            <TableCell><Code inline=true>"Enter"</Code></TableCell>
                            <TableCell>"Toggles expanded/collapsed"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Space"</Code></TableCell>
                            <TableCell>"Toggles expanded/collapsed"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Collapsible", link: "#collapsible" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}
