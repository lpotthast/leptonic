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
