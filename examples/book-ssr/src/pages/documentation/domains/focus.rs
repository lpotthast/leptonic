use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

use super::demos::focus::FocusDomainDemo;

#[component]
pub fn PageFocus() -> impl IntoView {
    view! {
        <Article>
            <h1 id="focus" class="anchor">
                "Focus"
                <AnchorLink href="#focus" description="Direct link to article header"/>
            </h1>

            <p>
                "Hooks and components for managing keyboard focus \u{2014} tracking focus events, "
                "programmatic focus movement, containment within a subtree, and visual indicators. "
                "These are essential for keyboard accessibility and WAI-ARIA compliance."
            </p>

            <p>
                "The focus system separates "
                <em>"observation"</em>" (knowing when focus changes) from "
                <em>"control"</em>" (moving focus programmatically) and "
                <em>"visual feedback"</em>" (showing a focus ring only for keyboard users)."
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
                            <TableCell><Link href=routes::doc::focus::UseFocus.materialize()>"use_focus"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Tracks focus and blur events on an element"</TableCell>
                            <TableCell>"React to an element gaining/losing focus"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::focus::UseFocusWithin.materialize()>"use_focus_within"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Detects when focus is within a container"</TableCell>
                            <TableCell>"Highlight a group when any child is focused"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Makes an element focusable with consistent behavior"</TableCell>
                            <TableCell>"Custom focusable elements (non-native inputs)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::focus::UseFocusManager.materialize()>"use_focus_manager"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Programmatically moves focus between elements"</TableCell>
                            <TableCell>"Arrow-key navigation within a widget"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::focus::UseHasTabbableChild.materialize()>"use_has_tabbable_child"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Checks if an element has tabbable children"</TableCell>
                            <TableCell>"Deciding tabindex for composite widgets"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Shows a focus ring only for keyboard navigation"</TableCell>
                            <TableCell>"Per-element keyboard focus indicator"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::focus::UseFocusVisible.materialize()>"use_focus_visible"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Detects keyboard-driven focus globally"</TableCell>
                            <TableCell>"Global focus-visible state for styling"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></TableCell>
                            <TableCell>"Atom"</TableCell>
                            <TableCell>"Contains focus within a DOM subtree"</TableCell>
                            <TableCell>"Modal dialogs, focus traps"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::focus::FocusRing.materialize()>"FocusRing"</Link></TableCell>
                            <TableCell>"Atom"</TableCell>
                            <TableCell>"Renders a visible focus indicator"</TableCell>
                            <TableCell>"Drop-in keyboard focus ring for any element"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="relationships" class="anchor">
                "Relationships"
                <AnchorLink href="#relationships" description="Direct link to section: Relationships"/>
            </h2>

            <h3>"Observation vs. Control"</h3>

            <p>
                "The focus hooks split into three categories:"
            </p>

            <ul>
                <li>
                    <strong>"Observe: "</strong>
                    <Link href=routes::doc::focus::UseFocus.materialize()>"use_focus"</Link>
                    " and "
                    <Link href=routes::doc::focus::UseFocusWithin.materialize()>"use_focus_within"</Link>
                    " \u{2014} passively watch focus changes."
                </li>
                <li>
                    <strong>"Control: "</strong>
                    <Link href=routes::doc::focus::UseFocusManager.materialize()>"use_focus_manager"</Link>
                    " and "
                    <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>
                    " \u{2014} programmatically move or contain focus."
                </li>
                <li>
                    <strong>"Bridge: "</strong>
                    <Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link>
                    " \u{2014} makes custom elements behave like native focusable elements, combining observation with correct tabindex management."
                </li>
            </ul>

            <h3>"Focus Ring System"</h3>

            <p>"The focus ring is a three-layer system:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"Layer"</TableHeaderCell>
                            <TableHeaderCell>"Name"</TableHeaderCell>
                            <TableHeaderCell>"Role"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>"Global detector"</TableCell>
                            <TableCell><Link href=routes::doc::focus::UseFocusVisible.materialize()>"use_focus_visible"</Link></TableCell>
                            <TableCell>"Tracks whether the user is navigating via keyboard"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Per-element hook"</TableCell>
                            <TableCell><Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></TableCell>
                            <TableCell>"Returns whether to show a ring on a specific element"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Rendered atom"</TableCell>
                            <TableCell><Link href=routes::doc::focus::FocusRing.materialize()>"FocusRing"</Link></TableCell>
                            <TableCell>"Drop-in component that renders the visible indicator"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3>"Cross-Domain"</h3>

            <ul>
                <li>
                    "Focus hooks compose into "
                    <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>
                    " (which includes "
                    <Code inline=true>"use_focus_ring"</Code>
                    " for keyboard focus indicators)."
                </li>
                <li>
                    <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>
                    " is used by the "
                    <Link href=routes::doc::overlays::UseOverlay.materialize()>"overlay system"</Link>
                    " to trap focus within modal dialogs."
                </li>
                <li>
                    "The merged props types in the "
                    <Link href=routes::doc::Interactions.materialize()>"Interactions"</Link>
                    " domain (e.g. "
                    <Code inline=true>"MergedPressHoverFocusRingProps"</Code>
                    ") combine interaction and focus hooks."
                </li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest focus hook: track whether an element is focused."</p>

            <DemoShell
                source=include_str!("demos/focus.rs")
                description="Focus management across elements"
            >
                <FocusDomainDemo />
            </DemoShell>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Focus", link: "#focus" },
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Relationships", link: "#relationships" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
            ]
        }/>
    }
}
