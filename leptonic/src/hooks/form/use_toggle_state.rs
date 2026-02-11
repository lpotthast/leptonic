use leptos::prelude::*;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/toggle/src/useToggleState.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// State for managing toggle state with internal signal.
#[derive(Clone, Copy)]
pub struct UseToggleStateReturn {
    /// Whether the toggle is selected.
    pub is_selected: Signal<bool>,

    /// Toggle the selection state.
    pub toggle: Callback<()>,

    /// Set the selection state.
    pub set_selected: Callback<bool>,
}

/// Creates internal state for a toggle component.
///
/// Use this when you want uncontrolled toggle state.
///
/// # Example
///
/// ```ignore
/// let state = use_toggle_state(false);
///
/// let toggle = use_toggle(UseToggleInput {
///     is_selected: state.is_selected,
///     on_change: Some(Callback::new(move |selected| {
///         state.set_selected.run(selected);
///     })),
///     ..Default::default()
/// });
/// ```
pub fn use_toggle_state(default_selected: bool) -> UseToggleStateReturn {
    let (is_selected, set_is_selected) = signal(default_selected);

    UseToggleStateReturn {
        is_selected: is_selected.into(),
        toggle: Callback::new(move |_| {
            set_is_selected.update(|v| *v = !*v);
        }),
        set_selected: Callback::new(move |selected| {
            set_is_selected.set(selected);
        }),
    }
}
