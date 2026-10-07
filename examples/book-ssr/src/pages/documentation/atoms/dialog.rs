use indoc::indoc;
use leptos::prelude::*;

use super::demos::{dialog_popover::DialogPopoverDemo, modal_alert::ModalAlertDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomDialog() -> impl IntoView {
    view! {
        <DocPage title="Dialog Atoms">
            <p>
                <Code inline=true>"Dialog"</Code>", "<Code inline=true>"DialogTitle"</Code>" and "
                <Code inline=true>"DialogDescription"</Code>" render the named content of an overlay, unstyled; "
                <Code inline=true>"DialogTrigger"</Code>" opens the overlay from a button. See the "
                <Link href=routes::doc::Dialog.materialize()>"Dialog overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <ul>
                    <li>
                        <AnchorLink href="#dialog"><Code inline=true>"Dialog"</Code></AnchorLink>" calls "
                        <Link href=routes::doc::dialog::Hook.materialize()>"use_dialog"</Link>
                        " for the role, the labelling and the focus on mount."
                    </li>
                    <li>
                        <AnchorLink href="#dialogtrigger"><Code inline=true>"DialogTrigger"</Code></AnchorLink>" calls "
                        <Link href=routes::doc::overlay_behavior::UseOverlayTriggerState.materialize()>"use_overlay_trigger_state"</Link>
                        " and passes the press handling and the trigger\u{2019}s ARIA attributes to its button through a "
                        <Link href=routes::doc::interactions::PressResponder.materialize()>"PressResponder"</Link>"."
                    </li>
                </ul>
            </Section>

            <Section title="Example">
                <p>
                    "A dialog in a "<Link href=routes::doc::modal::Atom.materialize()>"ModalContent"</Link>", opened by a "
                    "button. Inside a "<Code inline=true>"DialogTrigger"</Code>", the "<Code inline=true>"ModalBackdrop"</Code>
                    " takes its open state from the trigger:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::{
                            button::Button,
                            dialog::{Dialog, DialogTitle, DialogTrigger},
                            modal::{ModalBackdrop, ModalContent},
                        };

                        view! {
                            <DialogTrigger>
                                <Button>"Show terms"</Button>
                                <ModalBackdrop is_dismissable=true>
                                    <ModalContent>
                                        // Named by its title.
                                        <Dialog>
                                            <DialogTitle>"Terms of use"</DialogTitle>
                                            <p>"Please read the terms."</p>
                                        </Dialog>
                                    </ModalContent>
                                </ModalBackdrop>
                            </DialogTrigger>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A confirmation before a destructive action, in a "
                    <Link href=routes::doc::modal::Atom.materialize()>"modal"</Link>" whose open state is a signal of the "
                    "app. The dialog has the "<Code inline=true>"alertdialog"</Code>" role and its backdrop isn\u{2019}t "
                    "dismissable: a click on it does nothing, but "<Keys keys="Escape"/>" cancels. Focus starts on "
                    "\u{201c}Cancel\u{201d}, the first button, so pressing "<Keys keys="Enter"/>" right away is harmless."
                </p>
                <Demo
                    description="Alert dialog confirming a deletion, built from the modal and dialog atoms"
                    source=include_str!("demos/modal_alert.rs")
                >
                    <ModalAlertDemo/>
                </Demo>

                <Section title="In a Popover">
                    <p>
                        "A dialog in a "<Link href=routes::doc::popover::Atom.materialize()>"Popover"</Link>
                        ", opened by its "<Code inline=true>"DialogTrigger"</Code>". The trigger owns the open state, so the "
                        "demo needs no signal for it. The popover stays next to the button, closes on "<Keys keys="Escape"/>
                        " or a click outside, and focus returns to the button."
                    </p>
                    <Demo
                        description="Dialog with two checkboxes in a popover, opened by a button through a DialogTrigger"
                        source=include_str!("demos/dialog_popover.rs")
                    >
                        <DialogPopoverDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="DialogTrigger">
                <p>
                    "Opens and closes the overlay inside it (a "<Link href=routes::doc::popover::Atom.materialize()>"Popover"</Link>
                    " or a "<Link href=routes::doc::modal::Atom.materialize()>"ModalBackdrop"</Link>") when its pressable child (a "
                    <Code inline=true>"Button"</Code>") is pressed. The overlay finds the state and the trigger element through "
                    "context. The button gets "<Code inline=true>"aria-expanded"</Code>" and, while the overlay is open, "
                    <Code inline=true>"aria-controls"</Code>"; a "<Code inline=true>"Dialog"</Code>" without a title is "
                    "named by it. The open state is the trigger\u{2019}s own, or yours with "<Code inline=true>"is_open"</Code>
                    " and "<Code inline=true>"set_open"</Code>", so that buttons inside the dialog can close it."
                </p>

                <Section title="Props" id="dialog-trigger-props">
                    <ApiTable kind=ApiKind::Props of="DialogTrigger">
                        <ApiRow name="default_open" ty="bool" default="false">
                            "Whether the overlay starts open. Ignored with "<Code inline=true>"is_open"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">
                            "Called when the overlay opens or closes."
                        </ApiRow>
                        <ApiRow name="is_open" ty="Option<Signal<bool>>" default="None">
                            "Whether the overlay is open (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_open" ty="Option<Out<bool>>" default="None">
                            "Receives the open state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The trigger button and the overlay. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Dialog">
                <p>
                    "A "<Code inline=true>"<section>"</Code>" with the "<Code inline=true>"dialog"</Code>" or "
                    <Code inline=true>"alertdialog"</Code>" role and "<Code inline=true>"tabindex=\"-1\""</Code>
                    ". When it mounts and focus is not already inside it, it focuses itself, so screen readers announce it. "
                    "A rendered "<Code inline=true>"DialogTitle"</Code>" names it ("<Code inline=true>"aria-labelledby"</Code>
                    "); an alert dialog is also described by its "<Code inline=true>"DialogDescription"</Code>" ("
                    <Code inline=true>"aria-describedby"</Code>"). In a non-modal "<Code inline=true>"Popover"</Code>
                    ", the dialog makes the popover keep focus inside."
                </p>
                <p>
                    "Every dialog needs an accessible name; in debug builds, a dialog without one logs a warning. A dialog "
                    "without a visible title gets "<Code inline=true>"aria_label"</Code>", or is named by the button of its "
                    <Code inline=true>"DialogTrigger"</Code>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        <Dialog aria_label="Image preview">
                            <img src="/photo.jpg" alt="Sunset over the sea"/>
                        </Dialog>
                    "#)}
                </Code>
                <Section title="Props" id="dialog-props">
                    <ApiTable kind=ApiKind::Props of="Dialog">
                        <ApiRow name="role" ty="DialogRole" default="Dialog">
                            <Code inline=true>"DialogRole::AlertDialog"</Code>" for messages that need a response, such as a "
                            "confirmation."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names a dialog without a "<Code inline=true>"DialogTitle"</Code>"; a rendered "
                            <Code inline=true>"DialogTitle"</Code>" names it automatically."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">
                            "Ids of the elements naming the dialog, instead of its title."
                        </ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">
                            "Ids of the elements describing the dialog. Default for alert dialogs: its "
                            <Code inline=true>"DialogDescription"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the dialog "<Code inline=true>"<section>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">
                            "The content, with a "<Code inline=true>"DialogTitle"</Code>" and optionally a "
                            <Code inline=true>"DialogDescription"</Code>". Required."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="DialogTitle">
                <p>
                    "The dialog\u{2019}s title: a heading ("<Code inline=true>"<h2>"</Code>" unless you set "
                    <Code inline=true>"level"</Code>") that names the enclosing "<Code inline=true>"Dialog"</Code>
                    ". While it is rendered, the "<Code inline=true>"Dialog"</Code>"\u{2019}s "
                    <Code inline=true>"aria-labelledby"</Code>" points to it, unless the dialog has an "
                    <Code inline=true>"aria_label"</Code>". Choose the level that fits the document outline at the place "
                    "the dialog opens."
                </p>
                <Section title="Props" id="dialog-title-props">
                    <ApiTable kind=ApiKind::Props of="DialogTitle">
                        <ApiRow name="level" ty="HeadingLevel" default="H2">"The heading element: "<Code inline=true>"<h1>"</Code>" to "<Code inline=true>"<h6>"</Code>"."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the heading."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The title. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="DialogDescription">
                <p>
                    "The dialog\u{2019}s message, as a "<Code inline=true>"<div>"</Code>". An alert dialog is described by "
                    "it: while it is rendered, the "<Code inline=true>"Dialog"</Code>"\u{2019}s "
                    <Code inline=true>"aria-describedby"</Code>" points to it, so screen readers read the message when the "
                    "dialog opens. A regular dialog isn\u{2019}t described by it automatically; set "
                    <Code inline=true>"aria_describedby"</Code>" on the "<Code inline=true>"Dialog"</Code>" for that."
                </p>
                <Section title="Props" id="dialog-description-props">
                    <ApiTable kind=ApiKind::Props of="DialogDescription">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the "<Code inline=true>"<div>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The description. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>
                    "The atoms render no "<Code inline=true>"data-*"</Code>" attributes. Target the dialog\u{2019}s role instead: "
                    <Code inline=true>"role=\"dialog\""</Code>" or "<Code inline=true>"role=\"alertdialog\""</Code>
                    ", from its "<Code inline=true>"role"</Code>" prop."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. They render the classes "<Code inline=true>"leptonic-Dialog"</Code>", "
                    <Code inline=true>"leptonic-DialogTitle"</Code>" and "<Code inline=true>"leptonic-DialogDescription"</Code>
                    ", each followed by the "<Code inline=true>"classes"</Code>" you pass. Give the dialog its frame, unless "
                    "the overlay around it draws one (a popover usually does). The dialog receives focus when no element "
                    "inside it does: show a ring when that happens through the keyboard. The book\u{2019}s demos use these "
                    "rules:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-dialog { padding: 1.5em; border: 1px solid var(--border); border-radius: 12px; background: var(--surface); }
                        .my-dialog:focus-visible { outline: 2px solid var(--focus); outline-offset: 2px; }
                        .my-dialog[role="alertdialog"] { border-top: 4px solid var(--accent); }
                        .my-dialog-title { margin: 0 0 0.5em; font-size: 1.25em; }
                        .my-dialog-description { margin: 0 0 1.5em; color: var(--muted); }
                    "#)}
                </Code>
                <p>
                    "Leptonic also ships an optional atom theme that styles the default classes, for apps that don\u{2019}t "
                    "want to start from scratch: "<Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>"."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    <Code inline=true>"DialogTitle"</Code>" and "<Code inline=true>"DialogDescription"</Code>" connect to the "
                    "enclosing "<Code inline=true>"Dialog"</Code>" through context, wherever they are inside it. A title of "
                    "your own, as a Leptos component, wraps "<Code inline=true>"DialogTitle"</Code>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::dialog::DialogTitle, utils::heading_level::HeadingLevel};

                        #[component]
                        fn MyDialogHeading(children: Children) -> impl IntoView {
                            view! {
                                <DialogTitle level=HeadingLevel::H3 classes="my-heading">
                                    {children()}
                                </DialogTitle>
                            }
                        }
                    "#)}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Dialog.materialize()>"Dialog overview"</Link></li>
                <li><Link href=routes::doc::dialog::Hook.materialize()>"use_dialog"</Link></li>
                <li><Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
