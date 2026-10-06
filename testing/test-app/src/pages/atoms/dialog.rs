use leptonic::atoms::{
    button::Button,
    dialog::{Dialog, DialogDescription, DialogTitle},
    modal::{ModalBackdrop, ModalContent},
};
use leptonic::hooks::DialogRole;
use leptos::prelude::*;

/// A dismissable modal dialog named by `aria_label` (react-aria-components' `Dialog.test.js`
/// setup), opened by a button. A second modal next to it (in the same owner) closes through a
/// button closing it.
#[component]
pub fn PageAtomDialog() -> impl IntoView {
    let is_open = RwSignal::new(false);
    let is_other_open = RwSignal::new(false);

    view! {
        <h1>"Dialog"</h1>
        <button id="test-dialog-open" on:click=move |_| is_open.set(true)>"Open"</button>
        <ModalBackdrop
            state=is_open
            is_dismissable=true
        >
            <ModalContent>
                <Dialog aria_label="Settings">
                    <button>"Inside"</button>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
        // The `Button` atom (`use_press`) as opener.
        <Button attr:id="test-dialog-open-other" on_press=move |_| is_other_open.set(true)>
            "Open other"
        </Button>
        <ModalBackdrop
            state=is_other_open
        >
            // As crudkit's confirmation dialogs: an alert dialog with title, description and
            // `Button` atoms.
            <ModalContent>
                <Dialog role=DialogRole::AlertDialog>
                    <DialogTitle>"Other"</DialogTitle>
                    <DialogDescription>"Leave this page?"</DialogDescription>
                    <Button
                        attr:id="test-dialog-other-close"
                        on_press=move |_| is_other_open.set(false)
                    >
                        "Keep"
                    </Button>
                    <Button on_press=move |_| is_other_open.set(false)>"Leave"</Button>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
        <div>"Open: " <span id="test-dialog-is-open">{move || is_open.get().to_string()}</span></div>
    }
}
