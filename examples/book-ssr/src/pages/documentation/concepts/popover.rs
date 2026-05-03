use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::popover::PopoverConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PagePopoverOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="popover" class="anchor">
                "Popover"
                <AnchorLink href="#popover" description="Direct link to article header"/>
            </h1>

            <p>
                "Popovers are floating overlays anchored to a trigger element. "
                "They appear on hover or click and display contextual content \u{2014} "
                "tooltips with rich formatting, mini forms, or previews. "
                "Unlike modals, popovers don't block the rest of the page."
            </p>

            <p>
                "Leptonic provides popovers at three abstraction levels. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer. "
                "The popover hook combines "<Code inline=true>"use_overlay"</Code>
                " (dismiss behavior) with "<Code inline=true>"use_overlay_position"</Code>
                " (viewport-aware placement)."
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
                            <TableCell>"Show rich contextual content anchored to an element"</TableCell>
                            <TableCell><b>"Popover"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Block the page and require a decision"</TableCell>
                            <TableCell>"Modal"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Show a short text label on hover/focus"</TableCell>
                            <TableCell>"Tooltip"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Show an action list triggered by a button"</TableCell>
                            <TableCell>"Menu"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <p>
                "Popovers can be modal (blocking) or non-modal. "
                "Non-modal popovers allow interaction with the page behind them "
                "and are the more common case."
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
                <li><Link href=routes::doc::popover::Hook.materialize()>"Hook: use_popover"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Atom: Popover"</Link></li>
                <li><Link href=routes::doc::popover::Component.materialize()>"Component: Popover"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use a popover (component layer, click to toggle):"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    <Popover>
                        <PopoverTrigger slot>
                            <Button>"Click me"</Button>
                        </PopoverTrigger>
                        "Popover content appears here."
                    </Popover>
                "#)}
            </Code>

            <DemoShell description="Popover with trigger button" source=include_str!("demos/popover.rs")>
                <PopoverConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Popover accessibility depends on the use case. "
                "Focus trapping is automatic for modal popovers; "
                "non-modal popovers allow focus to move freely."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"aria-haspopup"</Code>" \u{2014} on the trigger, indicates a popup will appear"</li>
                <li><Code inline=true>"aria-expanded"</Code>" \u{2014} on the trigger, reflects open/closed state"</li>
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
                            <TableCell><Code inline=true>"Escape"</Code></TableCell>
                            <TableCell>"Closes the popover"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Tab"</Code></TableCell>
                            <TableCell>"Moves focus (non-modal popovers allow focus to leave)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Popover", link: "#popover" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}
