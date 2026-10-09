//! Cut, copy and paste of an element's data while it has focus ([`use_clipboard`]), in the data
//! format of drag and drop ([`DragItem`](crate::hooks::dnd::DragItem),
//! [`DropItem`](crate::hooks::dnd::DropItem)).

mod use_clipboard;

pub use use_clipboard::*;
