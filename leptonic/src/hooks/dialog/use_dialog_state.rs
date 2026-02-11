use leptos::prelude::*;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/overlays/src/useOverlayTriggerState.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// State for managing dialog visibility and confirmed state.
#[derive(Clone, Copy)]
pub struct UseDialogStateReturn {
    /// Whether the dialog is open.
    pub is_open: Signal<bool>,

    /// Open the dialog.
    pub open: Callback<()>,

    /// Close the dialog.
    pub close: Callback<()>,

    /// Close the dialog with a confirmed result.
    pub confirm: Callback<()>,

    /// Whether the dialog was confirmed (vs cancelled).
    pub is_confirmed: Signal<bool>,
}

/// Creates internal state for a dialog component with confirmation tracking.
pub fn use_dialog_state(default_open: bool) -> UseDialogStateReturn {
    let (is_open, set_is_open) = signal(default_open);
    let (is_confirmed, set_is_confirmed) = signal(false);

    UseDialogStateReturn {
        is_open: is_open.into(),
        open: Callback::new(move |_| {
            set_is_confirmed.set(false);
            set_is_open.set(true);
        }),
        close: Callback::new(move |_| {
            set_is_open.set(false);
        }),
        confirm: Callback::new(move |_| {
            set_is_confirmed.set(true);
            set_is_open.set(false);
        }),
        is_confirmed: is_confirmed.into(),
    }
}
