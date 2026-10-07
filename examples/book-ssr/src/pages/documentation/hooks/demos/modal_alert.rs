use leptonic::{atoms::focus_scope::FocusScope, components::prelude::*, hooks::*};
use leptos::{portal::Portal, prelude::*};

/// An alert dialog: `role="alertdialog"`, named by its title and described by its message. Escape cancels; a click
/// outside does nothing, as the backdrop isn't dismissable.
#[component]
pub fn AlertDialogDemo() -> impl IntoView {
    let (deleted, set_deleted) = signal(false);

    let state = use_overlay_trigger_state(UseOverlayTriggerStateInput::default());
    let UseModalBackdropReturn { modal_props, .. } = use_modal_backdrop(UseModalBackdropInput {
        state,
        is_dismissable: Signal::stored(false),
        is_keyboard_dismiss_disabled: Signal::stored(false),
        should_close_on_interact_outside: None,
        is_entering: Signal::stored(false),
    });
    let UseModalReturn {
        modal_props: aria_modal_props,
    } = use_modal(UseModalInput::default());
    let UseDialogReturn {
        dialog_props,
        title_props,
        content_props,
    } = use_dialog(UseDialogInput {
        role: DialogRole::AlertDialog,
        ..UseDialogInput::default()
    });

    let modal_attrs = StoredValue::new(modal_props.into_attrs());
    let aria_modal_attrs = StoredValue::new(aria_modal_props.into_attrs());
    let dialog_attrs = StoredValue::new(dialog_props.into_attrs());
    let title_attrs = StoredValue::new(title_props.into_attrs());
    let content_attrs = StoredValue::new(content_props.into_attrs());

    let delete = move || {
        set_deleted.set(true);
        state.close();
    };

    view! {
        // Deleting asks for confirmation, restoring doesn't.
        <Button on_press=move |_| if deleted.get() { set_deleted.set(false) } else { state.open() }>
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
                            {..modal_attrs.get_value()}
                            {..aria_modal_attrs.get_value()}
                            {..dialog_attrs.get_value()}
                            class="demo-dialog"
                        >
                            // Names the dialog.
                            <h2 {..title_attrs.get_value()} class="demo-dialog-title">"Delete draft.txt?"</h2>
                            // Describes the alert dialog: screen readers read it when the dialog opens.
                            <p {..content_attrs.get_value()} class="demo-dialog-description">
                                "The file is deleted permanently. This can\u{2019}t be undone."
                            </p>
                            <div class="demo-dialog-actions">
                                <Button on_press=move |_| state.close() color=ButtonColor::Secondary>"Cancel"</Button>
                                <Button on_press=move |_| delete() color=ButtonColor::Danger>"Delete"</Button>
                            </div>
                        </div>
                    </FocusScope>
                </div>
            </Portal>
        </Show>
    }
}
