//! The expansion state of a tree (`use_tree_expansion`) alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_tree_expansion.rs`): no item expanded, read by a mounted
//! component (mounted at first) that shows each of [`SMALL`] items' expansion
//! (`#perf-item-<i>`, key `i<i>`: "expanded" or "collapsed") and how many items are expanded
//! (`#perf-expanded`). Its controls ([`Control`]) mount and unmount it, set its size while
//! unmounted (`small`, `large`: [`LARGE`] items), expand and collapse the first item
//! (`toggle-first`), and set the expanded items (`expand-all`, `collapse-all`).

use std::collections::HashSet;

use leptonic::hooks::{
    collections::Key,
    tree::{TreeExpansion, UseTreeExpansionInput, use_tree_expansion},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The items of the small tree.
const SMALL: usize = 10;
/// The items of the large tree.
const LARGE: usize = 1_000;

/// The key of item `i`.
fn item_key(i: usize) -> Key {
    Key::from(format!("i{i}"))
}

/// Creates the expansion (into `state`, while mounted) and shows it.
#[component]
fn PerfExpansion(items: usize, state: StoredValue<Option<TreeExpansion>>) -> impl IntoView {
    let expansion = use_tree_expansion(UseTreeExpansionInput::default());
    state.set_value(Some(expansion));
    on_cleanup(move || state.set_value(None));
    let expanded_keys = expansion.expanded_keys;
    view! {
        <output id="perf-expanded">{move || expanded_keys.with(HashSet::len)}</output>
        <ul>
            {(0..items)
                .map(|i| {
                    let key = item_key(i);
                    view! {
                        <li id=format!("perf-item-{i}")>
                            {move || {
                                if expansion.is_expanded(&key) { "expanded" } else { "collapsed" }
                            }}
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let items = RwSignal::new(SMALL);
    let state = StoredValue::new(None::<TreeExpansion>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || items.set(SMALL)),
        Control::action("large", move || items.set(LARGE)),
        Control::action("toggle-first", move || {
            if let Some(expansion) = state.get_value() {
                expansion.toggle_key(item_key(0));
            }
        }),
        Control::action("expand-all", move || {
            if let Some(expansion) = state.get_value() {
                expansion.set_expanded_keys((0..items.get_untracked()).map(item_key).collect());
            }
        }),
        Control::action("collapse-all", move || {
            if let Some(expansion) = state.get_value() {
                expansion.set_expanded_keys(HashSet::new());
            }
        }),
    ];
    view! {
        <h1>"use_tree_expansion performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <PerfExpansion items=items.get_untracked() state />
        </Show>
    }
}
