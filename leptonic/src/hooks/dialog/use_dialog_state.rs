use leptos::prelude::*;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/overlays/src/useOverlayTriggerState.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - `useControlledState` / controlled `isOpen` prop: React Aria supports both
//   controlled (`isOpen` from parent) and uncontrolled (`defaultOpen`) patterns
//   because React components cannot share mutable state. In Leptos, `Signal<T>`
//   is `Copy` and inherently shared, so controlled state is unnecessary. The
//   hook always owns its `WriteSignal` internally and exposes a read-only
//   `Signal<bool>`. This ensures `on_open_change` always fires and the hook
//   can enforce invariants. See documentation/hooks-implementation.md for the
//   full rationale.
//
// =============================================================================

/// Input for [`use_dialog_state`].
#[derive(Debug, Default, Clone, Copy)]
pub struct UseDialogStateInput {
    /// Initial open state. Defaults to `false`.
    pub default_open: bool,

    /// Called whenever the open state changes.
    pub on_open_change: Option<Callback<bool>>,
}

/// State for managing dialog visibility and confirmed state.
#[derive(Clone, Copy)]
pub struct UseDialogStateReturn {
    /// Whether the dialog is open.
    pub is_open: Signal<bool>,

    /// Set the open state directly.
    pub set_open: Callback<bool>,

    /// Open the dialog.
    pub open: Callback<()>,

    /// Close the dialog.
    pub close: Callback<()>,

    /// Toggle the dialog.
    pub toggle: Callback<()>,

    /// Close the dialog with a confirmed result.
    pub confirm: Callback<()>,

    /// Whether the dialog was confirmed (vs cancelled).
    pub is_confirmed: Signal<bool>,
}

/// Creates internal state for a dialog component with confirmation tracking.
pub fn use_dialog_state(input: UseDialogStateInput) -> UseDialogStateReturn {
    let UseDialogStateInput {
        default_open,
        on_open_change,
    } = input;

    let (is_open, set_is_open) = signal(default_open);
    let (is_confirmed, set_is_confirmed) = signal(false);

    let update_open = move |value: bool| {
        set_is_open.set(value);
        if let Some(cb) = on_open_change {
            cb.run(value);
        }
    };

    UseDialogStateReturn {
        is_open: is_open.into(),
        set_open: Callback::new(update_open),
        open: Callback::new(move |_| {
            set_is_confirmed.set(false);
            update_open(true);
        }),
        close: Callback::new(move |_| {
            update_open(false);
        }),
        toggle: Callback::new(move |_| {
            update_open(!is_open.get_untracked());
        }),
        confirm: Callback::new(move |_| {
            set_is_confirmed.set(true);
            update_open(false);
        }),
        is_confirmed: is_confirmed.into(),
    }
}
