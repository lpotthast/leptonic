use leptos::prelude::*;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/searchfield/src/useSearchFieldState.ts

// No intentional deviations from the react-aria implementation.

/// State for managing search field state.
#[derive(Clone, Copy)]
pub struct UseSearchFieldStateReturn {
    /// The current value.
    pub value: Signal<String>,

    /// Set the value.
    pub set_value: Callback<String>,

    /// Clear the value.
    pub clear: Callback<()>,
}

/// Creates internal state for a search field component.
pub fn use_search_field_state() -> UseSearchFieldStateReturn {
    let (value, set_value) = signal(String::new());

    UseSearchFieldStateReturn {
        value: value.into(),
        set_value: Callback::new(move |v: String| {
            set_value.set(v);
        }),
        clear: Callback::new(move |_| {
            set_value.set(String::new());
        }),
    }
}
