//! Modal hooks for creating accessible modal dialogs.
//!
//! - `use_modal_backdrop` (react-aria's `useModalOverlay`): dismissing (Escape, interacting
//!   outside, through `use_overlay`), scroll prevention, and hiding everything outside the modal
//!   (inert, through `utils::aria_hide_outside`), which is what makes the modal modal.
//!
//! No `use_modal` (react-aria's `useModal`, the legacy `ModalProvider` API): as
//! react-aria-components, modals set no `aria-modal` (WebKit bug 211934: Safari focuses the first
//! focusable element of an `aria-modal` element itself); the inert content outside makes them
//! modal.
//!
//! ## Hook Composition
//!
//! The modal/dialog system uses composable layers, each handling a specific concern:
//!
//! 1. **Backdrop layer**: `use_modal_backdrop`, as above.
//! 2. **Dialog layer**: `use_dialog` (optional): ARIA role, labeling, focusing the dialog when it
//!    opens.
//! 3. **Focus layer**: the `FocusScope` atom: containing and restoring the focus (react-aria's
//!    `useModalOverlay` signals its `Overlay` to contain focus; here the consumer renders the
//!    scope, the `ModalContent` atom does).
//!
//! See each hook's deviation block for the details.

pub(crate) mod use_modal_backdrop;

pub use use_modal_backdrop::*;
