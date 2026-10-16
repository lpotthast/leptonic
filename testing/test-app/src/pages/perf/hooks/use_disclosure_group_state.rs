//! `use_disclosure_group_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_disclosure_group_state.rs`): the state (single
//! expansion, nothing expanded at first), its expanded keys shown in `#perf-expanded-keys`
//! (sorted, comma-separated), mounted at first. Its controls ([`Control`]) mount and unmount it,
//! toggle the keys `1` and `2` (`toggle_key`), set both or none (`set_expanded_keys`), switch
//! between single and multiple expansion, and read how often it reported a change (`changes`).

use std::collections::HashSet;

use leptonic::hooks::{
    collections::Key,
    disclosure::{
        DisclosureGroupExpansion, DisclosureGroupState, UseDisclosureGroupStateInput,
        use_disclosure_group_state,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let expansion = RwSignal::new(DisclosureGroupExpansion::Single);
    let state = StoredValue::new(None::<DisclosureGroupState>);
    let changes = StoredValue::new(0_u32);
    let call = move |method: &dyn Fn(&DisclosureGroupState)| {
        if let Some(state) = state.get_value() {
            method(&state);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("toggle-1", move || {
            call(&|state| state.toggle_key(&Key::from(1)));
        }),
        Control::action("toggle-2", move || {
            call(&|state| state.toggle_key(&Key::from(2)));
        }),
        Control::action("set-both", move || {
            call(&|state| state.set_expanded_keys(HashSet::from([Key::from(1), Key::from(2)])));
        }),
        Control::action("set-none", move || {
            call(&|state| state.set_expanded_keys(HashSet::new()));
        }),
        Control::action("multiple", move || {
            expansion.set(DisclosureGroupExpansion::Multiple);
        }),
        Control::action("single", move || {
            expansion.set(DisclosureGroupExpansion::Single);
        }),
        Control::read("changes", move || f64::from(changes.get_value())),
    ];
    view! {
        <h1>"use_disclosure_group_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Subject state expansion=expansion.into() changes />
        </Show>
    }
}

/// Calls the hook, hands its state to the page's controls and shows its expanded keys.
#[component]
fn Subject(
    state: StoredValue<Option<DisclosureGroupState>>,
    expansion: Signal<DisclosureGroupExpansion>,
    changes: StoredValue<u32>,
) -> impl IntoView {
    let group = use_disclosure_group_state(UseDisclosureGroupStateInput {
        expansion,
        is_disabled: Signal::stored(false),
        default_expanded_keys: HashSet::new(),
        value: None,
        on_expanded_change: Some(Callback::new(move |_| {
            changes.update_value(|count| *count += 1);
        })),
    });
    state.set_value(Some(group));
    on_cleanup(move || state.set_value(None));
    let expanded_keys = group.expanded_keys;
    let shown = move || {
        let mut keys: Vec<String> =
            expanded_keys.with(|keys| keys.iter().map(ToString::to_string).collect());
        keys.sort();
        keys.join(",")
    };
    view! { <span id="perf-expanded-keys">{shown}</span> }
}
