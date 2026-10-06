use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::modal::ModalConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageModalOverview() -> impl IntoView {
    view! {
        <DocPage title="Modal">
            <p>
                "Modals are dialogs that demand the user\u{2019}s attention. While open, they keep focus inside, block "
                "interaction with the rest of the page and prevent scrolling. Use them for confirmations, critical "
                "decisions or short flows that need a response."
            </p>

            <p>
                "A modal combines three hooks: "<Code inline=true>"use_dialog"</Code>" (ARIA semantics and focus), "
                <Code inline=true>"use_modal"</Code>" (marks it as modal) and "<Code inline=true>"use_modal_backdrop"</Code>
                " (dismissal and scroll prevention)."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Block the page and require a decision"</TableCell><TableCell><b>"Modal"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Show contextual content anchored to an element"</TableCell><TableCell><Link href=routes::doc::Popover.materialize()>"Popover"</Link></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Notify without blocking"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::components::Alert.materialize()>"Alert"</Link>" / "
                            <Link href=routes::doc::components::Toast.materialize()>"Toast"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow><TableCell>"Show a sliding panel from the edge"</TableCell><TableCell><Link href=routes::doc::components::Drawer.materialize()>"Drawer"</Link></TableCell></TableRow>
                </DocTable>

                <p>
                    "Modals are disruptive by design. If the information is supplementary rather than essential, a popover "
                    "or toast is less intrusive."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Modals exist as hooks, atoms and a component. Each layer builds on the one below: the atoms combine the "
                    "hooks, and the "<Code inline=true>"Modal"</Code>" component is built from the atoms. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::modal::Hook.materialize()>"Modal & dialog hooks"</Link></TableCell>
                        <TableCell>"State, dismissal, "<Code inline=true>"aria-modal"</Code>" and dialog semantics for elements you render yourself."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::modal::Atom.materialize()>"Modal & dialog atoms"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"ModalBackdrop"</Code>", "<Code inline=true>"ModalContent"</Code>" and "
                            <Code inline=true>"Dialog"</Code>" with the complete behavior \u{2014} portal, focus trap and "
                            "restore, dismissal, scroll lock, labelling \u{2014} but no styles. For modals in your own design."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::modal::Component.materialize()>"Modal component"</Link></TableCell>
                        <TableCell>"The atoms with leptonic\u{2019}s theme: a styled modal with header, body and footer."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The component is the quickest way to a modal. It is bound to your open state ("
                    <Code inline=true>"state=(read, write)"</Code>" or an "<Code inline=true>"RwSignal<bool>"</Code>
                    "), and dismissing it sets that state to "<Code inline=true>"false"</Code>". To style the modal yourself, "
                    "build it from the "<Link href=routes::doc::modal::Atom.materialize()>"atoms"</Link>" instead; they "
                    "bind the same open state, or a "<Code inline=true>"DialogTrigger"</Code>" around a button and the "
                    "modal owns it for you."
                </p>

                <Demo description="Modal dialog opened by a button" source=include_str!("demos/modal.rs") source_open=true>
                    <ModalConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Modals follow the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/" target=LinkTarget::_Blank>"Dialog (Modal) pattern"</LinkExt>
                    ". Focus moves into the modal when it opens, stays inside while it is open and returns to where it was "
                    "when it closes."
                </p>

                <ul>
                    <li>
                        <Code inline=true>"role=\"dialog\""</Code>", or "<Code inline=true>"\"alertdialog\""</Code>
                        " for messages that require a response."
                    </li>
                    <li>
                        <Code inline=true>"aria-modal=\"true\""</Code>" tells assistive technology to ignore the content "
                        "behind the modal, which is also made inert."
                    </li>
                    <li>
                        <Code inline=true>"aria-labelledby"</Code>" points to the title, which names the dialog as soon as "
                        "you render it (the component\u{2019}s "<Code inline=true>"ModalTitle"</Code>"). An alert dialog is "
                        "also described by its message ("<Code inline=true>"aria-describedby"</Code>")."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Escape">"Closes the modal."</KeyRow>
                    <KeyRow keys="Tab">"Moves focus to the next element inside the modal."</KeyRow>
                    <KeyRow keys="Shift + Tab">"Moves focus to the previous element inside the modal."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
