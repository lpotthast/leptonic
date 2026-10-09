use leptonic::{
    atoms::{
        button::Button,
        dialog::{Dialog, DialogDescription, DialogTitle},
        modal::{ModalBackdrop, ModalContent},
    },
    hooks::dialog::DialogRole,
};
use leptos::prelude::*;

/// A confirmation before a destructive action, as an alert dialog. Clicking the backdrop doesn't close it: the user
/// has to choose (Escape still cancels).
#[component]
pub fn ModalAlertDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);
    let (deleted, set_deleted) = signal(false);

    let delete = move || {
        set_deleted.set(true);
        is_open.set(false);
    };

    view! {
        // Deleting asks for confirmation, restoring doesn't. Focus returns to this button when the dialog closes.
        <Button
            on_press=move |_| if deleted.get() { set_deleted.set(false) } else { is_open.set(true) }
            classes="demo-btn"
        >
            {move || if deleted.get() { "Restore draft.txt" } else { "Delete draft.txt" }}
        </Button>
        <p class="demo-status">
            {move || if deleted.get() { "draft.txt was deleted." } else { "draft.txt exists." }}
        </p>

        <ModalBackdrop is_open=is_open set_open=is_open classes="demo-backdrop">
            <ModalContent classes="demo-modal">
                // Named by its `DialogTitle` and, as an alert dialog, described by its `DialogDescription`.
                <Dialog role=DialogRole::AlertDialog classes="demo-dialog">
                    <DialogTitle classes="demo-dialog-title">"Delete draft.txt?"</DialogTitle>
                    <DialogDescription classes="demo-dialog-description">
                        "The file is deleted permanently. This can\u{2019}t be undone."
                    </DialogDescription>
                    <div class="demo-dialog-actions">
                        <Button on_press=move |_| is_open.set(false) classes="demo-btn">"Cancel"</Button>
                        <Button on_press=move |_| delete() classes="demo-btn-danger">"Delete"</Button>
                    </div>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
    }
}
