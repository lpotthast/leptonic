use leptonic::{
    atoms::{button::Button, focus_scope::FocusScope},
    hooks::{
        dialog::{DialogRole, UseDialogInput, UseDialogReturn, use_dialog},
        modal::{UseModalBackdropInput, UseModalBackdropReturn, use_modal_backdrop},
        overlay::{UseOverlayTriggerStateInput, use_overlay_trigger_state},
    },
};
use leptos::{portal::Portal, prelude::*};

/// An alert dialog: `role="alertdialog"`, named by its title and described by its message. Escape cancels; a click
/// outside does nothing, as the backdrop isn't dismissable.
#[component]
pub fn AlertDialogDemo() -> impl IntoView {
    let (deleted, set_deleted) = signal(false);

    let state = use_overlay_trigger_state(UseOverlayTriggerStateInput::default());
    let UseModalBackdropReturn { modal, .. } = use_modal_backdrop(UseModalBackdropInput {
        state,
        is_dismissable: Signal::stored(false),
        is_keyboard_dismiss_disabled: Signal::stored(false),
        should_close_on_interact_outside: None,
        is_entering: Signal::stored(false),
    });
    let UseDialogReturn {
        dialog,
        title,
        content,
    } = use_dialog(UseDialogInput {
        role: DialogRole::AlertDialog,
        ..UseDialogInput::default()
    });

    // `<Show>` mounts the modal on every opening, and each mount spreads the parts again. A
    // `StoredValue` makes a part `Copy` for the nested children closures.
    let modal = StoredValue::new(modal);
    let dialog = StoredValue::new(dialog);
    let title = StoredValue::new(title);
    let content = StoredValue::new(content);

    let delete = move || {
        set_deleted.set(true);
        state.close();
    };

    view! {
        // Deleting asks for confirmation, restoring doesn't.
        <Button on_press=move |_| if deleted.get() { set_deleted.set(false) } else { state.open() } classes="demo-btn">
            {move || if deleted.get() { "Restore draft.txt" } else { "Delete draft.txt" }}
        </Button>
        <p class="demo-status">
            {move || if deleted.get() { "draft.txt was deleted." } else { "draft.txt exists." }}
        </p>

        <Show when=move || state.is_open.get()>
            <Portal>
                <div class="demo-backdrop">
                    <FocusScope contain=true restore_focus=true auto_focus=true>
                        <div
                            {..modal.with_value(|modal| modal.attrs())}
                            {..dialog.with_value(|dialog| dialog.attrs())}
                            class="demo-dialog"
                        >
                            // Names the dialog.
                            <h2 {..title.with_value(|title| title.attrs())} class="demo-dialog-title">"Delete draft.txt?"</h2>
                            // Describes the alert dialog: screen readers read it when the dialog opens.
                            <p {..content.with_value(|content| content.attrs())} class="demo-dialog-description">
                                "The file is deleted permanently. This can\u{2019}t be undone."
                            </p>
                            <div class="demo-dialog-actions">
                                <Button on_press=move |_| state.close() classes="demo-btn">"Cancel"</Button>
                                <Button on_press=move |_| delete() classes="demo-btn-danger">"Delete"</Button>
                            </div>
                        </div>
                    </FocusScope>
                </div>
            </Portal>
        </Show>
    }
}
