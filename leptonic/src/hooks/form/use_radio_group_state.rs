use leptos::prelude::*;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/radio/src/useRadioGroupState.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// State for managing radio group state.
#[derive(Clone, Copy)]
pub struct UseRadioGroupStateReturn<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// The current selected value.
    pub selected_value: Signal<Option<T>>,

    /// Set the selected value.
    pub set_selected: Callback<T>,
}

/// Creates internal state for a radio group component.
pub fn use_radio_group_state<T>(default_value: Option<T>) -> UseRadioGroupStateReturn<T>
where
    T: Clone + Send + Sync + 'static,
{
    let (selected, set_selected) = signal(default_value);

    UseRadioGroupStateReturn {
        selected_value: selected.into(),
        set_selected: Callback::new(move |value| {
            set_selected.set(Some(value));
        }),
    }
}
