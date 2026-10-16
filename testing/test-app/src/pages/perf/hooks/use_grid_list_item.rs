//! Hook items (`use_grid_list_item` on plain elements) in a hook grid list, for the performance
//! test of `use_grid_list_item` (`leptonic/tests/perf_tests/hooks/use_grid_list_item.rs`): the
//! list of the `use_grid_list` fixture ([`super::use_grid_list`]: [`SMALL`] items "Item <i>",
//! each with a button "Edit <i>"), with multiple selection, between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount the list (`mount`, `mount-with-actions`: with an action;
//! `unmount`), set its size while unmounted (`small`, `large`: [`LARGE`] items), add and remove
//! an item (mounting and unmounting a `use_grid_list_item`), disable the second item, prepare the
//! live announcer (`prepare-announcer`: its first announcement waits for its live regions) and
//! read how many items are selected (`selected`) and how often an action ran (`actions`).

use std::collections::HashSet;

use leptonic::{
    Assertiveness, announce,
    hooks::collections::{Key, SelectionMode, SelectionOptions, UseListStateInput, use_list_state},
};
use leptos::prelude::*;

use super::use_grid_list::{HookGridList, LARGE, SMALL, list_collection, selected_count};
use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let with_actions = RwSignal::new(false);
    let items = RwSignal::new(SMALL);
    let disabled = RwSignal::new(HashSet::<Key>::new());
    let actions = StoredValue::new(0_u32);
    let state = use_list_state(UseListStateInput {
        collection: list_collection(items),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            disabled_keys: disabled.into(),
            ..SelectionOptions::default()
        },
    });
    let controls = vec![
        Control::action("mount", move || {
            with_actions.set(false);
            mounted.set(true);
        }),
        Control::action("mount-with-actions", move || {
            with_actions.set(true);
            mounted.set(true);
        }),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || items.set(SMALL)),
        Control::action("large", move || items.set(LARGE)),
        Control::action("add-item", move || items.update(|items| *items += 1)),
        Control::action("remove-item", move || items.update(|items| *items -= 1)),
        Control::action("disable-second", move || {
            disabled.set(HashSet::from([Key::from(1_usize)]));
        }),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("selected", move || {
            selected_count(&untrack(|| state.selection.raw_selection()))
        }),
        Control::read("actions", move || f64::from(actions.get_value())),
    ];
    let on_action = Callback::new(move |_: Key| actions.update_value(|count| *count += 1));
    view! {
        <h1>"use_grid_list_item performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookGridList
                state
                items
                on_action=with_actions.get_untracked().then_some(on_action)
            />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
