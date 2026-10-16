//! The state of a tree (`use_tree_state`) alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_tree_state.rs`): [`SMALL`] parent items ("Parent <p>",
//! key `p<p>`), each with a child item ("Child <p>", key `c<p>`), all collapsed, with multiple
//! selection, created by a mounted component (mounted at first) that shows how many items are
//! visible (`#perf-visible`). Its controls ([`Control`]) mount and unmount it, set its size while
//! unmounted (`small`, `large`: [`LARGE`] parents), expand and collapse the first parent
//! (`toggle-first`), set the expanded items (`expand-all`, `collapse-all`), select the first
//! parent and none, and read how many items are visible (`visible`).
//!
//! [`tree_collection`] also builds the trees of the fixtures of the other tree hooks.

use std::collections::HashSet;

use leptonic::hooks::{
    collections::{CollectionMemo, Key, SelectionMode, SelectionOptions, use_collection},
    tree::{TreeState, UseTreeStateInput, use_tree_state},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The parent items of the small tree (10 items).
pub const SMALL: usize = 5;
/// The parent items of the large tree (1,000 items).
pub const LARGE: usize = 500;

/// `parents` parent items ("Parent <p>", key `p<p>`), each with a child item ("Child <p>", key
/// `c<p>`).
pub fn tree_collection(parents: RwSignal<usize>) -> CollectionMemo {
    use_collection(move |b| {
        for p in 0..parents.get() {
            let _ = b
                .item(format!("p{p}"), format!("Parent {p}"))
                .children(|c| {
                    c.item(format!("c{p}"), format!("Child {p}"));
                });
        }
    })
}

/// Every parent of a tree of `parents` parents.
pub fn all_parents(parents: usize) -> HashSet<Key> {
    (0..parents).map(|p| Key::from(format!("p{p}"))).collect()
}

/// Creates the state (into `state`, while mounted) and shows it.
#[component]
fn PerfState(parents: RwSignal<usize>, state: StoredValue<Option<TreeState>>) -> impl IntoView {
    let tree = use_tree_state(UseTreeStateInput {
        collection: tree_collection(parents),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
        default_expanded_keys: HashSet::new(),
        expanded_keys: None,
        on_expanded_change: None,
    });
    state.set_value(Some(tree));
    on_cleanup(move || state.set_value(None));
    let visible = tree.list.collection;
    view! { <output id="perf-visible">{move || visible.with(|c| c.items().count())}</output> }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let parents = RwSignal::new(SMALL);
    let state = StoredValue::new(None::<TreeState>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || parents.set(SMALL)),
        Control::action("large", move || parents.set(LARGE)),
        Control::action("toggle-first", move || {
            if let Some(tree) = state.get_value() {
                tree.expansion.toggle_key(Key::from("p0"));
            }
        }),
        Control::action("expand-all", move || {
            if let Some(tree) = state.get_value() {
                tree.set_expanded_keys(all_parents(parents.get_untracked()));
            }
        }),
        Control::action("collapse-all", move || {
            if let Some(tree) = state.get_value() {
                tree.set_expanded_keys(HashSet::new());
            }
        }),
        Control::action("select-first", move || {
            if let Some(tree) = state.get_value() {
                tree.list.selection.toggle_selection(&Key::from("p0"));
            }
        }),
        Control::action("clear-selection", move || {
            if let Some(tree) = state.get_value() {
                tree.list.selection.clear_selection();
            }
        }),
        Control::read("visible", move || {
            state.get_value().map_or(0.0, |tree| {
                let visible = tree.list.collection.with_untracked(|c| c.items().count());
                f64::from(u32::try_from(visible).unwrap_or(u32::MAX))
            })
        }),
        Control::read("selected", move || {
            state.get_value().map_or(0.0, |tree| {
                let selected = untrack(|| tree.list.selection.selected_keys().len());
                f64::from(u32::try_from(selected).unwrap_or(u32::MAX))
            })
        }),
    ];
    view! {
        <h1>"use_tree_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <PerfState parents state />
        </Show>
    }
}
