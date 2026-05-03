use leptos::prelude::*;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/utils/src/useControlledState.ts

// No intentional deviations from the react-aria implementation.

/// State for managing text field state.
#[derive(Clone, Copy)]
pub struct UseTextFieldStateReturn {
    /// The current value.
    pub value: Signal<String>,

    /// Set the value.
    pub set_value: Callback<String>,

    /// Clear the value.
    pub clear: Callback<()>,
}

/// Creates internal state for a text field component.
pub fn use_text_field_state(default_value: &str) -> UseTextFieldStateReturn {
    let (value, set_value) = signal(default_value.to_string());

    UseTextFieldStateReturn {
        value: value.into(),
        set_value: Callback::new(move |v: String| {
            set_value.set(v);
        }),
        clear: Callback::new(move |_| {
            set_value.set(String::new());
        }),
    }
}
