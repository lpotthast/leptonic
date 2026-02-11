use leptos::prelude::*;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/tooltip/src/useTooltipTriggerState.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// State for managing tooltip visibility.
#[derive(Clone, Copy)]
pub struct UseTooltipTriggerStateReturn {
    /// Whether the tooltip is open.
    pub is_open: Signal<bool>,

    /// Open the tooltip.
    pub open: Callback<()>,

    /// Close the tooltip.
    pub close: Callback<()>,
}

/// Creates internal state for a tooltip trigger.
pub fn use_tooltip_trigger_state(default_open: bool) -> UseTooltipTriggerStateReturn {
    let (is_open, set_is_open) = signal(default_open);

    UseTooltipTriggerStateReturn {
        is_open: is_open.into(),
        open: Callback::new(move |_| set_is_open.set(true)),
        close: Callback::new(move |_| set_is_open.set(false)),
    }
}
