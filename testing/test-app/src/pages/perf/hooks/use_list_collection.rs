//! `use_list_collection` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_list_collection.rs`): a collection of the items "Item 0",
//! "Item 1", ... built from a list of values ([`SMALL`] items at first, [`LARGE`] after the control `large`),
//! shown in a plain list (`#perf-list`, one `<li>` per collection item, `data-perf-item` holding
//! its key, the text its text value). Mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount it (building the collection) and change the values:
//! an item appended and removed, an item renamed, the order reversed, and data the collection
//! doesn't depend on (`touch`: the build runs again, the collection stays the same).

use leptonic::hooks::collections::{Key, UseListCollectionInput, use_list_collection};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The items at first.
const SMALL: u32 = 10;
/// The items after the control `large`.
const LARGE: u32 = 1_000;

/// An item of the data.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
    id: u32,
    name: String,
}

fn items(count: u32) -> Vec<Item> {
    (0..count)
        .map(|id| Item {
            id,
            name: format!("Item {id}"),
        })
        .collect()
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let data = RwSignal::new(items(SMALL));
    // Read by the build, without changing the collection.
    let touched = RwSignal::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || data.set(items(LARGE))),
        Control::action("append", move || {
            data.update(|data| {
                let id = data.iter().map(|item| item.id).max().map_or(0, |id| id + 1);
                data.push(Item {
                    id,
                    name: format!("Item {id}"),
                });
            });
        }),
        Control::action("remove-last", move || {
            data.update(|data| {
                data.pop();
            });
        }),
        Control::action("rename-first", move || {
            data.update(|data| "Renamed 0".clone_into(&mut data[0].name));
        }),
        Control::action("restore-first", move || {
            data.update(|data| "Item 0".clone_into(&mut data[0].name));
        }),
        Control::action("reverse", move || data.update(|data| data.reverse())),
        Control::action("touch", move || touched.update(|touched| *touched += 1)),
    ];
    view! {
        <h1>"use_list_collection performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Items data touched />
        </Show>
    }
}

/// The collection, built while mounted, shown as a list.
#[component]
fn Items(data: RwSignal<Vec<Item>>, touched: RwSignal<u32>) -> impl IntoView {
    let items = Signal::derive(move || {
        touched.track();
        data.get()
    });
    let collection = use_list_collection(UseListCollectionInput {
        items,
        key: |item: &Item| Key::from(item.id),
        text_value: |item: &Item| item.name.clone(),
    });
    let entries = move || {
        collection.with(|c| {
            c.items()
                .map(|node| (node.key.to_string(), node.text_value.to_string()))
                .collect::<Vec<_>>()
        })
    };
    view! {
        <ul id="perf-list">
            <For each=entries key=Clone::clone let:entry>
                <li data-perf-item=entry.0>{entry.1}</li>
            </For>
        </ul>
    }
}
