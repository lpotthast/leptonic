// Upstream: react-stately/src/tree/useTreeState.ts @ 99e6102368
// Upstream: react-stately/test/tree/useTreeState.test.js @ 99e6102368
// Upstream: react-aria-components/test/Tree.test.tsx @ 99e6102368
use std::{collections::HashSet, sync::Arc};

use leptos::prelude::*;

use crate::{
    ValueBinding,
    hooks::{
        collections::{
            CollectionMemo, ItemElements, Key, ListState, SelectionManager, SelectionOptions,
        },
        tree::{TreeExpansion, UseTreeExpansionInput, use_tree_expansion},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Expansion state (C4): `default_expanded_keys` and `on_expanded_change`, or `expanded_keys`
//   bound to app state, instead of a controlled `expandedKeys`, held by `use_tree_expansion`;
//   changes go through the state's `expansion` (`toggle_key`/`set_expanded_keys`).
// - The tree is a collection built with item children (`ItemBuilder::children`); the state's
//   list shows its visible part (`Collection::with_expanded`).
//
// =============================================================================

/// Input of [`use_tree_state`].
#[derive(Debug, Clone)]
pub struct UseTreeStateInput {
    /// The whole tree.
    pub collection: CollectionMemo,
    pub selection: SelectionOptions,
    /// The initially expanded items. Ignored when `expanded_keys` is bound.
    pub default_expanded_keys: HashSet<Key>,
    /// The expanded items as app state, replacing `default_expanded_keys`: the tree shows them,
    /// and expanding or collapsing items writes them.
    pub expanded_keys: Option<ValueBinding<HashSet<Key>>>,
    /// Called when items are expanded or collapsed.
    pub on_expanded_change: Option<Callback<HashSet<Key>>>,
}

/// The state of a tree: its visible items (with selection and focus) and which items are
/// expanded.
#[derive(Debug, Clone, Copy)]
pub struct TreeState {
    /// The visible items.
    pub list: ListState,
    /// Which items are expanded (also given to `use_tree`'s items): `toggle_key`,
    /// `set_expanded_keys`.
    pub expansion: TreeExpansion,
}

/// Creates the state of a tree (see [`TreeState`]).
pub fn use_tree_state(input: UseTreeStateInput) -> TreeState {
    crate::metrics::subject!();
    let UseTreeStateInput {
        collection,
        selection,
        default_expanded_keys,
        expanded_keys: binding,
        on_expanded_change,
    } = input;

    let expansion = use_tree_expansion(UseTreeExpansionInput {
        default_expanded_keys,
        expanded_keys: binding,
        on_expanded_change,
    });
    let expanded_keys = expansion.expanded_keys;

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
        expansion,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::{
        hooks::collections::Collection,
        testing::{flush_effects, with_owner},
    };

    /// a (a1, a2 (a2x)), b
    fn tree() -> CollectionMemo {
        Memo::new(|_| {
            Arc::new(Collection::build(|b| {
                b.item("a", "a").children(|c| {
                    c.item("a1", "a1");
                    c.item("a2", "a2").children(|c| {
                        c.item("a2x", "a2x");
                    });
                });
                b.item("b", "b");
            }))
        })
    }

    /// The visible items, sorted.
    fn visible(state: &TreeState) -> Vec<String> {
        let mut keys: Vec<String> = state
            .list
            .collection
            .with_untracked(|c| c.keys().map(ToString::to_string).collect());
        keys.sort();
        keys
    }

    fn keys(keys: &[&str]) -> HashSet<Key> {
        keys.iter().map(|k| Key::from(*k)).collect()
    }

    #[test]
    fn expanding_shows_children_and_reports_the_change() {
        with_owner(|| {
            let changes = RwSignal::new(Vec::new());
            let state = use_tree_state(UseTreeStateInput {
                collection: tree(),
                selection: SelectionOptions::default(),
                default_expanded_keys: HashSet::new(),
                expanded_keys: None,
                on_expanded_change: Some(Callback::new(move |keys: HashSet<Key>| {
                    changes.update(|c| c.push(keys));
                })),
            });
            assert_that!(visible(&state)).is_equal_to(vec!["a".to_owned(), "b".to_owned()]);
            state.expansion.toggle_key(Key::from("a"));
            assert_that!(visible(&state)).is_equal_to(vec![
                "a".to_owned(),
                "a1".to_owned(),
                "a2".to_owned(),
                "b".to_owned(),
            ]);
            assert_that!(state.expansion.is_expanded(&Key::from("a"))).is_true();
            // Collapsing hides the children again; setting the same keys reports nothing.
            state.expansion.toggle_key(Key::from("a"));
            state.expansion.set_expanded_keys(HashSet::new());
            assert_that!(visible(&state)).is_equal_to(vec!["a".to_owned(), "b".to_owned()]);
            assert_that!(changes.get_untracked()).is_equal_to(vec![keys(&["a"]), HashSet::new()]);
        });
    }

    #[test]
    fn a_collapsed_parent_hides_expanded_descendants() {
        with_owner(|| {
            let state = use_tree_state(UseTreeStateInput {
                collection: tree(),
                selection: SelectionOptions::default(),
                default_expanded_keys: keys(&["a2"]),
                expanded_keys: None,
                on_expanded_change: None,
            });
            assert_that!(visible(&state)).is_equal_to(vec!["a".to_owned(), "b".to_owned()]);
            state.expansion.toggle_key(Key::from("a"));
            assert_that!(visible(&state)).is_equal_to(vec![
                "a".to_owned(),
                "a1".to_owned(),
                "a2".to_owned(),
                "a2x".to_owned(),
                "b".to_owned(),
            ]);
        });
    }

    #[test]
    fn bound_expanded_keys_are_shown_and_written() {
        with_owner(|| {
            let expanded = RwSignal::new(keys(&["a"]));
            let state = use_tree_state(UseTreeStateInput {
                collection: tree(),
                selection: SelectionOptions::default(),
                default_expanded_keys: HashSet::new(),
                expanded_keys: Some(expanded.into()),
                on_expanded_change: None,
            });
            assert_that!(visible(&state)).is_equal_to(vec![
                "a".to_owned(),
                "a1".to_owned(),
                "a2".to_owned(),
                "b".to_owned(),
            ]);
            state.expansion.toggle_key(Key::from("b"));
            assert_that!(expanded.get_untracked()).is_equal_to(keys(&["a", "b"]));
            // The app changes it: the tree follows.
            expanded.set(HashSet::new());
            assert_that!(visible(&state)).is_equal_to(vec!["a".to_owned(), "b".to_owned()]);
        });
    }

    #[test]
    fn collapsing_the_parent_of_the_focused_item_clears_the_focus() {
        // react-stately's `useTreeState`: a focused key no longer in the tree is reset.
        with_owner(|| {
            let state = use_tree_state(UseTreeStateInput {
                collection: tree(),
                selection: SelectionOptions::default(),
                default_expanded_keys: keys(&["a"]),
                expanded_keys: None,
                on_expanded_change: None,
            });
            flush_effects();
            state
                .list
                .selection
                .set_focused_key(Some(Key::from("a1")), None);
            flush_effects();
            state.expansion.toggle_key(Key::from("b"));
            flush_effects();
            // Still visible: kept.
            assert_that!(state.list.selection.focused_key()).is_equal_to(Some(Key::from("a1")));
            state.expansion.toggle_key(Key::from("a"));
            flush_effects();
            assert_that!(state.list.selection.focused_key()).is_none();
        });
    }
}
