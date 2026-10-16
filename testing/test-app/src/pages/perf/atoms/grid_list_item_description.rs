//! `GridListItemDescription`s in the items of a `GridList` (`[role=grid]`), for the performance
//! test of `GridListItemDescription`
//! (`leptonic/tests/perf_tests/atoms/grid_list_item_description.rs`): [`SMALL`] items ("Item
//! <i>", key `i`), each described by "Description <i>", mounted at first. Its controls
//! ([`Control`]) mount and unmount the list and set its size while unmounted (`small`, `large`:
//! [`LARGE`] items).

use leptonic::{
    atoms::grid_list::{GridList, GridListItem, GridListItemDescription},
    hooks::collections::SelectionMode,
};
use leptos::prelude::*;

use super::grid_list::{LARGE, SMALL, list_collection};
use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let items = RwSignal::new(SMALL);
    let collection = list_collection(items);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || items.set(SMALL)),
        Control::action("large", move || items.set(LARGE)),
    ];
    view! {
        <h1>"GridListItemDescription performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <GridList
                collection=collection
                selection_mode=SelectionMode::Multiple
                aria_label="Perf list"
            >
                <For each=move || 0..items.get() key=|i| *i let:i>
                    <GridListItem key=i>
                        {format!("Item {i}")}
                        <GridListItemDescription>
                            {format!("Description {i}")}
                        </GridListItemDescription>
                    </GridListItem>
                </For>
            </GridList>
        </Show>
    }
}
