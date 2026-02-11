use leptos::prelude::*;

use super::use_time_field::TimeValue;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// State for managing time field.
#[derive(Clone, Copy)]
pub struct UseTimeFieldStateReturn {
    /// The current value.
    pub value: Signal<Option<TimeValue>>,

    /// Set the value.
    pub set_value: Callback<Option<TimeValue>>,

    /// Clear the value.
    pub clear: Callback<()>,
}

/// Creates internal state for a time field.
pub fn use_time_field_state(default_value: Option<TimeValue>) -> UseTimeFieldStateReturn {
    let (value, set_value_signal) = signal(default_value);

    UseTimeFieldStateReturn {
        value: value.into(),
        set_value: Callback::new(move |v| set_value_signal.set(v)),
        clear: Callback::new(move |_| set_value_signal.set(None)),
    }
}
