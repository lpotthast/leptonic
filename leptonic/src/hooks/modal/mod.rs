//! Overlay hooks for modals, dialogs, popovers, and tooltips.
//!
//! This module provides hooks for managing overlay behaviors including:
//! - Modal dialogs (`use_modal`, `use_modal_backdrop`, `use_dialog`)
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
//! ## React-aria Deviations
//!
//! This implementation is partially based on [React Aria's overlay hooks](https://github.com/adobe/react-spectrum/tree/main/packages/@react-aria/overlays)
//! (rest in ./overlay) but has the following deviations:
//!
//! ### Not Implemented
//!
//! - **`ariaHideOutside()`**: React Aria hides all DOM content outside the modal from screen
//!   readers using `aria-hidden` or `inert` attributes. Leptonic relies on `aria-modal="true"`
//!   which has varying browser support for hiding outside content. A future `use_aria_hide_outside`
//!   hook could provide this functionality.
//!
//! - **Overlay stacking context**: React Aria's `OverlayContainer` manages z-index stacking
//!   for nested modals. The old Leptonic `Modal` component had this feature, but it has not
//!   been ported to the hook system yet.
//!
//! ### Differences
//!
//! - **Naming**: React Aria's `useModalOverlay` with `underlayProps` is renamed to
//!   `use_modal_backdrop` with `backdrop_props` for clarity. Similarly, `use_popover`'s
//!   `underlayProps` is renamed to `backdrop_props`.

pub mod use_modal;
pub mod use_modal_backdrop;

pub use use_modal::*;
pub use use_modal_backdrop::*;
