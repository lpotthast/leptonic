//! Shared types for drag-and-drop collection operations.
//!
//! Contains `DropTarget`, `DropPosition`, `ItemDropTarget`, `AcceptedDragTypes`,
//! and all `Collection*Event` types used by the proper collection API
//! (`use_droppable_collection`, `use_draggable_collection_item`,
//! `use_collection_droppable_item`, `use_drop_indicator`).
//!
//! Simplified wrappers that bypass the collection machinery (hit-testing
//! delegates, drop indicators, keyboard navigation) must not be added here.

use crate::hooks::{DragItem, DragTypes, DropEffect};

//
// ## API DIFFERENCES
//
// - `DropTarget` is an enum with `Root` and `Item` variants instead of
//   react-aria's `DropTarget` type which uses a discriminated union with
//   `type: 'root' | 'item'`.
// - `AcceptedDragTypes` uses `All` / `Specific(Vec<String>)` instead of
//   react-aria's `'all' | Set<string>`.
// - Collection event types use `ItemDropTarget` (always item-targeted) instead
//   of react-aria's `ItemDropTarget` which is structurally identical but uses
//   TypeScript intersection types.
//

/// The position of a drop relative to the target item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropPosition {
    /// Drop before the target.
    Before,
    /// Drop after the target.
    After,
    /// Drop on the target (for nested items like tree nodes).
    On,
}

/// The target location for a drop operation within a collection.
///
/// This is an enum to support both root-level drops (on the collection itself)
/// and item-level drops (before, after, or on a specific item).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DropTarget {
    /// Drop on the collection root (not targeting any specific item).
    Root,
    /// Drop relative to a specific item.
    Item {
        /// The key of the target item.
        key: String,
        /// The position relative to the target.
        position: DropPosition,
    },
}

impl DropTarget {
    /// Returns `true` if this is a `Root` target.
    #[must_use]
    pub fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    /// Returns `true` if this is an `Item` target.
    #[must_use]
    pub fn is_item(&self) -> bool {
        matches!(self, Self::Item { .. })
    }

    /// Returns the key if this is an `Item` target.
    #[must_use]
    pub fn key(&self) -> Option<&str> {
        match self {
            Self::Root => None,
            Self::Item { key, .. } => Some(key),
        }
    }

    /// Returns the position if this is an `Item` target.
    #[must_use]
    pub fn position(&self) -> Option<DropPosition> {
        match self {
            Self::Root => None,
            Self::Item { position, .. } => Some(*position),
        }
    }

    /// Checks if two targets represent the same logical position.
    ///
    /// "After item A" is equivalent to "Before item B" when B immediately follows A.
    /// This method checks exact equality; use [`is_equivalent`](Self::is_equivalent)
    /// for adjacency-aware comparison.
    #[must_use]
    pub fn is_same(&self, other: &Self) -> bool {
        self == other
    }

    /// Checks if two targets are equivalent given an ordered list of keys.
    ///
    /// "After key[i]" is equivalent to "Before key[i+1]" in a flat list.
    #[must_use]
    pub fn is_equivalent(&self, other: &Self, keys: &[String]) -> bool {
        if self == other {
            return true;
        }
        match (self, other) {
            (
                Self::Item {
                    key: key_a,
                    position: DropPosition::After,
                },
                Self::Item {
                    key: key_b,
                    position: DropPosition::Before,
                },
            )
            | (
                Self::Item {
                    key: key_b,
                    position: DropPosition::Before,
                },
                Self::Item {
                    key: key_a,
                    position: DropPosition::After,
                },
            ) => {
                if let Some(idx_a) = keys.iter().position(|k| k == key_a) {
                    if idx_a + 1 < keys.len() {
                        return keys[idx_a + 1] == *key_b;
                    }
                }
                false
            }
            _ => false,
        }
    }
}

/// A drop target that always references a specific item.
///
/// Used in semantic collection events (reorder, insert, item drop, move)
/// where the target is always an item, never the root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemDropTarget {
    /// The key of the target item.
    pub key: String,
    /// The position relative to the target.
    pub position: DropPosition,
}

impl ItemDropTarget {
    /// Creates a new item drop target.
    #[must_use]
    pub fn new(key: impl Into<String>, position: DropPosition) -> Self {
        Self {
            key: key.into(),
            position,
        }
    }
}

impl From<ItemDropTarget> for DropTarget {
    fn from(item: ItemDropTarget) -> Self {
        Self::Item {
            key: item.key,
            position: item.position,
        }
    }
}

/// Specifies which MIME types a droppable collection accepts.
#[derive(Debug, Clone, Default)]
pub enum AcceptedDragTypes {
    /// Accept all MIME types.
    #[default]
    All,
    /// Accept only the specified MIME types.
    Specific(Vec<String>),
}

impl AcceptedDragTypes {
    /// Returns `true` if any of the given types are accepted.
    #[must_use]
    pub fn accepts_any(&self, types: &[String]) -> bool {
        match self {
            Self::All => true,
            Self::Specific(accepted) => {
                if accepted.is_empty() {
                    return true;
                }
                types.iter().any(|t| accepted.contains(t))
            }
        }
    }

    /// Returns `true` if the given [`DragTypes`] are accepted.
    ///
    /// For [`DragTypes::UnknownFiles`], returns `true` (optimistic acceptance
    /// since file types cannot be determined during drag).
    #[must_use]
    pub fn accepts_drag_types(&self, types: &DragTypes) -> bool {
        match self {
            Self::All => true,
            Self::Specific(accepted) => {
                if accepted.is_empty() {
                    return true;
                }
                match types {
                    DragTypes::UnknownFiles => true,
                    DragTypes::Known(known) => known.iter().any(|t| accepted.contains(t)),
                }
            }
        }
    }

    /// Converts to a `Vec<String>` for use with `UseDroppableInput::accepted_types`.
    #[must_use]
    pub fn to_accepted_types_vec(&self) -> Vec<String> {
        match self {
            Self::All => vec![],
            Self::Specific(types) => types.clone(),
        }
    }
}

/// Event fired when items are reordered within the same collection.
#[derive(Debug, Clone)]
pub struct CollectionReorderEvent {
    /// The keys of the items being moved.
    pub keys: Vec<String>,
    /// The target to reorder to.
    pub target: ItemDropTarget,
    /// The drop effect.
    pub drop_effect: DropEffect,
}

/// Event fired when external items are inserted into the collection.
#[derive(Debug, Clone)]
pub struct CollectionInsertEvent {
    /// The inserted items.
    pub items: Vec<DragItem>,
    /// The target to insert at.
    pub target: ItemDropTarget,
    /// The drop effect.
    pub drop_effect: DropEffect,
}

/// Event fired when items are dropped on the collection root.
#[derive(Debug, Clone)]
pub struct CollectionRootDropEvent {
    /// The dropped items.
    pub items: Vec<DragItem>,
    /// The drop effect.
    pub drop_effect: DropEffect,
}

/// Event fired when items are dropped on a specific item in the collection.
#[derive(Debug, Clone)]
pub struct CollectionItemDropEvent {
    /// The dropped items.
    pub items: Vec<DragItem>,
    /// The target item.
    pub target: ItemDropTarget,
    /// Whether the source is from the same collection.
    pub is_internal: bool,
    /// The drop effect.
    pub drop_effect: DropEffect,
}

/// Event fired when items are moved within the same collection
/// (internal drop with `DropPosition::On`).
#[derive(Debug, Clone)]
pub struct CollectionMoveEvent {
    /// The keys of the items being moved.
    pub keys: Vec<String>,
    /// The target to move to.
    pub target: ItemDropTarget,
    /// The drop effect.
    pub drop_effect: DropEffect,
}

/// Event fired when drag enters a collection drop target.
#[derive(Debug, Clone)]
pub struct CollectionDropEnterEvent {
    /// The resolved drop target.
    pub target: DropTarget,
}

/// Event fired when drag exits a collection drop target.
#[derive(Debug, Clone)]
pub struct CollectionDropExitEvent {
    /// The resolved drop target.
    pub target: DropTarget,
}

/// Event fired when a drop occurs on a collection (generic, before semantic dispatch).
#[derive(Debug, Clone)]
pub struct CollectionDropEvent {
    /// The dropped items.
    pub items: Vec<DragItem>,
    /// The drop effect.
    pub drop_effect: DropEffect,
    /// The resolved target in the collection.
    pub target: DropTarget,
    /// Whether the drag source is internal to this collection.
    pub is_internal: bool,
}
