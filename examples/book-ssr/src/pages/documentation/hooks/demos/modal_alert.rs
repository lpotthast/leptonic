use leptonic::{atoms::focus_scope::FocusScope, components::prelude::*, hooks::*};
use leptos::prelude::*;

/// An alert dialog: `role="alertdialog"`, and it only closes through its buttons.
#[component]
pub fn AlertDialogDemo() -> impl IntoView {
    // Whether the modal is open.
    let state = use_overlay_trigger_state(UseOverlayTriggerStateInput::default());

    // Neither outside clicks nor Escape close an alert dialog.
    let UseModalBackdropReturn { modal_props, .. } = use_modal_backdrop(UseModalBackdropInput {
        is_dismissable: false,
        is_keyboard_dismiss_disabled: true,
        ..UseModalBackdropInput::new(state)
    });

    let UseModalReturn {
        modal_props: aria_modal_props,
    } = use_modal(UseModalInput::default());

    let UseDialogReturn {
        dialog_props,
        title_props,
        content_props,
        ..
    } = use_dialog(UseDialogInput {
        role: DialogRole::AlertDialog,
        ..UseDialogInput::default()
    });

    let title_attrs = StoredValue::new(title_props.into_attrs());
    let content_attrs = StoredValue::new(content_props.into_attrs());
    let modal_attrs = StoredValue::new(modal_props.into_attrs());
    let aria_modal_attrs = StoredValue::new(aria_modal_props.into_attrs());
    let dialog_attrs = StoredValue::new(dialog_props.into_attrs());

    view! {
        <Button on_press=move |_| state.open() color=ButtonColor::Danger>"Delete Item"</Button>

        <Show when=move || state.is_open.get()>
            <div class="demo-modal-backdrop">
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    <div
                        {..modal_attrs.get_value()}
                        {..aria_modal_attrs.get_value()}
                        {..dialog_attrs.get_value()}
                        class="demo-overlays-dialog"
                    >
                        <h2 {..title_attrs.get_value()} class="demo-overlays-dialog-title demo-overlays-dialog-title-danger">
                            "Delete Item"
                        </h2>
                        <p {..content_attrs.get_value()} class="demo-overlays-dialog-description">
                            "This action cannot be undone. Are you sure you want to delete this item?"
                        </p>
                        <div class="demo-overlays-dialog-actions">
                            <Button on_press=move |_| state.close() color=ButtonColor::Secondary>"Cancel"</Button>
                            <Button on_press=move |_| state.close() color=ButtonColor::Danger>"Delete"</Button>
                        </div>
                    </div>
                </FocusScope>
            </div>
        </Show>
    }
}
