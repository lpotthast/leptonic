//! Dialog hooks: `use_dialog` (react-aria's `useDialog`): role, name, description and focusing a
//! dialog when it opens. Dismissing and hiding the page are `use_modal_backdrop`'s or `use_popover`'s.

pub(crate) mod use_dialog;

pub use use_dialog::*;
