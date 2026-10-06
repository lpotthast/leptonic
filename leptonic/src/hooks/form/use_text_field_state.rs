// Upstream: react-stately/src/utils/useControlledState.ts @ 99e6102368
use leptos::prelude::*;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state instead of react-aria's controlled/uncontrolled `value`: the field's value
//   lives in this state, changed through `set_value`. Reason: project-wide convention (C4); app
//   state binds through `From<RwSignal<String>>` instead of a controlled `value`.
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
    pub default_value: String,
    /// Called when the value changes.
    pub on_change: Option<Callback<String>>,
}

/// Creates the state of a text field.
pub fn use_text_field_state(input: UseTextFieldStateInput) -> TextFieldState {
    let UseTextFieldStateInput {
        default_value,
        on_change,
    } = input;
    let value = RwSignal::new(default_value);
    TextFieldState {
        value: value.into(),
        set_value: Callback::new(move |new: String| {
            if value.with_untracked(|v| *v != new) {
                value.set(new.clone());
                if let Some(on_change) = on_change {
                    on_change.run(new);
                }
            }
        }),
    }
}
