//! Merged Props types for composing multiple hooks.
//!
//! This module provides pre-defined merged Props types for common hook combinations.
//! Each merged type contains fields from multiple hooks, with appropriate merge
//! semantics applied:
//!
//! - Event handlers are chained (both run in sequence)
//! - Element capture attributes are kept (both capture refs)
//! - Other overlapping attributes use "last wins" semantics
//!
//! # Available Merged Types
//!
//! - [`MergedPressHoverProps`]: Combines `UsePressProps` + `UseHoverProps`
//! - [`MergedPressHoverFocusRingProps`]: Combines the above + `UseFocusRingProps`
//! - [`MergedButtonMenuTriggerProps`]: Combines `UseButtonProps` + `UseMenuTriggerProps`
//! - [`MergedOverlayOverlayPositionProps`]: Combines `UseOverlayProps` + `UseOverlayPositionProps`
//!
//! # Usage
//!
//! ```ignore
//! use leptonic::utils::MergeWith;
//! use leptonic::hooks::{use_press, use_hover, use_focus_ring};
//!
//! let press = use_press(press_input);
//! let hover = use_hover(hover_input);
//! let focus_ring = use_focus_ring(focus_ring_input);
//!
//! // Two-way merge
//! let press_hover = press.props.merge_with(hover.props);
//!
//! // Three-way merge
//! let combined = press.props
//!     .merge_with(hover.props)
//!     .merge_with(focus_ring.props);
//!
//! view! {
//!     <button {..combined.into_attrs()}>
//!         "Interactive button"
//!     </button>
//! }
//! ```

mod button_menu_trigger;
mod focusable_press;
mod focusable_press_focus_ring;
mod hover_focus_ring;
mod overlay_overlay_position;
mod press_focus_ring;
mod press_hover;
mod press_hover_focus_ring;

pub use button_menu_trigger::*;
pub use focusable_press::*;
pub use focusable_press_focus_ring::*;
pub use hover_focus_ring::*;
pub use overlay_overlay_position::*;
pub use press_focus_ring::*;
pub use press_hover::*;
pub use press_hover_focus_ring::*;
