use indoc::indoc;
use leptos::prelude::*;

use super::demos::{modal_drawer::ModalDrawerDemo, modal_form::ModalFormDemo};
use crate::{kit::*, routes};

/// A section of the modal hooks page.
fn hook_section(id: &str) -> String {
    format!("{}#{id}", routes::doc::modal::Hook.materialize())
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomModal() -> impl IntoView {
    view! {
        <DocPage title="Modal Atoms">
            <p>
                <Code inline=true>"ModalBackdrop"</Code>" and "<Code inline=true>"ModalContent"</Code>" render an unstyled "
                "modal: the blocking overlay that covers the page, keeps focus inside and closes on "<Keys keys="Escape"/>". The named content "
                "inside it is a "<Link href=routes::doc::dialog::Atom.materialize()>"Dialog"</Link>". See the "
                <Link href=routes::doc::Modal.materialize()>"Modal overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <ul>
                    <li>
                        <AnchorLink href="#modalbackdrop"><Code inline=true>"ModalBackdrop"</Code></AnchorLink>" calls "
                        <Link href=hook_section("use-modal-backdrop")>"use_modal_backdrop"</Link>": "<Keys keys="Escape"/>
                        ", outside clicks, overlay stacking, the scroll lock and the inert page."
                    </li>
                    <li>
                        <AnchorLink href="#modalcontent"><Code inline=true>"ModalContent"</Code></AnchorLink>" wraps its "
                        "children in a "<Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>
                        " that contains and restores focus, and starts a dismissable modal with a "
                        <Link href=routes::doc::overlay_behavior::DismissButton.materialize()>"DismissButton"</Link>"."
                    </li>
                </ul>
            </Section>

            <Section title="Example">
                <p>
                    <Code inline=true>"ModalBackdrop"</Code>" contains a "<Code inline=true>"ModalContent"</Code>
                    ", which contains the "<Link href=routes::doc::dialog::Atom.materialize()>"Dialog"</Link>
                    ". Here the backdrop shows your open state ("<Code inline=true>"is_open"</Code>"): dismissing the modal ("
                    <Keys keys="Escape"/>", a click outside) calls "<Code inline=true>"set_open"</Code>" with "
                    <Code inline=true>"false"</Code>", and your own buttons set the state directly:"
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

                            <ModalBackdrop is_open=is_open set_open=is_open is_dismissable=true>
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
                <p>
                    "The open state can also come from a "
                    <Link href=format!("{}#dialogtrigger", routes::doc::dialog::Atom.materialize())>"DialogTrigger"</Link>
                    " around the button and the backdrop, as in the demo: the button then gets "
                    <Code inline=true>"aria-expanded"</Code>" and "<Code inline=true>"aria-controls"</Code>", and a "
                    <Code inline=true>"Dialog"</Code>" without a title is named by it. Without either, the backdrop keeps "
                    "its own state, starting at "<Code inline=true>"default_open"</Code>"."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "A dismissable dialog with a form, opened through a "<Code inline=true>"DialogTrigger"</Code>
                    " whose open state is a signal of the app, so that saving can close it. The log shows how focus moves: "
                    "into the input when the dialog opens, around the dialog\u{2019}s elements with "<Keys keys="Tab"/>
                    ", and back to the trigger when it closes \u{2014} by saving, by \u{201c}Cancel\u{201d}, by "
                    <Keys keys="Escape"/>" or by a click on the backdrop."
                </p>
                <Demo
                    description="Dismissable dialog with a text input, logging how focus moves in and out"
                    source=include_str!("demos/modal_form.rs")
                >
                    <ModalFormDemo/>
                </Demo>
            </Section>

            <Section title="ModalBackdrop">
                <p>
                    "Renders the modal into the document body while it is open, as a "<Code inline=true>"<div>"</Code>" around its "
                    "children. While open, it prevents page "
                    "scrolling and makes everything outside the "<Code inline=true>"ModalContent"</Code>" inert. It is "
                    "unmounted when it closes."
                </p>
                <Section title="Props" id="modal-backdrop-props">
                    <ApiTable kind=ApiKind::Props of="ModalBackdrop">
                        <ApiRow name="is_open" ty="Option<Signal<bool>>" default="None">
                            "Whether the modal is open (controlled): a value or any signal. Default: the surrounding "<Code inline=true>"DialogTrigger"</Code>"\u{2019}s state."
                        </ApiRow>
                        <ApiRow name="set_open" ty="Option<Out<bool>>" default="None">
                            "Receives the open state (closing): an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="default_open" ty="Option<bool>" default="None">
                            "Whether the modal starts open, with its own state instead of a surrounding "<Code inline=true>"DialogTrigger"</Code>
                            "\u{2019}s. Ignored with "<Code inline=true>"is_open"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">
                            "Called when the modal opens or closes. With a surrounding "<Code inline=true>"DialogTrigger"</Code>
                            "\u{2019}s state, pass it to the trigger instead."
                        </ApiRow>
                        <ApiRow name="is_dismissable" ty="Signal<bool>" default="false">
                            "Whether a click outside the "<Code inline=true>"ModalContent"</Code>", e.g. on the backdrop, closes "
                            "the modal. It doesn\u{2019}t affect "<Keys keys="Escape"/>"."
                        </ApiRow>
                        <ApiRow name="is_keyboard_dismiss_disabled" ty="Signal<bool>" default="false">
                            "Whether "<Keys keys="Escape"/>" no longer closes the modal."
                        </ApiRow>
                        <ApiRow name="should_close_on_interact_outside" ty="Option<InteractOutsideFilter>" default="None">
                            "Decides per outside click whether the modal closes: called with the clicked element, "
                            <Code inline=true>"true"</Code>" closes. Only consulted when "<Code inline=true>"is_dismissable"</Code>
                            " is set."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the backdrop "<Code inline=true>"<div>"</Code>", added to "
                            <Code inline=true>"leptonic-ModalBackdrop"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="ChildrenFn">
                            "A "<Code inline=true>"ModalContent"</Code>". Rendered anew each time the modal opens. Required."
                        </ApiRow>
                    </ApiTable>
                    <p>
                        "See "<Link href=hook_section("use-modal-backdrop-input")>"use_modal_backdrop"</Link>
                        " for how these settings behave. When closing needs more than setting the state to "
                        <Code inline=true>"false"</Code>" (saving a draft, opening the next step), pass a closure as "
                        <Code inline=true>"set_open"</Code>": it runs with the new open state, also when the user dismisses "
                        "the modal."
                    </p>
                </Section>
            </Section>

            <Section title="ModalContent">
                <p>
                    "The modal panel: a "<Code inline=true>"<div>"</Code>" inside a "<Code inline=true>"FocusScope"</Code>
                    ". The page outside it is inert while it is open, which makes it modal for assistive technology. In a "
                    "dismissable backdrop, it starts with a visually hidden dismiss button for screen reader users who "
                    "can\u{2019}t press "<Keys keys="Escape"/>". It must be a child of "
                    <Code inline=true>"ModalBackdrop"</Code>": the backdrop\u{2019}s Escape handling and outside-click "
                    "detection are attached to this element, so everything outside it counts as outside the modal."
                </p>
                <Section title="Props" id="modal-content-props">
                    <ApiTable kind=ApiKind::Props of="ModalContent">
                        <ApiRow name="contain_focus" ty="Signal<bool>" default="true">
                            <Keys keys="Tab"/>" and "<Keys keys="Shift + Tab"/>" wrap around inside the modal, and focus that "
                            "leaves it is moved back."
                        </ApiRow>
                        <ApiRow name="restore_focus" ty="Signal<bool>" default="true">
                            "When the modal closes, focus returns to the element that had it when the modal opened, "
                            "typically the trigger. Read when the modal opens."
                        </ApiRow>
                        <ApiRow name="auto_focus" ty="Signal<bool>" default="false">
                            "When the modal opens, the first focusable element inside it is focused instead of the "
                            <Code inline=true>"Dialog"</Code>", which focuses itself. Read when the modal opens."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the panel "<Code inline=true>"<div>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">
                            "The modal\u{2019}s content, typically a "<Code inline=true>"Dialog"</Code>". Required."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-entering" ty="true">
                        "Set on the backdrop and on the panel while the modal opens, until the animations started by it finished."
                    </ApiRow>
                    <ApiRow name="data-exiting" ty="true">
                        "Set on the backdrop and on the panel while the modal closes. Both stay rendered until the exit "
                        "animations of either finished."
                    </ApiRow>
                </ApiTable>
                <p>
                    "The modal exists only while it is open or closing, so there is no closed state to style."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. "<Code inline=true>"ModalBackdrop"</Code>" renders the class "
                    <Code inline=true>"leptonic-ModalBackdrop"</Code>", "<Code inline=true>"ModalContent"</Code>" the class "
                    <Code inline=true>"leptonic-ModalContent"</Code>", each followed by the "<Code inline=true>"classes"</Code>
                    " you pass. The backdrop has to cover the viewport and place the panel; the "
                    <Link href=format!("{}#styling", routes::doc::dialog::Atom.materialize())>"dialog"</Link>
                    " inside the panel draws the frame. The backdrop also sets the CSS variables "
                    <Code inline=true>"--visual-viewport-height"</Code>" (the height left above an on-screen keyboard) and "
                    <Code inline=true>"--page-height"</Code>", for panels that have to fit them. The book\u{2019}s demos "
                    "use these rules:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-backdrop { position: fixed; inset: 0; display: flex; align-items: center; justify-content: center; }
                        .my-backdrop { padding: 1em; background: color-mix(in srgb, var(--muted) 60%, transparent); }
                        .my-panel { width: 100%; max-width: 26em; max-height: var(--visual-viewport-height); }
                    ")}
                </Code>
                <p>
                    "Leptonic also ships an optional atom theme that styles the default classes, for apps that don\u{2019}t "
                    "want to start from scratch: "<Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>"."
                </p>
                <Section title="Animation">
                    <p>
                        "Animate the backdrop and the panel with "<Code inline=true>"data-entering"</Code>" and "
                        <Code inline=true>"data-exiting"</Code>", as in the demo. When the modal closes, both stay rendered "
                        "until their exit animations finished, then they are removed together. While closing, the modal no "
                        "longer keeps focus inside."
                    </p>
                    <Code language=Language::Css>
                        {indoc!(r"
                            .my-backdrop[data-entering] { animation: fade 150ms ease-out; }
                            .my-backdrop[data-exiting] { animation: fade 150ms ease-in reverse forwards; }
                            .my-panel[data-entering] { animation: slide 150ms ease-out; }
                            .my-panel[data-exiting] { animation: slide 150ms ease-in reverse forwards; }

                            @keyframes fade { from { opacity: 0; } }
                            @keyframes slide { from { transform: translateY(0.75em); } }

                            @media (prefers-reduced-motion: reduce) {
                                @keyframes slide { from { transform: none; } }
                            }
                        ")}
                    </Code>
                </Section>
            </Section>

            <Section title="Drawer">
                <p>
                    "A drawer is a modal whose panel slides in from the left or right edge of the screen, such as a menu "
                    "on small screens or a set of filters. It needs nothing beyond these atoms: the backdrop lays the panel "
                    "out at one edge instead of the center, and the panel slides in while "<Code inline=true>"data-entering"</Code>
                    " and out while "<Code inline=true>"data-exiting"</Code>". It behaves like any modal: the focus stays "
                    "inside, "<Keys keys="Escape"/>" or a press outside closes it, and the page behind it can\u{2019}t be "
                    "used or scrolled. Without a visible title, name the dialog with "<Code inline=true>"aria_label"</Code>
                    "; give it a close button, as touch screen users have no "<Keys keys="Escape"/>". The book\u{2019}s own "
                    "menus on small screens are built this way."
                </p>
                <Demo description="Folder menu sliding in from the left, with a status line" source=include_str!("demos/modal_drawer.rs")>
                    <ModalDrawerDemo/>
                </Demo>
            </Section>

            <Section title="Composition">
                <p>
                    "The parts find each other through context: "<Code inline=true>"ModalContent"</Code>" takes the dismiss "
                    "handling from "<Code inline=true>"ModalBackdrop"</Code>", wherever it is inside it."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Modal.materialize()>"Modal overview"</Link></li>
                <li><Link href=routes::doc::modal::Hook.materialize()>"Modal Hooks"</Link></li>
                <li><Link href=routes::doc::dialog::Atom.materialize()>"Dialog Atoms"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
