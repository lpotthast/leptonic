// Upstream: react-stately/src/toggle/useToggleGroupState.ts @ 99e6102368
use std::collections::HashSet;

use leptos::prelude::*;

use crate::hooks::collections::Key;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `ToggleGroupSelectionMode` enum instead of the `'single' | 'multiple'` string union.
// - Hook-owned state (project-wide convention): no controlled `selectedKeys`.
//
// =============================================================================

/// How many toggle buttons of a group can be selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupSelectionMode {
    /// At most one (react-aria's default).
    #[default]
    Single,
    /// Any number.
    Multiple,
}

/// Input of [`use_toggle_group_state`].
#[derive(Debug, Clone)]
pub struct UseToggleGroupStateInput {
    pub selection_mode: ToggleGroupSelectionMode,
    /// Keeps at least one button selected.
    pub disallow_empty_selection: bool,
    /// The initially selected buttons.
    pub default_selected_keys: HashSet<Key>,
    /// Called with the selected buttons when they change.
    pub on_selection_change: Option<Callback<HashSet<Key>>>,
    pub is_disabled: Signal<bool>,
}

impl Default for UseToggleGroupStateInput {
    fn default() -> Self {
        Self {
            selection_mode: ToggleGroupSelectionMode::default(),
            disallow_empty_selection: false,
            default_selected_keys: HashSet::new(),
            on_selection_change: None,
            is_disabled: Signal::stored(false),
        }
    }
}

/// The state of a toggle button group: which buttons are selected.
#[derive(Debug, Clone, Copy)]
pub struct ToggleGroupState {
    pub selection_mode: ToggleGroupSelectionMode,
    pub is_disabled: Signal<bool>,
    /// The selected buttons.
    pub selected_keys: Signal<HashSet<Key>>,
    disallow_empty_selection: bool,
    set_keys: WriteSignal<HashSet<Key>>,
    on_selection_change: Option<Callback<HashSet<Key>>>,
}

impl ToggleGroupState {
    /// Whether the button `key` is selected.
    pub fn is_selected(&self, key: &Key) -> bool {
        self.selected_keys.with(|keys| keys.contains(key))
    }

    /// Replace the selected buttons.
    pub fn set_selected_keys(&self, keys: HashSet<Key>) {
        if self
            .selected_keys
            .with_untracked(|current| *current == keys)
        {
            return;
        }
        self.set_keys.set(keys.clone());
        if let Some(on_selection_change) = self.on_selection_change {
            on_selection_change.run(keys);
        }
    }

    /// Select or deselect the button `key` (respecting the selection mode and
    /// `disallow_empty_selection`).
    pub fn toggle_key(&self, key: &Key) {
        let current = self.selected_keys.get_untracked();
        let keys = match self.selection_mode {
            ToggleGroupSelectionMode::Multiple => {
                let mut keys = current.clone();
                if keys.contains(key) && (!self.disallow_empty_selection || keys.len() > 1) {
                    keys.remove(key);
                } else {
                    keys.insert(key.clone());
                }
                keys
            }
            ToggleGroupSelectionMode::Single => {
                if current.contains(key) && !self.disallow_empty_selection {
                    HashSet::new()
                } else {
                    HashSet::from([key.clone()])
                }
            }
        };
        self.set_selected_keys(keys);
    }

    /// Select or deselect the button `key`.
    pub fn set_selected(&self, key: &Key, is_selected: bool) {
        if is_selected != self.selected_keys.with_untracked(|keys| keys.contains(key)) {
            self.toggle_key(key);
        }
    }
}

/// Creates the state of a toggle button group.
pub fn use_toggle_group_state(input: UseToggleGroupStateInput) -> ToggleGroupState {
    let UseToggleGroupStateInput {
        selection_mode,
        disallow_empty_selection,
        default_selected_keys,
        on_selection_change,
        is_disabled,
    } = input;
    let (selected_keys, set_keys) = signal(default_selected_keys);
    ToggleGroupState {
        selection_mode,
        is_disabled,
        selected_keys: selected_keys.into(),
        disallow_empty_selection,
        set_keys,
        on_selection_change,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn keys(state: &ToggleGroupState) -> Vec<String> {
        let mut keys: Vec<String> = state
            .selected_keys
            .get_untracked()
            .iter()
            .map(ToString::to_string)
            .collect();
        keys.sort();
        keys
    }

    #[test]
    fn single_selection_replaces_and_deselects() {
        Owner::new().with(|| {
            let state = use_toggle_group_state(UseToggleGroupStateInput::default());
            state.toggle_key(&Key::from("a"));
            state.toggle_key(&Key::from("b"));
            assert_that!(keys(&state)).is_equal_to(vec!["b".to_owned()]);
            state.toggle_key(&Key::from("b"));
            assert_that!(keys(&state)).is_empty();
        });
    }

    #[test]
    fn multiple_selection_adds_and_removes() {
        Owner::new().with(|| {
            let state = use_toggle_group_state(UseToggleGroupStateInput {
                selection_mode: ToggleGroupSelectionMode::Multiple,
                ..UseToggleGroupStateInput::default()
            });
            state.toggle_key(&Key::from("a"));
            state.set_selected(&Key::from("b"), true);
            state.set_selected(&Key::from("b"), true);
            assert_that!(keys(&state)).is_equal_to(vec!["a".to_owned(), "b".to_owned()]);
            state.toggle_key(&Key::from("a"));
            assert_that!(keys(&state)).is_equal_to(vec!["b".to_owned()]);
        });
    }

    #[test]
    fn disallow_empty_selection_keeps_the_last_button() {
        Owner::new().with(|| {
            for selection_mode in [
                ToggleGroupSelectionMode::Single,
                ToggleGroupSelectionMode::Multiple,
            ] {
                let state = use_toggle_group_state(UseToggleGroupStateInput {
                    selection_mode,
                    disallow_empty_selection: true,
                    default_selected_keys: HashSet::from([Key::from("a")]),
                    ..UseToggleGroupStateInput::default()
                });
                state.toggle_key(&Key::from("a"));
                assert_that!(keys(&state)).is_equal_to(vec!["a".to_owned()]);
            }
        });
    }
}
