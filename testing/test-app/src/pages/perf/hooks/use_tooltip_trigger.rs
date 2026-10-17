//! `use_tooltip_trigger` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_tooltip_trigger.rs`): its props on a plain button
//! (`#perf-trigger`), with a `use_tooltip_trigger_state` without delays (opening and closing at
//! once, no timers) and a plain tooltip (`#perf-tooltip`, rendered while open, with the hook's
//! tooltip id), between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to,
//! mounted at first, closed. Its controls ([`Control`]) mount and unmount it, and disable and
//! enable it.

use std::time::Duration;

use leptonic::hooks::tooltip::{
    TooltipTriggerMode, UseTooltipTriggerInput, UseTooltipTriggerStateInput, use_tooltip_trigger,
    use_tooltip_trigger_state,
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
        <h1>"use_tooltip_trigger performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Subject disabled=disabled.into() />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// Calls the hook on a plain button, with a plain tooltip.
#[component]
fn Subject(disabled: Signal<bool>) -> impl IntoView {
    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay: Duration::ZERO,
        close_delay: Duration::ZERO,
        ..UseTooltipTriggerStateInput::default()
    });
    let trigger = use_tooltip_trigger(UseTooltipTriggerInput {
        state,
        is_disabled: disabled,
        trigger: TooltipTriggerMode::Hover,
        should_close_on_press: Signal::stored(true),
    });
    let tooltip_part = trigger.tooltip;
    let is_open = state.overlay.is_open;
    view! {
        <button id="perf-trigger" {..trigger.trigger.attrs()}>
            "Edit"
        </button>
        <Show when=move || is_open.get()>
            <span {..tooltip_part.attrs()} class="perf-tooltip" role="tooltip">
                "Edit the entry"
            </span>
        </Show>
    }
}
