//! `use_tab_list_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_tab_list_state.rs`): the state of [`FEW`] tabs (or
//! [`MANY`], control `use-many` while unmounted; keys `tab-1`, ...), its selected key shown in
//! `#perf-selected`, mounted at first. Its controls ([`Control`]) mount and unmount it, call its
//! methods (selecting the second, the first and the last tab), disable and enable the second tab
//! (`disabled_keys`), and remove the first tab from the collection and add it back.

use std::collections::HashSet;

use leptonic::hooks::{
    collections::{Key, use_collection},
    tabs::{TabListState, UseTabListStateInput, use_tab_list_state},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The tabs of the small tab list.
const FEW: usize = 3;
/// The tabs of the large tab list.
const MANY: usize = 1_000;

fn key(i: usize) -> Key {
    Key::from(format!("tab-{i}"))
}

/// The state, its selected key shown. Hands the state to the page's controls.
#[component]
#[allow(clippy::implicit_hasher)]
fn State(
    count: RwSignal<usize>,
    first: RwSignal<usize>,
    disabled_keys: RwSignal<HashSet<Key>>,
    state: StoredValue<Option<TabListState>>,
) -> impl IntoView {
    let collection = use_collection(move |b| {
        for i in first.get()..=count.get() {
            b.item(key(i), format!("Tab {i}"));
        }
    });
    let tabs = use_tab_list_state(UseTabListStateInput {
        collection,
        default_selected_key: None,
        selected_key: None,
        on_selection_change: None,
        disabled_keys: disabled_keys.into(),
        is_disabled: Signal::stored(false),
    });
    state.set_value(Some(tabs));
    on_cleanup(move || state.set_value(None));
    view! {
        <p>
            "Selected: "
            <span id="perf-selected">
                {move || tabs.selected_key().map(|key| key.to_string()).unwrap_or_default()}
            </span>
        </p>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let count = RwSignal::new(FEW);
    let first = RwSignal::new(1_usize);
    let disabled_keys = RwSignal::new(HashSet::<Key>::new());
    let state = StoredValue::new(None::<TabListState>);
    let select = move |i: usize| {
        if let Some(state) = state.get_value() {
            state.list.set_selected_key(Some(key(i)));
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-many", move || count.set(MANY)),
        Control::action("select-second", move || select(2)),
        Control::action("select-first", move || select(1)),
        Control::action("select-last", move || select(count.get_untracked())),
        Control::action("disable-second", move || {
            disabled_keys.set(HashSet::from([key(2)]));
        }),
        Control::action("enable-second", move || disabled_keys.set(HashSet::new())),
        Control::action("remove-first", move || first.set(2)),
        Control::action("add-first", move || first.set(1)),
    ];
    view! {
        <h1>"use_tab_list_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <State count first disabled_keys state />
        </Show>
    }
}
