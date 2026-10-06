use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ModalConceptDemo() -> impl IntoView {
    let (show_modal, set_show_modal) = signal(false);

    view! {
        <Button on_press=move |_| set_show_modal.set(true)>"Open Modal"</Button>

        // The modal is named by its `ModalTitle`.
        <Modal state=(show_modal, set_show_modal)>
            <ModalHeader>
                <ModalTitle>"Confirm"</ModalTitle>
            </ModalHeader>
            <ModalBody>"Are you sure?"</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| set_show_modal.set(false) color=ButtonColor::Secondary>
                        "Close"
                    </Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
