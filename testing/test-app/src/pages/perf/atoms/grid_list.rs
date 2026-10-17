//! The `GridList` alone (`[role=grid]`, labelled "Perf list"), for its performance test
//! (`leptonic/tests/perf_tests/atoms/grid_list.rs`): [`SMALL`] items ("Item <i>", key `i`) with
//! multiple selection, between two plain buttons (`#perf-before`, `#perf-after`) to tab from and
//! to, mounted at first. Its controls ([`Control`]) mount and unmount it, set its size (`small`,
//! `large`: [`LARGE`] items, while unmounted; `empty`: none), add and remove an item, change its
//! controlled selection and its reactive props (selection mode, disabled items, orientation),
//! prepare the live announcer (`prepare-announcer`: its first announcement waits for its live
//! regions) and read how many items are selected (`selected`, -1 for all).

use std::{collections::HashSet, sync::Arc};

use leptonic::{
    Assertiveness, Orientation, announce,
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
    let items = RwSignal::new(SMALL);
    let selection = RwSignal::new(Selection::default());
    let selection_mode = RwSignal::new(SelectionMode::Multiple);
    let disabled = RwSignal::new(HashSet::<Key>::new());
    let orientation = RwSignal::new(Orientation::Vertical);
    let collection = list_collection(items);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || items.set(SMALL)),
        Control::action("large", move || items.set(LARGE)),
        Control::action("empty", move || items.set(0)),
        Control::action("add-item", move || items.update(|items| *items += 1)),
        Control::action("remove-item", move || items.update(|items| *items -= 1)),
        Control::action("select-first", move || {
            selection.set(Selection::keys([Key::from(0_usize)]));
        }),
        Control::action("select-all", move || selection.set(Selection::All)),
        Control::action("clear-selection", move || {
            selection.set(Selection::default());
        }),
        Control::action("selection-none", move || {
            selection_mode.set(SelectionMode::None);
        }),
        Control::action("selection-multiple", move || {
            selection_mode.set(SelectionMode::Multiple);
        }),
        Control::action("disable-first", move || {
            disabled.set(HashSet::from([Key::from(0_usize)]));
        }),
        Control::action("enable-all", move || disabled.set(HashSet::new())),
        Control::action("horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::action("vertical", move || orientation.set(Orientation::Vertical)),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("selected", move || selection.with_untracked(selected_count)),
    ];
    view! {
        <h1>"GridList performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <GridList
                collection=collection
                selection_mode=selection_mode
                selection=selection
                set_selection=selection
                disabled_keys=disabled
                orientation=orientation
                aria_label="Perf list"
            >
                <For each=move || 0..items.get() key=|i| *i let:i>
                    <GridListItem key=i>{format!("Item {i}")}</GridListItem>
                </For>
            </GridList>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
