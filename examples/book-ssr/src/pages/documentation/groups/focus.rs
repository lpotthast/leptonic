use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::focus::FocusQuickStartDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageFocus() -> impl IntoView {
    view! {
        <DocPage title="Focus">
            <p>
                "Hooks, atoms and utilities for keyboard focus: tracking where the focus is, making elements focusable, moving "
                "focus programmatically, keeping it inside a subtree, and showing a focus ring only to keyboard users. The "
                "keyboard support of leptonic\u{2019}s concepts is built on them."
            </p>
            <p>
                "They belong together because they share one model of focus that separates "<em>"observation"</em>
                " (knowing when focus changes) from "<em>"control"</em>" (moving focus) and "<em>"visual feedback"</em>
                " (showing a focus ring only for keyboard users)."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Focus.materialize()/>
            </Section>

            <Section title="Relationships">
                <Section title="Observation vs. Control">
                    <ul>
                        <li>
                            <strong>"Observe: "</strong>
                            <Link href=routes::doc::focus::UseFocus.materialize()>"use_focus"</Link>" and "
                            <Link href=routes::doc::focus::UseFocusWithin.materialize()>"use_focus_within"</Link>
                            " watch focus changes without changing anything."
                        </li>
                        <li>
                            <strong>"Make focusable: "</strong>
                            <Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link>
                            " makes a custom element behave like a native focusable element: it manages the "
                            <Code inline=true>"tabindex"</Code>", observes focus and keys, and focuses the element on request. "
                            "The "<Link href=routes::doc::focus::Focusable.materialize()>"Focusable"</Link>
                            " atom applies it to a child, e.g. an info icon with a tooltip."
                        </li>
                        <li>
                            <strong>"Control: "</strong>
                            <Link href=routes::doc::focus::UseFocusManager.materialize()>"use_focus_manager"</Link>" moves "
                            "the focus between the elements of a container, "
                            <Link href=routes::doc::focus::FocusManagerProvider.materialize()>"FocusManagerProvider"</Link>
                            " renders such a container and hands the manager to its children, and "
                            <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>
                            " contains, restores and auto-focuses focus within a subtree."
                        </li>
                        <li>
                            <strong>"Tab stops: "</strong>
                            <Link href=routes::doc::focus::UseHasTabbableChild.materialize()>"use_has_tabbable_child"</Link>
                            " tells a container whether it holds tabbable elements, so it can become a tab stop itself "
                            "when it doesn\u{2019}t. The "<Link href=routes::doc::focus::Focusability.materialize()>"focusability"</Link>
                            " functions decide what counts as focusable and tabbable for all of them."
                        </li>
                    </ul>
                </Section>

                <Section title="Focus Ring System">
                    <p>"Focus rings are built from three parts:"</p>

                    <DocTable headers=&["Part", "Name", "Role"]>
                        <TableRow>
                            <TableCell>"Global detector"</TableCell>
                            <TableCell><Link href=routes::doc::focus::UseFocusVisible.materialize()>"use_focus_visible"</Link></TableCell>
                            <TableCell>"Tracks whether the user navigates with the keyboard"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Per-element hook"</TableCell>
                            <TableCell><Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></TableCell>
                            <TableCell>"Combines that with the element\u{2019}s focus state and sets "<Code inline=true>"data-focus-visible"</Code></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Atom"</TableCell>
                            <TableCell><Link href=routes::doc::focus::FocusRing.materialize()>"FocusRing"</Link></TableCell>
                            <TableCell>"Applies "<Code inline=true>"use_focus_ring"</Code>" to its child, without rendering an element"</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>

                <Section title="Other Areas">
                    <ul>
                        <li>
                            <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" includes "
                            <Code inline=true>"use_focus_ring"</Code>" and sets "<Code inline=true>"data-focus-visible"</Code>" itself."
                        </li>
                        <li>
                            "The "<Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link>" and "
                            <Link href=routes::doc::popover::Atom.materialize()>"Popover Atoms"</Link>" wrap their content in "
                            <Code inline=true>"FocusScope"</Code>" to contain and restore focus."
                        </li>
                        <li>
                            <Link href=routes::doc::Toolbar.materialize()>"Toolbars"</Link>" and "
                            <Link href=routes::doc::Radio.materialize()>"radio groups"</Link>" move the focus between "
                            "their controls with a focus manager; "<Link href=routes::doc::Grid.materialize()>"grids"</Link>
                            " and "<Link href=routes::doc::Tabs.materialize()>"tab panels"</Link>" use "
                            <Code inline=true>"use_has_tabbable_child"</Code>"."
                        </li>
                        <li>
                            "The merged props types of "<Link href=routes::doc::Interactions.materialize()>"Interactions"</Link>
                            " (e.g. "<Code inline=true>"MergedPressHoverFocusRingProps"</Code>
                            ") combine interaction and focus hooks on one element."
                        </li>
                    </ul>
                </Section>
            </Section>

            <Section title="Decision Guide">
                <DocTable headers=&["You want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"React when an element gains or loses focus"</TableCell>
                        <TableCell><Link href=routes::doc::focus::UseFocus.materialize()>"use_focus"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Highlight a group while focus is anywhere inside"</TableCell>
                        <TableCell><Link href=routes::doc::focus::UseFocusWithin.materialize()>"use_focus_within"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Make a non-interactive element (icon, scrollable region) a tab stop"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::focus::Focusable.materialize()>"Focusable"</Link>", or "
                            <Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a focus ring only for keyboard users"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::focus::FocusRing.materialize()>"FocusRing"</Link>", or "
                            <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Move focus with arrow keys in your own composite control"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::focus::FocusManagerProvider.materialize()>"FocusManagerProvider"</Link>", or "
                            <Link href=routes::doc::focus::UseFocusManager.materialize()>"use_focus_manager"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Keep DOM focus in an input while the arrow keys move through a list"</TableCell>
                        <TableCell><Link href=routes::doc::focus::VirtualFocus.materialize()>"virtual_focus"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Keep focus inside a dialog and restore it afterwards"</TableCell>
                        <TableCell><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The simplest focus hook, "<Link href=routes::doc::focus::UseFocus.materialize()>"use_focus"</Link>
                    ", tracks whether an element is focused:"
                </p>

                <Demo
                    description="Email field showing whether it has focus, using use_focus"
                    source=include_str!("demos/focus.rs")
                    source_open=true
                >
                    <FocusQuickStartDemo/>
                </Demo>
            </Section>
        </DocPage>
    }
}
