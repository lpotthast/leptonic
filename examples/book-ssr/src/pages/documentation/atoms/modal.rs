use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{modal_alert::ModalAlertDemo, modal_form::ModalFormDemo};
use crate::{kit::*, routes};

/// A section of the modal hooks page.
fn hook_section(id: &str) -> String {
    format!("{}#{id}", routes::doc::modal::Hook.materialize())
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomModal() -> impl IntoView {
    view! {
        <DocPage title="Modal and Dialog Atoms">
            <p>
                "The modal atoms render an unstyled modal with the behavior of the "
                <Link href=routes::doc::modal::Hook.materialize()>"modal and dialog hooks"</Link>". "
                <Code inline=true>"ModalBackdrop"</Code>" and "<Code inline=true>"ModalContent"</Code>
                " make up the modal: the blocking overlay that covers the page, traps focus and closes on Escape. "
                <Code inline=true>"Dialog"</Code>", "<Code inline=true>"DialogTitle"</Code>" and "
                <Code inline=true>"DialogDescription"</Code>" make up the dialog: the named content inside it. See the "
                <Link href=routes::doc::Modal.materialize()>"Modal overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <ul>
                    <li>
                        <Code inline=true>"ModalBackdrop"</Code>" calls "
                        <Link href=hook_section("use-modal-backdrop")>"use_modal_backdrop"</Link>", which combines "
                        <Link href=routes::doc::overlays::UseOverlay.materialize()>"use_overlay"</Link>
                        " (Escape, outside clicks, overlay stacking) and "
                        <Link href=routes::doc::interactions::UsePreventScroll.materialize()>"use_prevent_scroll"</Link>
                        ", and makes the rest of the page inert."
                    </li>
                    <li>
                        <Code inline=true>"ModalContent"</Code>" calls "<Link href=hook_section("use-modal")>"use_modal"</Link>
                        " and wraps its children in a "<Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>
                        " that contains, restores and auto-focuses focus."
                    </li>
                    <li>
                        <Code inline=true>"Dialog"</Code>" calls "<Link href=hook_section("use-dialog")>"use_dialog"</Link>
                        " for the role, the labelling and the focus on mount."
                    </li>
                </ul>
                <p>
                    "The open state comes from a surrounding "<Code inline=true>"DialogTrigger"</Code>" (see "
                    <a href="#opened-by-a-button">"Opened by a Button"</a>"), or "<Code inline=true>"ModalBackdrop"</Code>
                    " binds to your app\u{2019}s state ("<Code inline=true>"state=rw_signal"</Code>" or "
                    <Code inline=true>"state=(read, write)"</Code>"), or takes an "
                    <Link href=hook_section("use-overlay-trigger-state")>"use_overlay_trigger_state"</Link>
                    " shared with other parts of your UI."
                </p>
            </Section>

            <Section title="Example">
                <p>
                    <Code inline=true>"ModalBackdrop"</Code>" contains a "<Code inline=true>"ModalContent"</Code>
                    ", which contains the "<Code inline=true>"Dialog"</Code>". The backdrop is bound to your open state: "
                    "dismissing the modal ("<Keys keys="Escape"/>", a click outside) sets it to "<Code inline=true>"false"</Code>
                    ", and your own buttons set it directly:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::{
                            button::Button,
                            dialog::{Dialog, DialogTitle},
                            modal::{ModalBackdrop, ModalContent},
                        };

                        let is_open = RwSignal::new(false);

                        view! {
                            <Button on_press=move |_| is_open.set(true)>"Show terms"</Button>

                            <ModalBackdrop state=is_open is_dismissable=true>
                                <ModalContent>
                                    // Named by its title.
                                    <Dialog>
                                        <DialogTitle>"Terms of use"</DialogTitle>
                                        <p>"Please read the terms."</p>
                                        <Button on_press=move |_| is_open.set(false)>"Close"</Button>
                                    </Dialog>
                                </ModalContent>
                            </ModalBackdrop>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Opened by a Button">
                <p>
                    "When a button opens the modal, wrap both in a "<Code inline=true>"DialogTrigger"</Code>
                    ". It owns the open state, so "<Code inline=true>"ModalBackdrop"</Code>" needs no "
                    <Code inline=true>"state"</Code>"; the button gets "<Code inline=true>"aria-expanded"</Code>
                    " and "<Code inline=true>"aria-controls"</Code>", and focus returns to it when the modal closes. A "
                    <Code inline=true>"Dialog"</Code>" without a title is named by the button."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        <DialogTrigger>
                            <Button>"Show terms"</Button>
                            <ModalBackdrop is_dismissable=true>
                                <ModalContent>
                                    <Dialog>
                                        <DialogTitle>"Terms of use"</DialogTitle>
                                        <p>"Please read the terms."</p>
                                    </Dialog>
                                </ModalContent>
                            </ModalBackdrop>
                        </DialogTrigger>
                    "#)}
                </Code>
                <p>
                    "Its props: "<Code inline=true>"default_open"</Code>", "<Code inline=true>"on_open_change"</Code>
                    " and "<Code inline=true>"state"</Code>" (app state replacing both). See the "
                    <Link href=format!("{}#dialogtrigger", routes::doc::popover::Atom.materialize())>"DialogTrigger"</Link>
                    " reference on the Popover atom page."
                </p>
            </Section>

            <Section title="Demos">
                <Section title="Alert Dialog">
                    <p>
                        "A confirmation before a destructive action. The dialog has the "<Code inline=true>"alertdialog"</Code>
                        " role and is not dismissable: a click on the backdrop does nothing, but Escape cancels. Focus starts on "
                        "\u{201c}Cancel\u{201d}, the first button, so pressing "<Keys keys="Enter"/>" right away is harmless."
                    </p>
                    <Demo
                        description="Alert dialog confirming a deletion, built from the modal and dialog atoms"
                        source=include_str!("demos/modal_alert.rs")
                    >
                        <ModalAlertDemo/>
                    </Demo>
                </Section>

                <Section title="Form Dialog">
                    <p>
                        "A dismissable dialog with a form. The log shows how focus moves: into the input when the dialog "
                        "opens, around the dialog\u{2019}s elements with "<Keys keys="Tab"/>", and back to the trigger when it "
                        "closes \u{2014} by saving, by \u{201c}Cancel\u{201d}, by "<Keys keys="Escape"/>" or by a click on the backdrop."
                    </p>
                    <Demo
                        description="Dismissable dialog with a text input, logging how focus moves in and out"
                        source=include_str!("demos/modal_form.rs")
                    >
                        <ModalFormDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="ModalBackdrop">
                <p>
                    "Renders the modal into the document body while its "<Code inline=true>"state"</Code>" is open, as a "<Code inline=true>"<div>"</Code>" with the class "
                    <Code inline=true>"leptonic-modal-backdrop"</Code>" around its children. While open, it prevents page "
                    "scrolling and makes everything outside the "<Code inline=true>"ModalContent"</Code>" inert. It is "
                    "unmounted when it closes."
                </p>
                <Section title="Props" id="modal-backdrop-props">
                    <ApiTable kind=ApiKind::Props of="ModalBackdrop">
                        <ApiRow name="state" ty="Option<OverlayTriggerState>" default="the DialogTrigger\u{2019}s">
                            "Whether the modal is shown: app state ("<Code inline=true>"state=rw_signal"</Code>", "<Code inline=true>"state=(read, write)"</Code>
                            ") or a shared "<Code inline=true>"OverlayTriggerState"</Code>"; needed only without a surrounding "
                            <Code inline=true>"DialogTrigger"</Code>". "<Keys keys="Escape"/>" and, if dismissable, a "
                            "click outside the "<Code inline=true>"ModalContent"</Code>" close it."
                        </ApiRow>
                        <ApiRow name="is_dismissable" ty="bool" default="false">
                            "Whether a click outside the "<Code inline=true>"ModalContent"</Code>", e.g. on the backdrop, closes "
                            "the modal. It doesn\u{2019}t affect "<Keys keys="Escape"/>"."
                        </ApiRow>
                        <ApiRow name="is_keyboard_dismiss_disabled" ty="bool" default="false">
                            "Whether "<Keys keys="Escape"/>" no longer closes the modal."
                        </ApiRow>
                        <ApiRow name="should_close_on_interact_outside" ty="Option<Callback<web_sys::Element, bool>>" default="None">
                            "Decides per outside click whether the modal closes: called with the clicked element, "
                            <Code inline=true>"true"</Code>" closes. Only consulted when "<Code inline=true>"is_dismissable"</Code>
                            " is set."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the backdrop "<Code inline=true>"<div>"</Code>", added to "
                            <Code inline=true>"leptonic-modal-backdrop"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="ChildrenFn">
                            "A "<Code inline=true>"ModalContent"</Code>". Rendered anew each time the modal opens."
                        </ApiRow>
                    </ApiTable>
                    <p>
                        "See "<Link href=hook_section("use-modal-backdrop-input")>"use_modal_backdrop"</Link>
                        " for how these settings behave. When closing needs more than setting the state to "
                        <Code inline=true>"false"</Code>" (saving a draft, opening the next step), bind "
                        <Code inline=true>"ValueBinding::new(signal, callback)"</Code>": the callback runs with the new open "
                        "state, also when the user dismisses the modal. An "<Code inline=true>"OverlayTriggerState"</Code>
                        " with "<Code inline=true>"on_open_change"</Code>" works as well."
                    </p>
                </Section>
            </Section>

            <Section title="ModalContent">
                <p>
                    "The modal panel: a "<Code inline=true>"<div>"</Code>" with "<Code inline=true>"aria-modal=\"true\""</Code>
                    " inside a "<Code inline=true>"FocusScope"</Code>". It must be a child of "
                    <Code inline=true>"ModalBackdrop"</Code>": the backdrop\u{2019}s Escape handling and outside-click "
                    "detection are attached to this element, so everything outside it counts as outside the modal."
                </p>
                <Section title="Props" id="modal-content-props">
                    <ApiTable kind=ApiKind::Props of="ModalContent">
                        <ApiRow name="contain_focus" ty="bool" default="true">
                            <Keys keys="Tab"/>" and "<Keys keys="Shift + Tab"/>" wrap around inside the modal, and focus that "
                            "leaves it is moved back."
                        </ApiRow>
                        <ApiRow name="restore_focus" ty="bool" default="true">
                            "When the modal closes, focus returns to the element that had it when the modal opened, "
                            "typically the trigger."
                        </ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="true">
                            "When the modal opens, the first focusable element inside it is focused."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the panel "<Code inline=true>"<div>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The modal\u{2019}s content, typically a "<Code inline=true>"Dialog"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Dialog">
                <p>
                    "A "<Code inline=true>"<section>"</Code>" with the "<Code inline=true>"dialog"</Code>" or "
                    <Code inline=true>"alertdialog"</Code>" role and "<Code inline=true>"tabindex=\"-1\""</Code>
                    ". When it mounts and focus is not already inside it, it focuses itself, so screen readers announce it. "
                    "A rendered "<Code inline=true>"DialogTitle"</Code>" names it ("<Code inline=true>"aria-labelledby"</Code>
                    "); without one, set "<Code inline=true>"aria_label"</Code>". An alert dialog is also described by its "
                    <Code inline=true>"DialogDescription"</Code>" ("<Code inline=true>"aria-describedby"</Code>")."
                </p>
                <Section title="Props" id="dialog-props">
                    <ApiTable kind=ApiKind::Props of="Dialog">
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names a dialog without a "<Code inline=true>"DialogTitle"</Code>"; a rendered "<Code inline=true>"DialogTitle"</Code>
                            " names it automatically."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">
                            "Ids of the elements naming the dialog, instead of its title."
                        </ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">
                            "Ids of the elements describing the dialog. Default for alert dialogs: its "
                            <Code inline=true>"DialogDescription"</Code>"."
                        </ApiRow>
                        <ApiRow name="role" ty="DialogRole" default="Dialog">
                            <Code inline=true>"DialogRole::AlertDialog"</Code>" for messages that need a response, such as a "
                            "confirmation."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the dialog "<Code inline=true>"<section>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">
                            "The content, with a "<Code inline=true>"DialogTitle"</Code>" and optionally a "
                            <Code inline=true>"DialogDescription"</Code>"."
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
                        <ApiRow name="children" ty="Children">"The title."</ApiRow>
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
                        <ApiRow name="children" ty="Children">"The description."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>
                    "The atoms render no "<Code inline=true>"data-*"</Code>" attributes. The modal exists only while it is "
                    "open, so there is no open or closed state to style. Target the ARIA attributes instead:"
                </p>
                <DocTable headers=&["Attribute", "Element", "Description"]>
                    <TableRow>
                        <TableCell><Code inline=true>"role=\"dialog\""</Code>", "<Code inline=true>"role=\"alertdialog\""</Code></TableCell>
                        <TableCell><Code inline=true>"Dialog"</Code></TableCell>
                        <TableCell>"The dialog\u{2019}s role, from its "<Code inline=true>"role"</Code>" prop."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"aria-modal=\"true\""</Code></TableCell>
                        <TableCell><Code inline=true>"ModalContent"</Code></TableCell>
                        <TableCell>"Always set on the panel."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles of their own. Pass "<Code inline=true>"classes"</Code>" to each part: the "
                    "backdrop needs to cover the viewport and center the panel, the dialog gets the visual frame. The dialog "
                    "is focusable, and browsers draw an outline when it receives focus on mount; hide it with "
                    <Code inline=true>":focus"</Code>":"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-backdrop {
                            position: fixed;
                            inset: 0;
                            z-index: 1000;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            background: rgba(0, 0, 0, 0.5);
                        }
                        .my-panel { width: 100%; max-width: 26em; }
                        .my-dialog { padding: 1.5em; border-radius: 12px; background: white; }
                        .my-dialog:focus { outline: none; }
                        .my-dialog[role="alertdialog"] { border-top: 4px solid crimson; }
                    "#)}
                </Code>
                <p>
                    "The backdrop always has the class "<Code inline=true>"leptonic-modal-backdrop"</Code>", which the "
                    <Link href=routes::doc::modal::Component.materialize()>"Modal component"</Link>"\u{2019}s theme styles "
                    "(a fixed, blurred backdrop). If your app includes the leptonic theme, these styles apply to the atom too; "
                    "override them in your own class."
                </p>
                <Section title="Animation">
                    <p>
                        "The modal is mounted when it opens, so a CSS animation on the backdrop or the panel plays as an "
                        "enter animation, as in the demos. There is no exit animation: the modal is removed as soon as its "
                        <Code inline=true>"state"</Code>" closes, and the atoms "
                        "don\u{2019}t render "<Code inline=true>"data-entering"</Code>" or "<Code inline=true>"data-exiting"</Code>" states."
                    </p>
                    <Code language=Language::Css>
                        {indoc!(r"
                            .my-backdrop { animation: fade-in 150ms ease-out; }
                            .my-panel { animation: slide-in 150ms ease-out; }

                            @keyframes fade-in { from { opacity: 0; } }
                            @keyframes slide-in { from { transform: translateY(0.75em); } }

                            @media (prefers-reduced-motion: reduce) {
                                .my-backdrop, .my-panel { animation: none; }
                            }
                        ")}
                    </Code>
                </Section>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The atoms implement the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/" target=LinkTarget::_Blank>
                        "Dialog (Modal) pattern"
                    </LinkExt>" described on the "<Link href=routes::doc::Modal.materialize()>"Modal overview"</Link>":"
                </p>
                <ul>
                    <li>
                        <b>"Focus moves in."</b>" When the modal opens, "<Code inline=true>"ModalContent"</Code>
                        " focuses its first focusable element. Without one, the "<Code inline=true>"Dialog"</Code>" itself receives focus."
                    </li>
                    <li>
                        <b>"Focus stays in."</b>" "<Keys keys="Tab"/>" and "<Keys keys="Shift + Tab"/>" cycle through the modal\u{2019}s "
                        "elements. The rest of the page is inert, so it can\u{2019}t be focused or clicked."
                    </li>
                    <li>
                        <b>"Focus returns."</b>" When the modal closes, focus goes back to the element that had it before, "
                        "typically the trigger."
                    </li>
                    <li>
                        <b>"Dismissal."</b>" "<Keys keys="Escape"/>" closes the modal unless "
                        <Code inline=true>"is_keyboard_dismiss_disabled"</Code>" is set. A click outside the panel closes it "
                        "only with "<Code inline=true>"is_dismissable"</Code>". With modals stacked, only the topmost one closes."
                    </li>
                    <li>
                        <b>"No scrolling."</b>" The page doesn\u{2019}t scroll while the modal is open."
                    </li>
                    <li>
                        <b>"Semantics."</b>" The "<Code inline=true>"Dialog"</Code>" has the "<Code inline=true>"dialog"</Code>
                        " or "<Code inline=true>"alertdialog"</Code>" role and is named by its "
                        <Code inline=true>"DialogTitle"</Code>". An alert dialog is described by its "
                        <Code inline=true>"DialogDescription"</Code>". Every dialog needs an accessible name; in debug "
                        "builds, a dialog without one logs a warning."
                    </li>
                </ul>
                <p>
                    "A dialog without a visible title needs a label of its own. "<Code inline=true>"aria_label"</Code>
                    " renders it as the dialog\u{2019}s "<Code inline=true>"aria-label"</Code>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        <Dialog aria_label="Image preview">
                            <img src="/photo.jpg" alt="Sunset over the sea"/>
                        </Dialog>
                    "#)}
                </Code>
                <KeyboardTable>
                    <KeyRow keys="Escape">"Closes the modal, unless keyboard dismissal is disabled."</KeyRow>
                    <KeyRow keys="Tab">"Moves focus to the next element inside the modal, wrapping around."</KeyRow>
                    <KeyRow keys="Shift + Tab">"Moves focus to the previous element inside the modal, wrapping around."</KeyRow>
                </KeyboardTable>
            </Section>

            <Section title="Composition">
                <p>
                    "The parts find each other through context: "<Code inline=true>"ModalContent"</Code>" takes the dismiss "
                    "handling from "<Code inline=true>"ModalBackdrop"</Code>", and "<Code inline=true>"DialogTitle"</Code>" and "
                    <Code inline=true>"DialogDescription"</Code>" connect to the enclosing "<Code inline=true>"Dialog"</Code>
                    ", wherever they are inside it. Your own title component wraps "<Code inline=true>"DialogTitle"</Code>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
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
                <p>
                    "The "<Link href=routes::doc::modal::Component.materialize()>"Modal component"</Link>" is built this way: "
                    "it composes "<Code inline=true>"ModalBackdrop"</Code>", "<Code inline=true>"ModalContent"</Code>" and "
                    <Code inline=true>"Dialog"</Code>", and its "<Code inline=true>"ModalTitle"</Code>" is a themed "
                    <Code inline=true>"DialogTitle"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Modal.materialize()>"Modal overview"</Link></li>
                <li><Link href=routes::doc::modal::Hook.materialize()>"Modal & dialog hooks"</Link></li>
                <li><Link href=routes::doc::modal::Component.materialize()>"Modal component"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope atom"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
