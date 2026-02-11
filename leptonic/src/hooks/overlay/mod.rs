//! Overlay hooks for modals, dialogs, popovers, and tooltips.
//!
//! This module provides hooks for managing overlay behaviors including:
//! - Popovers (`use_popover`)
//! - Dismiss handling (`use_dismiss`)
//! - Overlay positioning (`use_overlay`, `use_overlay_position`, `use_overlay_trigger`)
//! - Tooltips (`use_tooltip_trigger`)
//!
//! ## Hook Composition
//!
//! The `use_popover` hook is a composition hook that combines:
//! - `use_overlay_position` for positioning relative to a trigger
//! - `use_prevent_scroll` for scroll prevention (when modal)
//! - Dismiss handling (Escape key + click outside)
//!
//! ## React-aria Deviations
//!
//! This implementation is partially based on [React Aria's overlay hooks](https://github.com/adobe/react-spectrum/tree/main/packages/@react-aria/overlays)
//! (rest in ./modal) but has the following deviations:
//!
//! ### Not Implemented
//!
//! - **Safari iOS `VoiceOver` workarounds**: React Aria includes specific workarounds for Safari
//!   on iOS to prevent `VoiceOver` from escaping focus traps. These are not yet implemented.
//!
//! - **`usePreventScroll` iOS Safari handling**: React Aria has extensive iOS Safari workarounds
//!   for scroll prevention. Our implementation may not work correctly on all iOS versions.
//!
//! ### Differences
//!
//! - **`FocusScope` as separate atom**: React Aria's `FocusScope` is a hook. In Leptonic, it's
//!   provided as a component/atom (`<FocusScope>`) that wraps modal content.

pub mod use_dismiss;
pub mod use_dismiss_button;
pub mod use_overlay;
pub mod use_overlay_position;
pub mod use_overlay_trigger;
pub mod use_popover;
pub mod use_tooltip_trigger;
pub mod use_tooltip_trigger_state;

pub use use_dismiss::*;
pub use use_dismiss_button::*;
pub use use_overlay::*;
pub use use_overlay_position::*;
pub use use_overlay_trigger::*;
pub use use_popover::*;
pub use use_tooltip_trigger::*;
pub use use_tooltip_trigger_state::*;
