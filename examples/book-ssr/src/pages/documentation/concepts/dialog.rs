use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::dialog::DialogConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageDialogOverview() -> impl IntoView {
    view! {
        <DocPage title="Dialog">
            <p>
                "A dialog is the content of an overlay that asks the user to read something or respond: a confirmation, a "
                "form, a message. It is named by its title, so screen readers announce what it is about, and it receives "
                "focus when it opens. An alert dialog is a dialog for an urgent message that needs a response; it is also "
                "described by its message."
            </p>

            <p>
                "A dialog always sits in an overlay: a "<Link href=routes::doc::Modal.materialize()>"modal"</Link>
                " blocks the page around it, a "<Link href=routes::doc::Popover.materialize()>"popover"</Link>
                " is anchored to the element that opened it. The overlay handles opening, dismissing and keeping focus; "
                "the dialog handles its semantics."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Give the content of a modal or popover a role and a name"</TableCell>
                        <TableCell><b>"Dialog"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Ask for a response to an urgent message"</TableCell>
                        <TableCell><b>"Dialog"</b>" with "<Code inline=true>"DialogRole::AlertDialog"</Code></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Block the page while the dialog is open"</TableCell>
                        <TableCell><Link href=routes::doc::Modal.materialize()>"Modal"</Link>" around the dialog"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show the dialog next to its trigger"</TableCell>
                        <TableCell><Link href=routes::doc::Popover.materialize()>"Popover"</Link>" around the dialog"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a list of actions"</TableCell>
                        <TableCell><Link href=routes::doc::Menu.materialize()>"Menu"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate. There is no styled dialog of its own: the "
                    <Link href=routes::doc::modal::Component.materialize()>"Modal Components"</Link>" and the "
                    <Link href=routes::doc::popover::Component.materialize()>"Popover Component"</Link>" contain one."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::dialog::Hook.materialize()>"use_dialog"</Link></TableCell>
                        <TableCell>"The role, the labelling and the focus on mount, for an element you render."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::dialog::Atom.materialize()>"Dialog Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"Dialog"</Code>", "<Code inline=true>"DialogTitle"</Code>" and "
                            <Code inline=true>"DialogDescription"</Code>", and "<Code inline=true>"DialogTrigger"</Code>
                            ", which opens the overlay around the dialog from a button."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A dialog in a modal, opened by a button. The "<Code inline=true>"DialogTrigger"</Code>
                    " connects the button and the modal; its open state is yours, so the dialog\u{2019}s buttons can close it."
                </p>

                <Demo
                    description="Dialog in a modal, opened by a button through a DialogTrigger"
                    source=include_str!("demos/dialog.rs")
                    source_open=true
                >
                    <DialogConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A dialog follows the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/" target=LinkTarget::Blank>"dialog pattern"</Link>
                    " together with the overlay around it:"
                </p>

                <ul>
                    <li>
                        <Code inline=true>"role=\"dialog\""</Code>", or "<Code inline=true>"\"alertdialog\""</Code>
                        " for messages that require a response."
                    </li>
                    <li>
                        <Code inline=true>"aria-labelledby"</Code>" points to the title, or "<Code inline=true>"aria-label"</Code>
                        " names a dialog without one. A dialog opened by a "<Code inline=true>"DialogTrigger"</Code>
                        " without either is named by the trigger. An alert dialog is also described by its message ("
                        <Code inline=true>"aria-describedby"</Code>")."
                    </li>
                    <li>
                        "The dialog is focused when it opens, unless focus already moved into it. The overlay keeps focus "
                        "inside and returns it to the trigger when the dialog closes; a dialog in a non-modal popover makes "
                        "the popover keep focus inside."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Tab / Shift + Tab">"Moves focus between the elements of the dialog, wrapping around."</KeyRow>
                    <KeyRow keys="Escape">"Closes the overlay around the dialog, unless it disables keyboard dismissal."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::dialog::Hook.materialize()>"use_dialog"</Link></li>
                <li><Link href=routes::doc::dialog::Atom.materialize()>"Dialog Atoms"</Link></li>
                <li><Link href=routes::doc::Modal.materialize()>"Modal overview"</Link></li>
                <li><Link href=routes::doc::Popover.materialize()>"Popover overview"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
