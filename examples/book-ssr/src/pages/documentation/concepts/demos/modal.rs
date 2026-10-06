use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ModalConceptDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);
    let (discarded, set_discarded) = signal(false);

    view! {
        <Button on_press=move |_| if discarded.get() { set_discarded.set(false) } else { is_open.set(true) }>
            {move || if discarded.get() { "Edit again" } else { "Discard changes" }}
        </Button>
        <p class="demo-status">
            {move || if discarded.get() { "Your changes were discarded." } else { "You have unsaved changes." }}
        </p>

        // Named by its `ModalTitle`. Escape and a click outside close it, like "Cancel".
        <Modal is_open=is_open set_open=is_open>
            <ModalHeader>
                <ModalTitle>"Discard changes?"</ModalTitle>
            </ModalHeader>
            <ModalBody>"Your unsaved changes will be lost."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| is_open.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                    <Button on_press=move |_| { set_discarded.set(true); is_open.set(false); } color=ButtonColor::Danger>
                        "Discard"
                    </Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
