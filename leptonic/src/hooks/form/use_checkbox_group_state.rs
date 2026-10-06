// Upstream: react-stately/src/checkbox/useCheckboxGroupState.ts @ 99e6102368
use std::collections::HashMap;

use leptos::prelude::*;

use super::use_form_validation_state::{
    UseFormValidationStateInput, UseFormValidationStateReturn, ValidateFn, ValidationBehavior,
    ValidationResult, merge_validation, use_form_validation_state,
};
use crate::hooks::collections::Key;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Values are `Key`s (strings or integers), not strings: what collections and toggle button
//   groups use; they render as the checkboxes' form values.
// - State (C4): `default_value` + `on_change`, or `value` bound to app state (a `ValueBinding`,
//   the atoms' `value` + `set_value`).
// - The validation behavior is set here (react-aria: on `useCheckboxGroup`), so the state and
//   its items read it from one place.
//
// =============================================================================

/// Input of [`use_checkbox_group_state`].
#[derive(Clone)]
pub struct UseCheckboxGroupStateInput {
    /// The initially checked values. Ignored when `value` is bound.
    pub default_value: Vec<Key>,
    /// The checked values as app state, replacing `default_value`.
    pub value: Option<crate::utils::ValueBinding<Vec<Key>>>,
    /// Called with the checked values when they change.
    pub on_change: Option<Callback<Vec<Key>>>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    /// At least one checkbox must be checked.
    pub is_required: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<Vec<Key>>>,
    pub validation_behavior: ValidationBehavior,
    /// The `name` of the checkboxes (for form submission and server errors).
    pub name: Option<String>,
}

impl Default for UseCheckboxGroupStateInput {
    fn default() -> Self {
        Self {
            default_value: Vec::new(),
            value: None,
            on_change: None,
            is_disabled: Signal::stored(false),
            is_read_only: Signal::stored(false),
            is_required: Signal::stored(false),
            is_invalid: Signal::stored(false),
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            name: None,
        }
    }
}

impl std::fmt::Debug for UseCheckboxGroupStateInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseCheckboxGroupStateInput")
            .field("default_value", &self.default_value)
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

/// The state of a checkbox group: its checked values and validation.
#[derive(Clone, Copy)]
pub struct CheckboxGroupState {
    /// The checked values, in the order they were checked.
    pub value: Signal<Vec<Key>>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    /// Whether a value is required: the group is required and nothing is checked.
    pub is_required: Signal<bool>,
    /// Whether the group's displayed validation is invalid.
    pub is_invalid: Signal<bool>,
    pub validation: UseFormValidationStateReturn,
    pub validation_behavior: ValidationBehavior,
    default_value: StoredValue<Vec<Key>>,
    name: StoredValue<Option<String>>,
    set_value: crate::utils::ValueBinding<Vec<Key>>,
    on_change: Option<Callback<Vec<Key>>>,
    invalid_values: StoredValue<HashMap<Key, ValidationResult>>,
}

impl std::fmt::Debug for CheckboxGroupState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CheckboxGroupState")
            .field("value", &self.value.get_untracked())
            .finish_non_exhaustive()
    }
}

impl CheckboxGroupState {
    /// Whether `value` is checked.
    pub fn is_selected(&self, value: &Key) -> bool {
        self.value.with(|v| v.contains(value))
    }

    /// The initially checked values (restored on form reset).
    pub fn default_value(&self) -> Vec<Key> {
        self.default_value.get_value()
    }

    /// The checkboxes' `name`.
    pub fn name(&self) -> Option<String> {
        self.name.get_value()
    }

    fn can_change(&self) -> bool {
        !self.is_disabled.get_untracked() && !self.is_read_only.get_untracked()
    }

    fn update(&self, value: Vec<Key>) {
        if self.value.with_untracked(|v| *v == value) {
            return;
        }
        self.set_value.set(value.clone());
        if let Some(on_change) = self.on_change {
            on_change.run(value);
        }
    }

    /// Replace the checked values.
    pub fn set_value(&self, value: Vec<Key>) {
        if self.can_change() {
            self.update(value);
        }
    }

    /// Check `value`.
    pub fn add_value(&self, value: Key) {
        if self.can_change() && !self.value.with_untracked(|v| v.contains(&value)) {
            let mut values = self.value.get_untracked();
            values.push(value);
            self.update(values);
        }
    }

    /// Uncheck `value`.
    pub fn remove_value(&self, value: &Key) {
        if self.can_change() && self.value.with_untracked(|v| v.contains(value)) {
            let mut values = self.value.get_untracked();
            values.retain(|v| v != value);
            self.update(values);
        }
    }

    /// Check or uncheck `value`.
    pub fn toggle_value(&self, value: Key) {
        if self.value.with_untracked(|v| v.contains(&value)) {
            self.remove_value(&value);
        } else {
            self.add_value(value);
        }
    }

    /// Record the (native) validity of the checkbox for `value`; the group's validity merges
    /// those of its checkboxes.
    pub fn set_invalid(&self, value: Key, validation: ValidationResult) {
        let merged = self.invalid_values.try_update_value(|invalid| {
            if validation.is_invalid {
                invalid.insert(value, validation);
            } else {
                invalid.remove(&value);
            }
            merge_validation(&invalid.values().cloned().collect::<Vec<_>>())
        });
        if let Some(merged) = merged {
            self.validation.update_validation.run(merged);
        }
    }
}

/// Creates the state of a checkbox group.
pub fn use_checkbox_group_state(input: UseCheckboxGroupStateInput) -> CheckboxGroupState {
    let UseCheckboxGroupStateInput {
        default_value,
        value,
        on_change,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        name,
    } = input;
    let set_value =
        value.unwrap_or_else(|| crate::utils::ValueBinding::from(RwSignal::new(default_value)));
    let value = set_value.value;
    let default_value = value.get_untracked();
    let validation = use_form_validation_state(UseFormValidationStateInput {
        is_invalid,
        value,
        validate,
        validation_behavior,
        name: name.clone(),
    });
    CheckboxGroupState {
        value,
        is_disabled,
        is_read_only,
        is_required: Signal::derive(move || is_required.get() && value.with(Vec::is_empty)),
        is_invalid: validation.is_invalid,
        validation,
        validation_behavior,
        default_value: StoredValue::new(default_value),
        name: StoredValue::new(name),
        set_value,
        on_change,
        invalid_values: StoredValue::new(HashMap::new()),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn keys(keys: &[&str]) -> Vec<Key> {
        keys.iter().map(|k| Key::from(*k)).collect()
    }

    #[test]
    fn adds_removes_and_toggles_values_in_order() {
        Owner::new().with(|| {
            let state = use_checkbox_group_state(UseCheckboxGroupStateInput {
                default_value: keys(&["b"]),
                ..UseCheckboxGroupStateInput::default()
            });
            state.add_value(Key::from("a"));
            state.add_value(Key::from("a"));
            state.toggle_value(Key::from("c"));
            state.remove_value(&Key::from("b"));
            assert_that!(state.value.get_untracked()).is_equal_to(keys(&["a", "c"]));
            assert_that!(state.is_selected(&Key::from("c"))).is_true();
            assert_that!(state.default_value()).is_equal_to(keys(&["b"]));
        });
    }

    #[test]
    fn read_only_and_disabled_groups_keep_their_value() {
        Owner::new().with(|| {
            for (is_read_only, is_disabled) in [(true, false), (false, true)] {
                let state = use_checkbox_group_state(UseCheckboxGroupStateInput {
                    is_read_only: Signal::stored(is_read_only),
                    is_disabled: Signal::stored(is_disabled),
                    ..UseCheckboxGroupStateInput::default()
                });
                state.add_value(Key::from("a"));
                state.set_value(keys(&["b"]));
                assert_that!(state.value.get_untracked()).is_empty();
            }
        });
    }

    #[test]
    fn required_until_something_is_checked() {
        Owner::new().with(|| {
            let state = use_checkbox_group_state(UseCheckboxGroupStateInput {
                is_required: Signal::stored(true),
                ..UseCheckboxGroupStateInput::default()
            });
            assert_that!(state.is_required.get_untracked()).is_true();
            state.add_value(Key::from("a"));
            assert_that!(state.is_required.get_untracked()).is_false();
        });
    }
}
