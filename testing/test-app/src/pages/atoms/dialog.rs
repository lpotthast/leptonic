use leptonic::{
    atoms::{
        button::Button,
        dialog::{Dialog, DialogDescription, DialogTitle, DialogTrigger},
        modal::{ModalBackdrop, ModalContent},
    },
    hooks::dialog::DialogRole,
};
use leptos::prelude::*;

/// A dismissable modal dialog named by `aria_label` (react-aria-components' `Dialog.test.js`
/// setup), opened by a button. A second modal next to it (in the same owner) closes through a
/// button closing it. A third one opens from a `DialogTrigger` (`#test-dialog-trigger`); its
/// "Count" button (`#test-dialog-count`) only counts and must not toggle the modal, and its
/// `#test-dialog-nested-trigger` opens a modal nested in its markup ("Nested"). A fourth one
/// (`#test-dialog-open-autofocus`) opts into `auto_focus`: its first button gets the focus instead
/// of the dialog. More modals, each opened by `#test-dialog-open-<name>` (react-aria's
/// `useDialog.test.js` setups):
/// - `described`: a dialog with a title and a description (not referenced: no alert dialog).
/// - `override`: an alert dialog with a description, described by `#test-dialog-custom-description`
///   instead.
/// - `untitled`: a dialog without title or label (warns).
/// - `labelledby`: a dialog named by `aria_labelledby` (`#test-dialog-external-title`).
/// - `shadow`: a dialog whose content focuses an input in a shadow root (`#test-dialog-shadow-host`)
///   when it mounts.
#[component]
pub fn PageAtomDialog() -> impl IntoView {
    let is_open = RwSignal::new(false);
    let is_other_open = RwSignal::new(false);
    let count = RwSignal::new(0u32);
    let is_animated_open = RwSignal::new(false);
    let is_autofocus_open = RwSignal::new(false);
    let described_open = RwSignal::new(false);
    let override_open = RwSignal::new(false);
    let untitled_open = RwSignal::new(false);
    let labelledby_open = RwSignal::new(false);
    let shadow_open = RwSignal::new(false);

    view! {
        <h1>"Dialog"</h1>
        <button id="test-dialog-open" on:click=move |_| is_open.set(true)>
            "Open"
        </button>
        <ModalBackdrop is_open=is_open set_open=is_open is_dismissable=true>
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
        <button id="test-dialog-open-animated" on:click=move |_| is_animated_open.set(true)>
            "Animated"
        </button>
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
        <Button id="test-dialog-open-other" on_press=move |_| is_other_open.set(true)>
            "Open other"
        </Button>
        <ModalBackdrop is_open=is_other_open set_open=is_other_open>
            // As crudkit's confirmation dialogs: an alert dialog with title, description and
            // `Button` atoms.
            <ModalContent>
                <Dialog role=DialogRole::AlertDialog>
                    <DialogTitle>"Other"</DialogTitle>
                    <DialogDescription>"Leave this page?"</DialogDescription>
                    <Button
                        id="test-dialog-other-close"
                        on_press=move |_| is_other_open.set(false)
                    >
                        "Keep"
                    </Button>
                    <Button on_press=move |_| is_other_open.set(false)>"Leave"</Button>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
        <DialogTrigger>
            <Button id="test-dialog-trigger">"Open triggered"</Button>
            <ModalBackdrop is_dismissable=true>
                <ModalContent>
                    <Dialog aria_label="Triggered">
                        <Button
                            id="test-dialog-count"
                            on_press=move |_| count.update(|c| *c += 1)
                        >
                            "Count "
                            {count}
                        </Button>
                        // A modal nested in this one's markup.
                        <DialogTrigger>
                            <Button id="test-dialog-nested-trigger">"Open nested"</Button>
                            <ModalBackdrop is_dismissable=true>
                                <ModalContent>
                                    <Dialog aria_label="Nested">
                                        <button id="test-dialog-nested-inside">
                                            "Inside nested"
                                        </button>
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
        <button id="test-dialog-open-described" on:click=move |_| described_open.set(true)>
            "Described"
        </button>
        <ModalBackdrop is_open=described_open set_open=described_open is_dismissable=true>
            <ModalContent>
                <Dialog>
                    <DialogTitle>"Described"</DialogTitle>
                    <DialogDescription>"A regular dialog's description."</DialogDescription>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
        <p id="test-dialog-custom-description">"A custom description."</p>
        <button id="test-dialog-open-override" on:click=move |_| override_open.set(true)>
            "Override"
        </button>
        <ModalBackdrop is_open=override_open set_open=override_open is_dismissable=true>
            <ModalContent>
                <Dialog
                    role=DialogRole::AlertDialog
                    aria_describedby="test-dialog-custom-description"
                >
                    <DialogTitle>"Override"</DialogTitle>
                    <DialogDescription>"Not the description."</DialogDescription>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
        <button id="test-dialog-open-untitled" on:click=move |_| untitled_open.set(true)>
            "Untitled"
        </button>
        <ModalBackdrop is_open=untitled_open set_open=untitled_open is_dismissable=true>
            <ModalContent>
                <Dialog>"No title"</Dialog>
            </ModalContent>
        </ModalBackdrop>
        <p id="test-dialog-external-title">"External title"</p>
        <button id="test-dialog-open-labelledby" on:click=move |_| labelledby_open.set(true)>
            "Labelledby"
        </button>
        <ModalBackdrop is_open=labelledby_open set_open=labelledby_open is_dismissable=true>
            <ModalContent>
                <Dialog aria_labelledby="test-dialog-external-title">"Named elsewhere"</Dialog>
            </ModalContent>
        </ModalBackdrop>
        <button id="test-dialog-open-shadow" on:click=move |_| shadow_open.set(true)>
            "Shadow"
        </button>
        <ModalBackdrop is_open=shadow_open set_open=shadow_open is_dismissable=true>
            <ModalContent>
                <Dialog aria_label="Shadow">
                    <ShadowInput />
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
        <div>
            "Open: " <span id="test-dialog-is-open">{move || is_open.get().to_string()}</span>
        </div>
    }
}

/// A host (`#test-dialog-shadow-host`) with an input in its open shadow root, focused once mounted
/// (before the dialog around it is focused).
#[component]
fn ShadowInput() -> impl IntoView {
    let host = NodeRef::<leptos::html::Div>::new();
    Effect::new(move |_| {
        let Some(host) = host.get() else {
            return;
        };
        let Ok(root) =
            host.attach_shadow(&web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open))
        else {
            return;
        };
        let Ok(input) = document().create_element("input") else {
            return;
        };
        let _ = input.set_attribute("aria-label", "In the shadow");
        let _ = root.append_child(&input);
        if let Ok(input) = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(input) {
            let _ = input.focus();
        }
    });
    view! { <div id="test-dialog-shadow-host" node_ref=host></div> }
}
