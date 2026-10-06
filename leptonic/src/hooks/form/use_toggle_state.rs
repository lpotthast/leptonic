// Upstream: react-stately/src/toggle/useToggleState.ts @ 99e6102368
use leptos::prelude::*;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state (C4): `default_selected` + `on_change`, or `value` bound to app state (the
//   atoms' `is_selected` + `set_selected`). A state driven by something else (a group's value, a
//   collection's selection) is built with `ToggleState::new`, which delegates changes to a
//   callback.
//
// =============================================================================

/// The state of a toggle (checkbox, switch, toggle button): whether it is selected.
#[derive(Debug, Clone, Copy)]
pub struct ToggleState {
    /// Whether the toggle is selected.
    pub is_selected: Signal<bool>,
    /// The initial selection, restored on form reset.
    pub default_selected: bool,
    set_selected: Callback<bool>,
}

impl ToggleState {
    /// A state that reads its selection from `is_selected` and hands changes to `set_selected`
    /// (for toggles whose selection lives elsewhere, e.g. in a group).
    pub fn new(
        is_selected: Signal<bool>,
        default_selected: bool,
        set_selected: Callback<bool>,
    ) -> Self {
        Self {
            is_selected,
            default_selected,
            set_selected,
        }
    }

    /// Select or deselect the toggle.
    pub fn set_selected(&self, is_selected: bool) {
        self.set_selected.run(is_selected);
    }

    /// Flip the selection.
    pub fn toggle(&self) {
        self.set_selected(!self.is_selected.get_untracked());
    }

    /// The same state, also calling `on_change` with each changed selection (for a state from
    /// elsewhere, like the `on_change` of [`use_toggle_state`]).
    #[must_use]
    pub fn with_on_change(self, on_change: Callback<bool>) -> Self {
        Self::new(
            self.is_selected,
            self.default_selected,
            Callback::new(move |selected: bool| {
                if selected == self.is_selected.get_untracked() {
                    return;
                }
                self.set_selected(selected);
                on_change.run(selected);
            }),
        )
    }
}

/// Binds the toggle to a signal (as Leptos' `bind:checked` does): it reads and writes the signal.
impl From<RwSignal<bool>> for ToggleState {
    fn from(signal: RwSignal<bool>) -> Self {
        Self::new(
            signal.into(),
            signal.get_untracked(),
            Callback::new(move |selected| signal.set(selected)),
        )
    }
}

/// Binds the toggle to a signal pair (as Leptos' `bind:checked` does).
impl From<(ReadSignal<bool>, WriteSignal<bool>)> for ToggleState {
    fn from((read, write): (ReadSignal<bool>, WriteSignal<bool>)) -> Self {
        Self::new(
            read.into(),
            read.get_untracked(),
            Callback::new(move |selected| write.set(selected)),
        )
    }
}

/// Input of [`use_toggle_state`].
#[derive(Debug, Clone, Copy)]
pub struct UseToggleStateInput {
    /// Whether the toggle is initially selected. Ignored when `value` is bound.
    pub default_selected: bool,
    /// The selection as app state, replacing `default_selected`.
    pub value: Option<crate::utils::ValueBinding<bool>>,
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
        value.unwrap_or_else(|| crate::utils::ValueBinding::from(RwSignal::new(default_selected)));
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

    #[test]
    fn toggles_and_reports_changes() {
        Owner::new().with(|| {
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
        Owner::new().with(|| {
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
