//! `use_disclosure_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_disclosure_state.rs`): the state (collapsed at first)
//! shown in `#perf-expanded` (`true`/`false`), mounted at first. Its controls ([`Control`]) mount
//! and unmount it, call its methods (`expand`, `collapse`, `toggle`), and read how often it
//! reported a change (`changes`).

use leptonic::hooks::disclosure::{DisclosureState, UseDisclosureStateInput, use_disclosure_state};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<DisclosureState>);
    let changes = StoredValue::new(0_u32);
    let call = move |method: fn(&DisclosureState)| {
        if let Some(state) = state.get_value() {
            method(&state);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("expand", move || call(DisclosureState::expand)),
        Control::action("collapse", move || call(DisclosureState::collapse)),
        Control::action("toggle", move || call(DisclosureState::toggle)),
        Control::read("changes", move || f64::from(changes.get_value())),
    ];
    view! {
        <h1>"use_disclosure_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Subject state changes />
        </Show>
    }
}

/// Calls the hook, hands its state to the page's controls and shows it.
#[component]
fn Subject(
    state: StoredValue<Option<DisclosureState>>,
    changes: StoredValue<u32>,
) -> impl IntoView {
    let disclosure = use_disclosure_state(UseDisclosureStateInput {
        default_expanded: false,
        value: None,
        on_expanded_change: Some(Callback::new(move |_| {
            changes.update_value(|count| *count += 1);
        })),
    });
    state.set_value(Some(disclosure));
    on_cleanup(move || state.set_value(None));
    let is_expanded = disclosure.is_expanded;
    view! { <span id="perf-expanded">{move || is_expanded.get().to_string()}</span> }
}
