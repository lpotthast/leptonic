use std::time::Duration;

use leptonic::atoms::prelude::{Button, Tooltip, TooltipTrigger};
use leptos::prelude::*;

#[component]
pub fn TooltipConceptDemo() -> impl IntoView {
    view! {
        <div class="demo-overlays-stage">
            // The tooltip describes the button while it is hovered or focused.
            <TooltipTrigger delay=Duration::from_millis(300)>
                <Button classes="demo-btn-primary">"Hover me"</Button>
                <Tooltip offset=4.0 classes="demo-overlays-tooltip">"This is a tooltip!"</Tooltip>
            </TooltipTrigger>
        </div>
    }
}
