//! Modal hooks for creating accessible modal dialogs.
//!
//! This module provides hooks for managing modal behaviors including:
//! - Aria-modal marking (`use_modal`)
//! - Dismiss behavior and scroll prevention (`use_modal_backdrop`)
//! - Open/close state management (`use_modal_state`)
//!
//! ## Hook Composition
//!
//! The modal/dialog system uses composable layers, each handling a specific concern:
//!
//! 1. **State Layer** — `use_modal_state` or `use_dialog_state`: Manages open/close state
//! 2. **Backdrop Layer** — `use_modal_backdrop`: Dismiss behavior (Escape, interact-outside
//!    via `use_overlay`) + scroll prevention
//! 3. **Aria-modal Layer** — `use_modal`: Sets `aria-modal="true"` for assistive technology
//! 4. **Dialog Layer** — `use_dialog` (optional): ARIA labeling, role, focus-on-mount
//! 5. **Focus Layer** — `FocusScope` atom: Focus trapping and restoration
//!
//! ## React-aria Deviations
//!
//! This implementation is partially based on [React Aria's overlay hooks](https://github.com/adobe/react-spectrum/tree/main/packages/@react-aria/overlays)
//! (rest in ./overlay) but has the following deviations:
//!
//! ### Implemented (in `use_modal_backdrop`)
//!
//! - **`ariaHideOutside()`**: When the modal is open, `use_modal_backdrop` sets the `inert`
//!   attribute on all DOM siblings outside the modal via `utils::aria_hide_outside`. This hides
//!   outside content from assistive technology AND prevents pointer/keyboard interaction. Works
//!   alongside `aria-modal="true"` (set by `use_modal`) for defense in depth. Reference counting
//!   and an observer stack support nested overlays correctly.
//!
//! - **`useOverlayFocusContain`**: React Aria's `useModalOverlay` signals to the parent
//!   `Overlay` component that focus should be contained. In Leptonic, focus containment is
//!   handled by the `FocusScope` atom which is applied by the consumer.
//!
//! ### Differences
//!
//! - **Naming**: React Aria's `useModalOverlay` with `underlayProps` is renamed to
//!   `use_modal_backdrop` with `backdrop_props` for clarity.
//!
//! - **`use_modal` scope**: React Aria's `useModal` manages `aria-hidden` on sibling
//!   elements via `ModalProvider` context. Leptonic's `use_modal` simply sets
//!   `aria-modal="true"` on the modal element, since modern browsers honour this attribute.

pub mod use_modal;
pub mod use_modal_backdrop;
pub mod use_modal_state;

pub use use_modal::*;
pub use use_modal_backdrop::*;
pub use use_modal_state::*;
