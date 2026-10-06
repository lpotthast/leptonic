use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::focus::FocusDomainDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageFocus() -> impl IntoView {
    view! {
        <DocPage title="Focus">
            <p>
                "Hooks and atoms for keyboard focus: tracking focus events, moving focus programmatically, keeping it inside "
                "a subtree, and showing a focus ring only to keyboard users. They are the foundation of keyboard accessibility "
                "in every leptonic widget."
            </p>

            <p>
                "The focus system separates "<em>"observation"</em>" (knowing when focus changes) from "<em>"control"</em>
                " (moving focus) and "<em>"visual feedback"</em>" (showing a focus ring only for keyboard users)."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Focus.materialize()/>
            </Section>

            <Section title="Relationships">
                <Section title="Observation vs. Control">
                    <p>"The focus hooks fall into three groups:"</p>

                    <ul>
                        <li>
                            <strong>"Observe: "</strong>
                            <Link href=routes::doc::focus::UseFocus.materialize()>"use_focus"</Link>" and "
                            <Link href=routes::doc::focus::UseFocusWithin.materialize()>"use_focus_within"</Link>
                            " watch focus changes without changing anything."
                        </li>
                        <li>
                            <strong>"Control: "</strong>
                            <Link href=routes::doc::focus::UseFocusManager.materialize()>"use_focus_manager"</Link>" and "
                            <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>
                            " move or contain focus."
                        </li>
                        <li>
                            <strong>"Bridge: "</strong>
                            <Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link>
                            " makes custom elements behave like native focusable elements: it observes focus and manages the "
                            <Code inline=true>"tabindex"</Code>"."
                        </li>
                    </ul>
                </Section>

                <Section title="Focus Ring System">
                    <p>"Focus rings build on three layers:"</p>

                    <DocTable headers=&["Layer", "Name", "Role"]>
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

                <Section title="Cross-Domain">
                    <ul>
                        <li>
                            <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" includes "
                            <Code inline=true>"use_focus_ring"</Code>" and sets "<Code inline=true>"data-focus-visible"</Code>" itself."
                        </li>
                        <li>
                            "The "<Link href=routes::doc::Modal.materialize()>"Modal"</Link>" and "
                            <Link href=routes::doc::popover::Atom.materialize()>"Popover"</Link>" atoms wrap their content in "
                            <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>
                            " to contain and restore focus."
                        </li>
                        <li>
                            "The merged props types of the "<Link href=routes::doc::Interactions.materialize()>"Interactions"</Link>
                            " domain (e.g. "<Code inline=true>"MergedPressHoverFocusRingProps"</Code>
                            ") combine interaction and focus hooks on one element."
                        </li>
                    </ul>
                </Section>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The simplest focus hook, "<Link href=routes::doc::focus::UseFocus.materialize()>"use_focus"</Link>
                    ", tracks whether an element is focused:"
                </p>

                <Demo
                    description="Focusable div showing whether it is focused, using use_focus"
                    source=include_str!("demos/focus.rs")
                    source_open=true
                >
                    <FocusDomainDemo/>
                </Demo>
            </Section>
        </DocPage>
    }
}
