use leptos::prelude::*;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/overlays/src/useOverlayTriggerState.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// State for managing modal visibility.
#[derive(Clone, Copy)]
pub struct UseModalStateReturn {
    /// Whether the modal is open.
    pub is_open: Signal<bool>,

    /// Open the modal.
    pub open: Callback<()>,

    /// Close the modal.
    pub close: Callback<()>,

    /// Toggle the modal.
    pub toggle: Callback<()>,
}

/// Creates internal state for a modal component.
pub fn use_modal_state(default_open: bool) -> UseModalStateReturn {
    let (is_open, set_is_open) = signal(default_open);

    UseModalStateReturn {
        is_open: is_open.into(),
        open: Callback::new(move |_| set_is_open.set(true)),
        close: Callback::new(move |_| set_is_open.set(false)),
        toggle: Callback::new(move |_| set_is_open.update(|v| *v = !*v)),
    }
}
