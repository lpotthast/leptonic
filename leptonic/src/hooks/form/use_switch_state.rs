use leptos::prelude::*;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/toggle/src/useToggleState.ts

// No intentional deviations from the react-aria implementation.

/// State for managing switch state.
#[derive(Clone, Copy)]
pub struct UseSwitchStateReturn {
    /// Whether the switch is selected.
    pub is_selected: Signal<bool>,

    /// Toggle the selection state.
    pub toggle: Callback<()>,

    /// Set the selection state.
    pub set_selected: Callback<bool>,
}

/// Creates internal state for a switch component.
pub fn use_switch_state(default_selected: bool) -> UseSwitchStateReturn {
    let (is_selected, set_is_selected) = signal(default_selected);

    UseSwitchStateReturn {
        is_selected: is_selected.into(),
        toggle: Callback::new(move |_| {
            set_is_selected.update(|v| *v = !*v);
        }),
        set_selected: Callback::new(move |selected| {
            set_is_selected.set(selected);
        }),
    }
}
