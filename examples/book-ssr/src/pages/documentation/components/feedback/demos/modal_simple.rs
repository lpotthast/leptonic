use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ModalSimpleDemo() -> impl IntoView {
    let (show_simple_modal, set_show_simple_modal) = signal(false);

    view! {
        <p><Button on_press=move |_| set_show_simple_modal.set(true)>"Show simple modal"</Button></p>

        <Modal
            show_when=show_simple_modal
            on_close=move |()| set_show_simple_modal.set(false)
        >
            <ModalHeader><ModalTitle>"Hello"</ModalTitle></ModalHeader>
            <ModalBody>"This is a simple modal."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| set_show_simple_modal.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
