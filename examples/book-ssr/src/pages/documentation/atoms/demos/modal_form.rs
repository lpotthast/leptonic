use leptonic::{
    atoms::{
        button::Button,
        dialog::{Dialog, DialogTitle},
        field::Label,
        input::Input,
        modal::{ModalBackdrop, ModalContent},
        text_field::TextField,
    },
    hooks::ButtonType,
};
use leptos::{ev, prelude::*};
use ringbuf::{
    HeapRb,
    traits::{Consumer, Observer, RingBuffer},
};

/// A dialog with a form. Focus moves to the input when it opens and back to the trigger when it closes, as the focus
/// log shows. Clicking the backdrop or pressing Escape closes it without saving.
#[component]
pub fn ModalFormDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (name, set_name) = signal(String::from("Leptonic"));
    let draft = RwSignal::new(String::new());
    let (focus_log, set_focus_log) = signal(HeapRb::<String>::new(50));

    let open = move |_| {
        draft.set(name.get_untracked());
        set_is_open.set(true);
    };
    let close = move || set_is_open.set(false);
    let save = move |e: ev::SubmitEvent| {
        e.prevent_default();
        let new_name = draft.get_untracked().trim().to_owned();
        if !new_name.is_empty() {
            set_name.set(new_name);
        }
        close();
    };

    // Logs where focus goes: the trigger, or elements inside the dialog.
    let log_focus = move |e: ev::FocusEvent| {
        let el = event_target::<web_sys::Element>(&e);
        let what = match el.tag_name().as_str() {
            "INPUT" => "the name input".to_owned(),
            "BUTTON" => format!(
                "the \u{201c}{}\u{201d} button",
                el.text_content().unwrap_or_default().trim()
            ),
            _ => return,
        };
        set_focus_log.update(|log| {
            log.push_overwrite(format!("Focus moved to {what}"));
        });
    };

    view! {
        <div class="demo-modal-atoms-row" on:focusin=log_focus>
            <Button on_press=open classes="demo-modal-atoms-btn">"Rename project"</Button>
            <span class="demo-modal-atoms-status">"Project: " <strong>{name}</strong></span>
        </div>

        <pre class="demo-modal-atoms-log">
            {move || {
                focus_log
                    .with(|log| {
                        if log.is_empty() {
                            "Open the dialog to see where focus moves.".to_owned()
                        } else {
                            log.iter().rev().cloned().collect::<Vec<_>>().join("\n")
                        }
                    })
            }}
        </pre>

        <ModalBackdrop
            state=(is_open, set_is_open)
            is_dismissable=true
            classes="demo-modal-atoms-backdrop"
        >
            <ModalContent classes="demo-modal-atoms-panel">
                <Dialog classes="demo-modal-atoms-dialog">
                    <DialogTitle classes="demo-modal-atoms-title">"Rename project"</DialogTitle>
                    // The dialog is rendered into the document body, so it logs its focus changes itself.
                    <form class="demo-modal-atoms-form" on:submit=save on:focusin=log_focus>
                        <TextField state=draft name="project-name" classes="demo-modal-atoms-field">
                            <Label classes="demo-modal-atoms-label">"Name"</Label>
                            <Input classes="demo-modal-atoms-input"/>
                        </TextField>
                        <div class="demo-modal-atoms-actions">
                            <Button on_press=move |_| close() classes="demo-modal-atoms-btn">"Cancel"</Button>
                            <Button button_type=ButtonType::Submit classes="demo-modal-btn-primary">
                                "Save"
                            </Button>
                        </div>
                    </form>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
    }
}
