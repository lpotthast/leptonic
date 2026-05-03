use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

use super::demos::overlays::OverlaysDomainDemo;

#[component]
pub fn PageOverlays() -> impl IntoView {
    view! {
        <Article>
            <h1 id="overlays" class="anchor">
                "Overlays"
                <AnchorLink href="#overlays" description="Direct link to article header"/>
            </h1>

            <p>
                "Infrastructure for building floating UI that appears above main content. "
                "The overlay hooks handle dismiss behavior, ARIA trigger attributes, and CSS positioning. "
                "Higher-level concepts like "
                <Link href=routes::doc::popover::Hook.materialize()>"Popover"</Link>", "
                <Link href=routes::doc::modal::Hook.materialize()>"Modal"</Link>", and "
                <Link href=routes::doc::tooltip::Hook.materialize()>"Tooltip"</Link>
                " compose these primitives internally."
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
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                            <TableHeaderCell>"When to Use"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Link href=routes::doc::overlays::UseOverlay.materialize()>"use_overlay"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Core overlay lifecycle and dismiss behavior"</TableCell>
                            <TableCell>"Any floating element that can be dismissed"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::overlays::UseOverlay.materialize()>"use_overlay_trigger"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"ARIA attributes linking trigger to overlay"</TableCell>
                            <TableCell>"Button or element that opens an overlay"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::overlays::UseOverlay.materialize()>"use_overlay_position"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"CSS positioning relative to a trigger"</TableCell>
                            <TableCell>"Popovers, tooltips, menus that float near a trigger"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Composed overlay with positioning and dismiss"</TableCell>
                            <TableCell>"Popover panels triggered by a button"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::overlays::DismissButton.materialize()>"DismissButton"</Link></TableCell>
                            <TableCell>"Atom"</TableCell>
                            <TableCell>"Screen-reader-only button to dismiss an overlay"</TableCell>
                            <TableCell>"Inside popovers and modals for accessibility"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="relationships" class="anchor">
                "Relationships"
                <AnchorLink href="#relationships" description="Direct link to section: Relationships"/>
            </h2>

            <h3>"Composition Hierarchy"</h3>

            <p>"Higher-level hooks compose the overlay primitives:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"Composed Hook"</TableHeaderCell>
                            <TableHeaderCell>"Built From"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"use_popover"</Code></TableCell>
                            <TableCell>"use_overlay + use_overlay_position + use_overlay_trigger + use_prevent_scroll"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_modal"</Code></TableCell>
                            <TableCell>"use_overlay + FocusScope"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_tooltip"</Code></TableCell>
                            <TableCell>"use_overlay + use_overlay_position"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3>"Cross-Domain"</h3>

            <ul>
                <li>
                    <Link href=routes::doc::interactions::UsePreventScroll.materialize()>"use_prevent_scroll"</Link>
                    " (from the "
                    <Link href=routes::doc::Interactions.materialize()>"Interactions"</Link>
                    " domain) locks body scroll while modals and popovers are open."
                </li>
                <li>
                    <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>
                    " (from the "
                    <Link href=routes::doc::Focus.materialize()>"Focus"</Link>
                    " domain) traps focus within modal dialogs."
                </li>
                <li>
                    <Link href=routes::doc::interactions::UseInteractOutside.materialize()>"use_interact_outside"</Link>
                    " (from the "
                    <Link href=routes::doc::Interactions.materialize()>"Interactions"</Link>
                    " domain) powers the dismiss-on-click-outside behavior in "
                    <Code inline=true>"use_overlay"</Code>"."
                </li>
                <li>
                    <Link href=routes::doc::overlays::DismissButton.materialize()>"DismissButton"</Link>
                    " is a screen-reader-only element placed inside popovers and modals, "
                    "allowing assistive technology users to dismiss the overlay via a hidden button."
                </li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>
                "The basic overlay pattern: a trigger button toggles visibility of floating content. "
                "The full overlay hooks add dismiss behavior, ARIA attributes, and positioning \u{2014} "
                "this demo shows the core open/close concept."
            </p>

            <DemoShell
                source=include_str!("demos/overlays.rs")
                description="Overlay positioning with trigger"
            >
                <OverlaysDomainDemo />
            </DemoShell>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Overlays", link: "#overlays" },
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Relationships", link: "#relationships" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
            ]
        }/>
    }
}
