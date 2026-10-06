use leptonic::{
    atoms::{
        button::Button,
        dialog::{Dialog, DialogTitle, DialogTrigger},
        popover::Popover,
    },
    hooks::PopoverModality,
};
use leptos::prelude::*;

/// Popovers opened by a `DialogTrigger` (react-aria-components' `Popover.test.js` setup): a modal one
/// with a dialog and a button inside, a non-modal one with a dialog and two buttons, and a button
/// outside.
#[component]
pub fn PageAtomPopover() -> impl IntoView {
    let inner_presses = RwSignal::new(0_u32);

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

        <button id="test-popover-outside">"Outside"</button>
    }
}
