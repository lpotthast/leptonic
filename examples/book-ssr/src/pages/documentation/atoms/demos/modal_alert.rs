use leptonic::{
    atoms::{
        button::Button,
        dialog::{Dialog, DialogDescription, DialogTitle},
        modal::{ModalBackdrop, ModalContent},
    },
    hooks::DialogRole,
};
use leptos::prelude::*;

/// A confirmation before a destructive action, as an alert dialog. Clicking the backdrop doesn't close it: the user
/// has to choose (Escape still cancels).
#[component]
pub fn ModalAlertDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (deleted, set_deleted) = signal(false);

    let delete = move || {
        set_deleted.set(true);
        set_is_open.set(false);
    };

    view! {
        <div class="demo-modal-atoms-row">
            // Deleting asks for confirmation, restoring doesn't. Focus returns to this button when the dialog closes.
            <Button
                on_press=move |_| if deleted.get() { set_deleted.set(false) } else { set_is_open.set(true) }
                classes="demo-modal-atoms-btn"
            >
                {move || if deleted.get() { "Restore draft.txt" } else { "Delete draft.txt" }}
            </Button>
            <span class="demo-modal-atoms-status">
                {move || if deleted.get() { "draft.txt was deleted." } else { "draft.txt exists." }}
            </span>
        </div>

        <ModalBackdrop state=(is_open, set_is_open) classes="demo-modal-atoms-backdrop">
            <ModalContent classes="demo-modal-atoms-panel">
                // The dialog is named by its `DialogTitle` and, as an alert dialog, described by its
                // `DialogDescription`.
                <Dialog role=DialogRole::AlertDialog classes="demo-modal-atoms-dialog">
                    <DialogTitle classes="demo-modal-atoms-title">"Delete draft.txt?"</DialogTitle>
                    <DialogDescription classes="demo-modal-atoms-description">
                        "The file is deleted permanently. This can\u{2019}t be undone."
                    </DialogDescription>
                    <div class="demo-modal-atoms-actions">
                        <Button on_press=move |_| set_is_open.set(false) classes="demo-modal-atoms-btn">"Cancel"</Button>
                        <Button on_press=move |_| delete() classes="demo-modal-btn-danger">
                            "Delete"
                        </Button>
                    </div>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
    }
}
