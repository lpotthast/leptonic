use leptonic::atoms::prelude::{Button, Dialog, DialogTitle, DialogTrigger, Popover};
use leptos::prelude::*;

#[component]
pub fn PopoverConceptDemo() -> impl IntoView {
    view! {
        <div class="demo-popover-stage">
            // Pressing the button opens and closes the popover; Escape and a press outside close it.
            <DialogTrigger>
                <Button classes="demo-btn">"Delivery"</Button>
                <Popover classes="demo-popover">
                    <Dialog>
                        <DialogTitle classes="demo-overlay-title">"Delivery"</DialogTitle>
                        <p class="demo-overlay-text">"Orders placed before 2 p.m. ship the same day."</p>
                    </Dialog>
                </Popover>
            </DialogTrigger>
        </div>
    }
}
