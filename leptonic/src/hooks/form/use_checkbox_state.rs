use leptos::prelude::*;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/checkbox/src/useCheckboxGroupState.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// State for managing checkbox state.
#[derive(Clone, Copy)]
pub struct UseCheckboxStateReturn {
    /// Whether the checkbox is selected.
    pub is_selected: Signal<bool>,

    /// Toggle the selection state.
    pub toggle: Callback<()>,

    /// Set the selection state.
    pub set_selected: Callback<bool>,
}

/// Creates internal state for a checkbox component.
pub fn use_checkbox_state(default_selected: bool) -> UseCheckboxStateReturn {
    let (is_selected, set_is_selected) = signal(default_selected);

    UseCheckboxStateReturn {
        is_selected: is_selected.into(),
        toggle: Callback::new(move |_| {
            set_is_selected.update(|v| *v = !*v);
        }),
        set_selected: Callback::new(move |selected| {
            set_is_selected.set(selected);
        }),
    }
}
