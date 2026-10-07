//! Drag and drop: [`use_drag`] and [`use_drop`] for single elements; collections with
//! [`use_draggable_collection_state`] / [`use_draggable_item`] (dragging items) and
//! [`use_droppable_collection_state`] / [`use_droppable_collection`] / [`use_droppable_item`] /
//! [`use_drop_indicator`] (dropping on, between and into items). Native drags work with the mouse
//! and touch; keyboard and screen reader drags go through the drag manager (Enter starts a drag,
//! Tab and the collection's keys move between drop targets, Enter drops, Escape cancels).
//! [`use_clipboard`] adds cut, copy and paste with the same data format.
//!
//! The DnD events (`DragStartEvent`, `DropEvent`, ...) don't implement `Propagation`: they are
//! callbacks of the hooks, not DOM events handed on to the app. The hooks stop the native drag
//! events' propagation themselves (the innermost drag source and drop target handle a drag, as in
//! react-aria), and a keyboard drag's key and focus events never reach the page.

pub(crate) mod drag_manager;
mod drop_target_keyboard_navigation;
mod list_drop_target_delegate;
mod messages;
mod types;
mod use_auto_scroll;
mod use_clipboard;
mod use_drag;
mod use_draggable_collection;
mod use_draggable_collection_state;
mod use_draggable_item;
mod use_drop;
mod use_drop_indicator;
mod use_droppable_collection;
mod use_droppable_collection_state;
mod use_droppable_item;
mod use_virtual_drop;
pub(crate) mod utils;

pub use drag_manager::{DragSessionInfo, is_virtual_dragging, use_drag_session};
pub use list_drop_target_delegate::*;
pub use types::*;
pub use use_auto_scroll::*;
pub use use_clipboard::*;
pub use use_drag::*;
pub use use_draggable_collection::*;
pub use use_draggable_collection_state::*;
pub use use_draggable_item::*;
pub use use_drop::*;
pub use use_drop_indicator::*;
pub use use_droppable_collection::*;
pub use use_droppable_collection_state::*;
pub use use_droppable_item::*;
pub use use_virtual_drop::*;
pub use utils::{DragModality, get_drag_modality, use_drag_modality};
