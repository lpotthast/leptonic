use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    modal_confirm::ModalConfirmDemo, modal_simple::ModalSimpleDemo, modal_staged::ModalStagedDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageModal() -> impl IntoView {
    view! {
        <DocPage title="Modal component">
            <p>
                "The themed "<Code inline=true>"Modal"</Code>" asks your users a critical question or informs them "
                "about something they must acknowledge, blocking the rest of the page until they respond. See the "
                <Link href=routes::doc::Modal.materialize()>"Modal overview"</Link>" for concept guidance."
            </p>

            <Demo
                description="Modal opened by a button and closed with Cancel, Escape or a click outside"
                source=include_str!("demos/modal_simple.rs")
            >
                <ModalSimpleDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="Modal">
                    <ApiRow name="state" ty="OverlayTriggerState">
                        "Whether the modal is open. Pass app state ("<Code inline=true>"state=rw_signal"</Code>", "
                        <Code inline=true>"state=(read, write)"</Code>"); dismissing the modal sets it to "<Code inline=true>"false"</Code>", see "
                        <a href="#dismissing">"Dismissing"</a>". Required."
                    </ApiRow>
                    <ApiRow name="is_dismissable" ty="bool" default="true">
                        "Whether a click or tap outside the modal closes it."
                    </ApiRow>
                    <ApiRow name="is_keyboard_dismiss_disabled" ty="bool" default="false">
                        "Whether pressing "<Keys keys="Escape"/>" no longer closes the modal."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                        "Names a modal without a "<Code inline=true>"ModalTitle"</Code>" (which names it automatically)."
                    </ApiRow>
                    <ApiRow name="role" ty="DialogRole" default="Dialog">
                        <Code inline=true>"Dialog"</Code>" or "<Code inline=true>"AlertDialog"</Code>
                        ". Use "<Code inline=true>"AlertDialog"</Code>" for urgent prompts that need an answer."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles of the modal panel."
                    </ApiRow>
                    <ApiRow name="children" ty="ChildrenFn">"The modal content."</ApiRow>
                </ApiTable>

                <Section title="ModalHeader, ModalTitle, ModalBody, ModalFooter">
                    <p>
                        "These components structure the modal content and apply the theme\u{2019}s spacing. Each takes "
                        <Code inline=true>"classes"</Code>", "<Code inline=true>"styles"</Code>" and children. "
                        <Code inline=true>"ModalTitle"</Code>" belongs in the "<Code inline=true>"ModalHeader"</Code>
                        ": it renders an "<Code inline=true>"<h2>"</Code>" that names the modal."
                    </p>
                </Section>
            </Section>

            <Section title="Stages">
                <p>
                    "Connect several modals by switching their states, for example to guide your users through the steps "
                    "of a process. The second modal binds "<Code inline=true>"ValueBinding::new(signal, callback)"</Code>
                    ": dismissing it calls the callback, which also reopens the first one."
                </p>

                <Demo
                    description="Two modals forming a two-step process with Next and Back"
                    source=include_str!("demos/modal_staged.rs")
                >
                    <ModalStagedDemo/>
                </Demo>
            </Section>

            <Section title="Reacting to User Input">
                <p>
                    "A modal can contain any reactive content. This one only lets you confirm after you enter "
                    "\u{201c}ok\u{201d} in its input field."
                </p>

                <Demo
                    description="Confirmation modal requiring text input before confirming"
                    source=include_str!("demos/modal_confirm.rs")
                >
                    <ModalConfirmDemo/>
                </Demo>
            </Section>

            <Section title="Dismissing">
                <p>
                    "Users expect a modal to close when they press "<Keys keys="Escape"/>" or click outside of it. In both "
                    "cases, the modal sets its bound state to "<Code inline=true>"false"</Code>". To run more logic on "
                    "close, bind "<Code inline=true>"ValueBinding::new(signal, callback)"</Code>" (as the "
                    <a href="#stages">"Stages"</a>" demo does), or pass an "<Code inline=true>"OverlayTriggerState"</Code>
                    " from "<Link href=routes::doc::modal::Hook.materialize()>"use_overlay_trigger_state"</Link>" with "
                    <Code inline=true>"on_open_change"</Code>". Set "<Code inline=true>"is_dismissable=false"</Code>
                    " to ignore clicks outside, and "<Code inline=true>"is_keyboard_dismiss_disabled=true"</Code>
                    " to ignore "<Keys keys="Escape"/>". With both set, only your own buttons close the modal."
                </p>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The modal renders an element with "<Code inline=true>"role=\"dialog\""</Code>" (or "
                    <Code inline=true>"alertdialog"</Code>") and "<Code inline=true>"aria-modal=\"true\""</Code>
                    ". It is rendered in a portal, prevents scrolling of the page, keeps focus inside while open and "
                    "restores focus to the previously focused element when it closes."
                </p>
                <p>
                    "Give every modal an accessible name: render its title in a "<Code inline=true>"ModalTitle"</Code>
                    ", as the demos do. While the "<Code inline=true>"ModalTitle"</Code>" is rendered, the dialog "
                    "references it with "<Code inline=true>"aria-labelledby"</Code>". For a modal without a visible title, "
                    "set "<Code inline=true>"aria_label"</Code>" instead. A modal with "
                    <Code inline=true>"role=DialogRole::AlertDialog"</Code>" is also described by a "
                    <Link href=routes::doc::modal::Atom.materialize()>"DialogDescription"</Link>
                    " inside it, so screen readers read its message when it opens."
                </p>
                <KeyboardTable>
                    <KeyRow keys="Escape">
                        "Closes the modal, unless "<Code inline=true>"is_keyboard_dismiss_disabled"</Code>" is set."
                    </KeyRow>
                    <KeyRow keys="Tab / Shift + Tab">
                        "Moves focus between the focusable elements of the modal, wrapping around."
                    </KeyRow>
                </KeyboardTable>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt modals to your design:"</p>
                <CssVariables prefix="--modal-" scss=theme_scss!("modal")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Modal.materialize()>"Modal overview"</Link></li>
                <li><Link href=routes::doc::modal::Hook.materialize()>"Modal hooks"</Link></li>
                <li><Link href=routes::doc::popover::Component.materialize()>"Popover component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
