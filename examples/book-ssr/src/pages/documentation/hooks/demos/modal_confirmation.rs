use leptonic::{
    atoms::focus_scope::FocusScope,
    hooks::*,
    utils::{classes::Classes, css::em},
};
use leptos::prelude::*;

/// Confirmation dialog demo using use_dialog_state.
/// Composes use_modal_backdrop (for dismiss behavior) + use_modal (for aria-modal)
/// + use_dialog (for ARIA semantics + focus).
#[component]
pub fn ConfirmationDialogDemo() -> impl IntoView {
    // Use dialog state with confirmation tracking
    let UseDialogStateReturn {
        is_open,
        set_open: _,
        open,
        close,
        toggle: _,
        confirm,
        is_confirmed,
    } = use_dialog_state(UseDialogStateInput::default());

    // Track the last action result
    let (last_result, set_last_result) = signal::<Option<bool>>(None);

    // Update last_result when dialog closes
    Effect::new(move || {
        if !is_open.get() && last_result.get().is_none() {
            // Dialog just closed, check confirmation status
            set_last_result.set(Some(is_confirmed.get()));
        }
    });

    // Backdrop layer - dismissable confirmation dialog
    let UseModalBackdropReturn {
        modal_props,
        backdrop_props,
        id: _,
    } = use_modal_backdrop(UseModalBackdropInput {
        is_open,
        on_close: close,
        is_dismissable: true,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
    });

    // Aria-modal layer
    let UseModalReturn {
        modal_props: aria_modal_props,
    } = use_modal(UseModalInput { is_disabled: false });

    // Dialog layer - ARIA labeling and focus on mount
    let UseDialogReturn {
        dialog_props,
        title_props,
        description_props,
        dialog_id: _,
    } = use_dialog(UseDialogInput {
        title: Some("Confirm Action".to_string()),
        description: Some("Do you want to proceed with this action?".to_string()),
        aria_label: None,
        role: DialogRole::Dialog,
    });

    let title_id = StoredValue::new(title_props.id.clone());
    let description_id = StoredValue::new(description_props.id.clone());
    let modal_attrs = StoredValue::new(modal_props.into_attrs());
    let aria_modal_attrs = StoredValue::new(aria_modal_props.into_attrs());
    let dialog_attrs = StoredValue::new(dialog_props.into_attrs());
    let backdrop_attrs = StoredValue::new(backdrop_props.into_attrs());

    view! {
        <div class=Classes::from("demo-flex-center-row")>
            <button
                on:click=move |_| {
                    set_last_result.set(None);
                    open.run(());
                }
                class=Classes::from("demo-btn-primary")
            >
                "Open Confirmation Dialog"
            </button>

            {
                let is_confirmed = Signal::derive(move || matches!(last_result.get(), Some(true)));
                view! {
                    <span class=Classes::builder()
                        .with_toggle(is_confirmed, "demo-state-active", "demo-state-inactive")
                        .build()
                    >
                        {move || match last_result.get() {
                            None => "No action taken yet".to_string(),
                            Some(true) => "Confirmed!".to_string(),
                            Some(false) => "Cancelled".to_string(),
                        }}
                    </span>
                }
            }
        </div>

        <Show when=move || is_open.get()>
            <div
                {..backdrop_attrs.get_value()}
                class=Classes::from("demo-modal-backdrop")
            >
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    <div
                        {..modal_attrs.get_value()}
                        {..aria_modal_attrs.get_value()}
                        {..dialog_attrs.get_value()}
                        class=Classes::from("demo-modal-panel")
                    >
                        <h2 id=title_id.get_value() class=Classes::from("demo-modal-title")>
                            "Confirm Action"
                        </h2>
                        <p id=description_id.get_value() class=Classes::from("demo-modal-description")>
                            "Do you want to proceed with this action? "
                            "Click Confirm to accept or Cancel to decline."
                        </p>
                        <leptonic::components::stack::Stack orientation=leptonic::components::prelude::StackOrientation::Horizontal spacing=em(0.5)>
                            <button
                                on:click=move |_| {
                                    set_last_result.set(Some(false));
                                    close.run(());
                                }
                                class=Classes::from("demo-btn")
                            >
                                "Cancel"
                            </button>
                            <button
                                on:click=move |_| {
                                    set_last_result.set(Some(true));
                                    confirm.run(());
                                }
                                class=Classes::from("demo-modal-btn-primary")
                            >
                                "Confirm"
                            </button>
                        </leptonic::components::stack::Stack>
                    </div>
                </FocusScope>
            </div>
        </Show>
    }
}
