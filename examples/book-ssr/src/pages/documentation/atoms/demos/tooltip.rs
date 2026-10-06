use std::time::Duration;

use leptonic::{atoms::prelude as atoms, components::prelude::Checkbox, hooks::PlacementY};
use leptos::prelude::*;

#[component]
pub fn TooltipDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    view! {
        <div class="demo-overlays-stage">
            // Any focusable atom inside a `TooltipTrigger` is its trigger.
            <atoms::TooltipTrigger delay=Duration::from_millis(600) is_disabled=disabled>
                <atoms::Button classes="demo-btn">"Edit"</atoms::Button>
                <atoms::Tooltip offset=6.0 classes="demo-overlays-tooltip">"Edit the document"</atoms::Tooltip>
            </atoms::TooltipTrigger>
            // Once a tooltip was shown, the next one opens without delay.
            <atoms::TooltipTrigger delay=Duration::from_millis(600) is_disabled=disabled>
                <atoms::Button classes="demo-btn">"Delete"</atoms::Button>
                <atoms::Tooltip placement_y=PlacementY::Below offset=6.0 classes="demo-overlays-tooltip">
                    "Delete the document"
                </atoms::Tooltip>
            </atoms::TooltipTrigger>
        </div>
        <Checkbox state=disabled>"Disable tooltips"</Checkbox>
    }
}
