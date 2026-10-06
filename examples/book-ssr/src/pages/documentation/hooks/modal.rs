use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    modal_alert::AlertDialogDemo, modal_basic::BasicModalDemo,
    modal_confirmation::ConfirmationDialogDemo, modal_non_dismissable::NonDismissableModalDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUseModalHook() -> impl IntoView {
    view! {
        <DocPage title="Modal & Dialog Hooks">
            <p>
                "These hooks build accessible modal dialogs: open state, dismissal, scroll prevention, "
                <Code inline=true>"aria-modal"</Code>", dialog roles and labelling, and focus on mount. "
                "See the "<Link href=routes::doc::Modal.materialize()>"Modal overview"</Link>" for concept guidance."
            </p>

            <Section title="Hook Composition">
                <p>"Each hook handles one concern. You combine the ones you need:"</p>

                <ol>
                    <li>
                        <b>"State"</b>": "<a href="#use-overlay-trigger-state"><Code inline=true>"use_overlay_trigger_state"</Code></a>
                        " owns the open state."
                    </li>
                    <li>
                        <b>"Backdrop"</b>": "<a href="#use-modal-backdrop"><Code inline=true>"use_modal_backdrop"</Code></a>
                        " closes the modal on Escape and outside clicks, prevents page scrolling and hides the rest of the "
                        "page from assistive technology."
                    </li>
                    <li>
                        <b>"Modal"</b>": "<a href="#use-modal"><Code inline=true>"use_modal"</Code></a>" sets "
                        <Code inline=true>"aria-modal=\"true\""</Code>"."
                    </li>
                    <li>
                        <b>"Dialog"</b>": "<a href="#use-dialog"><Code inline=true>"use_dialog"</Code></a>" sets the "
                        <Code inline=true>"dialog"</Code>" or "<Code inline=true>"alertdialog"</Code>
                        " role, names the dialog by its title, and focuses it when it opens."
                    </li>
                    <li>
                        <b>"Focus"</b>": the "<Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>
                        " atom keeps focus inside the dialog and restores it on close."
                    </li>
                </ol>
            </Section>

            <Section title="When to Use">
                <DocTable headers=&["Use case", "Hooks"]>
                    <TableRow>
                        <TableCell>"Modal dialog with a title"</TableCell>
                        <TableCell>
                            <Code inline=true>"use_overlay_trigger_state"</Code>", "<Code inline=true>"use_modal_backdrop"</Code>", "
                            <Code inline=true>"use_modal"</Code>", "<Code inline=true>"use_dialog"</Code>" and "
                            <Code inline=true>"FocusScope"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Alert dialog that requires a response"</TableCell>
                        <TableCell>"The same, with "<Code inline=true>"DialogRole::AlertDialog"</Code>" and a non-dismissable backdrop"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Confirmation dialog (confirm or cancel)"</TableCell>
                        <TableCell>"The same, remembering in your own signal which button closed it"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Modal without a visible title"</TableCell>
                        <TableCell><Code inline=true>"use_dialog"</Code>" with "<Code inline=true>"aria_label"</Code>" instead of a title element"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Overlay positioned next to a trigger"</TableCell>
                        <TableCell><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Other non-modal overlay"</TableCell>
                        <TableCell><Link href=routes::doc::overlays::UseOverlay.materialize()>"use_overlay"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <p>
                    "A complete modal dialog. The backdrop closes it on Escape and outside clicks, the page doesn\u{2019}t "
                    "scroll while it is open, and focus stays inside."
                </p>

                <Demo description="Modal dialog composed from the modal and dialog hooks" source=include_str!("demos/modal_basic.rs") source_open=true>
                    <BasicModalDemo/>
                </Demo>
            </Section>

            <Section title="Alert Dialog">
                <p>
                    "An alert dialog asks for a response to an important message. It uses "
                    <Code inline=true>"role=\"alertdialog\""</Code>" and typically ignores Escape and outside clicks:"
                </p>

                <Demo description="Alert dialog with Cancel and Delete buttons" source=include_str!("demos/modal_alert.rs")>
                    <AlertDialogDemo/>
                </Demo>
            </Section>

            <Section title="Non-Dismissable Modal">
                <p>
                    "With "<Code inline=true>"is_dismissable: false"</Code>" and "<Code inline=true>"is_keyboard_dismiss_disabled: true"</Code>
                    ", only your own controls close the modal, for example after a required step:"
                </p>

                <Demo description="Modal that only its own button closes" source=include_str!("demos/modal_non_dismissable.rs")>
                    <NonDismissableModalDemo/>
                </Demo>
            </Section>

            <Section title="Confirmation Dialog">
                <p>
                    "The demo remembers in a signal whether the dialog was confirmed. Closing it any other way, including "
                    "Escape, counts as cancelled:"
                </p>

                <Demo description="Confirmation dialog showing the last outcome" source=include_str!("demos/modal_confirmation.rs")>
                    <ConfirmationDialogDemo/>
                </Demo>
            </Section>

            <Section title="use_overlay_trigger_state">
                <ReactAria hook="useOverlayTriggerState"/>

                <p>
                    "Owns the open state of an overlay (modal, popover, menu, ...). Change it through the state\u{2019}s "
                    "methods, or bind it to a signal of your app with "<Code inline=true>"value"</Code>"."
                </p>

                <Section title="Input" id="use-overlay-trigger-state-input">
                    <ApiTable kind=ApiKind::Input of="UseOverlayTriggerStateInput">
                        <ApiRow name="default_open" ty="bool" default="false">"Whether the overlay starts open."</ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<bool>>" default="None">
                            "The open state as app state ("<Code inline=true>"ValueBinding::from(rw_signal)"</Code>"), replacing "
                            <Code inline=true>"default_open"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">
                            "Called when the overlay opens or closes."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="State" id="use-overlay-trigger-state-state">
                    <ApiTable kind=ApiKind::Fields of="OverlayTriggerState">
                        <ApiRow name="is_open" ty="Signal<bool>">"Whether the overlay is open."</ApiRow>
                        <ApiRow name="point" ty="Signal<Option<Point>>">"Where a point-anchored overlay (e.g. a context menu) opened."</ApiRow>
                    </ApiTable>
                    <DocTable headers=&["Method", "Purpose"]>
                        <TableRow>
                            <TableCell><Code inline=true>"open(), close(), toggle()"</Code></TableCell>
                            <TableCell>"Open, close or toggle the overlay."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_open(is_open)"</Code></TableCell>
                            <TableCell>"Sets the open state."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_point(point)"</Code></TableCell>
                            <TableCell>"Sets the point the overlay opens at."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="use_modal_backdrop">
                <ReactAria hook="useModalOverlay"/>

                <p>
                    "Provides the dismiss behavior of a modal through "
                    <Link href=routes::doc::overlays::UseOverlay.materialize()>"use_overlay"</Link>
                    ": Escape (ignored during IME composition) and outside clicks close only the topmost overlay. While the "
                    "modal is open, it prevents page scrolling and makes everything outside the modal inert, so assistive "
                    "technology ignores it. Unlike popovers, modals don\u{2019}t close on blur."
                </p>

                <Section title="Input" id="use-modal-backdrop-input">
                    <p><Code inline=true>"UseModalBackdropInput::new(state)"</Code>" sets the defaults: closed by "<Keys keys="Escape"/>
                        ", not by outside clicks."</p>

                    <ApiTable kind=ApiKind::Input of="UseModalBackdropInput">
                        <ApiRow name="state" ty="OverlayTriggerState">
                            "Whether the modal is open ("<Code inline=true>"use_overlay_trigger_state"</Code>"); dismissing closes it."
                        </ApiRow>
                        <ApiRow name="is_dismissable" ty="bool">"Whether clicking outside the modal closes it."</ApiRow>
                        <ApiRow name="is_keyboard_dismiss_disabled" ty="bool">"Ignore Escape."</ApiRow>
                        <ApiRow name="should_close_on_interact_outside" ty="Option<Callback<web_sys::Element, bool>>">
                            "Decides per outside element whether clicking it closes the modal. "
                            <Code inline=true>"None"</Code>" closes on every outside click."
                        </ApiRow>
                        <ApiRow name="is_entering" ty="Signal<bool>">
                            "While "<Code inline=true>"true"</Code>" (an entry animation runs), the page outside isn\u{2019}t hidden from "
                            "assistive technology yet."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-modal-backdrop-return">
                    <ApiTable kind=ApiKind::Return of="UseModalBackdropReturn">
                        <ApiRow name="modal_props" ty="UseModalBackdropModalProps">
                            "Spread "<Code inline=true>"modal_props.into_attrs()"</Code>" onto the modal element: its id, "
                            "the Escape handler and focus tracking."
                        </ApiRow>
                        <ApiRow name="id" ty="Oco<'static, str>">"The id of the modal element."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-modal-backdrop-example">
                    <Code language=Language::Rust>
                        {indoc!(r"
                            let UseModalBackdropReturn { modal_props, .. } =
                                use_modal_backdrop(UseModalBackdropInput {
                                    is_dismissable: true,
                                    ..UseModalBackdropInput::new(state)
                                });
                            // `modal_props.into_attrs()` go onto the modal element inside the
                            // backdrop. The backdrop itself needs no props.
                        ")}
                    </Code>
                </Section>
            </Section>

            <Section title="use_modal">
                <p>
                    "Sets "<Code inline=true>"aria-modal=\"true\""</Code>" on the modal element, which tells assistive "
                    "technology that the content outside is unavailable."
                </p>

                <Section title="Input" id="use-modal-input">
                    <ApiTable kind=ApiKind::Input of="UseModalInput">
                        <ApiRow name="is_disabled" ty="bool" default="false">
                            "Leave out "<Code inline=true>"aria-modal"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-modal-return">
                    <ApiTable kind=ApiKind::Return of="UseModalReturn">
                        <ApiRow name="modal_props" ty="UseModalProps">
                            "Spread "<Code inline=true>"modal_props.into_attrs()"</Code>" onto the modal element."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-modal-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let UseModalReturn { modal_props } = use_modal(UseModalInput::default());

                            view! { <div {..modal_props.into_attrs()}>"Modal content"</div> }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_dialog">
                <ReactAria hook="useDialog"/>

                <p>
                    "Gives an element dialog semantics: the "<Code inline=true>"dialog"</Code>" or "
                    <Code inline=true>"alertdialog"</Code>" role, an accessible name and "
                    <Code inline=true>"tabindex=\"-1\""</Code>". When the dialog mounts, it receives focus unless an element "
                    "inside it already has focus; with "<Code inline=true>"is_entering"</Code>", focusing waits until an "
                    "entry animation has finished. On iOS Safari, the dialog is blurred and focused again after 500 ms, so "
                    "VoiceOver announces it. The hook doesn\u{2019}t close the dialog; use "
                    <a href="#use-modal-backdrop"><Code inline=true>"use_modal_backdrop"</Code></a>" for that."
                </p>
                <p>
                    "Spread "<Code inline=true>"title_props"</Code>" onto the dialog\u{2019}s heading: while the heading is "
                    "rendered, the dialog\u{2019}s "<Code inline=true>"aria-labelledby"</Code>" points to it. An alert "
                    "dialog is also described by the element you spread "<Code inline=true>"content_props"</Code>" onto ("
                    <Code inline=true>"aria-describedby"</Code>"), so screen readers read its message when it opens. "
                    <Code inline=true>"aria_label"</Code>" names a dialog without a visible title; "
                    <Code inline=true>"aria_labelledby"</Code>" and "<Code inline=true>"aria_describedby"</Code>
                    " point to other elements instead. In debug builds, a dialog without a name logs a warning."
                </p>

                <Section title="Input" id="use-dialog-input">
                    <ApiTable kind=ApiKind::Input of="UseDialogInput">
                        <ApiRow name="aria_label" ty="MaybeProp<String>">
                            "Names a dialog without a title element; the title then doesn\u{2019}t name it."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>">
                            "Ids of the elements naming the dialog, instead of its title element."
                        </ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>">
                            "Ids of the elements describing the dialog. Default for alert dialogs: the content element ("
                            <Code inline=true>"content_props"</Code>")."
                        </ApiRow>
                        <ApiRow name="is_entering" ty="Signal<bool>">
                            "While "<Code inline=true>"true"</Code>" (an entry animation runs), the dialog isn\u{2019}t focused yet."
                        </ApiRow>
                        <ApiRow name="fallback_aria_labelledby" ty="Signal<Option<String>>">
                            "Names the dialog when it has no title, "<Code inline=true>"aria_label"</Code>" or "
                            <Code inline=true>"aria_labelledby"</Code>": the "<Code inline=true>"Dialog"</Code>
                            " atom passes its "<Code inline=true>"DialogTrigger"</Code>"\u{2019}s button."
                        </ApiRow>
                        <ApiRow name="role" ty="DialogRole">
                            <Code inline=true>"DialogRole::Dialog"</Code>" (the enum\u{2019}s default) or "
                            <Code inline=true>"DialogRole::AlertDialog"</Code>" for messages that require a response."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-dialog-return">
                    <ApiTable kind=ApiKind::Return of="UseDialogReturn">
                        <ApiRow name="dialog_props" ty="UseDialogProps">
                            "Spread "<Code inline=true>"dialog_props.into_attrs()"</Code>" onto the dialog element: id, role, "
                            "labelling, "<Code inline=true>"tabindex"</Code>", a blur handler and the element capture used to "
                            "focus it on mount."
                        </ApiRow>
                        <ApiRow name="title_props" ty="SlotProps">
                            "Spread onto the title element (a heading): while it is rendered, it names the dialog."
                        </ApiRow>
                        <ApiRow name="content_props" ty="SlotProps">
                            "Spread onto the content element: while it is rendered, it describes an alert dialog."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-dialog-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let UseDialogReturn { dialog_props, title_props, content_props } =
                                use_dialog(UseDialogInput {
                                    role: DialogRole::AlertDialog,
                                    ..UseDialogInput::default()
                                });

                            view! {
                                // Named by the heading, described by the paragraph.
                                <section {..dialog_props.into_attrs()}>
                                    <h2 {..title_props.into_attrs()}>"Delete file?"</h2>
                                    <p {..content_props.into_attrs()}>"It is deleted permanently."</p>
                                    <button>"Delete"</button>
                                </section>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        <Code inline=true>"role=\"dialog\""</Code>" or "<Code inline=true>"role=\"alertdialog\""</Code>
                        " come from "<Code inline=true>"use_dialog"</Code>". The dialog is named by its title element ("
                        <Code inline=true>"aria-labelledby"</Code>") or "<Code inline=true>"aria_label"</Code>
                        "; an alert dialog is described by its content element ("<Code inline=true>"aria-describedby"</Code>")."
                    </li>
                    <li>
                        <Code inline=true>"aria-modal=\"true\""</Code>" comes from "<Code inline=true>"use_modal"</Code>
                        "; "<Code inline=true>"use_modal_backdrop"</Code>" additionally makes the rest of the page inert."
                    </li>
                    <li>"Escape closes only the topmost modal."</li>
                    <li>
                        "Focus moves into the dialog when it opens. Wrap the dialog in a "
                        <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>" with "
                        <Code inline=true>"contain=true"</Code>" and "<Code inline=true>"restore_focus=true"</Code>
                        " to keep focus inside and return it to the trigger afterwards."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Escape">"Closes the modal, unless keyboard dismissal is disabled."</KeyRow>
                    <KeyRow keys="Tab">"Moves focus to the next element inside the modal."</KeyRow>
                    <KeyRow keys="Shift + Tab">"Moves focus to the previous element inside the modal."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Modal.materialize()>"Modal overview"</Link></li>
                <li><Link href=routes::doc::modal::Component.materialize()>"Modal component"</Link></li>
                <li><Link href=routes::doc::overlays::UseOverlay.materialize()>"use_overlay"</Link></li>
                <li><Link href=routes::doc::Popover.materialize()>"Popover"</Link></li>
                <li><Link href=routes::doc::interactions::UsePreventScroll.materialize()>"use_prevent_scroll"</Link></li>
                <li><Link href=routes::doc::interactions::UseInteractOutside.materialize()>"use_interact_outside"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
