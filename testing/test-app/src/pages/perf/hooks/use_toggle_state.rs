//! `use_toggle_state` (not selected at first), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_toggle_state.rs`): mounted at first, its selection
//! shown in `#perf-selection`. Its controls ([`Control`]) mount and unmount it, call its methods
//! (`toggle`, `select`: `set_selected(true)`), make it read-only, and read how often it reported
//! a change (`changes`).

use leptonic::hooks::form::{ToggleState, UseToggleStateInput, use_toggle_state};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let read_only = RwSignal::new(false);
    let changes = StoredValue::new(0_u32);
    // The mounted state, for the controls.
    let state = StoredValue::new(None::<ToggleState>);
    let call = move |method: fn(&ToggleState)| {
        move || {
            if let Some(state) = state.get_value() {
                method(&state);
            }
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("toggle", call(ToggleState::toggle)),
        Control::action("select", call(|state| state.set_selected(true))),
        Control::action("set-read-only", move || read_only.set(true)),
        Control::read("changes", move || f64::from(changes.get_value())),
    ];
    view! {
        <h1>"use_toggle_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <ShownState read_only changes state />
        </Show>
    }
}

/// Creates the state, shows its selection and hands the state to the page's controls while
/// mounted.
#[component]
fn ShownState(
    read_only: RwSignal<bool>,
    changes: StoredValue<u32>,
    state: StoredValue<Option<ToggleState>>,
) -> impl IntoView {
    let toggle = use_toggle_state(UseToggleStateInput {
        default_selected: false,
        value: None,
        on_change: Some(Callback::new(move |_: bool| {
            changes.update_value(|changes| *changes += 1);
        })),
        is_read_only: read_only.into(),
    });
    state.set_value(Some(toggle));
    on_cleanup(move || {
        state.try_update_value(|state| *state = None);
    });
    let is_selected = toggle.is_selected;
    view! {
        <p id="perf-selection">
            {move || if is_selected.get() { "selected" } else { "not selected" }}
        </p>
    }
}
