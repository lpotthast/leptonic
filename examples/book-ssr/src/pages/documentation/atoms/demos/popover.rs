use leptonic::{
    atoms::{
        dialog::{Dialog, DialogTitle, DialogTrigger},
        popover::Popover,
    },
    components::prelude::Button,
    hooks::PlacementY,
};
use leptos::prelude::*;

#[component]
pub fn PopoverDemo() -> impl IntoView {
    view! {
        // The trigger's `Button` opens and closes the popover; Escape, a click outside and moving focus
        // out close it.
        <DialogTrigger>
            <Button>"Press me"</Button>
            <Popover placement_y=PlacementY::Above classes="demo-overlays-popover">
                <Dialog>
                    <DialogTitle classes="demo-overlays-text">"Positioned above the trigger."</DialogTitle>
                </Dialog>
            </Popover>
        </DialogTrigger>
    }
}
