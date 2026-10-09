// Upstream: react-stately/src/toggle/useToggleState.ts @ 99e6102368
use leptos::prelude::*;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state (C4): `default_selected` + `on_change`, or `value` bound to app state (the
//   atoms' `is_selected` + `set_selected`). A state driven by something else (a group's value, a
//   collection's selection) binds `value` to it (`ValueBinding::new`).
//
// =============================================================================

/// The state of a toggle (checkbox, switch, toggle button): whether it is selected.
#[derive(Debug, Clone, Copy)]
pub struct ToggleState {
    /// Whether the toggle is selected.
    pub is_selected: Signal<bool>,
    /// The initial selection (of a bound value: its value at creation), restored on form reset.
    pub default_selected: bool,
    set_selected: Callback<bool>,
}

impl ToggleState {
    /// Select or deselect the toggle.
    pub fn set_selected(&self, is_selected: bool) {
        self.set_selected.run(is_selected);
    }

    /// Flip the selection.
    pub fn toggle(&self) {
        self.set_selected(!self.is_selected.get_untracked());
    }
}

/// Input of [`use_toggle_state`].
#[derive(Debug, Clone, Copy)]
pub struct UseToggleStateInput {
    /// Whether the toggle is initially selected. Ignored when `value` is bound.
    pub default_selected: bool,
    /// The selection as app state, replacing `default_selected`.
    pub value: Option<crate::ValueBinding<bool>>,
    /// Called when the selection changes.
    pub on_change: Option<Callback<bool>>,
    /// While `true`, the selection can't be changed.
    pub is_read_only: Signal<bool>,
}

impl Default for UseToggleStateInput {
    fn default() -> Self {
        Self {
            default_selected: false,
            value: None,
            on_change: None,
            is_read_only: Signal::stored(false),
        }
    }
}

/// Creates the state of a toggle.
pub fn use_toggle_state(input: UseToggleStateInput) -> ToggleState {
    let UseToggleStateInput {
        default_selected,
        value,
        on_change,
        is_read_only,
    } = input;
    let binding =
        value.unwrap_or_else(|| crate::ValueBinding::from(RwSignal::new(default_selected)));
    let is_selected = binding.value;
    ToggleState {
        is_selected,
        default_selected: is_selected.get_untracked(),
        set_selected: Callback::new(move |selected: bool| {
            if is_read_only.get_untracked() || is_selected.get_untracked() == selected {
                return;
            }
            binding.set(selected);
            if let Some(on_change) = on_change {
                on_change.run(selected);
            }
        }),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    #[test]
    fn toggles_and_reports_changes() {
        with_owner(|| {
            let changes = RwSignal::new(Vec::new());
            let state = use_toggle_state(UseToggleStateInput {
                on_change: Some(Callback::new(move |s| changes.update(|c| c.push(s)))),
                ..UseToggleStateInput::default()
            });
            state.toggle();
            state.set_selected(true);
            state.toggle();
            assert_that!(state.is_selected.get_untracked()).is_false();
            assert_that!(changes.get_untracked()).is_equal_to(vec![true, false]);
        });
    }

    #[test]
    fn read_only_ignores_changes() {
        with_owner(|| {
            let state = use_toggle_state(UseToggleStateInput {
                default_selected: true,
                is_read_only: Signal::stored(true),
                ..UseToggleStateInput::default()
            });
            state.toggle();
            assert_that!(state.is_selected.get_untracked()).is_true();
            assert_that!(state.default_selected).is_true();
        });
    }
}
