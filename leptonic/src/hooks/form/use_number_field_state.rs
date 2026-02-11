use leptos::prelude::*;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/numberfield/src/useNumberFieldState.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// State for managing number field state.
#[derive(Clone, Copy)]
pub struct UseNumberFieldStateReturn {
    /// The current value.
    pub value: Signal<Option<f64>>,

    /// Set the value.
    pub set_value: Callback<Option<f64>>,

    /// Increment by a step.
    pub increment: Callback<f64>,

    /// Decrement by a step.
    pub decrement: Callback<f64>,

    /// Clear the value.
    pub clear: Callback<()>,
}

/// Creates internal state for a number field component.
pub fn use_number_field_state(default_value: Option<f64>) -> UseNumberFieldStateReturn {
    let (value, set_value) = signal(default_value);

    UseNumberFieldStateReturn {
        value: value.into(),
        set_value: Callback::new(move |v: Option<f64>| {
            set_value.set(v);
        }),
        increment: Callback::new(move |step: f64| {
            set_value.update(|v| {
                *v = Some(v.unwrap_or(0.0) + step);
            });
        }),
        decrement: Callback::new(move |step: f64| {
            set_value.update(|v| {
                *v = Some(v.unwrap_or(0.0) - step);
            });
        }),
        clear: Callback::new(move |_| {
            set_value.set(None);
        }),
    }
}
