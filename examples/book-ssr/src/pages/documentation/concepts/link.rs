use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::link::LinkConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageLinkOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="link" class="anchor">
                "Link"
                <AnchorLink href="#link" description="Direct link to article header"/>
            </h1>

            <p>
                "Links are navigation elements that take users to another page, URL, "
                "or location within the current page. Leptonic provides two link types: "
                <Code inline=true>"Link"</Code>" for standard navigation and "
                <Code inline=true>"AnchorLink"</Code>" for smooth-scrolling to an anchor on the same page."
            </p>

            <p>
                "Leptonic provides links at two abstraction levels (hooks and atoms). "
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
                            <TableCell>"Navigate to another page or URL"</TableCell>
                            <TableCell><b>"Link"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Scroll to a section on the current page"</TableCell>
                            <TableCell><b>"AnchorLink"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Trigger an action (submit, delete)"</TableCell>
                            <TableCell>"Button"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Navigate with button styling"</TableCell>
                            <TableCell>"LinkButton (atom)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <p>
                "If it navigates, use a link. If it triggers an action, use a button \u{2014} "
                "regardless of how the element looks visually."
            </p>

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
                <li><Link href=routes::doc::link::UseLink.materialize()>"Hook: use_link"</Link></li>
                <li><Link href=routes::doc::link::UseAnchorLink.materialize()>"Hook: use_anchor_link"</Link></li>
                <li><Link href=routes::doc::link::LinkAtom.materialize()>"Atom: Link"</Link></li>
                <li><Link href=routes::doc::link::AnchorLinkAtom.materialize()>"Atom: AnchorLink"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use a link (atom layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    <Link href="https://github.com/lpotthast/leptonic">"Leptonic on GitHub"</Link>
                "#)}
            </Code>

            <DemoShell description="Styled link navigation" source=include_str!("demos/link.rs")>
                <LinkConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Native "<Code inline=true>"<a>"</Code>" elements provide built-in link semantics. "
                "When non-anchor elements are used as links, the hook adds the appropriate ARIA attributes."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"link\""</Code>" \u{2014} added automatically by the hook on non-anchor elements"</li>
                <li><Code inline=true>"aria-disabled"</Code>" \u{2014} \"true\" when disabled"</li>
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
                            <TableCell>"Activates the link"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Link", link: "#link" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}
