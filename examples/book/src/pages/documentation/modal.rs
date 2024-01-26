use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageModal() -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);
    let (show_modal2, set_show_modal2) = create_signal(false);
    let (show_confirm_modal, set_show_input_modal) = create_signal(false);

    view! {
        <H1>"Modals"</H1>

        <P><Button on_click=move |_| set_show_modal.set(true)>"Show staged modal"</Button></P>
        <P><Button on_click=move |_| set_show_input_modal.set(true)>"Show confirmation modal"</Button></P>

        <Modal show_when=show_modal>
            <ModalHeader><ModalTitle>"Sure?"</ModalTitle></ModalHeader>
            <ModalBody>"This ia a test modal."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| set_show_modal.set(false) color=ButtonColor::Danger>"Accept"</Button>
                    <Button on_click=move |_| set_show_modal2.set(true) color=ButtonColor::Info>"Next"</Button>
                    <Button on_click=move |_| set_show_modal.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>

        <Modal show_when=show_modal2>
            <ModalHeader><ModalTitle>"Next one"</ModalTitle></ModalHeader>
            <ModalBody>"This overlays..."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| set_show_modal2.set(false) color=ButtonColor::Secondary>"Back"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>

        <ConfirmModal
            show_when=show_confirm_modal
            requires_confirmation_of="ok".to_owned()
            on_accept=move || set_show_input_modal.set(false)
            on_cancel=move || set_show_input_modal.set(false)
        />

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r"
                --modal-color
                --modal-background-color
                --modal-padding
                --modal-font-size
                --modal-header-padding
                --modal-body-padding
                --modal-footer-padding
                --modal-border-radius
                --modal-box-shadow
            ")}
        </Code>
    }
}

#[component]
pub fn ConfirmModal<A, C>(
    #[prop(into)] show_when: Signal<bool>,
    requires_confirmation_of: String,
    on_accept: A,
    on_cancel: C,
) -> impl IntoView
where
    A: Fn() + Copy + 'static,
    C: Fn() + Copy + 'static,
{
    let (input, set_input) = create_signal(String::new());
    let required = requires_confirmation_of.clone();
    let confirmed = Signal::derive(move || input.get() == required);
    let disabled: OptionalMaybeSignal<bool> = Signal::derive(move || !confirmed.get()).into();

    let reqrequired = Callback::new(move |()| requires_confirmation_of.clone());

    view! {
        <Modal show_when=show_when>
            <ModalHeader><ModalTitle>"Delete repository?"</ModalTitle></ModalHeader>
            <ModalBody>
                "Please enter \""{reqrequired.call(())}"\" to confirm."
                <TextInput get=input set=set_input/>
            </ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| (on_accept)() disabled=disabled color=ButtonColor::Danger>"Accept"</Button>
                    <Button on_click=move |_| (on_cancel)() color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
