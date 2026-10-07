use leptonic::{
    atoms::{
        button::Button,
        dialog::{Dialog, DialogTitle, DialogTrigger},
        popover::Popover,
    },
    hooks::PopoverModality,
    utils::i18n::{I18nProvider, Locale},
};
use leptos::prelude::*;

/// Popovers opened by a `DialogTrigger` (react-aria-components' `Popover.test.js` setup): a modal one
/// with a dialog and a button inside, a non-modal one with a dialog and two buttons, and a button
/// outside.
#[component]
pub fn PageAtomPopover() -> impl IntoView {
    let inner_presses = RwSignal::new(0_u32);
    let with_dialog = RwSignal::new(true);
    let rtl: Locale = "ar-EG".parse().expect("a valid locale");

    view! {
        <h1>"Popover"</h1>
        <DialogTrigger>
            <Button attr:id="test-popover-trigger">"Settings"</Button>
            <Popover>
                <Dialog>
                    <DialogTitle>"Settings"</DialogTitle>
                    <Button
                        attr:id="test-popover-inner"
                        on_press=move |_| inner_presses.update(|n| *n += 1)
                    >
                        "Inner"
                    </Button>
                </Dialog>
            </Popover>
        </DialogTrigger>
        <div>"Inner presses: " <span id="test-popover-inner-presses">{inner_presses}</span></div>

        <DialogTrigger>
            <Button attr:id="test-popover-non-modal-trigger">"Info"</Button>
            <Popover modality=PopoverModality::NonModal>
                // The dialog makes the non-modal popover contain focus.
                <Dialog aria_label="Info">
                    "Non-modal content"
                    <Button attr:id="test-popover-non-modal-first">"First"</Button>
                    <Button attr:id="test-popover-non-modal-second">"Second"</Button>
                </Dialog>
            </Popover>
        </DialogTrigger>

        // Entry and exit animations: the popover renders `data-entering`, then `data-exiting` while
        // it stays rendered until its exit animation ends.
        <style>
            ".test-animated-popover[data-entering] { animation: test-fade 300ms; }
            .test-animated-popover[data-exiting] { animation: test-fade 300ms reverse; }
            @keyframes test-fade { from { opacity: 0; } to { opacity: 1; } }"
        </style>
        <DialogTrigger>
            <Button attr:id="test-popover-animated-trigger">"Animated"</Button>
            <Popover classes="test-animated-popover">
                <Dialog aria_label="Animated">"Animated content"</Dialog>
            </Popover>
        </DialogTrigger>

        // A scrolling region next to the triggers (not around them).
        <div id="test-popover-adjacent-scroll" style="height: 20px; overflow: auto;">
            <div style="height: 100px;">"Scrollable"</div>
        </div>

        // No title: the trigger (without an id of its own) names the dialog.
        <DialogTrigger>
            <Button>"Untitled"</Button>
            <Popover>
                <Dialog>"Untitled content"</Dialog>
            </Popover>
        </DialogTrigger>

        // No dialog inside: the popover is the dialog.
        <DialogTrigger>
            <Button attr:id="test-popover-standalone-trigger">"Standalone"</Button>
            <Popover>"Standalone content"</Popover>
        </DialogTrigger>

        // A non-modal popover with a dialog inside only while `#test-popover-with-dialog` is
        // checked: its focus containment is per opening.
        <label>
            <input
                type="checkbox"
                id="test-popover-with-dialog"
                prop:checked=with_dialog
                on:change=move |_| with_dialog.update(|with| *with = !*with)
            />
            "With dialog"
        </label>
        <DialogTrigger>
            <Button attr:id="test-popover-toggled-trigger">"Toggled"</Button>
            <Popover modality=PopoverModality::NonModal>
                {move || {
                    if with_dialog.get_untracked() {
                        view! {
                            <Dialog aria_label="Toggled">
                                <button id="test-popover-toggled-first">"First"</button>
                                <button id="test-popover-toggled-second">"Second"</button>
                            </Dialog>
                        }
                            .into_any()
                    } else {
                        view! {
                            <button id="test-popover-toggled-first">"First"</button>
                            <button id="test-popover-toggled-second">"Second"</button>
                        }
                            .into_any()
                    }
                }}
            </Popover>
        </DialogTrigger>

        // In a right-to-left subtree: the portalled popover keeps the direction.
        <I18nProvider locale=rtl>
            <DialogTrigger>
                <Button attr:id="test-popover-rtl-trigger">"RTL"</Button>
                <Popover classes="test-popover-rtl">
                    <Dialog aria_label="RTL">"Right to left"</Dialog>
                </Popover>
            </DialogTrigger>
        </I18nProvider>

        <button id="test-popover-outside">"Outside"</button>
    }
}
