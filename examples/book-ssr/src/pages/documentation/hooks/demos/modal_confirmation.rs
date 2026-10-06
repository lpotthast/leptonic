use leptonic::{atoms::focus_scope::FocusScope, components::prelude::*, hooks::*};
use leptos::prelude::*;

/// A confirmation dialog, remembering whether it was confirmed or dismissed.
#[component]
pub fn ConfirmationDialogDemo() -> impl IntoView {
    let state = use_overlay_trigger_state(UseOverlayTriggerStateInput::default());
    let is_confirmed = RwSignal::new(false);
    let open = move || {
        is_confirmed.set(false);
        state.open();
    };
    let confirm = move || {
        is_confirmed.set(true);
        state.close();
    };

    // The outcome of the last dialog, recorded when it closes (also through Escape or an outside click).
    let (outcome, set_outcome) = signal::<Option<bool>>(None);
    Effect::new(move |was_open: Option<bool>| {
        let open = state.is_open.get();
        if was_open == Some(true) && !open {
            set_outcome.set(Some(is_confirmed.get_untracked()));
        }
        open
    });

    let UseModalBackdropReturn { modal_props, .. } = use_modal_backdrop(UseModalBackdropInput {
        is_dismissable: true,
        is_keyboard_dismiss_disabled: false,
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
        role: DialogRole::Dialog,
        ..UseDialogInput::default()
    });

    let title_attrs = StoredValue::new(title_props.into_attrs());
    let content_attrs = StoredValue::new(content_props.into_attrs());
    let modal_attrs = StoredValue::new(modal_props.into_attrs());
    let aria_modal_attrs = StoredValue::new(aria_modal_props.into_attrs());
    let dialog_attrs = StoredValue::new(dialog_props.into_attrs());

    view! {
        <div class="demo-flex-center-row">
            <Button on_press=move |_| open()>"Open Confirmation Dialog"</Button>

            <span class=move || {
                if outcome.get() == Some(true) { "demo-state-active" } else { "demo-state-inactive" }
            }>
                {move || match outcome.get() {
                    None => "No action taken yet",
                    Some(true) => "Confirmed",
                    Some(false) => "Cancelled",
                }}
            </span>
        </div>

        <Show when=move || state.is_open.get()>
            <div class="demo-modal-backdrop">
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    <div
                        {..modal_attrs.get_value()}
                        {..aria_modal_attrs.get_value()}
                        {..dialog_attrs.get_value()}
                        class="demo-overlays-dialog"
                    >
                        <h2 {..title_attrs.get_value()} class="demo-overlays-dialog-title">"Confirm Action"</h2>
                        <p {..content_attrs.get_value()} class="demo-overlays-dialog-description">
                            "Do you want to proceed with this action?"
                        </p>
                        <div class="demo-overlays-dialog-actions">
                            <Button on_press=move |_| state.close() color=ButtonColor::Secondary>"Cancel"</Button>
                            <Button on_press=move |_| confirm()>"Confirm"</Button>
                        </div>
                    </div>
                </FocusScope>
            </div>
        </Show>
    }
}
