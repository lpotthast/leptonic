// Upstream: react-stately/src/disclosure/useDisclosureState.ts @ 99e6102368
use leptos::prelude::*;

use crate::ValueBinding;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state (C4): `default_expanded` + `on_expanded_change`, or `value` bound to app state,
//   instead of a controlled `isExpanded`.
// - A `Copy` struct with a read-only signal and methods (C3).
//
// =============================================================================

/// Input of [`use_disclosure_state`].
#[derive(Debug, Clone, Copy, Default)]
pub struct UseDisclosureStateInput {
    /// Whether the disclosure starts expanded. Ignored when `value` is bound.
    pub default_expanded: bool,
    /// The expanded state as app state, replacing `default_expanded`.
    pub value: Option<ValueBinding<bool>>,
    /// Called when the disclosure expands or collapses.
    pub on_expanded_change: Option<Callback<bool>>,
}

/// Whether a disclosure's panel is expanded.
#[derive(Debug, Clone, Copy)]
pub struct DisclosureState {
    pub is_expanded: Signal<bool>,
    set_expanded: Callback<bool>,
}

impl DisclosureState {
    pub fn set_expanded(&self, is_expanded: bool) {
        self.set_expanded.run(is_expanded);
    }

    pub fn expand(&self) {
        self.set_expanded(true);
    }

    pub fn collapse(&self) {
        self.set_expanded(false);
    }

    pub fn toggle(&self) {
        self.set_expanded(!self.is_expanded.get_untracked());
    }
}

/// Manages whether a disclosure is expanded.
pub fn use_disclosure_state(input: UseDisclosureStateInput) -> DisclosureState {
    let UseDisclosureStateInput {
        default_expanded,
        value,
        on_expanded_change,
    } = input;
    let binding = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_expanded)));
    let is_expanded = binding.value;
    DisclosureState {
        is_expanded,
        set_expanded: Callback::new(move |new: bool| {
            // As react-aria's `useControlledState`: only an actual change is applied and reported.
            if is_expanded.get_untracked() == new {
                return;
            }
            binding.set(new);
            if let Some(on_expanded_change) = on_expanded_change {
                on_expanded_change.run(new);
            }
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use assertr::prelude::*;

    use super::*;

    #[test]
    fn toggles_and_reports_changes_only() {
        crate::testing::with_owner(|| {
            let changes = Arc::new(Mutex::new(Vec::new()));
            let recorded = Arc::clone(&changes);
            let state = use_disclosure_state(UseDisclosureStateInput {
                on_expanded_change: Some(Callback::new(move |expanded| {
                    recorded.lock().unwrap().push(expanded);
                })),
                ..UseDisclosureStateInput::default()
            });
            assert_that!(state.is_expanded.get_untracked()).is_false();
            state.toggle();
            assert_that!(state.is_expanded.get_untracked()).is_true();
            state.expand();
            state.collapse();
            assert_that!(changes.lock().unwrap().clone()).is_equal_to(vec![true, false]);
        });
    }

    #[test]
    fn binds_app_state() {
        crate::testing::with_owner(|| {
            let expanded = RwSignal::new(true);
            let state = use_disclosure_state(UseDisclosureStateInput {
                value: Some(ValueBinding::from(expanded)),
                ..UseDisclosureStateInput::default()
            });
            assert_that!(state.is_expanded.get_untracked()).is_true();
            state.toggle();
            assert_that!(expanded.get_untracked()).is_false();
        });
    }
}
