use leptos::prelude::*;

use super::demos::modal::ModalConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageModalOverview() -> impl IntoView {
    view! {
        <DocPage title="Modal">
            <p>
                "Modals are overlays that demand the user\u{2019}s attention. While open, they keep focus inside, block "
                "interaction with the rest of the page and prevent scrolling. Use them for confirmations, critical "
                "decisions or short flows that need a response."
            </p>

            <p>
                "A modal is the overlay; the content inside it is a "<Link href=routes::doc::Dialog.materialize()>"dialog"</Link>
                ", which brings the role and the accessible name."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Block the page and require a decision"</TableCell><TableCell><b>"Modal"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Show contextual content anchored to an element"</TableCell><TableCell><Link href=routes::doc::Popover.materialize()>"Popover"</Link></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Name and focus the content of an overlay"</TableCell>
                        <TableCell><Link href=routes::doc::Dialog.materialize()>"Dialog"</Link>" (inside the modal or popover)"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Notify without blocking"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Toast.materialize()>"Toast"</Link>", or an "
                            <Link href=format!("{}#alert", routes::doc::Status.materialize())>"alert"</Link>" in the page"
                        </TableCell>
                    </TableRow>
                    <TableRow><TableCell>"Slide a panel over the page from its edge, such as a menu"</TableCell><TableCell><b>"Modal"</b>" as a "<Link href=format!("{}#drawer", routes::doc::modal::Atom.materialize())>"drawer"</Link></TableCell></TableRow>
                </DocTable>

                <p>
                    "Modals are disruptive by design. If the information is supplementary rather than essential, a popover "
                    "or toast is less intrusive."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "The atoms combine the hooks. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::modal::Hook.materialize()>"Modal Hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_modal_backdrop"</Code>" and "<Code inline=true>"use_modal"</Code>": dismissal, "
                            "scroll lock, an inert page and "<Code inline=true>"aria-modal"</Code>" for elements you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"ModalBackdrop"</Code>" and "<Code inline=true>"ModalContent"</Code>
                            " with the complete behavior \u{2014} portal, focus containment and restoration, dismissal, scroll "
                            "lock, entry and exit animations \u{2014} but no styles, around a "
                            <Link href=routes::doc::dialog::Atom.materialize()>"Dialog"</Link>
                            ". A "<Code inline=true>"DialogTrigger"</Code>" opens them from a button. For modals in your own design."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A confirmation built from the "<Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link>
                    " around a "<Link href=routes::doc::dialog::Atom.materialize()>"Dialog"</Link>". The backdrop takes your "
                    "open state as "<Code inline=true>"is_open"</Code>" and its setter as "<Code inline=true>"set_open"</Code>
                    ", and dismissing it sets that state to "<Code inline=true>"false"</Code>". The classes are the "
                    "book\u{2019}s own; the "<Link href=format!("{}#styling", routes::doc::modal::Atom.materialize())>"styling section"</Link>
                    " shows its CSS."
                </p>

                <Demo description="Confirmation modal opened by a button" source=include_str!("demos/modal.rs") source_open=true>
                    <ModalConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Modals follow the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/" target=LinkTarget::Blank>"Dialog (Modal) pattern"</Link>
                    ":"
                </p>

                <ul>
                    <li>
                        "Focus moves into the modal when it opens: to the dialog inside, which screen readers then announce "
                        "with its title (the atoms\u{2019} "<Code inline=true>"auto_focus"</Code>" moves it to the first "
                        "focusable element instead). It stays inside while the modal is open and returns to where it was, typically "
                        "the trigger, when the modal closes."
                    </li>
                    <li>
                        "The content behind the modal is inert: assistive technology ignores it, and it can\u{2019}t be "
                        "clicked or focused. The page doesn\u{2019}t scroll while the modal is open. A dismissable modal "
                        "starts with a visually hidden dismiss button for screen reader users who can\u{2019}t press "
                        <Keys keys="Escape"/>"."
                    </li>
                    <li>
                        "The "<Link href=routes::doc::Dialog.materialize()>"dialog"</Link>" inside has "
                        <Code inline=true>"role=\"dialog\""</Code>", or "<Code inline=true>"\"alertdialog\""</Code>
                        " for messages that require a response. "<Code inline=true>"aria-labelledby"</Code>
                        " points to its title; an alert dialog is also described by its message ("
                        <Code inline=true>"aria-describedby"</Code>")."
                    </li>
                    <li>
                        <Keys keys="Escape"/>" closes the modal unless you disable it. A click outside closes it only when "
                        "it is dismissable ("<Code inline=true>"is_dismissable"</Code>"). With modals stacked, only the topmost one "
                        "closes."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Escape">"Closes the modal, unless keyboard dismissal is disabled."</KeyRow>
                    <KeyRow keys="Tab">"Moves focus to the next element inside the modal, wrapping around."</KeyRow>
                    <KeyRow keys="Shift + Tab">"Moves focus to the previous element inside the modal, wrapping around."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::modal::Hook.materialize()>"Modal Hooks"</Link></li>
                <li><Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link></li>
                <li><Link href=routes::doc::Dialog.materialize()>"Dialog overview"</Link></li>
                <li><Link href=routes::doc::Popover.materialize()>"Popover overview"</Link></li>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
