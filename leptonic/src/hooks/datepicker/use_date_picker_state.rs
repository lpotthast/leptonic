use leptos::prelude::*;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// State for managing date picker.
#[derive(Clone, Copy)]
pub struct UseDatePickerStateReturn {
    /// The current value.
    pub value: Signal<Option<time::OffsetDateTime>>,

    /// Set the value.
    pub set_value: Callback<Option<time::OffsetDateTime>>,

    /// Whether the picker is open.
    pub is_open: Signal<bool>,

    /// Open the picker.
    pub open: Callback<()>,

    /// Close the picker.
    pub close: Callback<()>,

    /// Clear the value.
    pub clear: Callback<()>,
}

/// Creates internal state for a date picker.
pub fn use_date_picker_state(
    default_value: Option<time::OffsetDateTime>,
) -> UseDatePickerStateReturn {
    let (value, set_value_signal) = signal(default_value);
    let (is_open, set_is_open) = signal(false);

    UseDatePickerStateReturn {
        value: value.into(),
        set_value: Callback::new(move |v| set_value_signal.set(v)),
        is_open: is_open.into(),
        open: Callback::new(move |_| set_is_open.set(true)),
        close: Callback::new(move |_| set_is_open.set(false)),
        clear: Callback::new(move |_| set_value_signal.set(None)),
    }
}
