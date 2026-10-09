// Upstream: react-stately/src/tooltip/useTooltipTriggerState.ts @ 99e6102368
// Upstream: react-stately/test/tooltip/useTooltipTriggerState.test.js @ 99e6102368
use std::time::Duration;

use leptos::prelude::*;

#[cfg(not(feature = "ssr"))]
use super::tooltip_registry;
use super::tooltip_registry::{TOOLTIP_COOLDOWN, TOOLTIP_DELAY};
use crate::{
    ValueBinding,
    hooks::overlay::{OverlayTriggerState, UseOverlayTriggerStateInput, use_overlay_trigger_state},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state (C4): `default_open` + `on_open_change`, or `value` bound to app state,
//   instead of a controlled `isOpen`; the open state is an `OverlayTriggerState`.
// - A `Copy` struct of signals and methods (C3).
// - Delays are `Duration`s; `open`/`close` take a `TooltipTiming` instead of an `immediate` bool
//   (C10).
//
// ## DIFFERENT BEHAVIOR
// - The global warm-up state lives in thread-locals (`tooltip_registry`) instead of module
//   variables.
// - A pending warm-up of a disposed tooltip is cancelled (react-aria leaves the timer running,
//   harmless in JavaScript).
// - SSR: `open`/`close` change the state immediately, without timers.
//
// =============================================================================

/// Whether opening or closing a tooltip waits for its delay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipTiming {
    /// After the delay (`delay` when opening, `close_delay` when closing), as for hovering.
    #[default]
    Delayed,
    /// Right away, as for focus or Escape.
    Immediate,
}

/// Input of [`use_tooltip_trigger_state`].
#[derive(Debug, Clone, Copy)]
pub struct UseTooltipTriggerStateInput {
    /// How long hovering the trigger takes to open the first tooltip. Once one tooltip was open,
    /// others open right away until all were closed for a while.
    pub delay: Duration,
    /// How long the tooltip stays open after the pointer leaves.
    pub close_delay: Duration,
    /// Whether the tooltip starts open. Ignored when `value` is bound.
    pub default_open: bool,
    /// The open state as app state, replacing `default_open`.
    pub value: Option<ValueBinding<bool>>,
    /// Called when the tooltip opens or closes.
    pub on_open_change: Option<Callback<bool>>,
}

impl Default for UseTooltipTriggerStateInput {
    fn default() -> Self {
        Self {
            delay: TOOLTIP_DELAY,
            close_delay: TOOLTIP_COOLDOWN,
            default_open: false,
            value: None,
            on_open_change: None,
        }
    }
}

/// The state of a tooltip: whether it is open, and opening and closing it with react-aria's global
/// warm-up (the first tooltip waits for its delay, the next ones open right away) and cooldown.
/// Only one tooltip is open at a time.
#[derive(Debug, Clone, Copy)]
pub struct TooltipTriggerState {
    /// Whether the tooltip is open.
    pub overlay: OverlayTriggerState,
    /// Whether the current open or close transition should skip its animation: set when one
    /// tooltip replaces another during the warm-up period.
    pub should_skip_animation: Signal<bool>,
    open: Callback<TooltipTiming>,
    close: Callback<TooltipTiming>,
}

impl TooltipTriggerState {
    /// Whether the tooltip is open (tracked).
    pub fn is_open(&self) -> bool {
        self.overlay.is_open.get()
    }

    /// Opens the tooltip: after the warm-up delay (`Delayed`, unless another tooltip is warm), or
    /// right away (`Immediate`).
    pub fn open(&self, timing: TooltipTiming) {
        self.open.run(timing);
    }

    /// Closes the tooltip: after `close_delay` (`Delayed`), or right away (`Immediate`).
    pub fn close(&self, timing: TooltipTiming) {
        self.close.run(timing);
    }
}

/// Creates the state of a tooltip trigger (see [`TooltipTriggerState`]); use it with
/// `use_tooltip_trigger` and `use_tooltip`.
///
/// ```ignore
/// let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
///     delay: Duration::from_millis(300),
///     ..UseTooltipTriggerStateInput::default()
/// });
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_tooltip_trigger_state(input: UseTooltipTriggerStateInput) -> TooltipTriggerState {
    let UseTooltipTriggerStateInput {
        delay,
        close_delay,
        default_open,
        value,
        on_open_change,
    } = input;
    let overlay = use_overlay_trigger_state(UseOverlayTriggerStateInput {
        default_open,
        value,
        on_open_change,
    });
    let should_skip_animation = RwSignal::new(false);

    #[cfg(feature = "ssr")]
    {
        let _ = (delay, close_delay);
        TooltipTriggerState {
            overlay,
            should_skip_animation: should_skip_animation.into(),
            open: Callback::new(move |_| overlay.open()),
            close: Callback::new(move |_| overlay.close()),
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        let id = tooltip_registry::next_tooltip_id();
        let close_timeout: StoredValue<Option<leptos::prelude::TimeoutHandle>, LocalStorage> =
            StoredValue::new_local(None);
        let clear_close_timeout = move || {
            if let Some(handle) = close_timeout.try_get_value().flatten() {
                handle.clear();
            }
            close_timeout.try_set_value(None);
        };

        // Hides this tooltip: right away, or after `close_delay` (once: a pending close isn't
        // restarted). `instant` skips the exit animation.
        let hide_tooltip = move |immediate: bool, instant: bool| {
            should_skip_animation.try_set(instant);
            if immediate || close_delay.is_zero() {
                clear_close_timeout();
                overlay.close();
            } else if close_timeout.try_get_value().flatten().is_none()
                && let Ok(handle) = set_timeout_with_handle(
                    move || {
                        close_timeout.try_set_value(None);
                        overlay.close();
                    },
                    close_delay,
                )
            {
                close_timeout.try_set_value(Some(handle));
            }

            tooltip_registry::clear_warmup_timeout();
            if tooltip_registry::is_warmed_up()
                && let Ok(handle) = set_timeout_with_handle(
                    move || {
                        tooltip_registry::unregister_tooltip(id);
                        tooltip_registry::set_warmed_up(false);
                    },
                    close_delay.max(TOOLTIP_COOLDOWN),
                )
            {
                tooltip_registry::set_cooldown_timeout(handle);
            }
        };

        // This tooltip is the open one: others close right away, without animation.
        let ensure_tooltip_entry = move || {
            tooltip_registry::close_open_tooltips(id);
            tooltip_registry::register_tooltip(id, Box::new(move || hide_tooltip(true, true)));
        };

        let show_tooltip = move |instant: bool| {
            clear_close_timeout();
            ensure_tooltip_entry();
            should_skip_animation.try_set(instant);
            tooltip_registry::set_warmed_up(true);
            overlay.open();
            tooltip_registry::clear_warmup_timeout();
            tooltip_registry::clear_cooldown_timeout();
        };

        let warmup_tooltip = move || {
            ensure_tooltip_entry();
            let is_open = overlay.is_open.get_untracked();
            if !is_open && !tooltip_registry::is_warmed_up() {
                if let Ok(handle) = set_timeout_with_handle(
                    move || {
                        tooltip_registry::set_warmed_up(true);
                        // The first tooltip in a sequence animates in.
                        show_tooltip(false);
                    },
                    delay,
                ) {
                    tooltip_registry::set_warmup_timeout(id, handle);
                }
            } else if !is_open {
                // Already warmed up: appear instantly, without an animation.
                show_tooltip(true);
            }
        };

        on_cleanup(move || {
            clear_close_timeout();
            tooltip_registry::clear_warmup_timeout_of(id);
            tooltip_registry::unregister_tooltip(id);
        });

        TooltipTriggerState {
            overlay,
            should_skip_animation: should_skip_animation.into(),
            open: Callback::new(move |timing: TooltipTiming| {
                let has_pending_close = close_timeout.try_get_value().flatten().is_some();
                if timing == TooltipTiming::Delayed && !delay.is_zero() && !has_pending_close {
                    warmup_tooltip();
                } else {
                    // Immediate opens (focus, or no delay) skip the animation only if another
                    // tooltip is already warmed up.
                    show_tooltip(tooltip_registry::is_warmed_up());
                }
            }),
            close: Callback::new(move |timing: TooltipTiming| {
                hide_tooltip(timing == TooltipTiming::Immediate, false);
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    // Timers (delays, cooldown) need a browser; these cover what runs without them.

    #[test]
    fn immediate_open_shows_the_tooltip() {
        with_owner(|| {
            let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput::default());
            assert_that!(state.is_open()).is_false();
            state.open(TooltipTiming::Immediate);
            assert_that!(state.is_open()).is_true();
        });
    }

    #[test]
    fn a_bound_open_state_is_written() {
        with_owner(|| {
            let open = RwSignal::new(false);
            let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
                value: Some(open.into()),
                ..UseTooltipTriggerStateInput::default()
            });
            state.open(TooltipTiming::Immediate);
            assert_that!(open.get_untracked()).is_true();
            open.set(false);
            assert_that!(state.is_open()).is_false();
        });
    }
}
