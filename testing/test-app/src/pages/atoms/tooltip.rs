use std::time::Duration;

use leptonic::{
    atoms::{
        button::Button,
        tooltip::{Tooltip, TooltipTrigger},
    },
    hooks::TooltipTriggerMode,
};
use leptos::prelude::*;

/// Tooltips on `Button` atoms (react-aria-components' `Tooltip.test.js` and React Spectrum's
/// `TooltipTrigger.test.js` setups), with short delays, between focusable elements:
/// - "Edit" and "Delete": animated in and out (`.test-tooltip`, 300 ms), closing 100 ms after the
///   pointer left.
/// - "Save" (`#test-tooltip-save`): `should_close_on_press=false`, closing 800 ms after the pointer
///   left; presses count in `#test-tooltip-saves`.
/// - "Focus only" (`#test-tooltip-focus-only`): `trigger=Focus`.
/// - "Scrolled" (`#test-tooltip-scroll-trigger`) in a scrolling container
///   (`#test-tooltip-scroll-container`), without delay.
#[component]
pub fn PageAtomTooltip() -> impl IntoView {
    let saves = RwSignal::new(0_u32);
    view! {
        <h1>"Tooltip"</h1>
        <style>
            ".test-tooltip[data-entering] { animation: test-tooltip-fade 300ms; }
            .test-tooltip[data-exiting] { animation: test-tooltip-fade 300ms reverse; }
            @keyframes test-tooltip-fade { from { opacity: 0; } to { opacity: 1; } }"
        </style>
        <button id="test-tooltip-before">"Before"</button>
        <div style="display: flex; gap: 4em; margin: 6em 2em;">
            <TooltipTrigger delay=Duration::from_millis(300) close_delay=Duration::from_millis(100)>
                <Button attr:id="test-tooltip-edit">"Edit"</Button>
                <Tooltip classes="test-tooltip">"Edit the entry"</Tooltip>
            </TooltipTrigger>
            <TooltipTrigger delay=Duration::from_millis(300) close_delay=Duration::from_millis(100)>
                <Button attr:id="test-tooltip-delete">"Delete"</Button>
                <Tooltip classes="test-tooltip">"Delete the entry"</Tooltip>
            </TooltipTrigger>
            <TooltipTrigger
                delay=Duration::from_millis(300)
                close_delay=Duration::from_millis(800)
                should_close_on_press=false
            >
                <Button attr:id="test-tooltip-save" on_press=move |_| saves.update(|n| *n += 1)>
                    "Save"
                </Button>
                <Tooltip>"Save the entry"</Tooltip>
            </TooltipTrigger>
            <TooltipTrigger delay=Duration::ZERO trigger=TooltipTriggerMode::Focus>
                <Button attr:id="test-tooltip-focus-only">"Focus only"</Button>
                <Tooltip>"Shown on focus"</Tooltip>
            </TooltipTrigger>
        </div>
        <p id="test-tooltip-away">"Away from the triggers"</p>
        <p>"Saves: " <span id="test-tooltip-saves">{saves}</span></p>
        <div
            id="test-tooltip-scroll-container"
            style="overflow: scroll; height: 100px; margin: 4em 2em;"
        >
            <div style="height: 300px; padding-top: 2em;">
                <TooltipTrigger delay=Duration::ZERO>
                    <Button attr:id="test-tooltip-scroll-trigger">"Scrolled"</Button>
                    <Tooltip>"In a scrolling container"</Tooltip>
                </TooltipTrigger>
            </div>
        </div>
    }
}
