// Upstream: react-stately/src/list/useListState.ts @ 99e6102368
// Upstream: react-stately/src/list/useSingleSelectListState.ts @ 99e6102368
use std::sync::Arc;

use leptos::prelude::*;

use super::{
    Collection, CollectionMemo, ItemElements, Key, Node, NodeKind, Selection, SelectionManager,
    SelectionMode, SelectionOptions,
};
use crate::utils::ValueBinding;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The collection is passed in (built from application data with `use_collection`) instead of
//   being derived from rendered children.
// - `use_single_select_list_state`: hook-owned state (`default_selected_key` plus
//   `set_selected_key`), no controlled `selectedKey`.
// - The state also carries the item element registry (`ItemElements`), which replaces DOM
//   queries for item elements.
//
// =============================================================================

/// The state of a list-like collection (listbox, menu, grid list, ...): its items, selection,
/// focus and rendered item elements.
#[derive(Debug, Clone, Copy)]
pub struct ListState {
    pub collection: CollectionMemo,
    pub selection: SelectionManager,
    pub item_elements: ItemElements,
}

/// Input of [`use_list_state`].
#[derive(Debug, Clone)]
pub struct UseListStateInput {
    pub collection: CollectionMemo,
    pub selection: SelectionOptions,
}

/// Creates the state of a list-like collection.
///
/// When the focused item disappears from the collection (deleted, filtered out), focus moves to
/// the next item that still exists, or else to the previous one.
pub fn use_list_state(input: UseListStateInput) -> ListState {
    let UseListStateInput {
        collection,
        selection,
    } = input;
    let selection = SelectionManager::new(collection, selection);
    use_focused_key_reset(collection, selection);
    ListState {
        collection,
        selection,
        item_elements: ItemElements::new(),
    }
}

/// The same list state, showing `collection` (a subset of the state's collection, e.g. its
/// filtered items). Selection and focus are shared with `state`; when the focused item leaves
/// the view, focus moves to a neighbor.
pub fn use_list_state_view(state: ListState, collection: CollectionMemo) -> ListState {
    let selection = state.selection.with_collection(collection);
    use_focused_key_reset(collection, selection);
    ListState {
        collection,
        selection,
        item_elements: state.item_elements,
    }
}

fn use_focused_key_reset(collection: CollectionMemo, selection: SelectionManager) {
    Effect::new(move |previous: Option<Arc<Collection>>| {
        let current = collection.get();
        if let Some(previous) = previous
            && let Some(focused) = untrack(|| selection.focused_key())
            && !current.contains_key(&focused)
        {
            let next = next_focus_after_removal(&previous, &current, &focused, |key| {
                untrack(|| selection.is_disabled(key))
            });
            selection.set_focused_key(next, None);
        }
        current
    });
}

/// The item to focus after `focused` was removed from the collection: the next item (in the old
/// collection's order) that still exists and isn't disabled, else the previous one.
fn next_focus_after_removal(
    previous: &Collection,
    current: &Collection,
    focused: &Key,
    is_disabled: impl Fn(&Key) -> bool,
) -> Option<Key> {
    let usable = |key: &Key| {
        current
            .get(key)
            .is_some_and(|node| node.kind == NodeKind::Item && !is_disabled(key))
    };
    let mut key = previous.key_after(focused);
    while let Some(k) = key {
        if usable(k) {
            return Some(k.clone());
        }
        key = previous.key_after(k);
    }
    let mut key = previous.key_before(focused);
    while let Some(k) = key {
        if usable(k) {
            return Some(k.clone());
        }
        key = previous.key_before(k);
    }
    None
}

/// The state of a list in which exactly one item can be selected (a select's options, tabs).
#[derive(Debug, Clone, Copy)]
pub struct SingleSelectListState {
    pub list: ListState,
}

impl SingleSelectListState {
    /// The selected key, if any.
    pub fn selected_key(&self) -> Option<Key> {
        match self.list.selection.raw_selection() {
            Selection::Keys(keys) => keys.into_iter().next(),
            Selection::All => None,
        }
    }

    /// Select `key` (`None`: clear the selection).
    pub fn set_selected_key(&self, key: Option<Key>) {
        self.list.selection.set_selected_keys(key);
    }

    /// The node of the selected item.
    pub fn selected_item(&self) -> Option<Node> {
        let key = self.selected_key()?;
        self.list.collection.with(|c| c.get(&key).cloned())
    }
}

/// Input of [`use_single_select_list_state`].
#[derive(Debug, Clone)]
pub struct UseSingleSelectListStateInput {
    pub collection: CollectionMemo,
    /// Ignored when `selected_key` is bound.
    pub default_selected_key: Option<Key>,
    /// The selected key as app state, replacing `default_selected_key`.
    pub selected_key: Option<ValueBinding<Option<Key>>>,
    /// Called whenever the user selects an item, even if it was selected already.
    pub on_selection_change: Option<Callback<Option<Key>>>,
    pub disabled_keys: Signal<std::collections::HashSet<Key>>,
}

/// Creates the state of a single-selection list. Selecting an item again keeps it selected (but
/// still reports the selection, e.g. so a select closes its popover).
pub fn use_single_select_list_state(input: UseSingleSelectListStateInput) -> SingleSelectListState {
    let UseSingleSelectListStateInput {
        collection,
        default_selected_key,
        selected_key,
        on_selection_change,
        disabled_keys,
    } = input;
    // The bound key as the list's selection.
    let selection_binding = selected_key.map(|key| {
        ValueBinding::new(
            Signal::derive(move || Selection::keys(key.value.get())),
            Callback::new(move |selection: Selection| {
                if let Selection::Keys(keys) = selection {
                    key.set(keys.into_iter().next());
                }
            }),
        )
    });
    let list = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Single),
            default_selection: Selection::keys(default_selected_key),
            selection: selection_binding,
            on_selection_change: on_selection_change.map(|on_change| {
                Callback::new(move |selection: Selection| {
                    on_change.run(match selection {
                        Selection::Keys(keys) => keys.into_iter().next(),
                        Selection::All => None,
                    });
                })
            }),
            disallow_empty_selection: Signal::stored(true),
            disabled_keys,
            allow_duplicate_selection_events: true,
            ..SelectionOptions::default()
        },
    });
    SingleSelectListState { list }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn letters(keys: &[&str]) -> Collection {
        Collection::build(|b| {
            for key in keys {
                b.item(*key, key.to_uppercase());
            }
        })
    }

    #[test]
    fn focus_moves_to_the_next_remaining_item() {
        let before = letters(&["a", "b", "c", "d"]);
        let after = letters(&["a", "d"]);
        let next = next_focus_after_removal(&before, &after, &Key::from("b"), |_| false);
        assert_that!(next).is_equal_to(Some(Key::from("d")));
    }

    #[test]
    fn focus_moves_back_when_nothing_follows() {
        let before = letters(&["a", "b", "c"]);
        let after = letters(&["a"]);
        let next = next_focus_after_removal(&before, &after, &Key::from("c"), |_| false);
        assert_that!(next).is_equal_to(Some(Key::from("a")));
    }

    #[test]
    fn focus_skips_disabled_items() {
        let before = letters(&["a", "b", "c", "d"]);
        let after = letters(&["a", "c", "d"]);
        let next =
            next_focus_after_removal(&before, &after, &Key::from("b"), |k| *k == Key::from("c"));
        assert_that!(next).is_equal_to(Some(Key::from("d")));
    }

    #[test]
    fn single_select_reports_reselection() {
        Owner::new().with(|| {
            let changes = RwSignal::new(Vec::new());
            let collection: CollectionMemo = Memo::new(|_| Arc::new(letters(&["a", "b"])));
            let state = use_single_select_list_state(UseSingleSelectListStateInput {
                collection,
                default_selected_key: Some(Key::from("a")),
                selected_key: None,
                on_selection_change: Some(Callback::new(move |key| {
                    changes.update(|c| c.push(key));
                })),
                disabled_keys: Signal::stored(std::collections::HashSet::new()),
            });
            assert_that!(state.selected_key()).is_equal_to(Some(Key::from("a")));
            state.list.selection.select(&Key::from("a"), None);
            state.list.selection.select(&Key::from("b"), None);
            assert_that!(state.selected_key()).is_equal_to(Some(Key::from("b")));
            assert_that!(changes.get_untracked())
                .is_equal_to(vec![Some(Key::from("a")), Some(Key::from("b"))]);
        });
    }
}
