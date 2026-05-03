use leptonic::{atoms::focus_scope::FocusScope, hooks::*};
use leptos::prelude::*;
use leptos_classes::Classes;

/// Non-dismissable modal demo.
/// Uses use_dialog for proper ARIA labeling and focus-on-mount.
#[component]
pub fn NonDismissableModalDemo() -> impl IntoView {
    let UseModalStateReturn {
        is_open,
        set_open: _,
        open,
        close,
        toggle: _,
    } = use_modal_state(UseModalStateInput::default());

    // Backdrop layer - non-dismissable configuration
    let UseModalBackdropReturn {
        modal_props,
        backdrop_props,
        id: _,
    } = use_modal_backdrop(UseModalBackdropInput {
        is_open,
        on_close: close,
        is_dismissable: false,
        is_keyboard_dismiss_disabled: true,
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
        title: Some("Non-Dismissable Modal".to_string()),
        description: Some(
            "This modal cannot be closed by pressing Escape or clicking outside.".to_string(),
        ),
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
        <button
            on:click=move |_| open.run(())
            class=Classes::from("demo-btn-secondary")
        >
            "Open Non-Dismissable Modal"
        </button>

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
                            "Non-Dismissable Modal"
                        </h2>
                        <p id=description_id.get_value() class=Classes::from("demo-modal-description")>
                            "This modal cannot be closed by pressing Escape or clicking outside. "
                            "You must click the button below to close it."
                        </p>
                        <button
                            on:click=move |_| close.run(())
                            class=Classes::from("demo-modal-btn-primary")
                        >
                            "I Understand"
                        </button>
                    </div>
                </FocusScope>
            </div>
        </Show>
    }
}
