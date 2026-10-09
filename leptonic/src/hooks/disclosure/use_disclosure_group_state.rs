// Upstream: react-stately/src/disclosure/useDisclosureGroupState.ts @ 99e6102368
use std::collections::HashSet;

use leptos::prelude::*;

use crate::{ValueBinding, hooks::collections::Key};

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
//   set (react-aria: an effect after each render): the smallest key (react-aria: the first in insertion order, which a `HashSet` doesn't keep;
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
    /// Whether one or several disclosures can be expanded at once.
    pub expansion: Signal<DisclosureGroupExpansion>,
    /// Whether all disclosures of the group are disabled.
    pub is_disabled: Signal<bool>,
    /// The initially expanded disclosures. Ignored when `value` is bound.
    pub default_expanded_keys: HashSet<Key>,
    /// The expanded disclosures as app state, replacing `default_expanded_keys`.
    pub value: Option<ValueBinding<HashSet<Key>>>,
    /// Called with the expanded disclosures when they change.
    pub on_expanded_change: Option<Callback<HashSet<Key>>>,
}

/// Which disclosures of a group are expanded.
#[derive(Debug, Clone, Copy)]
pub struct DisclosureGroupState {
    pub expansion: Signal<DisclosureGroupExpansion>,
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
        match self.expansion.get_untracked() {
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
        if expansion.get_untracked() == DisclosureGroupExpansion::Single && keys.len() > 1 {
            keys.into_iter().min().into_iter().collect()
        } else {
            keys
        }
    };
    let binding =
        value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(single(default_expanded_keys))));
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
    // In a single-expansion group, several keys (bound, or from a switch to single expansion)
    // are reduced now and whenever they change to several.
    let reduce = move || {
        if expansion.get_untracked() == DisclosureGroupExpansion::Single
            && expanded_keys.with_untracked(|keys| keys.len() > 1)
        {
            set_expanded_keys.run(expanded_keys.get_untracked());
        }
    };
    reduce();
    Effect::new(move || {
        expanded_keys.track();
        expansion.track();
        reduce();
    });
    DisclosureGroupState {
        expansion,
        is_disabled,
        expanded_keys,
        set_expanded_keys,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use assertr::prelude::*;

    use super::*;
    use crate::testing::{flush_effects, with_owner};

    #[test]
    fn single_expansion_keeps_one_expanded() {
        with_owner(|| {
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
        with_owner(|| {
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
        with_owner(|| {
            let state = use_disclosure_group_state(UseDisclosureGroupStateInput {
                expansion: DisclosureGroupExpansion::Multiple.into(),
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

    /// A bound value changed by the app to several keys is reduced to one and reported (upstream
    /// reduces in an effect after each render).
    #[test]
    fn a_bound_value_changing_to_several_keys_is_reduced() {
        with_owner(|| {
            let (a, b) = (Key::from("a"), Key::from("b"));
            let bound = RwSignal::new(HashSet::new());
            let changes = Arc::new(Mutex::new(Vec::new()));
            let recorded = Arc::clone(&changes);
            let state = use_disclosure_group_state(UseDisclosureGroupStateInput {
                value: Some(bound.into()),
                on_expanded_change: Some(Callback::new(move |keys| {
                    recorded.lock().unwrap().push(keys);
                })),
                ..UseDisclosureGroupStateInput::default()
            });
            flush_effects();
            bound.set(HashSet::from([a.clone(), b]));
            flush_effects();
            assert_that!(state.expanded_keys.get_untracked())
                .is_equal_to(HashSet::from([a.clone()]));
            assert_that!(changes.lock().unwrap().clone()).is_equal_to(vec![HashSet::from([a])]);
        });
    }

    /// Switching a group with several expanded disclosures to single expansion keeps one.
    #[test]
    fn switching_to_single_expansion_keeps_one() {
        with_owner(|| {
            let expansion = RwSignal::new(DisclosureGroupExpansion::Multiple);
            let state = use_disclosure_group_state(UseDisclosureGroupStateInput {
                expansion: expansion.into(),
                ..UseDisclosureGroupStateInput::default()
            });
            flush_effects();
            let (a, b) = (Key::from("a"), Key::from("b"));
            state.toggle_key(&a);
            state.toggle_key(&b);
            assert_that!(state.expanded_keys.get_untracked().len()).is_equal_to(2);
            expansion.set(DisclosureGroupExpansion::Single);
            flush_effects();
            assert_that!(state.expanded_keys.get_untracked()).is_equal_to(HashSet::from([a]));
        });
    }
}
