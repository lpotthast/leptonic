use leptonic::components::prelude::*;
use leptos::prelude::*;

/// A modal that enables its destructive action only once the user typed the repository's name.
#[component]
pub fn ModalConfirmDemo() -> impl IntoView {
    const REPOSITORY: &str = "leptonic";

    let is_open = RwSignal::new(false);
    let typed = RwSignal::new(String::new());
    let (deleted, set_deleted) = signal(false);
    let is_confirmed = Signal::derive(move || typed.with(|typed| typed == REPOSITORY));

    // Opening starts with an empty input.
    let open = move || {
        typed.set(String::new());
        is_open.set(true);
    };
    let delete = move || {
        set_deleted.set(true);
        is_open.set(false);
    };

    view! {
        <Button on_press=move |_| if deleted.get() { set_deleted.set(false) } else { open() }>
            {move || if deleted.get() { "Restore repository" } else { "Delete repository" }}
        </Button>
        <p class="demo-status">
            {move || if deleted.get() { "The repository was deleted." } else { "The repository exists." }}
        </p>

        <Modal is_open=is_open set_open=is_open>
            <ModalHeader><ModalTitle>"Delete repository?"</ModalTitle></ModalHeader>
            <ModalBody>
                <TextField label=format!("Enter \u{201c}{REPOSITORY}\u{201d} to confirm") value=typed set_value=typed/>
            </ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| is_open.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                    <Button on_press=move |_| delete() is_disabled=Signal::derive(move || !is_confirmed.get()) color=ButtonColor::Danger>
                        "Delete"
                    </Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
