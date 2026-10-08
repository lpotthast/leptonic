use leptos::prelude::*;

use super::demos::toast::ToastConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageToastOverview() -> impl IntoView {
    view! {
        <DocPage title="Toast">
            <p>
                "A toast is a short notification that appears above the app, for example to confirm that something was "
                "saved, and usually closes by itself after a few seconds. It doesn\u{2019}t interrupt the user: the focus "
                "stays where it is, and screen readers announce the message when the toast appears."
            </p>
            <p>
                "Toasts are added to a queue from anywhere in the app and shown by a toast region, at the edge of the "
                "page. The region shows the newest toasts, up to a maximum; the others wait until one closes. Timeouts "
                "pause while the user hovers the toasts or has the focus in them, so nobody loses a message while "
                "reading it."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Confirm briefly that something happened, without interrupting the user"</TableCell>
                        <TableCell><b>"Toast"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a lasting message in the page, such as a warning above a form"</TableCell>
                        <TableCell>"An "<Link href=format!("{}#alert", routes::doc::Status.materialize())>"alert"</Link>" in the page"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Require a decision before the user continues"</TableCell>
                        <TableCell><Link href=routes::doc::Modal.materialize()>"Modal"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Only announce something to screen reader users"</TableCell>
                        <TableCell><Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live_announcer"</Link></TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "Don\u{2019}t put the only way to an action or a piece of information into a toast that closes by "
                    "itself: give toasts with actions no timeout, and offer the action elsewhere too."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::toast::Hook.materialize()>"Toast Hooks"</Link></TableCell>
                        <TableCell>
                            "The queue of toasts with pausable timeouts ("<Code inline=true>"ToastQueue"</Code>"), and the "
                            "attributes and focus handling of the region and of each toast, for elements you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::toast::Atom.materialize()>"Toast Atoms"</Link></TableCell>
                        <TableCell>
                            "An unstyled "<Code inline=true>"ToastRegion"</Code>" rendering the toasts of a queue at the end of "
                            "the page, and the parts of a toast: content, title, description and close button. Your queue "
                            "can hold any content type."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Add toasts to a "<Code inline=true>"ToastQueue"</Code>" and render them with a "
                    <Code inline=true>"ToastRegion"</Code>" from the atoms. An app creates one queue near its root and "
                    "provides it as context, so that any code can add toasts. The classes are the book\u{2019}s own; the "
                    <Link href=format!("{}#styling", routes::doc::toast::Atom.materialize())>"styling section"</Link>
                    " of the atoms shows their CSS."
                </p>
                <Demo
                    description="Save button showing a success toast"
                    source=include_str!("demos/toast.rs")
                    source_open=true
                >
                    <ToastConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "The toast region is a "<Link href=routes::doc::focus::UseLandmark.materialize()>"landmark"</Link>
                        " named by the number of toasts (\u{201c}2 notifications.\u{201d}). Keyboard users reach it with "
                        <Keys keys="F6"/>" and leave it with "<Keys keys="Shift + F6"/>"; screen reader users find it in "
                        "their reader\u{2019}s list of landmarks."
                    </li>
                    <li>
                        "Each toast is a non-modal "<Code inline=true>"alertdialog"</Code>", named by its title and described "
                        "by its description. Its content is an "<Code inline=true>"alert"</Code>": screen readers announce it "
                        "when the toast appears, without moving the focus."
                    </li>
                    <li>"Each toast has a close button named \u{201c}Close\u{201d}."</li>
                    <li>
                        "When a toast closes while it has the focus, the focus moves to the next toast. When the last toast "
                        "closes, or a toast is closed with the pointer, the focus returns to where it was before it entered "
                        "the region."
                    </li>
                    <li>
                        "While the pointer is over the region or the focus is in it, the timeouts of the toasts pause."
                    </li>
                    <li>"The region\u{2019}s name and the close button\u{2019}s name follow the locale."</li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="F6 / Shift + F6">"Moves the focus into the toast region, and on to the next or previous landmark."</KeyRow>
                    <KeyRow keys="Tab / Shift + Tab">"Moves the focus between the toasts and their close buttons."</KeyRow>
                    <KeyRow keys="Enter / Space">"On a close button: closes its toast."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::toast::Hook.materialize()>"Toast Hooks"</Link></li>
                <li><Link href=routes::doc::toast::Atom.materialize()>"Toast Atoms"</Link></li>
                <li><Link href=format!("{}#alert", routes::doc::Status.materialize())>"Alert"</Link></li>
                <li><Link href=routes::doc::focus::UseLandmark.materialize()>"use_landmark"</Link></li>
                <li><Link href=routes::doc::Status.materialize()>"Status"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
