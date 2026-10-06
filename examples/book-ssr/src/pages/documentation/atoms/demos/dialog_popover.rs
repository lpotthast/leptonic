use leptonic::{
    atoms::{
        button::Button,
        dialog::{Dialog, DialogTitle, DialogTrigger},
        popover::Popover,
    },
    components::prelude::Checkbox,
};
use leptos::prelude::*;

/// A dialog in a popover, opened by a button. The trigger owns the open state; Escape or a click outside closes the
/// popover.
#[component]
pub fn DialogPopoverDemo() -> impl IntoView {
    let by_email = RwSignal::new(true);
    let by_push = RwSignal::new(false);

    view! {
        <DialogTrigger>
            <Button classes="demo-btn">"Notifications"</Button>
            <Popover classes="demo-popover">
                // Named by its title; keeps focus inside the popover while it is open.
                <Dialog classes="demo-popover-dialog">
                    <DialogTitle classes="demo-overlay-title">"Notify me by"</DialogTitle>
                    <div class="demo-control-stack">
                        <Checkbox is_selected=by_email set_selected=by_email>"Email"</Checkbox>
                        <Checkbox is_selected=by_push set_selected=by_push>"Push message"</Checkbox>
                    </div>
                </Dialog>
            </Popover>
        </DialogTrigger>
        <p class="demo-status">
            {move || match (by_email.get(), by_push.get()) {
                (true, true) => "Notifications by email and push message.",
                (true, false) => "Notifications by email.",
                (false, true) => "Notifications by push message.",
                (false, false) => "No notifications.",
            }}
        </p>
    }
}
