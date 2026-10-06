use leptonic::{atoms::focus_scope::FocusScope, components::prelude::*, hooks::*};
use leptos::prelude::*;

/// A complete modal dialog: state, backdrop (dismissal and scroll prevention), `aria-modal`, dialog semantics and a
/// focus scope.
#[component]
pub fn BasicModalDemo() -> impl IntoView {
    // Whether the modal is open.
    let state = use_overlay_trigger_state(UseOverlayTriggerStateInput::default());

    // Escape, outside clicks and scroll prevention.
    let UseModalBackdropReturn { modal_props, .. } = use_modal_backdrop(UseModalBackdropInput {
        is_dismissable: true,
        is_keyboard_dismiss_disabled: false,
        ..UseModalBackdropInput::new(state)
    });

    // `aria-modal="true"`.
    let UseModalReturn {
        modal_props: aria_modal_props,
    } = use_modal(UseModalInput::default());

    // Role, labelling and focus on mount.
    let UseDialogReturn {
        dialog_props,
        title_props,
        content_props,
        ..
    } = use_dialog(UseDialogInput {
        role: DialogRole::Dialog,
        ..UseDialogInput::default()
    });

    // `<Show>` may render its children more than once. Props are not `Clone`, their attributes are.
    let title_attrs = StoredValue::new(title_props.into_attrs());
    let content_attrs = StoredValue::new(content_props.into_attrs());
    let modal_attrs = StoredValue::new(modal_props.into_attrs());
    let aria_modal_attrs = StoredValue::new(aria_modal_props.into_attrs());
    let dialog_attrs = StoredValue::new(dialog_props.into_attrs());

    view! {
        <Button on_press=move |_| state.open()>"Open Modal"</Button>

        <Show when=move || state.is_open.get()>
            <div class="demo-modal-backdrop">
                // Traps focus in the dialog and restores it when the dialog closes.
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    <div
                        {..modal_attrs.get_value()}
                        {..aria_modal_attrs.get_value()}
                        {..dialog_attrs.get_value()}
                        class="demo-overlays-dialog"
                    >
                        <h2 {..title_attrs.get_value()} class="demo-overlays-dialog-title">"Basic Modal"</h2>
                        <p {..content_attrs.get_value()} class="demo-overlays-dialog-description">
                            "Page scrolling is prevented while this modal is open. "
                            "Press Escape or click outside to close it."
                        </p>
                        <div class="demo-overlays-dialog-actions">
                            <Button on_press=move |_| state.close() color=ButtonColor::Secondary>"Cancel"</Button>
                            <Button on_press=move |_| state.close()>"Confirm"</Button>
                        </div>
                    </div>
                </FocusScope>
            </div>
        </Show>
    }
}
