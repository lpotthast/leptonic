use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ModalStagedDemo() -> impl IntoView {
    let (show_staged_modal1, set_show_staged_modal1) = signal(false);
    let (show_staged_modal2, set_show_staged_modal2) = signal(false);

    view! {
        <p><Button on_press=move |_| set_show_staged_modal1.set(true)>"Show staged modal"</Button></p>

        <Modal
            state=(show_staged_modal1, set_show_staged_modal1)
        >
            <ModalHeader><ModalTitle>"Sure?"</ModalTitle></ModalHeader>
            <ModalBody>"Continue to the next step?"</ModalBody>
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
            // Closing the second modal goes back to the first.
            state=ValueBinding::new(
                show_staged_modal2.into(),
                Callback::new(move |open| {
                    set_show_staged_modal2.set(open);
                    if !open {
                        set_show_staged_modal1.set(true);
                    }
                }),
            )
        >
            <ModalHeader><ModalTitle>"Next one"</ModalTitle></ModalHeader>
            <ModalBody>"This modal replaced the first one. Going back reopens it."</ModalBody>
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
