use leptonic::{
    atoms::{
        button::Button,
        dialog::{Dialog, DialogDescription, DialogTitle, DialogTrigger},
        modal::{ModalBackdrop, ModalContent},
    },
    hooks::DialogRole,
};
use leptos::prelude::*;

/// A dismissable modal dialog named by `aria_label` (react-aria-components' `Dialog.test.js`
/// setup), opened by a button. A second modal next to it (in the same owner) closes through a
/// button closing it. A third one opens from a `DialogTrigger` (`#test-dialog-trigger`); its
/// "Count" button (`#test-dialog-count`) only counts and must not toggle the modal, and its
/// `#test-dialog-nested-trigger` opens a modal nested in its markup ("Nested"). A fourth one
/// (`#test-dialog-open-autofocus`) opts into `auto_focus`: its first button gets the focus instead
/// of the dialog.
#[component]
pub fn PageAtomDialog() -> impl IntoView {
    let is_open = RwSignal::new(false);
    let is_other_open = RwSignal::new(false);
    let count = RwSignal::new(0u32);
    let is_animated_open = RwSignal::new(false);
    let is_autofocus_open = RwSignal::new(false);

    view! {
        <h1>"Dialog"</h1>
        <button id="test-dialog-open" on:click=move |_| is_open.set(true)>"Open"</button>
        <ModalBackdrop
            is_open=is_open
            set_open=is_open
            is_dismissable=true
        >
            <ModalContent>
                <Dialog aria_label="Settings">
                    <button>"Inside"</button>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
        // Entry and exit animations of backdrop and modal (react-aria-components' `ModalOverlay`).
        <style>
            ".test-animated-backdrop[data-entering], .test-animated-modal[data-entering] { animation: test-modal-fade 300ms; }
            .test-animated-backdrop[data-exiting], .test-animated-modal[data-exiting] { animation: test-modal-fade 300ms reverse; }
            @keyframes test-modal-fade { from { opacity: 0; } to { opacity: 1; } }"
        </style>
        <button id="test-dialog-open-animated" on:click=move |_| is_animated_open.set(true)>"Animated"</button>
        <ModalBackdrop
            is_open=is_animated_open
            set_open=is_animated_open
            is_dismissable=true
            classes="test-animated-backdrop"
        >
            <ModalContent classes="test-animated-modal">
                <Dialog aria_label="Animated">"Animated modal"</Dialog>
            </ModalContent>
        </ModalBackdrop>
        // The `Button` atom (`use_press`) as opener.
        <Button attr:id="test-dialog-open-other" on_press=move |_| is_other_open.set(true)>
            "Open other"
        </Button>
        <ModalBackdrop
            is_open=is_other_open
            set_open=is_other_open
        >
            // As crudkit's confirmation dialogs: an alert dialog with title, description and
            // `Button` atoms.
            <ModalContent>
                <Dialog role=DialogRole::AlertDialog>
                    <DialogTitle>"Other"</DialogTitle>
                    <DialogDescription>"Leave this page?"</DialogDescription>
                    <Button
                        attr:id="test-dialog-other-close"
                        on_press=move |_| is_other_open.set(false)
                    >
                        "Keep"
                    </Button>
                    <Button on_press=move |_| is_other_open.set(false)>"Leave"</Button>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
        <DialogTrigger>
            <Button attr:id="test-dialog-trigger">"Open triggered"</Button>
            <ModalBackdrop is_dismissable=true>
                <ModalContent>
                    <Dialog aria_label="Triggered">
                        <Button attr:id="test-dialog-count" on_press=move |_| count.update(|c| *c += 1)>
                            "Count " {count}
                        </Button>
                        // A modal nested in this one's markup.
                        <DialogTrigger>
                            <Button attr:id="test-dialog-nested-trigger">"Open nested"</Button>
                            <ModalBackdrop is_dismissable=true>
                                <ModalContent>
                                    <Dialog aria_label="Nested">
                                        <button id="test-dialog-nested-inside">"Inside nested"</button>
                                    </Dialog>
                                </ModalContent>
                            </ModalBackdrop>
                        </DialogTrigger>
                    </Dialog>
                </ModalContent>
            </ModalBackdrop>
        </DialogTrigger>
        <button id="test-dialog-open-autofocus" on:click=move |_| is_autofocus_open.set(true)>
            "Open auto focus"
        </button>
        <ModalBackdrop is_open=is_autofocus_open set_open=is_autofocus_open is_dismissable=true>
            <ModalContent auto_focus=true>
                <Dialog aria_label="Auto focus">
                    <button id="test-dialog-autofocus-first">"First"</button>
                    <button>"Second"</button>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
        <div>"Open: " <span id="test-dialog-is-open">{move || is_open.get().to_string()}</span></div>
    }
}
