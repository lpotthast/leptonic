// Upstream: react-stately/src/overlays/useOverlayTriggerState.ts @ 99e6102368
use leptos::prelude::*;

use crate::{Point, ValueBinding};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state (C4): `default_open` + `on_open_change`, or `value` bound to app state
//   (`ValueBinding::from(rw_signal)`), instead of a controlled `isOpen`.
// - A `Copy` struct of signals and methods (C3).
// - `point` is a typed [`Point`] (C13).
//
// =============================================================================

/// Input of [`use_overlay_trigger_state`].
#[derive(Debug, Clone, Copy, Default)]
pub struct UseOverlayTriggerStateInput {
    /// Whether the overlay starts open. Ignored when `value` is bound.
    pub default_open: bool,
    /// The open state as app state, replacing `default_open`.
    pub value: Option<ValueBinding<bool>>,
    /// Called when the overlay opens or closes.
    pub on_open_change: Option<Callback<bool>>,
}

/// Whether an overlay (popover, modal, menu, ...) is open, and the point it opened at.
#[derive(Debug, Clone, Copy)]
pub struct OverlayTriggerState {
    pub is_open: Signal<bool>,
    /// Where a point-anchored overlay (e.g. a context menu) opened.
    pub point: Signal<Option<Point>>,
    set_open: Callback<bool>,
    set_point: Callback<Option<Point>>,
}

impl OverlayTriggerState {
    pub fn set_open(&self, is_open: bool) {
        self.set_open.run(is_open);
    }

    pub fn open(&self) {
        self.set_open(true);
    }

    pub fn close(&self) {
        self.set_open(false);
    }

    pub fn toggle(&self) {
        self.set_open(!self.is_open.get_untracked());
    }

    pub fn set_point(&self, point: Option<Point>) {
        self.set_point.run(point);
    }
}

/// A state that opens and closes an overlay (popover, modal, ...): the overlay's
/// [`OverlayTriggerState`], or a component state adding its own closing logic (a select's, a combo
/// box's). Overlay hooks (`use_popover`) are generic over it (react-aria: structural typing of
/// `OverlayTriggerState`).
pub trait OverlayState: Copy + Send + Sync + 'static {
    /// Whether the overlay is open (tracked).
    fn is_open(&self) -> bool;
    /// Closes the overlay.
    fn close(&self);
    /// Where a point-anchored overlay (a context menu) opened. Default: none.
    fn point(&self) -> Signal<Option<Point>> {
        Signal::stored(None)
    }
}

impl OverlayState for OverlayTriggerState {
    fn is_open(&self) -> bool {
        self.is_open.get()
    }

    fn point(&self) -> Signal<Option<Point>> {
        self.point
    }

    fn close(&self) {
        OverlayTriggerState::close(self);
    }
}

/// Manages whether an overlay is open.
pub fn use_overlay_trigger_state(input: UseOverlayTriggerStateInput) -> OverlayTriggerState {
    let UseOverlayTriggerStateInput {
        default_open,
        value,
        on_open_change,
    } = input;
    let binding = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_open)));
    let is_open = binding.value;
    let point = RwSignal::new(None);
    OverlayTriggerState {
        is_open,
        point: point.into(),
        set_open: Callback::new(move |new: bool| {
            // As react-aria's `useControlledState`: only an actual change is applied and reported.
            if is_open.get_untracked() == new {
                return;
            }
            binding.set(new);
            if let Some(on_open_change) = on_open_change {
                on_open_change.run(new);
            }
        }),
        set_point: Callback::new(move |new| point.set(new)),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    #[test]
    fn opens_closes_and_toggles_reporting_changes_only() {
        with_owner(|| {
            let changes = RwSignal::new(Vec::new());
            let state = use_overlay_trigger_state(UseOverlayTriggerStateInput {
                on_open_change: Some(Callback::new(move |v| changes.update(|c| c.push(v)))),
                ..UseOverlayTriggerStateInput::default()
            });
            state.open();
            state.open();
            state.toggle();
            state.close();
            assert_that!(state.is_open.get_untracked()).is_false();
            assert_that!(changes.get_untracked()).is_equal_to(vec![true, false]);
        });
    }

    #[test]
    fn a_bound_value_is_read_and_written() {
        with_owner(|| {
            let app = RwSignal::new(true);
            let state = use_overlay_trigger_state(UseOverlayTriggerStateInput {
                value: Some(ValueBinding::from(app)),
                ..UseOverlayTriggerStateInput::default()
            });
            assert_that!(state.is_open.get_untracked()).is_true();
            state.close();
            assert_that!(app.get_untracked()).is_false();
        });
    }
}
