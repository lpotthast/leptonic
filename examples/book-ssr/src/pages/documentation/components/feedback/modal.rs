use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    modal_confirm::ModalConfirmDemo, modal_simple::ModalSimpleDemo, modal_staged::ModalStagedDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageModal() -> impl IntoView {
    view! {
        <DocPage title="Modal Components">
            <p>
                "The themed "<Code inline=true>"Modal"</Code>" asks your users a question or tells them something they "
                "must acknowledge, blocking the rest of the page until they respond. See the "
                <Link href=routes::doc::Modal.materialize()>"Modal overview"</Link>" for concept guidance."
            </p>

            <Demo
                description="Modal opened by a button and closed with Got it, Escape or a click outside"
                source=include_str!("demos/modal_simple.rs")
            >
                <ModalSimpleDemo/>
            </Demo>

            <Section title="Modal">
                <p>
                    "The modal: a "<Link href=routes::doc::modal::Atom.materialize()>"ModalBackdrop"</Link>" with a "
                    <Code inline=true>"ModalContent"</Code>" and a "<Link href=routes::doc::dialog::Atom.materialize()>"Dialog"</Link>
                    ", rendered into the document body while it is open. Put a "<Code inline=true>"ModalHeader"</Code>
                    ", a "<Code inline=true>"ModalBody"</Code>" and a "<Code inline=true>"ModalFooter"</Code>" inside."
                </p>
                <Section title="Props" id="modal-props">
                    <ApiTable kind=ApiKind::Props of="Modal">
                        <ApiRow name="is_open" ty="Signal<bool>">
                            "Whether the modal is open: a value or any signal. Required."
                        </ApiRow>
                        <ApiRow name="set_open" ty="Out<bool>">
                            "Receives the open state when "<Keys keys="Escape"/>" or (when dismissable) a click outside closes the "
                            "modal: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure, "<Code inline=true>"Callback"</Code>", \u{2026} Required."
                        </ApiRow>
                        <ApiRow name="is_dismissable" ty="Signal<bool>" default="true">
                            "Whether a click or tap outside the modal closes it."
                        </ApiRow>
                        <ApiRow name="is_keyboard_dismiss_disabled" ty="Signal<bool>" default="false">
                            "Whether "<Keys keys="Escape"/>" no longer closes the modal."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names a modal without a "<Code inline=true>"ModalTitle"</Code>" (which names it automatically)."
                        </ApiRow>
                        <ApiRow name="role" ty="DialogRole" default="Dialog">
                            <Code inline=true>"DialogRole::AlertDialog"</Code>" for an urgent prompt that needs an answer."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles of the modal panel ("<Code inline=true>".leptonic-modal"</Code>")."
                        </ApiRow>
                        <ApiRow name="children" ty="ChildrenFn">"The modal content. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ModalHeader">
                <p>"The top of the modal, centering its "<Code inline=true>"ModalTitle"</Code>"."</p>
                <Section title="Props" id="modal-header-props">
                    <ApiTable kind=ApiKind::Props of="ModalHeader">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles."</ApiRow>
                        <ApiRow name="children" ty="Children">"Typically a "<Code inline=true>"ModalTitle"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ModalTitle">
                <p>
                    "The modal\u{2019}s title: a themed "
                    <Link href=format!("{}#dialogtitle", routes::doc::dialog::Atom.materialize())>"DialogTitle"</Link>
                    ", rendered as an "<Code inline=true>"<h2>"</Code>". While it is rendered, it names the modal."
                </p>
                <Section title="Props" id="modal-title-props">
                    <ApiTable kind=ApiKind::Props of="ModalTitle">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles."</ApiRow>
                        <ApiRow name="children" ty="Children">"The title. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ModalBody">
                <p>"The content of the modal, with the theme\u{2019}s spacing. It scrolls when the content is too tall."</p>
                <Section title="Props" id="modal-body-props">
                    <ApiTable kind=ApiKind::Props of="ModalBody">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles."</ApiRow>
                        <ApiRow name="children" ty="Children">"The content. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ModalFooter">
                <p>"The bottom of the modal, aligning its buttons (in a "<Code inline=true>"ButtonWrapper"</Code>") to the end."</p>
                <Section title="Props" id="modal-footer-props">
                    <ApiTable kind=ApiKind::Props of="ModalFooter">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles."</ApiRow>
                        <ApiRow name="children" ty="Children">"The actions. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Dismissing">
                <p>
                    "Users expect a modal to close when they press "<Keys keys="Escape"/>" or click outside of it. In both "
                    "cases, the modal calls "<Code inline=true>"set_open"</Code>" with "<Code inline=true>"false"</Code>
                    ". To run more logic on close, pass a closure as "<Code inline=true>"set_open"</Code>", as the demo at "
                    "the top of the page does to count how often the notes were read. Set "
                    <Code inline=true>"is_dismissable=false"</Code>" to ignore clicks outside, and "
                    <Code inline=true>"is_keyboard_dismiss_disabled=true"</Code>" to ignore "<Keys keys="Escape"/>
                    ". With both set, only your own buttons close the modal."
                </p>
            </Section>

            <Section title="Stages">
                <p>
                    "Connect several modals through one state to guide your users through the steps of a process. Each "
                    "modal\u{2019}s "<Code inline=true>"set_open"</Code>" decides where dismissing it leads: the first step "
                    "cancels, the second goes back to the first."
                </p>

                <Demo
                    description="Two modals forming a two-step sign-up with Next, Back and Finish"
                    source=include_str!("demos/modal_staged.rs")
                >
                    <ModalStagedDemo/>
                </Demo>
            </Section>

            <Section title="Reacting to User Input">
                <p>
                    "A modal can contain any reactive content. This one enables its destructive action only after you "
                    "typed the repository\u{2019}s name."
                </p>

                <Demo
                    description="Confirmation modal requiring text input before deleting"
                    source=include_str!("demos/modal_confirm.rs")
                >
                    <ModalConfirmDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The modal has the full behavior of the "<Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link>
                    ", described on the "<Link href=routes::doc::Modal.materialize()>"Modal overview"</Link>". Compared with "
                    "them, it lacks:"
                </p>
                <ul>
                    <li>
                        "A description part: "<Code inline=true>"ModalBody"</Code>" doesn\u{2019}t describe an alert dialog. "
                        "Wrap the message in the "
                        <Link href=format!("{}#dialogdescription", routes::doc::dialog::Atom.materialize())>"DialogDescription"</Link>
                        " atom, so that screen readers read it when the modal opens."
                    </li>
                    <li>
                        "Opening through a "<Link href=format!("{}#dialogtrigger", routes::doc::dialog::Atom.materialize())>"DialogTrigger"</Link>
                        ": the modal needs your open state, so its button gets no "<Code inline=true>"aria-expanded"</Code>
                        " and "<Code inline=true>"aria-controls"</Code>". Build the modal from the atoms where the button "
                        "should announce it."
                    </li>
                </ul>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt modals to your design:"</p>
                <CssVariables prefix="--modal-" scss=theme_scss!("modal")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Modal.materialize()>"Modal overview"</Link></li>
                <li><Link href=routes::doc::modal::Hook.materialize()>"Modal Hooks"</Link></li>
                <li><Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link></li>
                <li><Link href=routes::doc::dialog::Atom.materialize()>"Dialog Atoms"</Link></li>
                <li><Link href=routes::doc::popover::Component.materialize()>"Popover Component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
