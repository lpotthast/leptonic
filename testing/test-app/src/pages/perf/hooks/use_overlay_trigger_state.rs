//! `use_overlay_trigger_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_overlay_trigger_state.rs`): the state (closed at first)
//! shown in `#perf-open` (`true`/`false`), mounted at first. Its controls ([`Control`]) mount and
//! unmount it, call its methods (`open`, `close`, `toggle`), and read how often it reported a
//! change (`changes`).

use leptonic::hooks::overlay::{
    OverlayTriggerState, UseOverlayTriggerStateInput, use_overlay_trigger_state,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<OverlayTriggerState>);
    let changes = StoredValue::new(0_u32);
    let call = move |method: fn(&OverlayTriggerState)| {
        if let Some(state) = state.get_value() {
            method(&state);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || call(OverlayTriggerState::open)),
        Control::action("close", move || call(OverlayTriggerState::close)),
        Control::action("toggle", move || call(OverlayTriggerState::toggle)),
        Control::read("changes", move || f64::from(changes.get_value())),
    ];
    view! {
        <h1>"use_overlay_trigger_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Subject state changes />
        </Show>
    }
}

/// Calls the hook, hands its state to the page's controls and shows it.
#[component]
fn Subject(
    state: StoredValue<Option<OverlayTriggerState>>,
    changes: StoredValue<u32>,
) -> impl IntoView {
    let overlay = use_overlay_trigger_state(UseOverlayTriggerStateInput {
        default_open: false,
        value: None,
        on_open_change: Some(Callback::new(move |_| {
            changes.update_value(|count| *count += 1);
        })),
    });
    state.set_value(Some(overlay));
    on_cleanup(move || state.set_value(None));
    let is_open = overlay.is_open;
    view! { <span id="perf-open">{move || is_open.get().to_string()}</span> }
}
