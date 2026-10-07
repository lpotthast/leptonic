// Upstream: react-stately/src/tree/useTreeState.ts @ 99e6102368
use std::{collections::HashSet, sync::Arc};

use leptos::prelude::*;

use crate::hooks::collections::{
    CollectionMemo, ItemElements, Key, ListState, SelectionManager, SelectionOptions,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state: `default_expanded_keys` with `toggle_key`/`set_expanded_keys` instead of
//   a controlled `expandedKeys`.
// - The tree is a collection built with item children (`ItemBuilder::children`); the state's
//   list shows its visible part (`Collection::with_expanded`).
//
// =============================================================================

/// Which items of a tree are expanded, and how to change that.
#[derive(Debug, Clone, Copy)]
pub struct TreeExpansion {
    pub expanded_keys: Signal<HashSet<Key>>,
    toggle: Callback<Key>,
}

impl TreeExpansion {
    pub fn is_expanded(&self, key: &Key) -> bool {
        self.expanded_keys.with(|keys| keys.contains(key))
    }

    /// Expands a collapsed item, collapses an expanded one.
    pub fn toggle_key(&self, key: Key) {
        self.toggle.run(key);
    }
}

/// Input of [`use_tree_state`].
#[derive(Debug, Clone)]
pub struct UseTreeStateInput {
    /// The whole tree.
    pub collection: CollectionMemo,
    pub selection: SelectionOptions,
    pub default_expanded_keys: HashSet<Key>,
    /// Called when items are expanded or collapsed.
    pub on_expanded_change: Option<Callback<HashSet<Key>>>,
}

/// The state of a tree: its visible items (with selection and focus) and which items are
/// expanded.
#[derive(Debug, Clone, Copy)]
pub struct TreeState {
    /// The visible items.
    pub list: ListState,
    pub expansion: TreeExpansion,
    set_expanded: Callback<HashSet<Key>>,
}

impl TreeState {
    pub fn set_expanded_keys(&self, keys: HashSet<Key>) {
        self.set_expanded.run(keys);
    }
}

/// Creates the state of a tree (see [`TreeState`]).
pub fn use_tree_state(input: UseTreeStateInput) -> TreeState {
    let UseTreeStateInput {
        collection,
        selection,
        default_expanded_keys,
        on_expanded_change,
    } = input;

    let expanded_keys = RwSignal::new(default_expanded_keys);
    let set_expanded = Callback::new(move |keys: HashSet<Key>| {
        if expanded_keys.with_untracked(|current| *current != keys) {
            expanded_keys.set(keys.clone());
            if let Some(on_expanded_change) = on_expanded_change {
                on_expanded_change.run(keys);
            }
        }
    });
    let toggle = Callback::new(move |key: Key| {
        let mut keys = expanded_keys.get_untracked();
        if !keys.remove(&key) {
            keys.insert(key);
        }
        set_expanded.run(keys);
    });

    let visible: CollectionMemo = Memo::new(move |_| {
        expanded_keys.with(|expanded| Arc::new(collection.with(|c| c.with_expanded(expanded))))
    });
    let selection = SelectionManager::new(visible, selection);

    // A focused item hidden by collapsing its parent loses focus.
    Effect::new(move |_| {
        let focused = selection.focused_key();
        if let Some(focused) = focused
            && !visible.with(|c| c.contains_key(&focused))
        {
            selection.set_focused_key(None, None);
        }
    });

    TreeState {
        list: ListState {
            collection: visible,
            selection,
            item_elements: ItemElements::new(),
        },
        expansion: TreeExpansion {
            expanded_keys: expanded_keys.into(),
            toggle,
        },
        set_expanded,
    }
}
