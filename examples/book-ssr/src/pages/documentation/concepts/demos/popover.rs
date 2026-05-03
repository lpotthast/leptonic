use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn PopoverConceptDemo() -> impl IntoView {
    view! {
        <Popover>
            <PopoverTrigger slot>
                // Button auto-inherits toggle from PressResponder context.
                <Button on_press=|_| {}>"Click me"</Button>
            </PopoverTrigger>
            "Popover content appears here."
        </Popover>
    }
}
