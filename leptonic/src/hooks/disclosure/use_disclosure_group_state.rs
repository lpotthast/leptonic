// Upstream: react-stately/src/disclosure/useDisclosureGroupState.ts @ 99e6102368
use std::collections::HashSet;

use leptos::prelude::*;

use crate::{hooks::collections::Key, utils::ValueBinding};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `allowsMultipleExpanded` is the `DisclosureGroupExpansion` enum.
// - Hook-owned state (C4): `default_expanded_keys` + `on_expanded_change`, or `value` bound to app
//   state, instead of controlled `expandedKeys`.
// - A `Copy` struct with read-only signals and methods (C3).
//
// ## DIFFERENT BEHAVIOR
// - More than one expanded key in a single-expansion group is reduced to one when the keys are
//   set (react-aria: an effect after each render): the first of `default_expanded_keys`, the
//   smallest of a set (react-aria: the first in insertion order, which a `HashSet` doesn't keep;
//   the smallest is the same on the server and the client). A bound `value` is reduced on
//   creation (so the server renders one) and in an effect whenever it changes; both report the
//   reduction through `on_expanded_change`, as react-aria's effect does.
//
// =============================================================================

/// How many disclosures of a group can be expanded at once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DisclosureGroupExpansion {
    /// Expanding one collapses the others (an accordion).
    #[default]
    Single,
    /// Any number.
    Multiple,
}

/// Input of [`use_disclosure_group_state`].
#[derive(Debug, Clone, Default)]
pub struct UseDisclosureGroupStateInput {
    pub expansion: DisclosureGroupExpansion,
    /// Whether all disclosures of the group are disabled.
    pub is_disabled: Signal<bool>,
    /// The initially expanded disclosures, in order. Ignored when `value` is bound.
    pub default_expanded_keys: Vec<Key>,
    /// The expanded disclosures as app state, replacing `default_expanded_keys`.
    pub value: Option<ValueBinding<HashSet<Key>>>,
    /// Called with the expanded disclosures when they change.
    pub on_expanded_change: Option<Callback<HashSet<Key>>>,
}

/// Which disclosures of a group are expanded.
#[derive(Debug, Clone, Copy)]
pub struct DisclosureGroupState {
    pub expansion: DisclosureGroupExpansion,
    pub is_disabled: Signal<bool>,
    pub expanded_keys: Signal<HashSet<Key>>,
    set_expanded_keys: Callback<HashSet<Key>>,
}

impl DisclosureGroupState {
    pub fn set_expanded_keys(&self, keys: HashSet<Key>) {
        self.set_expanded_keys.run(keys);
    }

    /// Whether the disclosure `key` is expanded (tracked).
    pub fn is_expanded(&self, key: &Key) -> bool {
        self.expanded_keys.with(|keys| keys.contains(key))
    }

    /// Expands or collapses the disclosure `key`; in a single-expansion group, expanding it
    /// collapses the others.
    pub fn toggle_key(&self, key: &Key) {
        let mut keys = self.expanded_keys.get_untracked();
        match self.expansion {
            DisclosureGroupExpansion::Multiple => {
                if !keys.remove(key) {
                    keys.insert(key.clone());
                }
            }
            DisclosureGroupExpansion::Single => {
                keys = if keys.contains(key) {
                    HashSet::new()
                } else {
                    HashSet::from([key.clone()])
                };
            }
        }
        self.set_expanded_keys(keys);
    }
}

/// Manages which disclosures of a group are expanded.
pub fn use_disclosure_group_state(input: UseDisclosureGroupStateInput) -> DisclosureGroupState {
    let UseDisclosureGroupStateInput {
        expansion,
        is_disabled,
        default_expanded_keys,
        value,
        on_expanded_change,
    } = input;
    let single = move |keys: HashSet<Key>| {
        if expansion == DisclosureGroupExpansion::Single && keys.len() > 1 {
            keys.into_iter().min().into_iter().collect()
        } else {
            keys
        }
    };
    let defaults: HashSet<Key> = match expansion {
        DisclosureGroupExpansion::Single => default_expanded_keys.into_iter().take(1).collect(),
        DisclosureGroupExpansion::Multiple => default_expanded_keys.into_iter().collect(),
    };
    let binding = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(defaults)));
    let expanded_keys = binding.value;
    let set_expanded_keys = Callback::new(move |keys: HashSet<Key>| {
        let keys = single(keys);
        if expanded_keys.with_untracked(|current| *current == keys) {
            return;
        }
        binding.set(keys.clone());
        if let Some(on_expanded_change) = on_expanded_change {
            on_expanded_change.run(keys);
        }
    });
    if expansion == DisclosureGroupExpansion::Single {
        // A bound value with several keys: reduced now and whenever it changes to several.
        let reduce = move || {
            if expanded_keys.with_untracked(|keys| keys.len() > 1) {
                set_expanded_keys.run(expanded_keys.get_untracked());
            }
        };
        reduce();
        Effect::new(move || {
            expanded_keys.track();
            reduce();
        });
    }
    DisclosureGroupState {
        expansion,
        is_disabled,
        expanded_keys,
        set_expanded_keys,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn single_expansion_keeps_one_expanded() {
        Owner::new().with(|| {
            let state = use_disclosure_group_state(UseDisclosureGroupStateInput::default());
            let (a, b) = (Key::from("a"), Key::from("b"));
            state.toggle_key(&a);
            assert_that!(state.expanded_keys.get_untracked())
                .is_equal_to(HashSet::from([a.clone()]));
            state.toggle_key(&b);
            assert_that!(state.expanded_keys.get_untracked())
                .is_equal_to(HashSet::from([b.clone()]));
            state.toggle_key(&b);
            assert_that!(state.expanded_keys.get_untracked()).is_empty();
            // Setting several keys keeps one.
            state.set_expanded_keys(HashSet::from([a, b]));
            assert_that!(state.expanded_keys.get_untracked().len()).is_equal_to(1);
        });
    }

    #[test]
    fn a_bound_value_with_several_keys_is_reduced_and_reported() {
        use std::sync::{Arc, Mutex};

        Owner::new().with(|| {
            let (a, b) = (Key::from("a"), Key::from("b"));
            let bound = RwSignal::new(HashSet::from([a.clone(), b]));
            let changes = Arc::new(Mutex::new(Vec::new()));
            let recorded = Arc::clone(&changes);
            let state = use_disclosure_group_state(UseDisclosureGroupStateInput {
                value: Some(bound.into()),
                on_expanded_change: Some(Callback::new(move |keys| {
                    recorded.lock().unwrap().push(keys);
                })),
                ..UseDisclosureGroupStateInput::default()
            });
            assert_that!(state.expanded_keys.get_untracked())
                .is_equal_to(HashSet::from([a.clone()]));
            assert_that!(bound.get_untracked()).is_equal_to(HashSet::from([a.clone()]));
            assert_that!(changes.lock().unwrap().clone()).is_equal_to(vec![HashSet::from([a])]);
        });
    }

    #[test]
    fn multiple_expansion_toggles_each() {
        Owner::new().with(|| {
            let state = use_disclosure_group_state(UseDisclosureGroupStateInput {
                expansion: DisclosureGroupExpansion::Multiple,
                ..UseDisclosureGroupStateInput::default()
            });
            let (a, b) = (Key::from("a"), Key::from("b"));
            state.toggle_key(&a);
            state.toggle_key(&b);
            assert_that!(state.expanded_keys.get_untracked())
                .is_equal_to(HashSet::from([a.clone(), b]));
            state.toggle_key(&a);
            assert_that!(state.is_expanded(&a)).is_false();
        });
    }
}
