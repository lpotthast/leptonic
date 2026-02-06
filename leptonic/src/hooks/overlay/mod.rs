//! Overlay hooks for modals, dialogs, popovers, and tooltips.
//!
//! This module provides hooks for managing overlay behaviors including:
//! - Modal dialogs (`use_modal`, `use_modal_backdrop`, `use_dialog`)
//! - Popovers (`use_popover`)
//! - Dismiss handling (`use_dismiss`)
//! - Overlay positioning (`use_overlay`, `use_overlay_position`, `use_overlay_trigger`)
//! - Tooltips (`use_tooltip_trigger`)
//!
//! ## Hook Composition
//!
//! The modal/dialog system uses composable layers:
//!
//! 1. **State Layer** - `use_modal_state` or `use_dialog_state`: Manages open/close state
//! 2. **Behavior Layer** - `use_modal`: ARIA attributes, Escape key handling
//! 3. **Backdrop Layer** - `use_modal_backdrop`: Scroll prevention, backdrop clicks
//! 4. **Dialog Layer** - `use_dialog` (optional): aria-labelledby/describedby, `AlertDialog` role
//! 5. **Focus Layer** - `FocusScope` atom: Focus trapping and restoration
//!
//! The `use_popover` hook is a composition hook that combines:
//! - `use_overlay_position` for positioning relative to a trigger
//! - `use_prevent_scroll` for scroll prevention (when modal)
//! - Dismiss handling (Escape key + click outside)
//!
//! ## React-aria Deviations
//!
//! This implementation is based on [React Aria's overlay hooks](https://github.com/adobe/react-spectrum/tree/main/packages/@react-aria/overlays)
//! but has the following deviations:
//!
//! ### Not Implemented
//!
//! - **`ariaHideOutside()`**: React Aria hides all DOM content outside the modal from screen
//!   readers using `aria-hidden` or `inert` attributes. Leptonic relies on `aria-modal="true"`
//!   which has varying browser support for hiding outside content. A future `use_aria_hide_outside`
//!   hook could provide this functionality.
//!
//! - **Safari iOS `VoiceOver` workarounds**: React Aria includes specific workarounds for Safari
//!   on iOS to prevent `VoiceOver` from escaping focus traps. These are not yet implemented.
//!
//! - **Overlay stacking context**: React Aria's `OverlayContainer` manages z-index stacking
//!   for nested modals. The old Leptonic `Modal` component had this feature, but it has not
//!   been ported to the hook system yet.
//!
//! - **`usePreventScroll` iOS Safari handling**: React Aria has extensive iOS Safari workarounds
//!   for scroll prevention. Our implementation may not work correctly on all iOS versions.
//!
//! ### Differences
//!
//! - **`FocusScope` as separate atom**: React Aria's `FocusScope` is a hook. In Leptonic, it's
//!   provided as a component/atom (`<FocusScope>`) that wraps modal content.
//!
//! - **Naming**: React Aria's `useModalOverlay` with `underlayProps` is renamed to
//!   `use_modal_backdrop` with `backdrop_props` for clarity. Similarly, `use_popover`'s
//!   `underlayProps` is renamed to `backdrop_attrs`.

pub mod use_dialog;
pub mod use_dismiss;
pub mod use_modal;
pub mod use_modal_backdrop;
pub mod use_overlay;
pub mod use_overlay_position;
pub mod use_overlay_trigger;
pub mod use_popover;
pub mod use_tooltip_trigger;

pub use use_dialog::*;
pub use use_dismiss::*;
pub use use_modal::*;
pub use use_modal_backdrop::*;
pub use use_overlay::*;
pub use use_overlay_position::*;
pub use use_overlay_trigger::*;
pub use use_popover::*;
pub use use_tooltip_trigger::*;
