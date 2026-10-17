//! `use_node_aria_label` alone, for its performance test (`leptonic/tests/perf_tests/hooks/use_node_aria_label.rs`): a
//! collection of the items "Item 0", "Item 1", ... ([`SMALL`] at first, [`LARGE`] after the
//! control `large`, set while unmounted), each labelled "Label <name>", rendered keyed with
//! each item's `aria-label` from its node (its text is "Item") (`#perf-list`, one `<li>` per item, `data-perf-item` holding its key). Mounted at
//! first.
//!
//! Its controls ([`Control`]) mount and unmount it, rename the first item (`rename-first`: its
//! node changes), add an item in front (`prepend`: the other nodes move, but don't change), and
//! remove the first item (`remove-first`).

use leptonic::hooks::collections::{
    Key, SelectionManager, SelectionOptions, UseNodeAriaLabelInput, use_collection,
    use_node_aria_label,
};
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

/// The collection, with each item rendered from its node.
#[component]
fn PerfList(data: RwSignal<Vec<Item>>) -> impl IntoView {
    let collection = use_collection(move |b| {
        data.with(|data| {
            for item in data {
                let _ = b
                    .item(Key::from(item.id), item.name.clone())
                    .aria_label(format!("Label {}", item.name));
            }
        });
    });
    let selection = SelectionManager::new(collection, SelectionOptions::default());
    view! {
        <ul id="perf-list">
            <For
                each=move || collection.with(|c| c.keys().cloned().collect::<Vec<_>>())
                key=Clone::clone
                children=move |key: Key| {
                    let key_text = key.to_string();
                    let aria_label = use_node_aria_label(UseNodeAriaLabelInput { selection, key });
                    view! {
                        <li data-perf-item=key_text aria-label=move || aria_label.get()>"Item"</li>
                    }
                }
            />
        </ul>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let data = RwSignal::new(items(SMALL));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || data.set(items(LARGE))),
        Control::action("rename-first", move || {
            data.update(|data| {
                if let Some(first) = data.first_mut() {
                    first.name.push_str(" renamed");
                }
            });
        }),
        Control::action("prepend", move || {
            data.update(|data| {
                let id = data.iter().map(|item| item.id).max().map_or(0, |id| id + 1);
                data.insert(
                    0,
                    Item {
                        id,
                        name: format!("Item {id}"),
                    },
                );
            });
        }),
        Control::action("remove-first", move || {
            data.update(|data| {
                if !data.is_empty() {
                    data.remove(0);
                }
            });
        }),
    ];
    view! {
        <h1>"use_node_aria_label performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <PerfList data />
        </Show>
    }
}
