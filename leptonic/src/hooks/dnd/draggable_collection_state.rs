//! State management for draggable collections with selection awareness.
//!
//! Tracks which items are being dragged, supports multi-select drag (where
//! dragging a selected item drags all selected items), and fires semantic
//! events for drag lifecycle.
//!
//! Based on react-aria's `useDraggableCollectionState` from
//! `@react-aria/dnd/src/useDraggableCollectionState.ts`.

use std::collections::HashSet;

use leptos::prelude::*;

use crate::hooks::{AllowedDropOperations, DragItem, DragPreviewImage, DragStartEvent, DropEffect};

//
// ## RUST-NATIVE DESIGN
//
// - `is_dragging`, `get_keys_for_drag`, `get_items` are `impl` methods on the
//   struct instead of callbacks in the return type. These are pure queries on
//   internal state, not user-supplied behavior.
// - Uses `StoredValue` for input callbacks instead of closures/refs.
// - `is_internal` is computed via `global_dnd_state::is_internal_drop_operation()`
//   instead of being tracked as mutable state.
//
// ## API DIFFERENCES
//
// - `DraggableCollectionState` is a standalone state struct, not a hook return.
//   Created via `use_draggable_collection_state()`.
// - Selection is accessed through `UseSelectionStateReturn` from leptonic's
//   selection module instead of react-aria's collection/selection manager.
//

/// Event fired when a selection-aware drag starts.
#[derive(Debug, Clone)]
pub struct DraggableCollectionStartEvent {
    /// The keys being dragged (may include multiple selected items).
    pub keys: HashSet<String>,
    /// The x coordinate of the drag start.
    pub x: f64,
    /// The y coordinate of the drag start.
    pub y: f64,
}

/// Event fired during a selection-aware drag.
#[derive(Debug, Clone)]
pub struct DraggableCollectionMoveEvent {
    /// The keys being dragged.
    pub keys: HashSet<String>,
    /// The current x coordinate.
    pub x: f64,
    /// The current y coordinate.
    pub y: f64,
}

/// Event fired when a selection-aware drag ends.
#[derive(Debug, Clone)]
pub struct DraggableCollectionEndEvent {
    /// The keys that were being dragged.
    pub keys: HashSet<String>,
    /// The final x coordinate.
    pub x: f64,
    /// The final y coordinate.
    pub y: f64,
    /// The drop effect that was performed.
    pub drop_effect: DropEffect,
    /// Whether the drop was on the same collection (internal).
    pub is_internal: bool,
}

/// Input for creating a [`DraggableCollectionState`].
#[derive(Clone)]
pub struct DraggableCollectionStateInput {
    /// Ordered keys in the collection.
    pub collection_keys: Signal<Vec<String>>,

    /// Whether a key is currently selected. Used to determine which keys to
    /// include in a multi-select drag.
    pub is_selected: Callback<String, bool>,

    /// Returns all currently selected keys.
    pub selected_keys: Signal<HashSet<String>>,

    /// Returns the drag items for the given set of keys.
    pub get_items: Callback<Vec<String>, Vec<DragItem>>,

    /// Optional callback to provide a custom drag preview.
    pub preview: Option<Callback<Vec<DragItem>, Option<DragPreviewImage>>>,

    /// Returns the allowed drop operations for this drag source.
    pub get_allowed_drop_operations: Option<Callback<(), AllowedDropOperations>>,

    /// The collection container element. Used to detect internal (same-collection)
    /// drops via [`global_dnd_state::is_internal_drop_operation`].
    pub collection_ref: Signal<Option<web_sys::Element>>,

    /// Called when drag starts.
    pub on_drag_start: Option<Callback<DraggableCollectionStartEvent>>,

    /// Called during drag.
    pub on_drag_move: Option<Callback<DraggableCollectionMoveEvent>>,

    /// Called when drag ends.
    pub on_drag_end: Option<Callback<DraggableCollectionEndEvent>>,
}

/// State for a draggable collection with selection awareness.
///
/// When an item that is part of a multi-selection is dragged, all selected
/// items are included in the drag. Single unselected items can still be
/// dragged individually.
///
/// Created via [`use_draggable_collection_state`].
#[derive(Clone)]
pub struct DraggableCollectionState {
    /// The key that was initially grabbed to start the drag.
    dragged_key: RwSignal<Option<String>>,
    /// All keys being dragged (includes selected keys when applicable).
    dragging_keys: RwSignal<HashSet<String>>,
    /// The input configuration.
    input: DraggableCollectionStateInput,
}

impl DraggableCollectionState {
    /// Returns a read-only signal of the key that initiated the drag.
    #[must_use]
    pub fn dragged_key(&self) -> Signal<Option<String>> {
        self.dragged_key.into()
    }

    /// Returns a read-only signal of all keys currently being dragged.
    #[must_use]
    pub fn dragging_keys(&self) -> Signal<HashSet<String>> {
        self.dragging_keys.into()
    }

    /// Returns `true` if the given key is part of the current drag.
    #[must_use]
    pub fn is_dragging(&self, key: &str) -> bool {
        self.dragging_keys.with_untracked(|keys| keys.contains(key))
    }

    /// Computes the set of keys that should be dragged when the given key
    /// is grabbed.
    ///
    /// If the key is selected, all selected keys are included. If the key
    /// is not selected, only that key is returned.
    #[must_use]
    pub fn get_keys_for_drag(&self, key: &str) -> HashSet<String> {
        let is_selected = self.input.is_selected.run(key.to_owned());

        if is_selected {
            self.input.selected_keys.get_untracked()
        } else {
            HashSet::from([key.to_owned()])
        }
    }

    /// Returns the drag items for the given key (and all co-dragged keys).
    #[must_use]
    pub fn get_items(&self, key: &str) -> Vec<DragItem> {
        let keys = self.get_keys_for_drag(key);
        let keys_vec: Vec<String> = keys.into_iter().collect();
        self.input.get_items.run(keys_vec)
    }

    /// Returns the allowed drop operations for this drag source.
    #[must_use]
    pub fn get_allowed_drop_operations(&self) -> AllowedDropOperations {
        self.input
            .get_allowed_drop_operations
            .map_or(AllowedDropOperations::ALL, |cb| cb.run(()))
    }

    /// Returns the custom drag preview, if configured.
    #[must_use]
    pub fn get_preview(&self, items: Vec<DragItem>) -> Option<DragPreviewImage> {
        self.input.preview.and_then(|cb| cb.run(items))
    }

    /// Called when a drag starts on the given key.
    ///
    /// Sets up internal state, sets the global dragging collection element,
    /// and fires `on_drag_start`.
    pub fn start_drag(&self, key: &str, event: &DragStartEvent) {
        let keys = self.get_keys_for_drag(key);

        self.dragged_key.set(Some(key.to_owned()));
        self.dragging_keys.set(keys.clone());

        // Set global dragging keys for cross-collection coordination.
        super::global_dnd_state::set_dragging_keys(&keys);

        // Set the source collection element for internal drop detection.
        let element = self.input.collection_ref.get_untracked();
        super::global_dnd_state::set_dragging_collection_element(element.as_ref());

        if let Some(on_start) = self.input.on_drag_start {
            on_start.run(DraggableCollectionStartEvent {
                keys,
                x: event.x,
                y: event.y,
            });
        }
    }

    /// Called during a drag move.
    pub fn move_drag(&self, x: f64, y: f64) {
        let keys = self.dragging_keys.get_untracked();
        if let Some(on_move) = self.input.on_drag_move {
            on_move.run(DraggableCollectionMoveEvent { keys, x, y });
        }
    }

    /// Called when the drag ends.
    ///
    /// Determines whether the drop was internal (same-collection) by checking
    /// the global dragging/drop collection element state. Clears internal
    /// state and fires `on_drag_end`.
    pub fn end_drag(&self, drop_effect: DropEffect, x: f64, y: f64) {
        let keys = self.dragging_keys.get_untracked();

        // Determine if this was an internal drop by comparing the source
        // collection element (set in start_drag) with the drop collection
        // element (set by use_droppable_collection).
        let is_internal = self
            .input
            .collection_ref
            .get_untracked()
            .is_some_and(|el| super::global_dnd_state::is_internal_drop_operation(&el));

        self.dragged_key.set(None);
        self.dragging_keys.set(HashSet::new());
        super::global_dnd_state::clear_dragging_keys();
        super::global_dnd_state::set_dragging_collection_element(None);

        if let Some(on_end) = self.input.on_drag_end {
            on_end.run(DraggableCollectionEndEvent {
                keys,
                x,
                y,
                drop_effect,
                is_internal,
            });
        }
    }
}

/// Creates the draggable collection state.
pub fn use_draggable_collection_state(
    input: DraggableCollectionStateInput,
) -> DraggableCollectionState {
    let dragged_key = RwSignal::new(None);
    let dragging_keys = RwSignal::new(HashSet::new());

    DraggableCollectionState {
        dragged_key,
        dragging_keys,
        input,
    }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::*;

    use super::*;

    #[test]
    fn test_get_keys_for_drag_selected_key() {
        let _owner = Owner::new();
        _owner.with(|| {
            let selected = HashSet::from(["a".to_string(), "b".to_string(), "c".to_string()]);
            let state = use_draggable_collection_state(DraggableCollectionStateInput {
                collection_keys: Signal::derive(|| {
                    vec!["a".into(), "b".into(), "c".into(), "d".into()]
                }),
                is_selected: Callback::new({
                    let selected = selected.clone();
                    move |key: String| selected.contains(&key)
                }),
                selected_keys: Signal::derive(move || selected.clone()),
                get_items: Callback::new(|keys: Vec<String>| {
                    keys.into_iter().map(DragItem::text).collect()
                }),
                preview: None,
                get_allowed_drop_operations: None,
                collection_ref: Signal::derive(|| None),
                on_drag_start: None,
                on_drag_move: None,
                on_drag_end: None,
            });

            // Dragging a selected key should include all selected keys
            let keys = state.get_keys_for_drag("a");
            assert_eq!(
                keys,
                HashSet::from(["a".to_string(), "b".to_string(), "c".to_string()])
            );

            // Dragging an unselected key should include only that key
            let keys = state.get_keys_for_drag("d");
            assert_eq!(keys, HashSet::from(["d".to_string()]));
        });
    }

    #[test]
    fn test_get_items_for_drag() {
        let _owner = Owner::new();
        _owner.with(|| {
            let state = use_draggable_collection_state(DraggableCollectionStateInput {
                collection_keys: Signal::derive(|| vec!["x".into(), "y".into()]),
                is_selected: Callback::new(|_: String| false),
                selected_keys: Signal::derive(HashSet::new),
                get_items: Callback::new(|keys: Vec<String>| {
                    keys.into_iter().map(DragItem::text).collect()
                }),
                preview: None,
                get_allowed_drop_operations: None,
                collection_ref: Signal::derive(|| None),
                on_drag_start: None,
                on_drag_move: None,
                on_drag_end: None,
            });

            let items = state.get_items("x");
            assert_eq!(items.len(), 1);
            assert_eq!(items[0].data(), "x");
        });
    }
}
