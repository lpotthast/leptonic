use std::time::Duration;

use leptonic::atoms::{
    button::Button,
    tooltip::{Tooltip, TooltipTrigger},
};
use leptos::prelude::*;

/// Two tooltips on `Button` atoms (react-aria-components' `Tooltip.test.js` setup), with short
/// delays, between focusable elements.
#[component]
pub fn PageAtomTooltip() -> impl IntoView {
    view! {
        <h1>"Tooltip"</h1>
        <button id="test-tooltip-before">"Before"</button>
        <div style="display: flex; gap: 4em; margin: 6em 2em;">
            <TooltipTrigger delay=Duration::from_millis(300) close_delay=Duration::from_millis(100)>
                <Button attr:id="test-tooltip-edit">"Edit"</Button>
                <Tooltip>"Edit the entry"</Tooltip>
            </TooltipTrigger>
            <TooltipTrigger delay=Duration::from_millis(300) close_delay=Duration::from_millis(100)>
                <Button attr:id="test-tooltip-delete">"Delete"</Button>
                <Tooltip>"Delete the entry"</Tooltip>
            </TooltipTrigger>
        </div>
        <p id="test-tooltip-away">"Away from the triggers"</p>
    }
}
