use std::time::Duration;

use leptonic::atoms::{
    button::Button,
    tooltip::{Tooltip, TooltipTrigger},
};
use leptos::prelude::*;

#[component]
pub fn TooltipConceptDemo() -> impl IntoView {
    view! {
        <div class="demo-tooltip-stage">
            // The tooltip describes the button while it is hovered or focused. Hovering opens it after
            // half a second (default: 1.5 s).
            <TooltipTrigger delay=Duration::from_millis(500)>
                <Button classes="demo-btn">"Publish"</Button>
                <Tooltip offset=6.0 classes="demo-tooltip">"Make the post visible to everyone"</Tooltip>
            </TooltipTrigger>
        </div>
    }
}
