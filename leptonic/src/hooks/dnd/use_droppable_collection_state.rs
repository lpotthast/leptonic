// Upstream: react-stately/src/dnd/useDroppableCollectionState.ts @ 99e6102368
use std::collections::HashSet;

use leptos::prelude::*;

use super::types::{
    AcceptedDragTypes, DragTypes, DropOperation, DropPosition, DropTarget,
    DroppableCollectionActivateEvent, DroppableCollectionDropEvent, DroppableCollectionEnterEvent,
    DroppableCollectionExitEvent, DroppableCollectionInsertDropEvent,
    DroppableCollectionOnItemDropEvent, DroppableCollectionReorderEvent,
    DroppableCollectionRootDropEvent, ItemDropTarget,
};
use crate::hooks::collections::{Collection, Key, ListState};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state: the current `target` is a signal.
// - `should_accept_item_drop` and `get_drop_operation` take query structs.
//
// =============================================================================

/// Asks whether an item accepts a drop on it.
#[derive(Debug, Clone)]
pub struct ItemDropQuery {
    pub target: ItemDropTarget,
    pub types: DragTypes,
}

/// Asks for the drop operation at a target.
#[derive(Debug, Clone)]
pub struct CollectionDropOperationQuery {
    pub target: DropTarget,
    pub types: DragTypes,
    /// In order of preference.
    pub allowed_operations: Vec<DropOperation>,
}

/// What a collection drop target decides a drop operation from.
#[derive(Debug, Clone)]
pub struct DropOperationEvent {
    pub target: DropTarget,
    pub types: DragTypes,
    pub allowed_operations: Vec<DropOperation>,
    /// The drag comes from this collection.
    pub is_internal: bool,
    pub dragging_keys: HashSet<Key>,
}

/// Callbacks and settings of a droppable collection.
#[derive(Clone, Default)]
pub struct DroppableCollectionOptions {
    /// The data types the collection accepts.
    pub accepted_drag_types: AcceptedDragTypes,
    /// External data dropped between items.
    pub on_insert: Option<Callback<DroppableCollectionInsertDropEvent>>,
    /// Data dropped on the collection itself.
    pub on_root_drop: Option<Callback<DroppableCollectionRootDropEvent>>,
    /// Data dropped on an item.
    pub on_item_drop: Option<Callback<DroppableCollectionOnItemDropEvent>>,
    /// Items of the collection dropped between other items of the same parent.
    pub on_reorder: Option<Callback<DroppableCollectionReorderEvent>>,
    /// Items of the collection dropped anywhere in it.
    pub on_move: Option<Callback<DroppableCollectionReorderEvent>>,
    /// Whether an item accepts a drop on it.
    pub should_accept_item_drop: Option<Callback<ItemDropQuery, bool>>,
    pub on_drop_enter: Option<Callback<DroppableCollectionEnterEvent>>,
    pub on_drop_activate: Option<Callback<DroppableCollectionActivateEvent>>,
    pub on_drop_exit: Option<Callback<DroppableCollectionExitEvent>>,
    /// Handles all drops itself (instead of `on_insert`, `on_reorder`, ...).
    pub on_drop: Option<Callback<DroppableCollectionDropEvent>>,
    /// The operation for a drop at a target. Defaults to the first allowed operation.
    pub get_drop_operation: Option<Callback<CollectionDropOperationQuery, DropOperation>>,
}

/// Input of [`use_droppable_collection_state`].
#[derive(Clone)]
pub struct UseDroppableCollectionStateInput {
    /// The collection and its selection.
    pub list: ListState,
    pub options: DroppableCollectionOptions,
    pub is_disabled: Signal<bool>,
}

/// The state of a collection that accepts drops.
#[derive(Clone, Copy)]
pub struct DroppableCollectionState {
    pub list: ListState,
    /// The current drop target.
    pub target: Signal<Option<DropTarget>>,
    pub is_disabled: Signal<bool>,
    set_target_signal: WriteSignal<Option<DropTarget>>,
    pub(crate) options: StoredValue<DroppableCollectionOptions>,
}

impl std::fmt::Debug for DroppableCollectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DroppableCollectionState")
            .field("target", &self.target.get_untracked())
            .finish_non_exhaustive()
    }
}

impl DroppableCollectionState {
    /// Make `target` the current drop target, firing exit and enter events.
    pub fn set_target(&self, target: Option<DropTarget>) {
        if self.is_drop_target(target.as_ref()) {
            return;
        }
        let (on_exit, on_enter) = self
            .options
            .with_value(|o| (o.on_drop_exit, o.on_drop_enter));
        if let Some(current) = self.target.get_untracked()
            && let Some(on_exit) = on_exit
        {
            on_exit.run(DroppableCollectionExitEvent {
                x: 0.0,
                y: 0.0,
                target: current,
            });
        }
        if let Some(new) = &target
            && let Some(on_enter) = on_enter
        {
            on_enter.run(DroppableCollectionEnterEvent {
                x: 0.0,
                y: 0.0,
                target: new.clone(),
            });
        }
        self.set_target_signal.set(target);
    }

    /// Whether `target` is (equivalent to) the current drop target: "after A" is the same
    /// position as "before B" when B follows A.
    pub fn is_drop_target(&self, target: Option<&DropTarget>) -> bool {
        let (Some(target), Some(current)) = (target, self.target.get()) else {
            return false;
        };
        if *target == current {
            return true;
        }
        let (DropTarget::Item(a), DropTarget::Item(b)) = (target, &current) else {
            return false;
        };
        if a.key == b.key
            || a.drop_position == b.drop_position
            || a.drop_position == DropPosition::On
            || b.drop_position == DropPosition::On
        {
            return false;
        }
        self.list.collection.with(|c| {
            opposite_target(c, a).as_ref() == Some(b) || opposite_target(c, b).as_ref() == Some(a)
        })
    }

    /// The drop operation for a drop at `e.target`.
    pub fn get_drop_operation(&self, e: &DropOperationEvent) -> DropOperation {
        if e.is_internal
            && let DropTarget::Item(target) = &e.target
            && !e.dragging_keys.is_empty()
        {
            if e.dragging_keys.contains(&target.key) && target.drop_position == DropPosition::On {
                return DropOperation::Cancel;
            }
            // Items can't be dropped into themselves.
            let into_dragged = self.list.collection.with_untracked(|c| {
                let mut key = Some(target.key.clone());
                while let Some(k) = key {
                    let parent = c.get(&k).and_then(|n| n.parent_key.clone());
                    if parent.as_ref().is_some_and(|p| e.dragging_keys.contains(p)) {
                        return true;
                    }
                    key = parent;
                }
                false
            });
            if into_dragged {
                return DropOperation::Cancel;
            }
        }
        self.default_drop_operation(e)
    }

    fn default_drop_operation(&self, e: &DropOperationEvent) -> DropOperation {
        if self.is_disabled.get_untracked() {
            return DropOperation::Cancel;
        }
        self.options.with_value(|o| {
            if !o.accepted_drag_types.accepts(&e.types) {
                return DropOperation::Cancel;
            }
            let item = match &e.target {
                DropTarget::Item(item) => Some(item),
                DropTarget::Root => None,
            };
            let between = item.is_some_and(|t| t.drop_position != DropPosition::On);
            let on_item = item.is_some_and(|t| t.drop_position == DropPosition::On);
            let is_valid_insert = o.on_insert.is_some() && between && !e.is_internal;
            let is_valid_reorder = o.on_reorder.is_some()
                && between
                && e.is_internal
                && item.is_some_and(|t| {
                    self.list
                        .collection
                        .with_untracked(|c| is_dragging_within_parent(c, t, &e.dragging_keys))
                });
            let is_item_drop_allowed = !on_item
                || o.should_accept_item_drop.is_none_or(|accept| {
                    item.is_some_and(|t| {
                        accept.run(ItemDropQuery {
                            target: t.clone(),
                            types: e.types.clone(),
                        })
                    })
                });
            let is_valid_move =
                o.on_move.is_some() && item.is_some() && e.is_internal && is_item_drop_allowed;
            let is_valid_root_drop = o.on_root_drop.is_some() && item.is_none() && !e.is_internal;
            let is_valid_on_item_drop = o.on_item_drop.is_some()
                && on_item
                && !(e.is_internal && item.is_some_and(|t| e.dragging_keys.contains(&t.key)))
                && is_item_drop_allowed;
            if o.on_drop.is_some()
                || is_valid_insert
                || is_valid_reorder
                || is_valid_move
                || is_valid_root_drop
                || is_valid_on_item_drop
            {
                match o.get_drop_operation {
                    Some(get) => get.run(CollectionDropOperationQuery {
                        target: e.target.clone(),
                        types: e.types.clone(),
                        allowed_operations: e.allowed_operations.clone(),
                    }),
                    None => e
                        .allowed_operations
                        .first()
                        .copied()
                        .unwrap_or(DropOperation::Cancel),
                }
            } else {
                DropOperation::Cancel
            }
        })
    }
}

/// The same position seen from the neighboring item: "before B" is "after A" when A precedes B.
fn opposite_target(collection: &Collection, target: &ItemDropTarget) -> Option<ItemDropTarget> {
    let node = collection.get(&target.key)?;
    match target.drop_position {
        DropPosition::Before => node.prev_key.clone().map(|key| ItemDropTarget {
            key,
            drop_position: DropPosition::After,
        }),
        DropPosition::After => node.next_key.clone().map(|key| ItemDropTarget {
            key,
            drop_position: DropPosition::Before,
        }),
        DropPosition::On => None,
    }
}

/// Whether all dragged items have the target's parent (a reorder within one level).
fn is_dragging_within_parent(
    collection: &Collection,
    target: &ItemDropTarget,
    dragging_keys: &HashSet<Key>,
) -> bool {
    let target_parent = collection.get(&target.key).map(|n| n.parent_key.clone());
    dragging_keys
        .iter()
        .all(|key| collection.get(key).map(|n| n.parent_key.clone()) == target_parent)
}

/// Creates the state of a collection that accepts drops.
pub fn use_droppable_collection_state(
    input: UseDroppableCollectionStateInput,
) -> DroppableCollectionState {
    let UseDroppableCollectionStateInput {
        list,
        options,
        is_disabled,
    } = input;
    let (target, set_target_signal) = signal(None);
    DroppableCollectionState {
        list,
        target: target.into(),
        is_disabled,
        set_target_signal,
        options: StoredValue::new(options),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use assertr::prelude::*;

    use super::*;
    use crate::hooks::collections::{UseListStateInput, use_list_state};

    fn state(options: DroppableCollectionOptions) -> DroppableCollectionState {
        let collection = Memo::new(|_| {
            Arc::new(Collection::build(|b| {
                b.item("a", "A");
                b.item("b", "B").children(|c| {
                    c.item("b1", "B1");
                });
                b.item("c", "C");
            }))
        });
        let list = use_list_state(UseListStateInput {
            collection,
            selection: crate::hooks::collections::SelectionOptions::default(),
        });
        use_droppable_collection_state(UseDroppableCollectionStateInput {
            list,
            options,
            is_disabled: Signal::stored(false),
        })
    }

    fn query(target: DropTarget, is_internal: bool, dragging: &[&str]) -> DropOperationEvent {
        DropOperationEvent {
            target,
            types: DragTypes::of_items(&[super::super::types::DragItem::text("x")]),
            allowed_operations: vec![DropOperation::Move, DropOperation::Copy],
            is_internal,
            dragging_keys: dragging.iter().map(|k| Key::from(*k)).collect(),
        }
    }

    #[test]
    fn reorders_need_a_reorder_handler_and_the_same_parent() {
        Owner::new().with(|| {
            let s = state(DroppableCollectionOptions {
                on_reorder: Some(Callback::new(|_| {})),
                ..DroppableCollectionOptions::default()
            });
            let before_c = DropTarget::item("c", DropPosition::Before);
            assert_that!(s.get_drop_operation(&query(before_c.clone(), true, &["a"])))
                .is_equal_to(DropOperation::Move);
            // An external drop between items needs `on_insert`.
            assert_that!(s.get_drop_operation(&query(before_c, false, &[])))
                .is_equal_to(DropOperation::Cancel);
            // Dragging a nested item next to a top-level one is not a reorder.
            let after_a = DropTarget::item("a", DropPosition::After);
            assert_that!(s.get_drop_operation(&query(after_a, true, &["b1"])))
                .is_equal_to(DropOperation::Cancel);
        });
    }

    #[test]
    fn items_cant_be_dropped_on_themselves_or_into_their_children() {
        Owner::new().with(|| {
            let s = state(DroppableCollectionOptions {
                on_move: Some(Callback::new(|_| {})),
                ..DroppableCollectionOptions::default()
            });
            let on_b = DropTarget::item("b", DropPosition::On);
            assert_that!(s.get_drop_operation(&query(on_b, true, &["b"])))
                .is_equal_to(DropOperation::Cancel);
            let after_b1 = DropTarget::item("b1", DropPosition::After);
            assert_that!(s.get_drop_operation(&query(after_b1, true, &["b"])))
                .is_equal_to(DropOperation::Cancel);
            let on_c = DropTarget::item("c", DropPosition::On);
            assert_that!(s.get_drop_operation(&query(on_c, true, &["a"])))
                .is_equal_to(DropOperation::Move);
        });
    }

    #[test]
    fn positions_between_two_items_are_the_same_target() {
        Owner::new().with(|| {
            let s = state(DroppableCollectionOptions::default());
            s.set_target(Some(DropTarget::item("a", DropPosition::After)));
            assert_that!(s.is_drop_target(Some(&DropTarget::item("b", DropPosition::Before))))
                .is_true();
            assert_that!(s.is_drop_target(Some(&DropTarget::item("c", DropPosition::Before))))
                .is_false();
        });
    }
}
