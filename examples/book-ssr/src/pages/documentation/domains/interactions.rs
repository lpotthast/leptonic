use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

use super::demos::interactions::InteractionsDomainDemo;

#[component]
pub fn PageInteractions() -> impl IntoView {
    view! {
        <Article>
            <h1 id="interactions" class="anchor">
                "Interactions"
                <AnchorLink href="#interactions" description="Direct link to article header"/>
            </h1>

            <p>
                "Low-level hooks for handling pointer, keyboard, and scroll interactions. "
                "These hooks form the behavioral foundation for all interactive components \u{2014} "
                "higher-level concepts like Button, Slider, and Menu compose them internally."
            </p>

            <p>
                "Each hook normalizes browser differences across mouse, touch, keyboard, and screen reader input, "
                "so components built on top receive a consistent event model regardless of input device."
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
                            <TableCell><Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Normalizes press interactions across pointer, keyboard, and screen readers"</TableCell>
                            <TableCell>"Any clickable/tappable element"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Detects hover state with pointer events"</TableCell>
                            <TableCell>"Hover-dependent UI (tooltips, highlights)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Tracks pointer move events relative to an element"</TableCell>
                            <TableCell>"Drag-like interactions (sliders, color pickers)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Handles raw keyboard events on an element"</TableCell>
                            <TableCell>"Custom keyboard shortcuts beyond press"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::interactions::UseInteractOutside.materialize()>"use_interact_outside"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Detects interactions outside of an element"</TableCell>
                            <TableCell>"Dismiss-on-click-outside (popovers, menus)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::interactions::UseScrollWheel.materialize()>"use_scroll_wheel"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Handles scroll wheel events"</TableCell>
                            <TableCell>"Custom scroll behavior (number steppers)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::interactions::UsePreventScroll.materialize()>"use_prevent_scroll"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Prevents body scrolling while active"</TableCell>
                            <TableCell>"Modals, overlays that lock scroll"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::interactions::Dnd.materialize()>"Drag & Drop"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Drag and drop interactions"</TableCell>
                            <TableCell>"Reorderable lists, file upload zones"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::interactions::PressResponder.materialize()>"PressResponder"</Link></TableCell>
                            <TableCell>"Atom"</TableCell>
                            <TableCell>"Injects press behavior into descendant pressable elements via context"</TableCell>
                            <TableCell>"Trigger components (menu triggers, dialog triggers)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="relationships" class="anchor">
                "Relationships"
                <AnchorLink href="#relationships" description="Direct link to section: Relationships"/>
            </h2>

            <p>
                <Code inline=true>"use_press"</Code>" and "<Code inline=true>"use_hover"</Code>
                " are the most commonly paired hooks \u{2014} nearly every interactive element needs both. "
                "The library provides pre-built merged props types for frequent combinations:"
            </p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"Merged Type"</TableHeaderCell>
                            <TableHeaderCell>"Combines"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"MergedPressHoverProps"</Code></TableCell>
                            <TableCell>"use_press + use_hover"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"MergedPressHoverFocusRingProps"</Code></TableCell>
                            <TableCell>"use_press + use_hover + use_focus_ring"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"MergedPressFocusRingProps"</Code></TableCell>
                            <TableCell>"use_press + use_focus_ring"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"MergedHoverFocusRingProps"</Code></TableCell>
                            <TableCell>"use_hover + use_focus_ring"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"MergedFocusablePressProps"</Code></TableCell>
                            <TableCell>"use_focusable + use_press"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"MergedFocusablePressFocusRingProps"</Code></TableCell>
                            <TableCell>"use_focusable + use_press + use_focus_ring"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <p>
                "Cross-domain relationships:"
            </p>

            <ul>
                <li>
                    <Link href=routes::doc::interactions::UseInteractOutside.materialize()>"use_interact_outside"</Link>
                    " is used by the "<Link href=routes::doc::overlays::UseOverlay.materialize()>"overlay system"</Link>
                    " to dismiss popovers and menus when clicking outside."
                </li>
                <li>
                    <Link href=routes::doc::interactions::UsePreventScroll.materialize()>"use_prevent_scroll"</Link>
                    " is shared with the "<Link href=routes::doc::overlays::UseOverlay.materialize()>"overlay system"</Link>
                    " to lock body scroll while modals are open."
                </li>
                <li>
                    <Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link>
                    " powers slider thumb dragging in the "
                    <Link href=routes::doc::slider::Hook.materialize()>"slider"</Link>" hooks."
                </li>
                <li>
                    <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>
                    " handles raw key events, while "
                    <Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>
                    " specifically handles Enter/Space as press activations."
                </li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest interaction: a pressable element that counts presses."</p>

            <DemoShell
                source=include_str!("demos/interactions.rs")
                description="Press and hover interaction tracking"
            >
                <InteractionsDomainDemo />
            </DemoShell>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Interactions", link: "#interactions" },
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Relationships", link: "#relationships" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
            ]
        }/>
    }
}
