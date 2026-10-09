// Upstream: react-stately/src/checkbox/useCheckboxGroupState.ts @ 99e6102368
// Upstream: react-stately/test/checkbox/useCheckboxGroupState.test.tsx @ 99e6102368

use std::collections::HashSet;

use leptos::prelude::*;

use super::use_form_validation_state::{
    FormValidationState, UseFormValidationStateInput, ValidateFn, ValidationBehavior,
    ValidationResult, merge_validation, use_form_validation_state,
};
use crate::hooks::collections::Key;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Values are `Key`s (strings or integers), not strings: what collections and toggle button
//   groups use; they render as the checkboxes' form values. The checked values are a
//   `HashSet<Key>` (react-aria: an array in the order checked; set-valued values are sets, C16).
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
    pub default_value: HashSet<Key>,
    /// The checked values as app state, replacing `default_value`.
    pub value: Option<crate::ValueBinding<HashSet<Key>>>,
    /// Called with the checked values when they change.
    pub on_change: Option<Callback<HashSet<Key>>>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    /// At least one checkbox must be checked.
    pub is_required: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<HashSet<Key>>>,
    pub validation_behavior: ValidationBehavior,
    /// The `name` of the checkboxes (for form submission and server errors).
    pub name: Option<String>,
}

impl Default for UseCheckboxGroupStateInput {
    fn default() -> Self {
        Self {
            default_value: HashSet::new(),
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
    /// The checked values.
    pub value: Signal<HashSet<Key>>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    /// Whether a value is required: the group is required and nothing is checked.
    pub is_required: Signal<bool>,
    /// Whether the group's displayed validation is invalid.
    pub is_invalid: Signal<bool>,
    pub validation: FormValidationState,
    pub validation_behavior: ValidationBehavior,
    default_value: StoredValue<HashSet<Key>>,
    name: StoredValue<Option<String>>,
    set_value: crate::ValueBinding<HashSet<Key>>,
    on_change: Option<Callback<HashSet<Key>>>,
    /// The invalid checkboxes' validations, in the order they became invalid (react-aria: a
    /// `Map`, which keeps insertion order), so the group's errors keep that order.
    invalid_values: StoredValue<Vec<(Key, ValidationResult)>>,
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
    pub fn default_value(&self) -> HashSet<Key> {
        self.default_value.get_value()
    }

    /// The checkboxes' `name`.
    pub fn name(&self) -> Option<String> {
        self.name.get_value()
    }

    fn can_change(&self) -> bool {
        !self.is_disabled.get_untracked() && !self.is_read_only.get_untracked()
    }

    fn update(&self, value: HashSet<Key>) {
        if self.value.with_untracked(|v| *v == value) {
            return;
        }
        self.set_value.set(value.clone());
        if let Some(on_change) = self.on_change {
            on_change.run(value);
        }
    }

    /// Replace the checked values.
    pub fn set_value(&self, value: HashSet<Key>) {
        if self.can_change() {
            self.update(value);
        }
    }

    /// Check `value`.
    pub fn add_value(&self, value: Key) {
        if self.can_change() && !self.value.with_untracked(|v| v.contains(&value)) {
            let mut values = self.value.get_untracked();
            values.insert(value);
            self.update(values);
        }
    }

    /// Uncheck `value`.
    pub fn remove_value(&self, value: &Key) {
        if self.can_change() && self.value.with_untracked(|v| v.contains(value)) {
            let mut values = self.value.get_untracked();
            values.remove(value);
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
            let existing = invalid.iter_mut().find(|(other, _)| *other == value);
            match (validation.is_invalid, existing) {
                (true, Some((_, entry))) => *entry = validation,
                (true, None) => invalid.push((value, validation)),
                (false, _) => invalid.retain(|(other, _)| *other != value),
            }
            merge_validation(
                &invalid
                    .iter()
                    .map(|(_, validation)| validation.clone())
                    .collect::<Vec<_>>(),
            )
        });
        if let Some(merged) = merged {
            self.validation.update_validation(merged);
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
        value.unwrap_or_else(|| crate::ValueBinding::from(RwSignal::new(default_value)));
    let value = set_value.value;
    let default_value = value.get_untracked();
    let validation = use_form_validation_state(UseFormValidationStateInput {
        builtin_validation: Signal::default(),
        is_invalid,
        value,
        validate,
        validation_behavior,
        names: name.clone().into_iter().collect(),
    });
    CheckboxGroupState {
        value,
        is_disabled,
        is_read_only,
        is_required: Signal::derive(move || is_required.get() && value.with(HashSet::is_empty)),
        is_invalid: validation.is_invalid,
        validation,
        validation_behavior,
        default_value: StoredValue::new(default_value),
        name: StoredValue::new(name),
        set_value,
        on_change,
        invalid_values: StoredValue::new(Vec::new()),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::{
        super::use_form_validation_state::{DEFAULT_VALIDATION_RESULT, ValidityStateSnapshot},
        *,
    };
    use crate::testing::with_owner;

    fn keys(keys: &[&str]) -> HashSet<Key> {
        keys.iter().map(|k| Key::from(*k)).collect()
    }

    #[test]
    fn adds_removes_and_toggles_values() {
        with_owner(|| {
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

    // "should be possible to control the value", "should call the provided `onChange` callback
    // whenever value changes".
    #[test]
    fn a_bound_value_is_read_and_changes_are_reported() {
        with_owner(|| {
            let app = RwSignal::new(keys(&["a"]));
            let changes = RwSignal::new(Vec::new());
            let state = use_checkbox_group_state(UseCheckboxGroupStateInput {
                value: Some(crate::ValueBinding::from(app)),
                on_change: Some(Callback::new(move |v| changes.update(|c| c.push(v)))),
                ..UseCheckboxGroupStateInput::default()
            });
            assert_that!(state.is_selected(&Key::from("a"))).is_true();
            state.add_value(Key::from("b"));
            assert_that!(app.get_untracked()).is_equal_to(keys(&["a", "b"]));
            app.set(keys(&["c"]));
            assert_that!(state.value.get_untracked()).is_equal_to(keys(&["c"]));
            state.remove_value(&Key::from("c"));
            assert_that!(changes.get_untracked())
                .is_equal_to(vec![keys(&["a", "b"]), HashSet::new()]);
        });
    }

    // "should go back to the initial value after `toggleState` being called twice on the same
    // value".
    #[test]
    fn toggling_twice_restores_the_value() {
        with_owner(|| {
            let state = use_checkbox_group_state(UseCheckboxGroupStateInput {
                default_value: keys(&["a", "b"]),
                ..UseCheckboxGroupStateInput::default()
            });
            state.toggle_value(Key::from("a"));
            state.toggle_value(Key::from("a"));
            assert_that!(state.value.get_untracked()).is_equal_to(keys(&["a", "b"]));
        });
    }

    #[test]
    fn read_only_and_disabled_groups_keep_their_value() {
        with_owner(|| {
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
        with_owner(|| {
            let state = use_checkbox_group_state(UseCheckboxGroupStateInput {
                is_required: Signal::stored(true),
                ..UseCheckboxGroupStateInput::default()
            });
            assert_that!(state.is_required.get_untracked()).is_true();
            state.add_value(Key::from("a"));
            assert_that!(state.is_required.get_untracked()).is_false();
        });
    }
    #[test]
    fn errors_keep_the_order_checkboxes_became_invalid() {
        with_owner(|| {
            let state = use_checkbox_group_state(UseCheckboxGroupStateInput {
                validation_behavior: ValidationBehavior::Aria,
                ..UseCheckboxGroupStateInput::default()
            });
            let invalid = |message: &str| ValidationResult {
                is_invalid: true,
                validation_errors: vec![message.to_owned()],
                validation_details: ValidityStateSnapshot::default(),
            };
            for key in ["z", "m", "a", "q", "b"] {
                state.set_invalid(Key::from(key), invalid(key));
            }
            state.set_invalid(Key::from("a"), DEFAULT_VALIDATION_RESULT);
            state.set_invalid(Key::from("m"), invalid("m2"));
            assert_that!(state.validation.validation_errors.get_untracked())
                .is_equal_to(["z", "m2", "q", "b"].map(str::to_owned).to_vec());
        });
    }
}
