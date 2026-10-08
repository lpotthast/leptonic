// Upstream: react-stately/src/utils/useControlledState.ts @ 99e6102368
use leptos::prelude::*;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state (C4): `default_value` + `on_change`, or `value` bound to app state (the atoms'
//   `value` + `set_value`); every change goes through `set_value`.
//
// =============================================================================

/// The value of a text field.
#[derive(Debug, Clone, Copy)]
pub struct TextFieldState {
    pub value: Signal<String>,
    set_value: Callback<String>,
}

impl TextFieldState {
    /// A state whose value lives elsewhere (e.g. in a combo box state).
    pub fn new(value: Signal<String>, set_value: Callback<String>) -> Self {
        Self { value, set_value }
    }

    pub fn set_value(&self, value: String) {
        self.set_value.run(value);
    }

    /// This state, also calling `on_change` when the value changes.
    #[must_use]
    pub fn with_on_change(self, on_change: Callback<String>) -> Self {
        Self::new(
            self.value,
            Callback::new(move |value: String| {
                if self.value.with_untracked(|v| *v == value) {
                    return;
                }
                self.set_value(value.clone());
                on_change.run(value);
            }),
        )
    }
}

/// Binds the field to a signal (as Leptos' `bind:value` does).
impl From<RwSignal<String>> for TextFieldState {
    fn from(signal: RwSignal<String>) -> Self {
        Self::new(signal.into(), Callback::new(move |value| signal.set(value)))
    }
}

/// Binds the field to a signal pair (as Leptos' `bind:value` does).
impl From<(ReadSignal<String>, WriteSignal<String>)> for TextFieldState {
    fn from((read, write): (ReadSignal<String>, WriteSignal<String>)) -> Self {
        Self::new(read.into(), Callback::new(move |value| write.set(value)))
    }
}

/// Input of [`use_text_field_state`].
#[derive(Debug, Clone, Default)]
pub struct UseTextFieldStateInput {
    /// The initial value. Ignored when `value` is bound.
    pub default_value: String,
    /// The value as app state, replacing `default_value`.
    pub value: Option<crate::utils::ValueBinding<String>>,
    /// Called when the value changes.
    pub on_change: Option<Callback<String>>,
}

/// Creates the state of a text field.
pub fn use_text_field_state(input: UseTextFieldStateInput) -> TextFieldState {
    let UseTextFieldStateInput {
        default_value,
        value,
        on_change,
    } = input;
    let binding =
        value.unwrap_or_else(|| crate::utils::ValueBinding::from(RwSignal::new(default_value)));
    let value = binding.value;
    TextFieldState {
        value,
        set_value: Callback::new(move |new: String| {
            if value.with_untracked(|v| *v != new) {
                binding.set(new.clone());
                if let Some(on_change) = on_change {
                    on_change.run(new);
                }
            }
        }),
    }
}

#[cfg(test)]
mod tests {
    // Upstream: react-stately/test/utils/useControlledState.test.tsx.
    use assertr::prelude::*;

    use super::*;
    use crate::{testing::with_owner, utils::ValueBinding};

    /// The values `on_change` was called with.
    fn recorder() -> (RwSignal<Vec<String>>, Callback<String>) {
        let calls = RwSignal::new(Vec::new());
        let callback = Callback::new(move |value| calls.update(|c| c.push(value)));
        (calls, callback)
    }

    fn value(state: &TextFieldState) -> String {
        state.value.get_untracked()
    }

    // Upstream: "can handle default setValue behavior, wont invoke onChange for the same value
    // twice in a row".
    #[test]
    fn owned_value_reports_only_changes() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let state = use_text_field_state(UseTextFieldStateInput {
                default_value: "defaultValue".to_owned(),
                value: None,
                on_change: Some(on_change),
            });
            assert_that!(value(&state)).is_equal_to("defaultValue".to_owned());
            assert_that!(changes.get_untracked()).is_empty();

            state.set_value("newValue".to_owned());
            assert_that!(value(&state)).is_equal_to("newValue".to_owned());
            state.set_value("newValue2".to_owned());
            state.set_value("newValue2".to_owned());
            assert_that!(value(&state)).is_equal_to("newValue2".to_owned());
            // A new value, but not the last one: reported.
            state.set_value("newValue".to_owned());
            assert_that!(changes.get_untracked()).is_equal_to(vec![
                "newValue".to_owned(),
                "newValue2".to_owned(),
                "newValue".to_owned(),
            ]);
        });
    }

    // Upstream: "can handle controlled setValue behavior", "can handle controlled callback
    // setValue behavior after prop change".
    #[test]
    fn a_read_only_binding_keeps_its_value_and_reports_other_values() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let controlled = RwSignal::new("controlledValue".to_owned());
            let state = use_text_field_state(UseTextFieldStateInput {
                default_value: "defaultValue".to_owned(),
                // The app doesn't take the changes.
                value: Some(ValueBinding::new(controlled.into(), Callback::new(|_| {}))),
                on_change: Some(on_change),
            });
            assert_that!(value(&state)).is_equal_to("controlledValue".to_owned());

            state.set_value("newValue".to_owned());
            assert_that!(value(&state)).is_equal_to("controlledValue".to_owned());
            assert_that!(changes.get_untracked()).is_equal_to(vec!["newValue".to_owned()]);
            // The controlled value: no change.
            state.set_value("controlledValue".to_owned());
            assert_that!(changes.get_untracked().len()).is_equal_to(1);

            // The app changes its value: compared with the new one.
            controlled.set("updated".to_owned());
            assert_that!(value(&state)).is_equal_to("updated".to_owned());
            state.set_value("newValue".to_owned());
            state.set_value("newValue".to_owned());
            assert_that!(changes.get_untracked().len()).is_equal_to(3);
            state.set_value("updated".to_owned());
            assert_that!(changes.get_untracked().len()).is_equal_to(3);
        });
    }

    #[test]
    fn a_bound_signal_takes_the_changes() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let app = RwSignal::new("a".to_owned());
            let state = use_text_field_state(UseTextFieldStateInput {
                default_value: String::new(),
                value: Some(ValueBinding::from(app)),
                on_change: Some(on_change),
            });
            state.set_value("b".to_owned());
            assert_that!(app.get_untracked()).is_equal_to("b".to_owned());
            state.set_value("b".to_owned());
            assert_that!(changes.get_untracked()).is_equal_to(vec!["b".to_owned()]);
        });
    }

    #[test]
    fn a_signal_state_with_on_change_reports_only_changes() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let app = RwSignal::new("a".to_owned());
            let state = TextFieldState::from(app).with_on_change(on_change);
            state.set_value("a".to_owned());
            state.set_value("b".to_owned());
            state.set_value("b".to_owned());
            assert_that!(app.get_untracked()).is_equal_to("b".to_owned());
            assert_that!(changes.get_untracked()).is_equal_to(vec!["b".to_owned()]);
        });
    }
}
