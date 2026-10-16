//! `use_single_select_list_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_single_select_list_state.rs`): the state of a single
//! selection of the items "Item 0", "Item 1", ... ([`SMALL`] at first, [`LARGE`] after the control
//! `large`), shown in a plain list (`#perf-list`, one `<li>` per item with `data-perf-item` holding
//! its number and `aria-selected` from the state) and the selected item's text (`#perf-selected`).
//! Mounted at first (mounting creates the state). The controls ([`Control`]) select the first or
//! the last item, select the selected item again (a reported reselection), clear the selection, and
//! read how often it reported a selection (`changes`).

use std::collections::HashSet;

use leptonic::hooks::collections::{
    Key, SingleSelectListState, UseListCollectionInput, UseSingleSelectListStateInput,
    use_list_collection, use_single_select_list_state,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The items at first.
pub const SMALL: u32 = 10;
/// The items after the control `large`.
pub const LARGE: u32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let items = RwSignal::new((0..SMALL).collect::<Vec<u32>>());
    let changes = StoredValue::new(0_u32);
    // The state while mounted, for the controls.
    let state = StoredValue::new(None::<SingleSelectListState>);
    let with_state = move |run: fn(&SingleSelectListState, u32)| {
        move || {
            let last = items.with_untracked(|items| items.last().copied().unwrap_or_default());
            state.with_value(|state| {
                if let Some(state) = state {
                    run(state, last);
                }
            });
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action(
            "select-first",
            with_state(|state, _| state.set_selected_key(Some(Key::from(0_u32)))),
        ),
        Control::action(
            "select-last",
            with_state(|state, last| state.set_selected_key(Some(Key::from(last)))),
        ),
        Control::action(
            "reselect-first",
            with_state(|state, _| state.list.selection.select(&Key::from(0_u32), None)),
        ),
        Control::action("clear", with_state(|state, _| state.set_selected_key(None))),
        Control::read("changes", move || f64::from(changes.get_value())),
    ];
    view! {
        <h1>"use_single_select_list_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Items items state changes />
        </Show>
    }
}

/// The state (created while mounted), shown as a list.
#[component]
fn Items(
    items: RwSignal<Vec<u32>>,
    state: StoredValue<Option<SingleSelectListState>>,
    changes: StoredValue<u32>,
) -> impl IntoView {
    let single = use_single_select_list_state(UseSingleSelectListStateInput {
        collection: use_list_collection(UseListCollectionInput {
            items: items.into(),
            key: |i| Key::from(*i),
            text_value: |i| format!("Item {i}"),
        }),
        default_selected_key: None,
        selected_key: None,
        on_selection_change: Some(Callback::new(move |_| {
            changes.update_value(|changes| *changes += 1);
        })),
        disabled_keys: Signal::stored(HashSet::new()),
    });
    state.set_value(Some(single));
    on_cleanup(move || state.set_value(None));
    let selection = single.list.selection;
    view! {
        <p id="perf-selected">
            {move || single.selected_item().map(|node| node.text_value.to_string())}
        </p>
        <ul id="perf-list">
            <For each=move || items.get() key=|i| *i let:i>
                <li
                    data-perf-item=i
                    aria-selected=move || selection.is_selected(&Key::from(i)).to_string()
                >
                    {format!("Item {i}")}
                </li>
            </For>
        </ul>
    }
}
