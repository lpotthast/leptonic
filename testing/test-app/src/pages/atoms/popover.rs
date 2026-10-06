use leptonic::atoms::{
    button::Button,
    dialog::{Dialog, DialogTitle, DialogTrigger},
    popover::Popover,
};
use leptonic::hooks::PopoverModality;
use leptos::prelude::*;

/// Popovers opened by a `DialogTrigger` (react-aria-components' `Popover.test.js` setup): a modal one
/// with a dialog and a button inside, a non-modal one, and a button outside.
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
                <Dialog aria_label="Info">"Non-modal content"</Dialog>
            </Popover>
        </DialogTrigger>

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
