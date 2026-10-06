use leptonic::atoms::prelude::{Button, Dialog, DialogTitle, DialogTrigger, Popover};
use leptos::prelude::*;

#[component]
pub fn PopoverAnimatedDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);

    view! {
        <div class="demo-popover-stage">
            // The stylesheet animates the popover while `data-entering` and `data-exiting` are set. A
            // closing popover stays rendered until its exit animation finished.
            <DialogTrigger is_open=is_open set_open=is_open>
                <Button classes="demo-btn">"Order details"</Button>
                <Popover classes=["demo-popover", "demo-popover-animated"]>
                    <Dialog>
                        <DialogTitle classes="demo-overlay-title">"Order details"</DialogTitle>
                        <p class="demo-overlay-text">"Three items, shipped in one parcel."</p>
                    </Dialog>
                </Popover>
            </DialogTrigger>
        </div>
        <p class="demo-status">{move || if is_open.get() { "Open." } else { "Closed." }}</p>
    }
}
