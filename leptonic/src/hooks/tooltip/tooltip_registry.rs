//! Thread-local global state for the tooltip warmup/cooldown system.
//!
//! This mirrors react-aria's module-level state in `useTooltipTriggerState.ts`.
//! The warmup/cooldown system ensures:
//! - The first tooltip requires a delay before appearing (cold state)
//! - Once a tooltip has been shown, subsequent tooltips appear instantly (warm state)
//! - After all tooltips close, a cooldown period resets back to cold state
//! - Only one tooltip is visible at a time (singleton enforcement)

#[cfg(not(feature = "ssr"))]
use std::cell::{Cell, RefCell};

#[cfg(not(feature = "ssr"))]
use leptos::prelude::TimeoutHandle;

/// Default delay before showing the first tooltip (in ms).
pub const TOOLTIP_DELAY: u32 = 1500;

/// Default cooldown period after all tooltips close before resetting warmup state (in ms).
pub const TOOLTIP_COOLDOWN: u32 = 500;

#[cfg(not(feature = "ssr"))]
thread_local! {
    /// Map of tooltip ID → force-close callback. When a new tooltip opens,
    /// it calls all other registered tooltips' close callbacks.
    static TOOLTIPS: RefCell<Vec<(u64, Box<dyn Fn()>)>> = const { RefCell::new(Vec::new()) };

    /// Auto-incrementing ID counter for tooltip registration.
    static TOOLTIP_ID_COUNTER: Cell<u64> = const { Cell::new(0) };

    /// Whether the warmup period has elapsed and subsequent tooltips
    /// should appear without delay.
    static GLOBAL_WARMED_UP: Cell<bool> = const { Cell::new(false) };

    /// Pending warmup timer handle. Set when the first tooltip starts its
    /// delay; cleared when the tooltip is shown or cancelled.
    static GLOBAL_WARMUP_TIMEOUT: Cell<Option<TimeoutHandle>> = const { Cell::new(None) };

    /// Pending cooldown timer handle. Set when all tooltips close; cleared
    /// when a new tooltip opens or when the cooldown expires.
    static GLOBAL_COOLDOWN_TIMEOUT: Cell<Option<TimeoutHandle>> = const { Cell::new(None) };
}

/// Allocate a new unique tooltip ID.
#[cfg(not(feature = "ssr"))]
pub(super) fn next_tooltip_id() -> u64 {
    TOOLTIP_ID_COUNTER.with(|c| {
        let id = c.get();
        c.set(id.wrapping_add(1));
        id
    })
}

/// Register a tooltip with its force-close callback.
#[cfg(not(feature = "ssr"))]
pub(super) fn register_tooltip(id: u64, close_fn: Box<dyn Fn()>) {
    TOOLTIPS.with_borrow_mut(|tooltips| {
        // Replace if already registered (shouldn't happen, but be safe).
        if let Some(pos) = tooltips.iter().position(|(tid, _)| *tid == id) {
            tooltips[pos] = (id, close_fn);
        } else {
            tooltips.push((id, close_fn));
        }
    });
}

/// Unregister a tooltip.
#[cfg(not(feature = "ssr"))]
pub(super) fn unregister_tooltip(id: u64) {
    TOOLTIPS.with_borrow_mut(|tooltips| {
        tooltips.retain(|(tid, _)| *tid != id);
    });
}

/// Close all open tooltips except the one with the given ID.
#[cfg(not(feature = "ssr"))]
pub(super) fn close_open_tooltips(except_id: u64) {
    TOOLTIPS.with_borrow(|tooltips| {
        for (tid, close_fn) in tooltips {
            if *tid != except_id {
                close_fn();
            }
        }
    });
}

/// Check if the system is in warmed-up state (subsequent tooltips skip delay).
#[cfg(not(feature = "ssr"))]
pub(super) fn is_warmed_up() -> bool {
    GLOBAL_WARMED_UP.with(Cell::get)
}

/// Set the warmed-up state.
#[cfg(not(feature = "ssr"))]
pub(super) fn set_warmed_up(value: bool) {
    GLOBAL_WARMED_UP.with(|c| c.set(value));
}

/// Clear the pending warmup timeout, if any.
#[cfg(not(feature = "ssr"))]
pub(super) fn clear_warmup_timeout() {
    GLOBAL_WARMUP_TIMEOUT.with(|c| {
        if let Some(handle) = c.take() {
            handle.clear();
        }
    });
}

/// Clear the pending cooldown timeout, if any.
#[cfg(not(feature = "ssr"))]
pub(super) fn clear_cooldown_timeout() {
    GLOBAL_COOLDOWN_TIMEOUT.with(|c| {
        if let Some(handle) = c.take() {
            handle.clear();
        }
    });
}

/// Set a warmup timeout. Clears any existing one first.
#[cfg(not(feature = "ssr"))]
pub(super) fn set_warmup_timeout(handle: TimeoutHandle) {
    clear_warmup_timeout();
    GLOBAL_WARMUP_TIMEOUT.with(|c| c.set(Some(handle)));
}

/// Set a cooldown timeout. Clears any existing one first.
#[cfg(not(feature = "ssr"))]
pub(super) fn set_cooldown_timeout(handle: TimeoutHandle) {
    clear_cooldown_timeout();
    GLOBAL_COOLDOWN_TIMEOUT.with(|c| c.set(Some(handle)));
}
