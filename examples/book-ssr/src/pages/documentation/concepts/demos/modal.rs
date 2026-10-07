use leptonic::atoms::{
    button::Button,
    dialog::{Dialog, DialogTitle},
    modal::{ModalBackdrop, ModalContent},
};
use leptos::prelude::*;

#[component]
pub fn ModalConceptDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);
    let (discarded, set_discarded) = signal(false);

    view! {
        <Button
            on_press=move |_| if discarded.get() { set_discarded.set(false) } else { is_open.set(true) }
            classes="demo-btn"
        >
            {move || if discarded.get() { "Edit again" } else { "Discard changes" }}
        </Button>
        <p class="demo-status">
            {move || if discarded.get() { "Your changes were discarded." } else { "You have unsaved changes." }}
        </p>

        // Escape and a click outside close it, like "Cancel".
        <ModalBackdrop is_open=is_open set_open=is_open is_dismissable=true classes="demo-backdrop">
            <ModalContent classes="demo-modal">
                // Named by its title.
                <Dialog classes="demo-dialog">
                    <DialogTitle classes="demo-dialog-title">"Discard changes?"</DialogTitle>
                    <p class="demo-dialog-description">"Your unsaved changes will be lost."</p>
                    <div class="demo-dialog-actions">
                        <Button on_press=move |_| is_open.set(false) classes="demo-btn">"Cancel"</Button>
                        <Button
                            on_press=move |_| {
                                set_discarded.set(true);
                                is_open.set(false);
                            }
                            classes="demo-btn-danger"
                        >
                            "Discard"
                        </Button>
                    </div>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
    }
}
