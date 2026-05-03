use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ModalStagedDemo() -> impl IntoView {
    let (show_staged_modal1, set_show_staged_modal1) = signal(false);
    let (show_staged_modal2, set_show_staged_modal2) = signal(false);

    view! {
        <p><Button on_press=move |_| set_show_staged_modal1.set(true)>"Show staged modal"</Button></p>

        <Modal
            show_when=show_staged_modal1
            on_close=move |()| set_show_staged_modal1.set(false)
        >
            <ModalHeader><ModalTitle>"Sure?"</ModalTitle></ModalHeader>
            <ModalBody>"This is a test modal."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| {
                        set_show_staged_modal1.set(false);
                        set_show_staged_modal2.set(true);
                    } color=ButtonColor::Info>"Next"</Button>
                    <Button on_press=move |_| set_show_staged_modal1.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>

        <Modal
            show_when=show_staged_modal2
            on_close=move |()| {
                set_show_staged_modal2.set(false);
                set_show_staged_modal1.set(true);
            }
        >
            <ModalHeader><ModalTitle>"Next one"</ModalTitle></ModalHeader>
            <ModalBody>"This overlays..."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| {
                        set_show_staged_modal2.set(false);
                        set_show_staged_modal1.set(true);
                    } color=ButtonColor::Secondary>"Back"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
