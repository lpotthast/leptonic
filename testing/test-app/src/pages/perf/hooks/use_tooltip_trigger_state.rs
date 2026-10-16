//! `use_tooltip_trigger_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_tooltip_trigger_state.rs`): the state with its default
//! delays (1,500 ms to open the first tooltip, 500 ms to close), closed at first, shown in
//! `#perf-open` (`true`/`false`), mounted at first. Its controls ([`Control`]) mount and unmount
//! it, open and close it right away (`TooltipTiming::Immediate`) or delayed
//! (`TooltipTiming::Delayed`), and read how often it reported a change (`changes`).

use leptonic::hooks::tooltip::{
    TooltipTiming, TooltipTriggerState, UseTooltipTriggerStateInput, use_tooltip_trigger_state,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<TooltipTriggerState>);
    let changes = StoredValue::new(0_u32);
    let call = move |method: fn(&TooltipTriggerState, TooltipTiming), timing: TooltipTiming| {
        if let Some(state) = state.get_value() {
            method(&state, timing);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || {
            call(TooltipTriggerState::open, TooltipTiming::Immediate);
        }),
        Control::action("open-delayed", move || {
            call(TooltipTriggerState::open, TooltipTiming::Delayed);
        }),
        Control::action("close", move || {
            call(TooltipTriggerState::close, TooltipTiming::Immediate);
        }),
        Control::action("close-delayed", move || {
            call(TooltipTriggerState::close, TooltipTiming::Delayed);
        }),
        Control::read("changes", move || f64::from(changes.get_value())),
    ];
    view! {
        <h1>"use_tooltip_trigger_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Subject state changes />
        </Show>
    }
}

/// Calls the hook, hands its state to the page's controls and shows it.
#[component]
fn Subject(
    state: StoredValue<Option<TooltipTriggerState>>,
    changes: StoredValue<u32>,
) -> impl IntoView {
    let tooltip = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        on_open_change: Some(Callback::new(move |_| {
            changes.update_value(|count| *count += 1);
        })),
        ..UseTooltipTriggerStateInput::default()
    });
    state.set_value(Some(tooltip));
    on_cleanup(move || state.set_value(None));
    let is_open = tooltip.overlay.is_open;
    view! { <span id="perf-open">{move || is_open.get().to_string()}</span> }
}
