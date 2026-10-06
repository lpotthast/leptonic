use leptonic::{components::prelude::*, hooks::PlacementY};
use leptos::prelude::*;

#[component]
pub fn PopoverConceptDemo() -> impl IntoView {
    view! {
        <Popover placement_y=PlacementY::Below>
            <PopoverTrigger slot>
                // The popover toggles itself when this button is pressed; `on_press` is for your own logic.
                <Button on_press=|_| {}>"Click me"</Button>
            </PopoverTrigger>
            "Popover content appears here."
        </Popover>
    }
}
