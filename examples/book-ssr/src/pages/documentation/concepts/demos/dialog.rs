use leptonic::{
    atoms::{
        dialog::{Dialog, DialogTitle, DialogTrigger},
        modal::{ModalBackdrop, ModalContent},
    },
    components::prelude::*,
};
use leptos::prelude::*;

#[component]
pub fn DialogConceptDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);
    let (accepted, set_accepted) = signal(false);

    view! {
        // The trigger opens the modal and gets `aria-expanded`; focus returns to it on close.
        <DialogTrigger is_open=is_open set_open=is_open>
            <Button>"Show terms"</Button>
            <ModalBackdrop is_dismissable=true classes="demo-backdrop">
                <ModalContent classes="demo-modal">
                    // Named by its title.
                    <Dialog classes="demo-dialog">
                        <DialogTitle classes="demo-dialog-title">"Terms of use"</DialogTitle>
                        <p class="demo-dialog-description">"Be kind, and don\u{2019}t share your password."</p>
                        <div class="demo-dialog-actions">
                            <Button on_press=move |_| is_open.set(false) color=ButtonColor::Secondary>"Close"</Button>
                            <Button on_press=move |_| { set_accepted.set(true); is_open.set(false); }>"Accept"</Button>
                        </div>
                    </Dialog>
                </ModalContent>
            </ModalBackdrop>
        </DialogTrigger>
        <p class="demo-status">
            {move || if accepted.get() { "You accepted the terms." } else { "You haven\u{2019}t accepted the terms yet." }}
        </p>
    }
}
