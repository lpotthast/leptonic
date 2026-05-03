//! State management for droppable collections.
//!
//! Equivalent to react-aria's `useDroppableCollectionState` from
//! `@react-aria/dnd/src/useDroppableCollectionState.ts`.
//!
//! Tracks the current drop target, validates drop operations, and determines
//! which semantic callback should fire based on the drop context.

use std::collections::HashSet;

use leptos::prelude::*;

use super::types::{
    AcceptedDragTypes, CollectionDropEnterEvent, CollectionDropEvent, CollectionDropExitEvent,
    CollectionInsertEvent, CollectionItemDropEvent, CollectionMoveEvent, CollectionReorderEvent,
    CollectionRootDropEvent, DropPosition, DropTarget, ItemDropTarget,
};
use crate::hooks::{AllowedDropOperations, DragItem, DragTypes, DropEffect};

//
// ## LEPTOS-SPECIFIC ADAPTATIONS
//
// - Hook-owned state: `target` is owned internally as `RwSignal`, exposed as
//   read-only `Signal<Option<DropTarget>>`. React-aria: Uses React state.
// - Uses `Callback` from Leptos instead of plain functions.
//
// ## API DIFFERENCES
//
// - `get_drop_operation` returns `DropEffect` directly instead of being
//   computed from `getAllowedDropOperations` and `getDropOperation`.
//   Rationale: Simpler API; react-aria's two-level indirection is unnecessary.
//

/// Input for creating a [`DroppableCollectionState`].
#[derive(Clone)]
pub struct DroppableCollectionStateInput {
    /// Which drag types this collection accepts.
    pub accepted_drag_types: AcceptedDragTypes,

    /// Whether the collection is disabled.
    pub is_disabled: Signal<bool>,

    /// Ordered keys in the collection.
    pub collection_keys: Signal<Vec<String>>,

    /// Called when items are reordered within the collection.
    pub on_reorder: Option<Callback<CollectionReorderEvent>>,

    /// Called when external items are inserted.
    pub on_insert: Option<Callback<CollectionInsertEvent>>,

    /// Called when items are dropped on the collection root.
    pub on_root_drop: Option<Callback<CollectionRootDropEvent>>,

    /// Called when items are dropped on a specific item.
    pub on_item_drop: Option<Callback<CollectionItemDropEvent>>,

    /// Called when items are moved within the collection (internal, On position).
    pub on_move: Option<Callback<CollectionMoveEvent>>,

    /// Called to validate whether a specific item can accept a drop.
    pub should_accept_item_drop: Option<Callback<ShouldAcceptItemDropEvent, bool>>,

    /// Called when drag enters the collection.
    pub on_drop_enter: Option<Callback<CollectionDropEnterEvent>>,

    /// Called when drag exits the collection.
    pub on_drop_exit: Option<Callback<CollectionDropExitEvent>>,

    /// Called when a drop occurs on the collection (generic).
    pub on_drop: Option<Callback<CollectionDropEvent>>,

    /// Custom drop operation override.
    pub get_drop_operation: Option<Callback<CollectionGetDropOperationEvent, DropEffect>>,
}

impl Default for DroppableCollectionStateInput {
    fn default() -> Self {
        Self {
            accepted_drag_types: AcceptedDragTypes::All,
            is_disabled: Signal::derive(|| false),
            collection_keys: Signal::derive(Vec::new),
            on_reorder: None,
            on_insert: None,
            on_root_drop: None,
            on_item_drop: None,
            on_move: None,
            should_accept_item_drop: None,
            on_drop_enter: None,
            on_drop_exit: None,
            on_drop: None,
            get_drop_operation: None,
        }
    }
}

/// Event for the `should_accept_item_drop` callback.
#[derive(Debug, Clone)]
pub struct ShouldAcceptItemDropEvent {
    /// The target item.
    pub target: ItemDropTarget,
    /// The types of data being dragged.
    pub types: DragTypes,
}

/// Event for the `get_drop_operation` callback.
#[derive(Debug, Clone)]
pub struct CollectionGetDropOperationEvent {
    /// The resolved target.
    pub target: DropTarget,
    /// The types of data being dragged.
    pub types: DragTypes,
    /// The allowed operations from the drag source.
    pub allowed_operations: AllowedDropOperations,
    /// Whether the drag is internal (same collection).
    pub is_internal: bool,
    /// The keys being dragged (if internal).
    pub dragging_keys: HashSet<String>,
}

/// State for a droppable collection.
///
/// Tracks the current drop target and provides validation logic for determining
/// which drop operations are allowed. Created via [`use_droppable_collection_state`].
#[derive(Clone)]
pub struct DroppableCollectionState {
    /// The current drop target within the collection.
    target: RwSignal<Option<DropTarget>>,

    /// Key to focus after a drop completes. Set by [`dispatch_drop`](Self::dispatch_drop),
    /// watched by an `Effect` in `use_droppable_collection` to focus the element.
    pub(crate) focus_after_drop_key: RwSignal<Option<String>>,

    /// The input configuration.
    input: DroppableCollectionStateInput,
}

impl DroppableCollectionState {
    /// Returns a read-only signal of the current drop target.
    #[must_use]
    pub fn target(&self) -> Signal<Option<DropTarget>> {
        self.target.into()
    }

    /// Sets the current drop target, firing enter/exit events as appropriate.
    pub fn set_target(&self, new_target: Option<&DropTarget>) {
        let old_target = self.target.get_untracked();

        let is_same = match (&old_target, new_target) {
            (None, None) => true,
            (Some(old), Some(new)) => self
                .input
                .collection_keys
                .with_untracked(|keys| old.is_equivalent(new, keys)),
            _ => false,
        };

        if is_same {
            return;
        }

        // Fire exit on old target
        if let Some(ref old) = old_target {
            if let Some(on_exit) = self.input.on_drop_exit {
                on_exit.run(CollectionDropExitEvent {
                    target: old.clone(),
                });
            }
        }

        self.target.set(new_target.cloned());

        // Fire enter on new target
        if let Some(new) = new_target {
            if let Some(on_enter) = self.input.on_drop_enter {
                on_enter.run(CollectionDropEnterEvent {
                    target: new.clone(),
                });
            }
        }
    }

    /// Returns `true` if the given target matches the current drop target.
    #[must_use]
    pub fn is_drop_target(&self, target: &DropTarget) -> bool {
        self.target.with(|current| {
            current.as_ref().is_some_and(|current| {
                self.input
                    .collection_keys
                    .with(|keys| current.is_equivalent(target, keys))
            })
        })
    }

    /// Determines the drop operation for the given target and context.
    ///
    /// Returns `DropEffect::None` if the drop is not valid.
    #[must_use]
    pub fn get_drop_operation(
        &self,
        target: &DropTarget,
        types: &DragTypes,
        allowed_operations: AllowedDropOperations,
        is_internal: bool,
        dragging_keys: &HashSet<String>,
    ) -> DropEffect {
        // Custom override takes precedence
        let custom_result = self.input.get_drop_operation.map(|get_op| {
            get_op.run(CollectionGetDropOperationEvent {
                target: target.clone(),
                types: types.clone(),
                allowed_operations,
                is_internal,
                dragging_keys: dragging_keys.clone(),
            })
        });
        if let Some(effect) = custom_result {
            return effect;
        }

        if self.input.is_disabled.get_untracked() {
            return DropEffect::None;
        }

        // Check accepted types
        if !self.input.accepted_drag_types.accepts_drag_types(types) {
            return DropEffect::None;
        }

        match target {
            DropTarget::Root => {
                // Root drops require on_root_drop callback
                if self.input.on_root_drop.is_some() {
                    default_drop_effect(allowed_operations, is_internal)
                } else {
                    DropEffect::None
                }
            }
            DropTarget::Item { key, position } => {
                // Prevent self-drop (dragging an item onto itself)
                if is_internal && dragging_keys.contains(key) {
                    return DropEffect::None;
                }

                match position {
                    DropPosition::On => {
                        // "On" requires on_item_drop or on_move
                        let has_item_drop = self.input.on_item_drop.is_some();
                        let has_move = self.input.on_move.is_some() && is_internal;

                        if !has_item_drop && !has_move {
                            return DropEffect::None;
                        }

                        // Check should_accept_item_drop
                        if let Some(should_accept) = self.input.should_accept_item_drop {
                            let accepted = should_accept.run(ShouldAcceptItemDropEvent {
                                target: ItemDropTarget {
                                    key: key.clone(),
                                    position: *position,
                                },
                                types: types.clone(),
                            });
                            if !accepted {
                                return DropEffect::None;
                            }
                        }

                        default_drop_effect(allowed_operations, is_internal)
                    }
                    DropPosition::Before | DropPosition::After => {
                        if is_internal {
                            // Internal before/after → reorder or move
                            if self.input.on_reorder.is_some() {
                                default_drop_effect(allowed_operations, true)
                            } else {
                                DropEffect::None
                            }
                        } else {
                            // External before/after → insert
                            if self.input.on_insert.is_some() {
                                default_drop_effect(allowed_operations, false)
                            } else {
                                DropEffect::None
                            }
                        }
                    }
                }
            }
        }
    }

    /// Dispatches the drop event to the appropriate semantic callback.
    ///
    /// Also sets [`focus_after_drop_key`](Self::focus_after_drop_key) based on
    /// the drop target for post-drop focus management.
    pub fn dispatch_drop(
        &self,
        items: Vec<DragItem>,
        drop_effect: DropEffect,
        target: &DropTarget,
        is_internal: bool,
        dragging_keys: &HashSet<String>,
    ) {
        // Set focus target based on drop position.
        match target {
            DropTarget::Root => {
                // No specific focus target for root drops.
            }
            DropTarget::Item { key, .. } => {
                self.focus_after_drop_key.set(Some(key.clone()));
            }
        }

        // Fire generic on_drop first
        if let Some(on_drop) = self.input.on_drop {
            on_drop.run(CollectionDropEvent {
                items: items.clone(),
                drop_effect,
                target: target.clone(),
                is_internal,
            });
        }

        match target {
            DropTarget::Root => {
                if let Some(on_root_drop) = self.input.on_root_drop {
                    on_root_drop.run(CollectionRootDropEvent { items, drop_effect });
                }
            }
            DropTarget::Item { key, position } => {
                let item_target = ItemDropTarget {
                    key: key.clone(),
                    position: *position,
                };

                match position {
                    DropPosition::On => {
                        if is_internal {
                            if let Some(on_move) = self.input.on_move {
                                on_move.run(CollectionMoveEvent {
                                    keys: dragging_keys.iter().cloned().collect(),
                                    target: item_target,
                                    drop_effect,
                                });
                                return;
                            }
                        }
                        if let Some(on_item_drop) = self.input.on_item_drop {
                            on_item_drop.run(CollectionItemDropEvent {
                                items,
                                target: item_target,
                                is_internal,
                                drop_effect,
                            });
                        }
                    }
                    DropPosition::Before | DropPosition::After => {
                        if is_internal {
                            if let Some(on_reorder) = self.input.on_reorder {
                                on_reorder.run(CollectionReorderEvent {
                                    keys: dragging_keys.iter().cloned().collect(),
                                    target: item_target,
                                    drop_effect,
                                });
                            }
                        } else if let Some(on_insert) = self.input.on_insert {
                            on_insert.run(CollectionInsertEvent {
                                items,
                                target: item_target,
                                drop_effect,
                            });
                        }
                    }
                }
            }
        }
    }
}

/// Creates the droppable collection state.
pub fn use_droppable_collection_state(
    input: DroppableCollectionStateInput,
) -> DroppableCollectionState {
    let target = RwSignal::new(None);
    let focus_after_drop_key = RwSignal::new(None);

    DroppableCollectionState {
        target,
        focus_after_drop_key,
        input,
    }
}

/// Returns the default drop effect given the allowed operations.
fn default_drop_effect(allowed: AllowedDropOperations, is_internal: bool) -> DropEffect {
    if is_internal {
        // Prefer move for internal operations
        if allowed.contains_effect(DropEffect::Move) {
            DropEffect::Move
        } else if allowed.contains_effect(DropEffect::Copy) {
            DropEffect::Copy
        } else if allowed.contains_effect(DropEffect::Link) {
            DropEffect::Link
        } else {
            DropEffect::None
        }
    } else {
        // Prefer copy for external operations
        if allowed.contains_effect(DropEffect::Copy) {
            DropEffect::Copy
        } else if allowed.contains_effect(DropEffect::Move) {
            DropEffect::Move
        } else if allowed.contains_effect(DropEffect::Link) {
            DropEffect::Link
        } else {
            DropEffect::None
        }
    }
}
