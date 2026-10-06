use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn PopoverConceptDemo() -> impl IntoView {
    view! {
        <div class="demo-popover-stage">
            <Popover>
                // Pressing the button opens and closes the popover.
                <PopoverTrigger slot>
                    <Button>"Delivery"</Button>
                </PopoverTrigger>
                "Orders placed before 2 p.m. ship the same day."
            </Popover>
        </div>
    }
}
