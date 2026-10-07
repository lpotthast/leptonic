// Upstream: react-stately/src/tabs/useTabListState.ts @ 99e6102368
use std::collections::HashSet;

use leptos::prelude::*;

use crate::{
    hooks::collections::{
        Collection, CollectionMemo, Key, SingleSelectListState, UseSingleSelectListStateInput,
        use_single_select_list_state,
    },
    utils::ValueBinding,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state (C4): `default_selected_key` and `on_selection_change`, or `selected_key`
//   bound to app state, instead of a controlled `selectedKey`. Without a default, the first
//   enabled tab is selected.
//
// ## DIFFERENT BEHAVIOR
// - A bound `selected_key` that isn't a tab is replaced by the first enabled tab, which is written
//   to the app state (react-aria leaves a controlled `selectedKey` alone). Reason: one tab is
//   always selected, and the app state shows which. A disabled selected tab stays selected, as in
//   react-aria.
//
// =============================================================================

/// Input of [`use_tab_list_state`].
#[derive(Debug, Clone)]
pub struct UseTabListStateInput {
    /// The tabs.
    pub collection: CollectionMemo,
    /// The initially selected tab. Defaults to the first enabled tab. Ignored when
    /// `selected_key` is bound.
    pub default_selected_key: Option<Key>,
    /// The selected tab as app state, replacing `default_selected_key`. A key that isn't a tab
    /// is replaced by the first enabled tab; a disabled tab stays selected.
    pub selected_key: Option<ValueBinding<Key>>,
    /// Called with the key of the tab the user selects.
    pub on_selection_change: Option<Callback<Key>>,
    pub disabled_keys: Signal<HashSet<Key>>,
    /// Disables all tabs.
    pub is_disabled: Signal<bool>,
}

/// The state of a tab list: its tabs, the selected tab and keyboard focus.
#[derive(Debug, Clone, Copy)]
pub struct TabListState {
    pub list: SingleSelectListState,
    /// All tabs are disabled.
    pub is_disabled: Signal<bool>,
}

impl TabListState {
    /// The selected tab.
    pub fn selected_key(&self) -> Option<Key> {
        self.list.selected_key()
    }
}

/// Creates the state of a tab list (see [`TabListState`]). One tab is always selected: the
/// default, or the first enabled tab; if the selected tab disappears, the first enabled tab. A
/// disabled tab can be selected (by default or bound key) and stays selected.
pub fn use_tab_list_state(input: UseTabListStateInput) -> TabListState {
    let UseTabListStateInput {
        collection,
        default_selected_key,
        selected_key,
        on_selection_change,
        disabled_keys,
        is_disabled,
    } = input;
    // Without a default or bound key, the first enabled tab is selected. Until the first render
    // completes, that default follows the disabled keys: tabs disabled only as they render (the
    // `Tab` atom's `is_disabled`) are skipped already while rendering, on the server too, as
    // react-aria-components knows them from its collection before rendering. It is fixed once
    // rendered (react-aria computes it once), and replaced by every selection.
    let first_enabled_default =
        (default_selected_key.is_none() && selected_key.is_none()).then(|| {
            let selected = RwSignal::new(None::<Key>);
            let value = Signal::derive(move || {
                selected
                    .get()
                    .or_else(|| collection.with(|c| disabled_keys.with(|d| default_key(c, d))))
            });
            (selected, value)
        });
    let default_selected_key = default_selected_key
        .or_else(|| untrack(|| collection.with(|c| disabled_keys.with(|d| default_key(c, d)))));
    let list = use_single_select_list_state(UseSingleSelectListStateInput {
        collection,
        default_selected_key,
        // A tab is always selected: the list's `None` (never written, as the list disallows an
        // empty selection) doesn't reach the app state.
        selected_key: match (selected_key, first_enabled_default) {
            (Some(key), _) => Some(ValueBinding::new(
                Signal::derive(move || Some(key.value.get())),
                Callback::new(move |selected: Option<Key>| {
                    if let Some(selected) = selected {
                        key.set(selected);
                    }
                }),
            )),
            (None, Some((selected, value))) => Some(ValueBinding::new(
                value,
                Callback::new(move |key: Option<Key>| {
                    if key.is_some() {
                        selected.set(key);
                    }
                }),
            )),
            (None, None) => None,
        },
        on_selection_change: on_selection_change.map(|on_change| {
            Callback::new(move |key: Option<Key>| {
                if let Some(key) = key {
                    on_change.run(key);
                }
            })
        }),
        disabled_keys,
    });

    // Keep a valid tab selected, and the focused tab in sync with the selection while the tab
    // list doesn't have focus.
    let selection = list.list.selection;
    Effect::new(move |last_selected: Option<Option<Key>>| {
        // Every source up front, the app's inputs (collection, disabled keys, bound key) before
        // what derives from them ("Effect Read Order"): the disabled keys are otherwise read only
        // without a valid selection.
        collection.track();
        disabled_keys.track();
        let mut selected = list.selected_key();
        // Rendered: the first enabled tab stays selected by default.
        if last_selected.is_none()
            && let Some((default, _)) = first_enabled_default
            && default.with_untracked(Option::is_none)
        {
            default.set(selected.clone());
        }
        let exists = selected
            .as_ref()
            .is_some_and(|key| collection.with(|c| c.contains_key(key)));
        if !exists {
            selected = collection.with(|c| disabled_keys.with(|d| default_key(c, d)));
            if let Some(key) = &selected {
                untrack(|| selection.set_selected_keys([key.clone()]));
            }
        }
        let focused_key = untrack(|| selection.focused_key());
        let is_focused = untrack(|| selection.is_focused());
        let changed = last_selected.as_ref().is_some_and(|last| *last != selected);
        if (selected.is_some() && focused_key.is_none()) || (!is_focused && changed) {
            selection.set_focused_key(selected.clone(), None);
        }
        selected
    });

    TabListState { list, is_disabled }
}

/// The first enabled tab (or the first tab, if all are disabled).
fn default_key(collection: &Collection, disabled_keys: &HashSet<Key>) -> Option<Key> {
    let enabled = |key: &Key| {
        !disabled_keys.contains(key) && collection.get(key).is_some_and(|n| !n.is_disabled)
    };
    collection
        .items()
        .map(|n| &n.key)
        .find(|key| enabled(key))
        .or_else(|| collection.first_key())
        .cloned()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use assertr::prelude::*;

    use super::*;

    fn tabs(keys: &'static [&'static str]) -> CollectionMemo {
        Memo::new(move |_| {
            Arc::new(Collection::build(|b| {
                for key in keys {
                    b.item(*key, key.to_uppercase());
                }
            }))
        })
    }

    /// Tabs disabled as they render (before any effect runs, as on the server) are skipped by
    /// the default selection.
    #[test]
    fn the_default_skips_tabs_disabled_while_rendering() {
        Owner::new().with(|| {
            let disabled = RwSignal::new(HashSet::new());
            let state = use_tab_list_state(UseTabListStateInput {
                selected_key: None,
                collection: tabs(&["a", "b", "c"]),
                default_selected_key: None,
                on_selection_change: None,
                disabled_keys: disabled.into(),
                is_disabled: Signal::stored(false),
            });
            assert_that!(state.selected_key()).is_equal_to(Some(Key::from("a")));
            disabled.set(HashSet::from([Key::from("a")]));
            assert_that!(state.selected_key()).is_equal_to(Some(Key::from("b")));
            // Selecting replaces the default.
            state.list.list.selection.select(&Key::from("c"), None);
            disabled.set(HashSet::from([Key::from("a"), Key::from("b")]));
            assert_that!(state.selected_key()).is_equal_to(Some(Key::from("c")));
        });
    }

    #[test]
    fn a_disabled_bound_key_stays_selected() {
        Owner::new().with(|| {
            let selected = RwSignal::new(Key::from("b"));
            let state = use_tab_list_state(UseTabListStateInput {
                selected_key: Some(selected.into()),
                collection: tabs(&["a", "b", "c"]),
                default_selected_key: None,
                on_selection_change: None,
                disabled_keys: Signal::stored(HashSet::from([Key::from("b")])),
                is_disabled: Signal::stored(false),
            });
            assert_that!(state.selected_key()).is_equal_to(Some(Key::from("b")));
            assert_that!(selected.get_untracked()).is_equal_to(Key::from("b"));
        });
    }

    #[test]
    fn a_bound_selected_key_is_shown_and_written() {
        Owner::new().with(|| {
            let selected = RwSignal::new(Key::from("b"));
            let state = use_tab_list_state(UseTabListStateInput {
                selected_key: Some(selected.into()),
                collection: tabs(&["a", "b", "c"]),
                default_selected_key: None,
                on_selection_change: None,
                disabled_keys: Signal::default(),
                is_disabled: Signal::stored(false),
            });
            assert_that!(state.selected_key()).is_equal_to(Some(Key::from("b")));
            // Selecting writes the app state.
            state.list.list.selection.select(&Key::from("c"), None);
            assert_that!(selected.get_untracked()).is_equal_to(Key::from("c"));
            // Changes of the app state are shown.
            selected.set(Key::from("a"));
            assert_that!(state.selected_key()).is_equal_to(Some(Key::from("a")));
        });
    }
}
