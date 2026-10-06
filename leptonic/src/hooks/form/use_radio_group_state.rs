// Upstream: react-stately/src/radio/useRadioGroupState.ts @ 99e6102368
use leptos::prelude::*;

use super::use_form_validation_state::{
    UseFormValidationStateInput, UseFormValidationStateReturn, ValidateFn, ValidationBehavior,
    use_form_validation_state,
};
use crate::{hooks::collections::Key, utils::id::use_id};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Values are `Key`s (strings or integers), not strings: what collections and toggle button
//   groups use; they render as the radios' form values.
// - Hook-owned state (project-wide convention): no controlled `value`.
// - The validation behavior is set here (react-aria: on `useRadioGroup`), so the state and its
//   radios read it from one place.
// - The generated group name comes from `use_id` (hydration-stable). React-aria: a random
//   instance number plus a counter.
//
// =============================================================================

/// Input of [`use_radio_group_state`].
#[derive(Clone)]
pub struct UseRadioGroupStateInput {
    /// The initially selected value.
    pub default_value: Option<Key>,
    /// Called with the selected value when it changes.
    pub on_change: Option<Callback<Option<Key>>>,
    /// The radios' `name` (for form submission). Generated when `None`.
    pub name: Option<String>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_required: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<Option<Key>>>,
    pub validation_behavior: ValidationBehavior,
}

impl Default for UseRadioGroupStateInput {
    fn default() -> Self {
        Self {
            default_value: None,
            on_change: None,
            name: None,
            is_disabled: Signal::stored(false),
            is_read_only: Signal::stored(false),
            is_required: Signal::stored(false),
            is_invalid: Signal::stored(false),
            validate: None,
            validation_behavior: ValidationBehavior::default(),
        }
    }
}

impl std::fmt::Debug for UseRadioGroupStateInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseRadioGroupStateInput")
            .field("default_value", &self.default_value)
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

/// The state of a radio group: its selected value, the radio focused last, and validation.
#[derive(Clone, Copy)]
pub struct RadioGroupState {
    /// The selected value.
    pub selected_value: Signal<Option<Key>>,
    /// The radio focused last (decides the group's tab stop while nothing is selected).
    pub last_focused_value: Signal<Option<Key>>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_required: Signal<bool>,
    /// Whether the group's displayed validation is invalid.
    pub is_invalid: Signal<bool>,
    pub validation: UseFormValidationStateReturn,
    pub validation_behavior: ValidationBehavior,
    name: StoredValue<String>,
    default_selected_value: StoredValue<Option<Key>>,
    set_selected: WriteSignal<Option<Key>>,
    set_last_focused: WriteSignal<Option<Key>>,
    on_change: Option<Callback<Option<Key>>>,
}

impl std::fmt::Debug for RadioGroupState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RadioGroupState")
            .field("selected_value", &self.selected_value.get_untracked())
            .finish_non_exhaustive()
    }
}

impl RadioGroupState {
    /// The radios' `name`.
    pub fn name(&self) -> String {
        self.name.get_value()
    }

    /// The initially selected value (restored on form reset).
    pub fn default_selected_value(&self) -> Option<Key> {
        self.default_selected_value.get_value()
    }

    /// Select `value` (ignored while the group is disabled or read-only).
    pub fn set_selected_value(&self, value: Option<Key>) {
        if self.is_disabled.get_untracked() || self.is_read_only.get_untracked() {
            return;
        }
        if self.selected_value.with_untracked(|v| *v != value) {
            self.set_selected.set(value.clone());
            if let Some(on_change) = self.on_change {
                on_change.run(value);
            }
        }
        self.validation.commit_validation.run(());
    }

    /// Record the radio focused last.
    pub fn set_last_focused_value(&self, value: Option<Key>) {
        self.set_last_focused.set(value);
    }
}

/// Creates the state of a radio group.
pub fn use_radio_group_state(input: UseRadioGroupStateInput) -> RadioGroupState {
    let UseRadioGroupStateInput {
        default_value,
        on_change,
        name,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
    } = input;
    let name = name.unwrap_or_else(|| use_id("radio-group"));
    let (selected_value, set_selected) = signal(default_value.clone());
    let (last_focused_value, set_last_focused) = signal(None);
    let validation = use_form_validation_state(UseFormValidationStateInput {
        is_invalid,
        value: selected_value.into(),
        validate,
        validation_behavior,
        name: Some(name.clone()),
    });
    RadioGroupState {
        selected_value: selected_value.into(),
        last_focused_value: last_focused_value.into(),
        is_disabled,
        is_read_only,
        is_required,
        is_invalid: validation.is_invalid,
        validation,
        validation_behavior,
        name: StoredValue::new(name),
        default_selected_value: StoredValue::new(default_value),
        set_selected,
        set_last_focused,
        on_change,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn selects_values_unless_read_only_or_disabled() {
        Owner::new().with(|| {
            let changes = RwSignal::new(Vec::new());
            let is_read_only = RwSignal::new(false);
            let state = use_radio_group_state(UseRadioGroupStateInput {
                default_value: Some(Key::from("a")),
                on_change: Some(Callback::new(move |v| changes.update(|c| c.push(v)))),
                is_read_only: is_read_only.into(),
                ..UseRadioGroupStateInput::default()
            });
            state.set_selected_value(Some(Key::from("a")));
            state.set_selected_value(Some(Key::from("b")));
            is_read_only.set(true);
            state.set_selected_value(Some(Key::from("c")));
            assert_that!(state.selected_value.get_untracked()).is_equal_to(Some(Key::from("b")));
            assert_that!(changes.get_untracked()).is_equal_to(vec![Some(Key::from("b"))]);
            assert_that!(state.default_selected_value()).is_equal_to(Some(Key::from("a")));
        });
    }

    #[test]
    fn named_explicitly_or_generated() {
        Owner::new().with(|| {
            let named = use_radio_group_state(UseRadioGroupStateInput {
                name: Some("plan".to_owned()),
                ..UseRadioGroupStateInput::default()
            });
            assert_that!(named.name()).is_equal_to("plan".to_owned());
            let generated = use_radio_group_state(UseRadioGroupStateInput::default());
            assert_that!(generated.name().is_empty()).is_false();
        });
    }
}
