use leptonic::{
    atoms::focus_scope::FocusScope,
    hooks::*,
    utils::{classes::Classes, css::em},
};
use leptos::prelude::*;

/// Basic modal demo with scroll prevention via use_modal_backdrop.
/// Uses use_dialog for proper ARIA labeling and focus-on-mount.
#[component]
pub fn BasicModalDemo() -> impl IntoView {
    // State layer
    let UseModalStateReturn {
        is_open,
        set_open: _,
        open,
        close,
        toggle: _,
    } = use_modal_state(UseModalStateInput::default());

    // Backdrop layer - dismiss behavior (Escape, outside click) + scroll prevention
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
        title: Some("Basic Modal".to_string()),
        description: Some("This modal has scroll prevention enabled.".to_string()),
        aria_label: None,
        role: DialogRole::Dialog,
    });

    // Convert Props to Attrs before storing in StoredValue (Props is non-Clone, Attrs is Clone)
    let title_id = StoredValue::new(title_props.id.clone());
    let description_id = StoredValue::new(description_props.id.clone());
    let modal_attrs = StoredValue::new(modal_props.into_attrs());
    let aria_modal_attrs = StoredValue::new(aria_modal_props.into_attrs());
    let dialog_attrs = StoredValue::new(dialog_props.into_attrs());
    let backdrop_attrs = StoredValue::new(backdrop_props.into_attrs());

    view! {
        <button
            on:click=move |_| open.run(())
            class=Classes::from("demo-btn-primary")
        >
            "Open Modal"
        </button>

        <Show when=move || is_open.get()>
            // Backdrop with backdrop_props (pointerdown Firefox fix)
            <div
                {..backdrop_attrs.get_value()}
                class=Classes::from("demo-modal-backdrop")
            >
                // FocusScope traps focus within the modal and restores it on close
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    // Modal - spread modal_props (overlay behavior) + aria_modal_props + dialog_props
                    <div
                        {..modal_attrs.get_value()}
                        {..aria_modal_attrs.get_value()}
                        {..dialog_attrs.get_value()}
                        class=Classes::from("demo-modal-panel")
                    >
                        <h2 id=title_id.get_value() class=Classes::from("demo-modal-title")>
                            "Basic Modal"
                        </h2>
                        <p id=description_id.get_value() class=Classes::from("demo-modal-description")>
                            "This modal has scroll prevention enabled. Try scrolling the page - it won't work! "
                            "Press Escape or click outside to close."
                        </p>
                        <leptonic::components::stack::Stack orientation=leptonic::components::prelude::StackOrientation::Horizontal spacing=em(0.5)>
                            <button
                                on:click=move |_| close.run(())
                                class=Classes::from("demo-btn")
                            >
                                "Cancel"
                            </button>
                            <button
                                on:click=move |_| close.run(())
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
