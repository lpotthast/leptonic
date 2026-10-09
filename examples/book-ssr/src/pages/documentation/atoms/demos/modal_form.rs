use std::collections::VecDeque;

use leptonic::{
    atoms::{
        button::Button,
        dialog::{Dialog, DialogTitle, DialogTrigger},
        field::Label,
        input::Input,
        modal::{ModalBackdrop, ModalContent},
        text_field::TextField,
    },
    hooks::button::ButtonType,
};
use leptos::{ev, prelude::*};

/// A dialog with a form, opened through a `DialogTrigger`. Focus moves to the input when it opens and back to the
/// trigger when it closes, as the focus log shows. Clicking the backdrop or pressing Escape closes it without saving.
#[component]
pub fn ModalFormDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);
    let (name, set_name) = signal(String::from("Leptonic"));
    let draft = RwSignal::new(String::new());
    let (focus_log, set_focus_log) = signal(VecDeque::<String>::new());

    // Runs for every change of the open state, also when the user dismisses the modal.
    let set_open = move |open: bool| {
        if open {
            draft.set(name.get_untracked());
        }
        is_open.set(open);
    };
    let save = move |e: ev::SubmitEvent| {
        e.prevent_default();
        let new_name = draft.get_untracked().trim().to_owned();
        if !new_name.is_empty() {
            set_name.set(new_name);
        }
        is_open.set(false);
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
            log.push_front(format!("Focus moved to {what}"));
            log.truncate(50);
        });
    };

    view! {
        <div on:focusin=log_focus>
            <DialogTrigger is_open=is_open set_open=set_open>
                <Button classes="demo-btn">"Rename project"</Button>
                <ModalBackdrop is_dismissable=true classes="demo-backdrop">
                    <ModalContent classes="demo-modal">
                        <Dialog classes="demo-dialog">
                            <DialogTitle classes="demo-dialog-title">"Rename project"</DialogTitle>
                            // The modal is rendered into the document body, so it logs its focus changes itself.
                            <form class="demo-form" on:submit=save on:focusin=log_focus>
                                <TextField value=draft set_value=draft name="project-name" classes="demo-field">
                                    <Label classes="demo-field-label">"Name"</Label>
                                    <Input classes="demo-atom-input"/>
                                </TextField>
                                <div class="demo-dialog-actions">
                                    <Button on_press=move |_| is_open.set(false) classes="demo-btn">"Cancel"</Button>
                                    <Button button_type=ButtonType::Submit classes="demo-btn-primary">"Save"</Button>
                                </div>
                            </form>
                        </Dialog>
                    </ModalContent>
                </ModalBackdrop>
            </DialogTrigger>
        </div>
        <p class="demo-status">"Project: " {name}</p>

        <pre class="demo-event-log">
            {move || {
                focus_log
                    .with(|log| {
                        if log.is_empty() {
                            "Open the dialog to see where focus moves.".to_owned()
                        } else {
                            log.iter().cloned().collect::<Vec<_>>().join("\n")
                        }
                    })
            }}
        </pre>
    }
}
