//! Drag and drop: [`use_drag`] and [`use_drop`] for single elements; collections with
//! [`use_draggable_collection_state`] / [`use_draggable_item`] (dragging items) and
//! [`use_droppable_collection_state`] / [`use_droppable_collection`] / [`use_droppable_item`] /
//! [`use_drop_indicator`] (dropping on, between and into items). Native drags work with the mouse
//! and touch; keyboard and screen reader drags go through the drag manager (Enter starts a drag,
//! Tab and the collection's keys move between drop targets, Enter drops, Escape cancels).
//! [`use_clipboard`](crate::hooks::clipboard::use_clipboard) adds cut, copy and paste with the same data
//! format.
//!
//! The DnD events (`DragStartEvent`, `DropEvent`, ...) don't implement `Propagation`: they are
//! callbacks of the hooks, not DOM events handed on to the app. The hooks stop the native drag
//! events' propagation themselves (the innermost drag source and drop target handle a drag, as in
//! react-aria), and a keyboard drag's key and focus events never reach the page.

/// Keyboard and screen reader drag sessions.
pub(crate) mod drag_manager;

/// The drop target the arrow keys move to during keyboard drags.
pub(crate) mod drop_target_keyboard_navigation;

/// The drop target at a point of a list or grid (native drags).
pub(crate) mod list_drop_target_delegate;

/// The texts drag and drop shows and announces.
pub(crate) mod messages;

/// Drag items, drop items, drop targets, operations and events.
pub(crate) mod types;

/// Scrolling a drop target while a native drag nears its edges.
pub(crate) mod use_auto_scroll;

/// A draggable element.
pub(crate) mod use_drag;

/// A collection whose items can be dragged.
pub(crate) mod use_draggable_collection;

/// The state of a collection whose items can be dragged.
pub(crate) mod use_draggable_collection_state;

/// A draggable item of a collection.
pub(crate) mod use_draggable_item;

/// A drop target element.
pub(crate) mod use_drop;

/// A drop position between items.
pub(crate) mod use_drop_indicator;

/// A collection that accepts drops.
pub(crate) mod use_droppable_collection;

/// The state of a collection that accepts drops.
pub(crate) mod use_droppable_collection_state;

/// An item of a droppable collection.
pub(crate) mod use_droppable_item;

/// How to drop on a drop target during keyboard drags.
pub(crate) mod use_virtual_drop;

/// The global drag and drop state and data transfer helpers.
pub(crate) mod utils;

pub use drag_manager::{DragSessionInfo, is_virtual_dragging, use_drag_session};
pub use list_drop_target_delegate::*;
pub use types::*;
pub use use_auto_scroll::*;
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
