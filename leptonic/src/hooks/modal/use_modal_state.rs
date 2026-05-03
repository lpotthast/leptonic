use leptos::prelude::*;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/overlays/src/useOverlayTriggerState.ts

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

/// Input for [`use_modal_state`].
#[derive(Debug, Default, Clone, Copy)]
pub struct UseModalStateInput {
    /// Initial open state. Defaults to `false`.
    pub default_open: bool,

    /// Called whenever the open state changes.
    pub on_open_change: Option<Callback<bool>>,
}

/// State for managing modal visibility.
#[derive(Clone, Copy)]
pub struct UseModalStateReturn {
    /// Whether the modal is open.
    pub is_open: Signal<bool>,

    /// Set the open state directly.
    pub set_open: Callback<bool>,

    /// Open the modal.
    pub open: Callback<()>,

    /// Close the modal.
    pub close: Callback<()>,

    /// Toggle the modal.
    pub toggle: Callback<()>,
}

/// Creates internal state for a modal component.
pub fn use_modal_state(input: UseModalStateInput) -> UseModalStateReturn {
    let UseModalStateInput {
        default_open,
        on_open_change,
    } = input;

    let (is_open, set_is_open) = signal(default_open);

    let update_open = move |value: bool| {
        set_is_open.set(value);
        if let Some(cb) = on_open_change {
            cb.run(value);
        }
    };

    UseModalStateReturn {
        is_open: is_open.into(),
        set_open: Callback::new(update_open),
        open: Callback::new(move |_| update_open(true)),
        close: Callback::new(move |_| update_open(false)),
        toggle: Callback::new(move |_| update_open(!is_open.get_untracked())),
    }
}
