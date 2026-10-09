use leptonic::{
    IntoAttrs,
    atoms::{
        button::Button,
        checkbox::{CheckboxButton, CheckboxField},
        focus_scope::FocusScope,
    },
    hooks::{
        dialog::{UseDialogInput, UseDialogReturn, use_dialog},
        modal::{UseModalBackdropInput, UseModalBackdropReturn, use_modal_backdrop},
        overlay::{UseOverlayTriggerStateInput, use_overlay_trigger_state},
    },
};
use leptos::{portal::Portal, prelude::*};

/// A modal dialog built from the hooks: the open state, the backdrop (dismissal, scroll lock, inert page),
/// the dialog semantics and a focus scope. The checkboxes choose how the user can dismiss it.
#[component]
pub fn ModalHooksDemo() -> impl IntoView {
    let close_on_outside_click = RwSignal::new(true);
    let close_on_escape = RwSignal::new(true);

    // How the modal closed last: through one of its buttons, or dismissed.
    let (closed_by, set_closed_by) = signal(None::<&'static str>);
    let button_pressed = StoredValue::new(None::<&'static str>);

    // The open state. `on_open_change` also runs when Escape or an outside click closes the modal.
    let state = use_overlay_trigger_state(UseOverlayTriggerStateInput {
        on_open_change: Some(Callback::new(move |is_open: bool| {
            if !is_open {
                let button = button_pressed.get_value();
                set_closed_by.set(Some(
                    button.unwrap_or("dismissed with Escape or an outside click"),
                ));
                button_pressed.set_value(None);
            }
        })),
        ..UseOverlayTriggerStateInput::default()
    });
    let close_with = move |button: &'static str| {
        button_pressed.set_value(Some(button));
        state.close();
    };

    // Escape, outside clicks, the scroll lock and the inert page.
    let UseModalBackdropReturn { modal_props, .. } = use_modal_backdrop(UseModalBackdropInput {
        is_dismissable: close_on_outside_click.into(),
        is_keyboard_dismiss_disabled: Signal::derive(move || !close_on_escape.get()),
        state,
        should_close_on_interact_outside: None,
        is_entering: Signal::stored(false),
    });
    // The dialog role, the name from the title, and focus when it opens.
    let UseDialogReturn {
        dialog_props,
        title_props,
        ..
    } = use_dialog(UseDialogInput::default());

    // `<Show>` renders the modal anew on every opening: keep the attributes to spread them each time.
    let modal_attrs = StoredValue::new(modal_props.into_attrs());
    let dialog_attrs = StoredValue::new(dialog_props.into_attrs());
    let title_attrs = StoredValue::new(title_props.into_attrs());

    view! {
        <Button on_press=move |_| state.open() classes="demo-btn">"Discard draft\u{2026}"</Button>
        <p class="demo-status">
            {move || match closed_by.get() {
                None => "The modal hasn\u{2019}t been closed yet.".to_owned(),
                Some(how) => format!("Last closed: {how}."),
            }}
        </p>
        <div class="demo-controls">
            <CheckboxField is_selected=close_on_outside_click set_selected=close_on_outside_click>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Close on outside click"
                </CheckboxButton>
            </CheckboxField>
            <CheckboxField is_selected=close_on_escape set_selected=close_on_escape>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Close on Escape"
                </CheckboxButton>
            </CheckboxField>
        </div>

        <Show when=move || state.is_open.get()>
            // Rendered into the document body, above the page.
            <Portal>
                <div class="demo-backdrop">
                    // Keeps focus in the dialog and returns it to the button when the dialog closes.
                    <FocusScope contain=true restore_focus=true auto_focus=true>
                        <div
                            {..modal_attrs.get_value()}
                            {..dialog_attrs.get_value()}
                            class="demo-dialog"
                        >
                            <h2 {..title_attrs.get_value()} class="demo-dialog-title">"Discard the draft?"</h2>
                            <p class="demo-dialog-description">"Your unsaved changes will be lost."</p>
                            <div class="demo-dialog-actions">
                                <Button on_press=move |_| close_with("with \u{201c}Keep editing\u{201d}") classes="demo-btn">
                                    "Keep editing"
                                </Button>
                                <Button on_press=move |_| close_with("with \u{201c}Discard\u{201d}") classes="demo-btn-danger">
                                    "Discard"
                                </Button>
                            </div>
                        </div>
                    </FocusScope>
                </div>
            </Portal>
        </Show>
    }
}
