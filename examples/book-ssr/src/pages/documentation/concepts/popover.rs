use leptos::prelude::*;

use super::demos::popover::PopoverConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PagePopoverOverview() -> impl IntoView {
    view! {
        <DocPage title="Popover">
            <p>
                "A popover is an overlay anchored to the element that opened it, its trigger. It shows content that "
                "belongs to the trigger: a short explanation, a small form, a preview. It is placed next to the trigger "
                "and moves to the other side when there is no room."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Show content, even interactive, next to the element it belongs to"</TableCell><TableCell><b>"Popover"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Block the page and require a decision"</TableCell><TableCell><Link href=routes::doc::Modal.materialize()>"Modal"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Show a short text on hover or focus"</TableCell><TableCell><Link href=routes::doc::Tooltip.materialize()>"Tooltip"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Offer a list of actions"</TableCell><TableCell><Link href=routes::doc::Menu.materialize()>"Menu"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Let the user pick one value from a list"</TableCell><TableCell><Link href=routes::doc::Select.materialize()>"Select"</Link></TableCell></TableRow>
                </DocTable>

                <p>
                    "A popover is modal or non-modal. A modal popover takes over the page until it closes: a press outside "
                    "closes it, focus stays inside and the page behind it is inert and doesn\u{2019}t scroll. A non-modal "
                    "popover leaves the page usable: presses outside reach the page, and moving focus out of the popover or "
                    "scrolling the page closes it. The atoms are modal by default, like menus and selects."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Popovers exist as a hook and as atoms. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></TableCell>
                        <TableCell>"Placement and dismissal for an overlay you render and wire to its trigger yourself."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::popover::Atom.materialize()>"Popover Atoms"</Link></TableCell>
                        <TableCell>
                            "An unstyled popover opened by a "<Code inline=true>"DialogTrigger"</Code>" around a button, an "
                            "arrow pointing at the trigger, and data attributes for styling and animation."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A "<Code inline=true>"DialogTrigger"</Code>" connects a button and a popover: the button opens and "
                    "closes it. The classes are the book\u{2019}s own; the "
                    <Link href=format!("{}#styling", routes::doc::popover::Atom.materialize())>"styling section"</Link>
                    " of the atoms shows how to style a popover."
                </p>

                <Demo description="Popover opened by a button through a DialogTrigger" source=include_str!("demos/popover.rs") source_open=true>
                    <PopoverConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A popover\u{2019}s content is a dialog, following the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/" target=LinkTarget::Blank>"Dialog pattern"</Link>
                    ":"
                </p>

                <ul>
                    <li>
                        "The content has "<Code inline=true>"role=\"dialog\""</Code>" (or "<Code inline=true>"\"alertdialog\""</Code>
                        "). Its title names it; without one, the trigger does."
                    </li>
                    <li>
                        "The trigger has "<Code inline=true>"aria-expanded"</Code>" and, while the popover is open, "
                        <Code inline=true>"aria-controls"</Code>" pointing to it."
                    </li>
                    <li>
                        "When the popover opens, focus moves into the dialog and stays there; when it closes, focus returns "
                        "to the trigger."
                    </li>
                    <li>
                        "A hidden dismiss button at the end of the popover (and at its start, if modal) lets screen reader "
                        "users close it without a keyboard."
                    </li>
                    <li>"A modal popover hides the rest of the page from assistive technology while it is open."</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Enter / Space">"On the trigger: opens or closes the popover."</KeyRow>
                    <KeyRow keys="Tab / Shift + Tab">"Moves focus within the popover."</KeyRow>
                    <KeyRow keys="Escape">"Closes the popover and returns focus to the trigger."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover Atoms"</Link></li>
                <li><Link href=routes::doc::Dialog.materialize()>"Dialog"</Link></li>
                <li><Link href=routes::doc::Tooltip.materialize()>"Tooltip"</Link></li>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
