use indoc::indoc;
use leptos::prelude::*;

use super::demos::modal_basic::ModalHooksDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseModalHook() -> impl IntoView {
    view! {
        <DocPage title="Modal Hooks">
            <p>
                <AnchorLink href="#use-modal-backdrop"><Code inline=true>"use_modal_backdrop"</Code></AnchorLink>" and "
                <AnchorLink href="#use-modal"><Code inline=true>"use_modal"</Code></AnchorLink>" make an element you "
                "render a modal: dismissable, blocking the page and marked for assistive technology. See the "
                <Link href=routes::doc::Modal.materialize()>"Modal overview"</Link>" for concept guidance."
            </p>

            <Section title="Example">
                <p>
                    "A complete modal dialog. Five pieces work together: "
                    <Link href=routes::doc::overlay_behavior::UseOverlayTriggerState.materialize()><Code inline=true>"use_overlay_trigger_state"</Code></Link>
                    " owns the open state, "<Code inline=true>"use_modal_backdrop"</Code>" closes the modal and locks the "
                    "page, "<Code inline=true>"use_modal"</Code>" sets "<Code inline=true>"aria-modal"</Code>", "
                    <Link href=routes::doc::dialog::Hook.materialize()>"use_dialog"</Link>" gives the content its role and "
                    "name, and a "<Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>" keeps focus "
                    "inside and returns it to the button. The modal is rendered in a "<Code inline=true>"Portal"</Code>
                    ", so it covers the page wherever the button is."
                </p>
                <p>
                    "The checkboxes switch how the user can dismiss the modal. With both off, only its own buttons close it, "
                    "as for a required step. The status line shows how it closed: "<Code inline=true>"on_open_change"</Code>
                    " of the state runs for every way of closing."
                </p>

                <Demo
                    description="Modal dialog composed from the modal and dialog hooks, with switchable dismissal"
                    source=include_str!("demos/modal_basic.rs")
                    source_open=true
                >
                    <ModalHooksDemo/>
                </Demo>
            </Section>

            <Section title="use_modal_backdrop">
                <ReactAria hook="useModalOverlay"/>

                <p>
                    "Makes the element you spread "<Code inline=true>"modal_props"</Code>" onto a modal, through "
                    <Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link>". It closes the modal "
                    "on "<Keys keys="Escape"/>" and, when "<Code inline=true>"is_dismissable"</Code>" is set, on a click "
                    "outside of it; with modals stacked, only the topmost one closes. A modal doesn\u{2019}t close when focus "
                    "leaves it. While the modal is open, the page doesn\u{2019}t scroll ("
                    <Link href=routes::doc::overlay_behavior::UsePreventScroll.materialize()>"use_prevent_scroll"</Link>
                    ") and everything outside the modal element is inert, so it can\u{2019}t be clicked or focused and "
                    "assistive technology ignores it."
                </p>
                <p>
                    "The backdrop element behind the modal needs no props: a click on it counts as a click outside."
                </p>

                <Section title="Input" id="use-modal-backdrop-input">
                    <p>
                        "Pass a "<Code inline=true>"UseModalBackdropInput"</Code>" with every field named; the Default column "
                        "gives the value for fields you don\u{2019}t need."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseModalBackdropInput">
                        <ApiRow name="state" ty="OverlayTriggerState">
                            "Whether the modal is open, from "
                            <Link href=routes::doc::overlay_behavior::UseOverlayTriggerState.materialize()>"use_overlay_trigger_state"</Link>
                            ". Dismissing the modal closes it. Required."
                        </ApiRow>
                        <ApiRow name="is_dismissable" ty="Signal<bool>" default="false">
                            "Whether a click outside the modal closes it."
                        </ApiRow>
                        <ApiRow name="is_keyboard_dismiss_disabled" ty="Signal<bool>" default="false">
                            "Whether "<Keys keys="Escape"/>" no longer closes the modal."
                        </ApiRow>
                        <ApiRow name="should_close_on_interact_outside" ty="Option<InteractOutsideFilter>" default="None">
                            "Decides per clicked element whether a click outside closes the modal ("<Code inline=true>"true"</Code>
                            " closes). Only consulted with "<Code inline=true>"is_dismissable"</Code>"; "
                            <Code inline=true>"None"</Code>" closes on every outside click."
                        </ApiRow>
                        <ApiRow name="is_entering" ty="Signal<bool>" default="false">
                            "While "<Code inline=true>"true"</Code>" (an entry animation runs), the page outside isn\u{2019}t hidden "
                            "from assistive technology yet, and an open parent modal doesn\u{2019}t hide this one."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-modal-backdrop-return">
                    <ApiTable kind=ApiKind::Return of="UseModalBackdropReturn">
                        <ApiRow name="modal_props" ty="UseModalBackdropModalProps">
                            "Spread "<Code inline=true>"{..modal_props.into_attrs()}"</Code>" onto the modal element: its id, "
                            "the element capture, the "<Keys keys="Escape"/>" handler and focus tracking."
                        </ApiRow>
                        <ApiRow name="id" ty="Oco<'static, str>">
                            "The id of the modal element, e.g. for the "<Code inline=true>"aria-controls"</Code>" of a trigger."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-modal-backdrop-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::hooks::*;

                            let state = use_overlay_trigger_state(UseOverlayTriggerStateInput::default());
                            let UseModalBackdropReturn { modal_props, .. } =
                                use_modal_backdrop(UseModalBackdropInput {
                                    state,
                                    is_dismissable: true.into(),
                                    is_keyboard_dismiss_disabled: false.into(),
                                    should_close_on_interact_outside: None,
                                    is_entering: false.into(),
                                });

                            // `<Show>` renders the modal on every opening: keep the attributes.
                            let modal_attrs = StoredValue::new(modal_props.into_attrs());

                            view! {
                                <Show when=move || state.is_open.get()>
                                    // A click on the backdrop counts as a click outside the modal.
                                    <div class="backdrop">
                                        <div {..modal_attrs.get_value()}>"Modal content"</div>
                                    </div>
                                </Show>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_modal">
                <ReactAriaSource path="overlays/useModal.tsx"/>

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
                            "Spread "<Code inline=true>"{..modal_props.into_attrs()}"</Code>" onto the modal element."
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

            <SeeAlso>
                <li><Link href=routes::doc::Modal.materialize()>"Modal overview"</Link></li>
                <li><Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link></li>
                <li><Link href=routes::doc::dialog::Hook.materialize()>"use_dialog"</Link></li>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
