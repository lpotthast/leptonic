// Upstream: react-stately/src/tree/useTreeState.ts @ 99e6102368
//! Which items of a tree are expanded (`use_tree_expansion`).

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - A hook of its own, shared by `use_tree_state` and `use_table_state`'s tree tables
//   (react-stately: the expansion part of `useTreeState`, which tree tables repeat), so that a
//   hierarchy of your own expands as leptonic's trees do.
// - Expansion state (C4): `default_expanded_keys` and `on_expanded_change`, or `expanded_keys`
//   bound to app state, instead of a controlled `expandedKeys`; changes go through `toggle_key`
//   and `set_expanded_keys`, which report only actual changes.
// - `is_expanded` notifies a reader only when its own key's expansion changes.
//
// =============================================================================

use std::collections::HashSet;

use leptos::prelude::*;

use crate::{ValueBinding, hooks::collections::Key, utils::keyed_triggers::KeyedTriggers};

/// Input of [`use_tree_expansion`].
#[derive(Debug, Clone, Default)]
pub struct UseTreeExpansionInput {
    /// The initially expanded items. Ignored when `expanded_keys` is bound.
    pub default_expanded_keys: HashSet<Key>,

    /// The expanded items as app state, replacing `default_expanded_keys`: expanding or
    /// collapsing items writes them, and changing them changes the expansion.
    pub expanded_keys: Option<ValueBinding<HashSet<Key>>>,

    /// Called when items are expanded or collapsed, with all expanded keys.
    pub on_expanded_change: Option<Callback<HashSet<Key>>>,
}

/// Which items of a tree are expanded, and how to change that (from [`use_tree_expansion`]).
/// `Copy`.
#[derive(Debug, Clone, Copy)]
pub struct TreeExpansion {
    /// The keys of the expanded items.
    pub expanded_keys: Signal<HashSet<Key>>,
    set: Callback<HashSet<Key>>,
    /// Readers of single items' expansion, by key (`performance.md` R1).
    items: KeyedTriggers<Key>,
}

impl TreeExpansion {
    /// Whether the item `key` is expanded (tracked: notifies only when this key's expansion
    /// changes, not on every expansion change).
    pub fn is_expanded(&self, key: &Key) -> bool {
        self.items.track(key);
        self.expanded_keys.with_untracked(|keys| keys.contains(key))
    }

    /// Expands a collapsed item, collapses an expanded one.
    pub fn toggle_key(&self, key: Key) {
        let mut keys = self.expanded_keys.get_untracked();
        if !keys.remove(&key) {
            keys.insert(key);
        }
        self.set.run(keys);
    }

    /// Expands exactly the items of `keys`. Reports nothing when they already are.
    pub fn set_expanded_keys(&self, keys: HashSet<Key>) {
        self.set.run(keys);
    }
}

/// The expansion state of a tree: which items are expanded, toggling an item, and setting all
/// expanded items at once, reporting each change to `on_expanded_change`.
///
/// `use_tree_state` and tree tables (`use_table_state` with a `TableTreeInput`) hold one; use it
/// for a hierarchy of your own (a file browser built from `use_grid_list`, nested disclosures
/// following one set of keys). It holds no items: expanding a key that isn't in your tree just
/// remembers it.
///
/// ```ignore
/// use leptonic::hooks::tree::{UseTreeExpansionInput, use_tree_expansion};
///
/// let expansion = use_tree_expansion(UseTreeExpansionInput {
///     default_expanded_keys: HashSet::from([Key::from("documents")]),
///     ..UseTreeExpansionInput::default()
/// });
/// expansion.toggle_key(Key::from("photos"));
/// let photos_open = move || expansion.is_expanded(&Key::from("photos"));
/// ```
pub fn use_tree_expansion(input: UseTreeExpansionInput) -> TreeExpansion {
    crate::metrics::subject!();
    let UseTreeExpansionInput {
        default_expanded_keys,
        expanded_keys: binding,
        on_expanded_change,
    } = input;
    let binding =
        binding.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_expanded_keys)));
    let expanded_keys = binding.value;
    let set = Callback::new(move |keys: HashSet<Key>| {
        if expanded_keys.with_untracked(|current| *current != keys) {
            binding.set(keys.clone());
            if let Some(on_expanded_change) = on_expanded_change {
                on_expanded_change.run(keys);
            }
        }
    });
    // Every change of the expanded keys (through the state or by the app) reaches the readers
    // of the keys that changed, right away.
    let items = KeyedTriggers::following(expanded_keys);
    TreeExpansion {
        expanded_keys,
        set,
        items,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    fn keys(keys: &[&str]) -> HashSet<Key> {
        keys.iter().map(|k| Key::from(*k)).collect()
    }

    #[test]
    fn toggling_expands_and_collapses_and_reports_each_change() {
        with_owner(|| {
            let changes = RwSignal::new(Vec::new());
            let expansion = use_tree_expansion(UseTreeExpansionInput {
                default_expanded_keys: keys(&["a"]),
                on_expanded_change: Some(Callback::new(move |keys: HashSet<Key>| {
                    changes.update(|c| c.push(keys));
                })),
                ..UseTreeExpansionInput::default()
            });
            assert_that!(expansion.is_expanded(&Key::from("a"))).is_true();
            expansion.toggle_key(Key::from("b"));
            expansion.toggle_key(Key::from("a"));
            assert_that!(expansion.expanded_keys.get_untracked()).is_equal_to(keys(&["b"]));
            assert_that!(changes.get_untracked())
                .is_equal_to(vec![keys(&["a", "b"]), keys(&["b"])]);
        });
    }

    #[test]
    fn setting_the_same_keys_reports_nothing() {
        with_owner(|| {
            let changes = RwSignal::new(0);
            let expansion = use_tree_expansion(UseTreeExpansionInput {
                default_expanded_keys: keys(&["a"]),
                on_expanded_change: Some(Callback::new(move |_: HashSet<Key>| {
                    changes.update(|c| *c += 1);
                })),
                ..UseTreeExpansionInput::default()
            });
            expansion.set_expanded_keys(keys(&["a"]));
            assert_that!(changes.get_untracked()).is_equal_to(0);
            expansion.set_expanded_keys(keys(&["a", "c"]));
            assert_that!(changes.get_untracked()).is_equal_to(1);
            assert_that!(expansion.is_expanded(&Key::from("c"))).is_true();
        });
    }

    #[test]
    fn bound_keys_are_written_and_followed() {
        with_owner(|| {
            let expanded = RwSignal::new(keys(&["a"]));
            let expansion = use_tree_expansion(UseTreeExpansionInput {
                default_expanded_keys: keys(&["ignored"]),
                expanded_keys: Some(expanded.into()),
                on_expanded_change: None,
            });
            assert_that!(expansion.is_expanded(&Key::from("ignored"))).is_false();
            expansion.toggle_key(Key::from("b"));
            assert_that!(expanded.get_untracked()).is_equal_to(keys(&["a", "b"]));
            // The app changes it: the expansion follows.
            expanded.set(HashSet::new());
            assert_that!(expansion.is_expanded(&Key::from("a"))).is_false();
        });
    }

    #[test]
    fn a_reader_of_one_key_is_notified_only_by_its_changes() {
        with_owner(|| {
            let expansion = use_tree_expansion(UseTreeExpansionInput::default());
            let runs = RwSignal::new(0);
            let a = Memo::new(move |_| {
                runs.update_untracked(|r| *r += 1);
                expansion.is_expanded(&Key::from("a"))
            });
            assert_that!(a.get_untracked()).is_false();
            expansion.toggle_key(Key::from("b"));
            assert_that!(a.get_untracked()).is_false();
            assert_that!(runs.get_untracked()).is_equal_to(1);
            expansion.toggle_key(Key::from("a"));
            assert_that!(a.get_untracked()).is_true();
            assert_that!(runs.get_untracked()).is_equal_to(2);
        });
    }
}
