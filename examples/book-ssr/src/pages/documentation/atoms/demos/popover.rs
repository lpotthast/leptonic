use leptonic::{
    atoms::prelude::{Button, Dialog, DialogTitle, DialogTrigger, OverlayArrow, Popover},
    hooks::Placement,
};
use leptos::prelude::*;

#[component]
pub fn PopoverDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);

    view! {
        <div class="demo-popover-stage">
            // Pressing the button opens and closes the popover. Escape, a press outside and moving
            // focus out close it.
            <DialogTrigger is_open=is_open set_open=is_open>
                <Button classes="demo-btn">"Shipping"</Button>
                <Popover placement=Placement::Top classes="demo-popover">
                    <Dialog>
                        <DialogTitle classes="demo-overlay-title">"Shipping"</DialogTitle>
                        <p class="demo-overlay-text">"Orders placed before 2 p.m. ship the same day."</p>
                    </Dialog>
                    // Points at the button. The stylesheet draws a triangle and turns it with
                    // `data-placement`: the popover flips below the button when there is no room above.
                    <OverlayArrow classes="demo-popover-arrow">
                        <span class="demo-popover-arrow-shape"></span>
                    </OverlayArrow>
                </Popover>
            </DialogTrigger>
        </div>
        <p class="demo-status">
            {move || if is_open.get() { "The popover is open." } else { "The popover is closed." }}
        </p>
    }
}
