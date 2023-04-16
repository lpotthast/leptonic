use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageModal(cx: Scope) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(cx, false);
    let (show_modal2, set_show_modal2) = create_signal(cx, false);

    view! { cx,
        <h2>"Modals"</h2>

        <Button on_click=move |_| set_show_modal.set(true)>"Show Modal"</Button>

        <Modal display_if=show_modal>
            <ModalHeader><ModalTitle>"Sure?"</ModalTitle></ModalHeader>
            <ModalBody>"This ia a test modal."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| set_show_modal.set(false) color=ButtonColor::Danger>"Accept"</Button>
                    <Button on_click=move |_| {
                        //set_show_modal.set(false);
                        set_show_modal2.set(true);
                    } color=ButtonColor::Info>"Next"</Button>
                    <Button on_click=move |_| set_show_modal.set(false)>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>

        <Modal display_if=show_modal2>
            <ModalHeader><ModalTitle>"Next one"</ModalTitle></ModalHeader>
            <ModalBody>"This overlays..."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| set_show_modal2.set(false)>"Back"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
