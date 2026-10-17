//! `GridListItem`s in a `GridList` (`[role=grid]`), for the performance test of `GridListItem`
//! (`leptonic/tests/perf_tests/atoms/grid_list_item.rs`): [`SMALL`] items ("Item <i>", key `i`,
//! each with a button "Edit <i>") with multiple selection, between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount the list (`mount`, `mount-with-actions`: with an action;
//! `unmount`), set its size while unmounted (`small`, `large`: [`LARGE`] items), add and remove
//! an item (mounting and unmounting a `GridListItem`), disable the second item, prepare the live
//! announcer (`prepare-announcer`: its first announcement waits for its live regions) and read
//! how many items are selected (`selected`) and how often an action ran (`actions`).

use std::{collections::HashSet, sync::Arc};

use leptonic::{
    Assertiveness, announce,
    atoms::grid_list::{GridList, GridListItem},
    hooks::collections::{Collection, CollectionMemo, Key, Selection, SelectionMode},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The items of the small list.
const SMALL: usize = 10;
/// The items of the large list.
const LARGE: usize = 1_000;

/// How many items of `selection` are selected (-1: all).
fn selected_count(selection: &Selection) -> f64 {
    match selection {
        Selection::All => -1.0,
        Selection::Keys(keys) => f64::from(u32::try_from(keys.len()).unwrap_or(u32::MAX)),
    }
}

/// `items` items ("Item <i>", key `i`).
fn list_collection(items: RwSignal<usize>) -> CollectionMemo {
    Memo::new(move |_| {
        Arc::new(Collection::build(|b| {
            for i in 0..items.get() {
                b.item(i, format!("Item {i}"));
            }
        }))
    })
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let with_actions = RwSignal::new(false);
    let items = RwSignal::new(SMALL);
    let selection = RwSignal::new(Selection::default());
    let disabled = RwSignal::new(HashSet::<Key>::new());
    let actions = StoredValue::new(0_u32);
    let collection = list_collection(items);
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
        Control::read("selected", move || selection.with_untracked(selected_count)),
        Control::read("actions", move || f64::from(actions.get_value())),
    ];
    let items_view = move || {
        view! {
            <For each=move || 0..items.get() key=|i| *i let:i>
                <GridListItem key=i>
                    {format!("Item {i}")} <button>{format!("Edit {i}")}</button>
                </GridListItem>
            </For>
        }
    };
    let list = move || {
        if with_actions.get_untracked() {
            view! {
                <GridList
                    collection=collection
                    selection_mode=SelectionMode::Multiple
                    selection=selection
                    set_selection=selection
                    disabled_keys=disabled
                    on_action=move |_| actions.update_value(|count| *count += 1)
                    aria_label="Perf list"
                >
                    {items_view()}
                </GridList>
            }
            .into_any()
        } else {
            view! {
                <GridList
                    collection=collection
                    selection_mode=SelectionMode::Multiple
                    selection=selection
                    set_selection=selection
                    disabled_keys=disabled
                    aria_label="Perf list"
                >
                    {items_view()}
                </GridList>
            }
            .into_any()
        }
    };
    view! {
        <h1>"GridListItem performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>{list}</Show>
        <button id="perf-after">"After"</button>
    }
}
