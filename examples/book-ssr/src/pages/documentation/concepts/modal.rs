use leptonic::components::prelude::*;
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
                            <Link href=routes::doc::Alert.materialize()>"Alert"</Link>" / "
                            <Link href=routes::doc::Toast.materialize()>"Toast"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow><TableCell>"Show a side panel next to the content"</TableCell><TableCell><Link href=routes::doc::Drawer.materialize()>"Drawer"</Link></TableCell></TableRow>
                </DocTable>

                <p>
                    "Modals are disruptive by design. If the information is supplementary rather than essential, a popover "
                    "or toast is less intrusive."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Each layer builds on the one below: the atoms combine the hooks, and the "<Code inline=true>"Modal"</Code>
                    " component is built from the atoms. See "
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
                    <TableRow>
                        <TableCell><Link href=routes::doc::modal::Component.materialize()>"Modal Components"</Link></TableCell>
                        <TableCell>"The atoms with leptonic\u{2019}s theme: a styled modal with header, body and footer."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The component is the quickest way to a modal. It takes your open state as "
                    <Code inline=true>"is_open"</Code>" and its setter as "<Code inline=true>"set_open"</Code>
                    ", and dismissing it sets that state to "<Code inline=true>"false"</Code>". To style the modal yourself, "
                    "build it from the "<Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link>" instead."
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
                        "Focus moves into the modal when it opens: to its first focusable element, or to the dialog itself "
                        "when it has none. It stays inside while the modal is open and returns to where it was, typically "
                        "the trigger, when the modal closes."
                    </li>
                    <li>
                        <Code inline=true>"aria-modal=\"true\""</Code>" tells assistive technology to ignore the content "
                        "behind the modal, which is also made inert: it can\u{2019}t be clicked or focused. The page "
                        "doesn\u{2019}t scroll while the modal is open."
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
                        "it is dismissable: the "<Code inline=true>"Modal"</Code>" component is by default, the atoms and "
                        "hooks only when you opt in. With modals stacked, only the topmost one closes."
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
                <li><Link href=routes::doc::modal::Component.materialize()>"Modal Components"</Link></li>
                <li><Link href=routes::doc::Dialog.materialize()>"Dialog overview"</Link></li>
                <li><Link href=routes::doc::Popover.materialize()>"Popover overview"</Link></li>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
