//! `use_toggle_group_state` (multiple selection) in a component of its own, showing whether each
//! of the keys `a`, `b`, `c` is selected as `data-selected` on `#perf-key-a`, `#perf-key-b`,
//! `#perf-key-c`, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_toggle_group_state.rs`): mounted at first, nothing
//! selected. Its controls ([`Control`]) mount and unmount it, call its methods (`toggle-a`,
//! `toggle-b`, `deselect-a`, `select-all`, `clear`) and switch `disallow_empty_selection` on
//! (`disallow-empty`).

use std::collections::HashSet;

use leptonic::{
    flag,
    hooks::{
        button::{
            ToggleGroupSelectionMode, ToggleGroupState, UseToggleGroupStateInput,
            use_toggle_group_state,
        },
        collections::Key,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disallow_empty = RwSignal::new(false);
    let state = StoredValue::new(None::<ToggleGroupState>);
    // Created once, so that the controls allocate no keys.
    let (a, b) = (Key::from("a"), Key::from("b"));
    let all: HashSet<Key> = ["a", "b", "c"].into_iter().map(Key::from).collect();
    let with_state = move |call: &dyn Fn(ToggleGroupState)| {
        if let Some(state) = state.get_value() {
            call(state);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("toggle-a", {
            let a = a.clone();
            move || with_state(&|state| state.toggle_key(&a))
        }),
        Control::action("toggle-b", move || {
            with_state(&|state| state.toggle_key(&b));
        }),
        Control::action("deselect-a", move || {
            with_state(&|state| state.set_selected(&a, false));
        }),
        Control::action("select-all", move || {
            with_state(&|state| state.set_selected_keys(all.clone()));
        }),
        Control::action("clear", move || {
            with_state(&|state| state.set_selected_keys(HashSet::new()));
        }),
        Control::action("disallow-empty", move || disallow_empty.set(true)),
    ];
    view! {
        <h1>"use_toggle_group_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Selection disallow_empty state />
        </Show>
    }
}

/// Creates the state and shows which keys are selected.
#[component]
fn Selection(
    disallow_empty: RwSignal<bool>,
    state: StoredValue<Option<ToggleGroupState>>,
) -> impl IntoView {
    let group = use_toggle_group_state(UseToggleGroupStateInput {
        selection_mode: ToggleGroupSelectionMode::Multiple,
        disallow_empty_selection: disallow_empty.into(),
        ..UseToggleGroupStateInput::default()
    });
    state.set_value(Some(group));
    let selected = move |key: &'static str| {
        let key = Key::from(key);
        flag(Signal::derive(move || group.is_selected(&key)))
    };
    view! {
        <span id="perf-key-a" data-selected=selected("a")>
            "A"
        </span>
        <span id="perf-key-b" data-selected=selected("b")>
            "B"
        </span>
        <span id="perf-key-c" data-selected=selected("c")>
            "C"
        </span>
    }
}
