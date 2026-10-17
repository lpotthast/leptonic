//! `use_tooltip` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_tooltip.rs`): its props on a plain element
//! (`#perf-tooltip`, always rendered), with a `use_tooltip_trigger_state` without delays (opening
//! and closing at once, no timers), closed at first, shown in `#perf-open` (`true`/`false`),
//! between two plain buttons (`#perf-before`, `#perf-after`), mounted at first. Its controls
//! ([`Control`]) mount and unmount it, and disable and enable it.

use std::time::Duration;

use leptonic::hooks::tooltip::{
    UseTooltipInput, UseTooltipTriggerStateInput, use_tooltip, use_tooltip_trigger_state,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
    ];
    view! {
        <h1>"use_tooltip performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Subject disabled=disabled.into() />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// Calls the hook on a plain element, with its state shown.
#[component]
fn Subject(disabled: Signal<bool>) -> impl IntoView {
    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay: Duration::ZERO,
        close_delay: Duration::ZERO,
        ..UseTooltipTriggerStateInput::default()
    });
    let tooltip = use_tooltip(UseTooltipInput {
        is_disabled: disabled,
        state: Some(state),
    });
    let is_open = state.overlay.is_open;
    view! {
        <div id="perf-tooltip" {..tooltip.tooltip.attrs()}>
            "Edit the entry"
        </div>
        <span id="perf-open">{move || is_open.get().to_string()}</span>
    }
}
