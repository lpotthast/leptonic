//! Overlay hooks for popovers and overlay positioning.
//!
//! This module provides hooks for managing overlay behaviors including:
//! - Popovers (`use_popover`)
//! - Overlay dismiss handling (integrated into `use_overlay`)
//! - Overlay positioning (`use_overlay`, `use_overlay_position`, `use_overlay_trigger`)
//!
//! ## Hook Composition
//!
//! The `use_popover` hook is a composition hook that combines:
//! - `use_overlay` for dismiss handling (Escape, click outside, blur) and overlay stacking
//! - `use_overlay_position` for positioning relative to a trigger
//! - `use_prevent_scroll` for scroll prevention (when modal)
//!
//! Additionally, `use_close_on_scroll` closes an overlay when the trigger's
//! scrollable ancestors scroll (preventing stale positioning).
//!
//! See each hook's deviation block for how it differs from react-aria. Module-wide: focus
//! containment is the `FocusScope` atom (react-aria: the `FocusScope` component inside its
//! `Overlay`), rendered by the overlay atoms.

mod calculate_position;
pub(crate) mod overlay_focus_contain;
pub(crate) mod use_close_on_scroll;
pub(crate) mod use_overlay;
pub(crate) mod use_overlay_position;
pub(crate) mod use_overlay_trigger;
pub(crate) mod use_overlay_trigger_state;
pub(crate) mod use_popover;
pub(crate) mod use_prevent_scroll;
mod visible_overlays;

pub use overlay_focus_contain::*;
pub use use_close_on_scroll::*;
pub use use_overlay::*;
pub use use_overlay_position::*;
pub use use_overlay_trigger::*;
pub use use_overlay_trigger_state::*;
pub use use_popover::*;
pub use use_prevent_scroll::*;
