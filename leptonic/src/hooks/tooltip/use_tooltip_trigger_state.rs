#[cfg(not(feature = "ssr"))]
use std::time::Duration;

use leptos::prelude::*;

#[cfg(not(feature = "ssr"))]
use super::tooltip_registry;
use super::tooltip_registry::{TOOLTIP_COOLDOWN, TOOLTIP_DELAY};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/tooltip/src/useTooltipTriggerState.ts

//
// ## LEPTOS-SPECIFIC ADAPTATIONS
//
// - Thread-local global state (`tooltip_registry.rs`) instead of module-level
//   JS variables. Uses `thread_local!` with `Cell`/`RefCell` for the warmup,
//   cooldown, and tooltip registry state.
//
// - `Callback<bool>` for `open`/`close` instead of `() => void`. The bool
//   parameter indicates whether the operation should be immediate (skip delays).
//
// - `Signal<bool>` for controlled `is_open` instead of React's controlled props
//   pattern.
//
// - `StoredValue<Option<TimeoutHandle>, LocalStorage>` for the per-instance
//   close timeout instead of a React ref.
//
// - SSR branch: When `#[cfg(feature = "ssr")]`, `open`/`close` simply toggle
//   the signal immediately with no timers or global state.
//

/// Input parameters for the `use_tooltip_trigger_state` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseTooltipTriggerStateInput {
    /// Delay before showing the tooltip (in ms). Default: 1500ms.
    pub delay: u32,

    /// Delay before hiding the tooltip (in ms). Default: 500ms.
    pub close_delay: u32,

    /// Controlled open state. When `Some`, the tooltip open state is externally controlled.
    pub is_open: Option<Signal<bool>>,

    /// Default open state for uncontrolled mode.
    pub default_open: bool,

    /// Callback when the open state changes.
    pub on_open_change: Option<Callback<bool>>,
}

impl Default for UseTooltipTriggerStateInput {
    fn default() -> Self {
        Self {
            delay: TOOLTIP_DELAY,
            close_delay: TOOLTIP_COOLDOWN,
            is_open: None,
            default_open: false,
            on_open_change: None,
        }
    }
}

/// State for managing tooltip visibility with warmup/cooldown behavior.
///
/// This implements react-aria's global warmup/cooldown system:
/// - The first tooltip requires a delay before appearing (cold state)
/// - Once a tooltip has been shown, subsequent tooltips appear instantly (warm state)
/// - After all tooltips close, a cooldown period resets back to cold state
/// - Only one tooltip is visible at a time
#[derive(Clone, Copy)]
pub struct UseTooltipTriggerStateReturn {
    /// Whether the tooltip is open.
    pub is_open: Signal<bool>,

    /// Open the tooltip. The `bool` parameter indicates whether to open immediately
    /// (`true` = skip warmup delay, `false` = respect warmup delay).
    pub open: Callback<bool>,

    /// Close the tooltip. The `bool` parameter indicates whether to close immediately
    /// (`true` = skip close delay, `false` = respect close delay).
    pub close: Callback<bool>,
}

/// Creates state for a tooltip trigger with warmup/cooldown behavior.
///
/// This hook manages the open/close state of a tooltip with react-aria's global
/// warmup/cooldown system. It should be used together with `use_tooltip_trigger`.
///
/// # Example
///
/// ```ignore
/// let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
///     delay: 300,
///     close_delay: 100,
///     ..Default::default()
/// });
/// let trigger = use_tooltip_trigger(UseTooltipTriggerInput::default(), state);
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_tooltip_trigger_state(
    input: UseTooltipTriggerStateInput,
) -> UseTooltipTriggerStateReturn {
    let UseTooltipTriggerStateInput {
        delay,
        close_delay,
        is_open: controlled_is_open,
        default_open,
        on_open_change,
    } = input;

    let (internal_open, set_internal_open) = signal(default_open);
    let is_open: Signal<bool> = controlled_is_open.unwrap_or_else(|| internal_open.into());

    let update_open = move |new_open: bool| {
        set_internal_open.set(new_open);
        if let Some(on_change) = on_open_change {
            on_change.run(new_open);
        }
    };

    #[cfg(feature = "ssr")]
    {
        let _ = delay;
        let _ = close_delay;
        return UseTooltipTriggerStateReturn {
            is_open,
            open: Callback::new(move |_immediate: bool| {
                update_open(true);
            }),
            close: Callback::new(move |_immediate: bool| {
                update_open(false);
            }),
        };
    }

    #[cfg(not(feature = "ssr"))]
    {
        let tooltip_id = tooltip_registry::next_tooltip_id();

        // Per-instance close timeout handle.
        let close_timeout: StoredValue<Option<TimeoutHandle>, LocalStorage> =
            StoredValue::new_local(None);

        // Helper: clear the per-instance close timeout.
        let clear_close_timeout = move || {
            if let Some(handle) = close_timeout.get_value() {
                handle.clear();
                close_timeout.set_value(None);
            }
        };

        // show_tooltip: immediately show the tooltip and update global state.
        let show_tooltip = move || {
            clear_close_timeout();
            tooltip_registry::close_open_tooltips(tooltip_id);
            tooltip_registry::register_tooltip(
                tooltip_id,
                Box::new(move || {
                    update_open(false);
                }),
            );
            tooltip_registry::set_warmed_up(true);
            update_open(true);
            tooltip_registry::clear_warmup_timeout();
            tooltip_registry::clear_cooldown_timeout();
        };

        // warmup_tooltip: handle the warmup logic — if cold, start a delay timer;
        // if warm, show immediately.
        let warmup_tooltip = move || {
            tooltip_registry::close_open_tooltips(tooltip_id);
            tooltip_registry::register_tooltip(
                tooltip_id,
                Box::new(move || {
                    update_open(false);
                }),
            );

            if tooltip_registry::is_warmed_up() {
                // Warm state: show immediately.
                show_tooltip();
            } else {
                // Cold state: start the warmup delay.
                if let Ok(handle) = set_timeout_with_handle(
                    move || {
                        tooltip_registry::set_warmed_up(true);
                        show_tooltip();
                    },
                    Duration::from_millis(u64::from(delay)),
                ) {
                    tooltip_registry::set_warmup_timeout(handle);
                }
            }
        };

        // hide_tooltip: immediately hide the tooltip.
        let hide_tooltip = move || {
            clear_close_timeout();
            update_open(false);
        };

        let open = Callback::new(move |immediate: bool| {
            if immediate || delay == 0 {
                show_tooltip();
            } else {
                // If there's a pending close timeout, we're transitioning between
                // tooltips — show immediately (the close was pending but cancelled).
                let has_pending_close = close_timeout.get_value().is_some();
                if has_pending_close {
                    show_tooltip();
                } else {
                    warmup_tooltip();
                }
            }
        });

        let close = Callback::new(move |immediate: bool| {
            if immediate || close_delay == 0 {
                hide_tooltip();
            } else {
                // Schedule delayed close.
                if let Ok(handle) = set_timeout_with_handle(
                    move || {
                        hide_tooltip();
                    },
                    Duration::from_millis(u64::from(close_delay)),
                ) {
                    close_timeout.set_value(Some(handle));
                }
            }

            // Clear any pending warmup (user moved away before warmup completed).
            tooltip_registry::clear_warmup_timeout();

            // If warmed up, start the cooldown timer.
            if tooltip_registry::is_warmed_up() {
                let cooldown_duration = u64::from(close_delay.max(TOOLTIP_COOLDOWN));
                if let Ok(handle) = set_timeout_with_handle(
                    move || {
                        tooltip_registry::set_warmed_up(false);
                    },
                    Duration::from_millis(cooldown_duration),
                ) {
                    tooltip_registry::set_cooldown_timeout(handle);
                }
            }
        });

        on_cleanup(move || {
            clear_close_timeout();
            tooltip_registry::unregister_tooltip(tooltip_id);
        });

        UseTooltipTriggerStateReturn {
            is_open,
            open,
            close,
        }
    }
}
