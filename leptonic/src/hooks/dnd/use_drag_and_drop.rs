use std::collections::HashSet;

use leptos::prelude::*;

use crate::hooks::{
    DragEndEvent, DragItem, DragMoveEvent, DragStartEvent, DropEffect, DropEnterEvent, DropEvent,
    DropExitEvent, DropMoveEvent, DropOperationEvent,
};
// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/dnd/src/useDragAndDrop.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Options for configuring a draggable item in the drag and drop system.
#[derive(Clone)]
pub struct DraggableOptions {
    /// The items that will be dragged (the key parameter is the item's key).
    pub get_items: Callback<String, Vec<DragItem>>,

    /// The allowed drop effect.
    pub allowed_drop_effect: DropEffect,

    /// Callback when drag starts.
    pub on_drag_start: Option<Callback<DragStartEvent>>,

    /// Callback during drag.
    pub on_drag_move: Option<Callback<DragMoveEvent>>,

    /// Callback when drag ends.
    pub on_drag_end: Option<Callback<DragEndEvent>>,
}

impl Default for DraggableOptions {
    fn default() -> Self {
        Self {
            get_items: Callback::new(|_| vec![]),
            allowed_drop_effect: DropEffect::All,
            on_drag_start: None,
            on_drag_move: None,
            on_drag_end: None,
        }
    }
}

/// Options for configuring a drop zone in the drag and drop system.
#[derive(Clone, Default)]
pub struct DroppableOptions {
    /// The acceptable MIME types.
    pub accepted_types: Vec<String>,

    /// Callback to determine if a drag is acceptable.
    pub get_drop_operation: Option<Callback<DropOperationEvent, DropEffect>>,

    /// Callback when drag enters the drop zone.
    pub on_drop_enter: Option<Callback<DropEnterEvent>>,

    /// Callback when drag is over the drop zone.
    pub on_drop_move: Option<Callback<DropMoveEvent>>,

    /// Callback when drag exits the drop zone.
    pub on_drop_exit: Option<Callback<DropExitEvent>>,

    /// Callback when an item is dropped.
    pub on_drop: Option<Callback<DropEvent>>,
}

/// Input parameters for the `use_drag_and_drop` hook.
#[derive(Clone)]
pub struct UseDragAndDropInput {
    /// Whether drag and drop is disabled.
    pub is_disabled: Signal<bool>,

    /// Options for the draggable behavior.
    pub drag_options: Option<DraggableOptions>,

    /// Options for the droppable behavior.
    pub drop_options: Option<DroppableOptions>,

    /// Callback when items are reordered (for sortable lists).
    pub on_reorder: Option<Callback<ReorderEvent>>,

    /// Callback when items are inserted.
    pub on_insert: Option<Callback<InsertEvent>>,

    /// Callback when items are removed.
    pub on_remove: Option<Callback<RemoveEvent>>,
}

impl Default for UseDragAndDropInput {
    fn default() -> Self {
        Self {
            is_disabled: Signal::derive(|| false),
            drag_options: None,
            drop_options: None,
            on_reorder: None,
            on_insert: None,
            on_remove: None,
        }
    }
}

/// Event fired when items are reordered.
#[derive(Debug, Clone)]
pub struct ReorderEvent {
    /// The keys of the items being moved.
    pub keys: Vec<String>,
    /// The target to drop before/after.
    pub target: DropTarget,
}

/// Event fired when items are inserted.
#[derive(Debug, Clone)]
pub struct InsertEvent {
    /// The items being inserted.
    pub items: Vec<DragItem>,
    /// The target to insert before/after.
    pub target: DropTarget,
}

/// Event fired when items are removed.
#[derive(Debug, Clone)]
pub struct RemoveEvent {
    /// The keys of the items being removed.
    pub keys: Vec<String>,
}

/// The target location for a drop operation.
#[derive(Debug, Clone)]
pub struct DropTarget {
    /// The key of the target item.
    pub key: String,
    /// The position relative to the target.
    pub position: DropPosition,
}

/// The position of a drop relative to the target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropPosition {
    /// Drop before the target.
    Before,
    /// Drop after the target.
    After,
    /// Drop on the target (for nested items).
    On,
}

/// Shared state for drag and drop operations within a collection.
///
/// This state is passed to `use_draggable_item` and `use_droppable_item` hooks
/// so they can coordinate reordering, insertion, and removal behavior.
#[derive(Clone)]
pub struct DragAndDropState {
    /// Keys that belong to this collection (for detecting internal vs external drops).
    pub collection_keys: RwSignal<HashSet<String>>,

    /// Whether an internal reorder happened during the current drag.
    pub internal_reorder_happened: RwSignal<bool>,

    /// Whether drag and drop is disabled.
    pub is_disabled: Signal<bool>,

    /// Signal to set when dragging starts/ends.
    pub set_is_dragging: WriteSignal<bool>,

    /// Callback when items are reordered within the collection.
    pub on_reorder: Option<Callback<ReorderEvent>>,

    /// Callback when items are inserted from another collection.
    pub on_insert: Option<Callback<InsertEvent>>,

    /// Callback when items are removed (moved to another collection).
    pub on_remove: Option<Callback<RemoveEvent>>,

    /// Options for draggable behavior.
    pub drag_options: Option<DraggableOptions>,

    /// Options for droppable behavior.
    pub drop_options: Option<DroppableOptions>,
}

/// The return value of the `use_drag_and_drop` hook.
pub struct UseDragAndDropReturn {
    /// Shared state to pass to `use_draggable_item` and `use_droppable_item`.
    pub state: DragAndDropState,

    /// Whether a drag operation is in progress.
    pub is_dragging: Signal<bool>,
}

/// Provides a complete drag and drop solution for collections.
///
/// This hook returns shared state that should be passed to `use_draggable_item`
/// and `use_droppable_item` hooks called at the component level for each item.
///
/// # Example
///
/// ```ignore
/// // In the parent component:
/// let dnd = use_drag_and_drop(UseDragAndDropInput {
///     drag_options: Some(DraggableOptions {
///         get_items: Callback::new(|key: String| {
///             vec![DragItem::text(key)]
///         }),
///         ..Default::default()
///     }),
///     drop_options: Some(DroppableOptions {
///         accepted_types: vec!["text/plain".to_string()],
///         ..Default::default()
///     }),
///     on_reorder: Some(Callback::new(|e: ReorderEvent| {
///         tracing::info!("Reorder: {:?} -> {:?}", e.keys, e.target);
///     })),
///     ..Default::default()
/// });
///
/// // In each item component:
/// #[component]
/// fn Item(key: String, state: DragAndDropState) -> impl IntoView {
///     let drag = use_draggable_item(key.clone(), state.clone());
///     let drop = use_droppable_item(key, state);
///     view! {
///         <div {..drag.drag_props} {..drop.drop_props}>...</div>
///     }
/// }
/// ```
pub fn use_drag_and_drop(input: UseDragAndDropInput) -> UseDragAndDropReturn {
    let UseDragAndDropInput {
        is_disabled: disabled,
        drag_options,
        drop_options,
        on_reorder,
        on_insert,
        on_remove,
    } = input;

    let (is_dragging, set_is_dragging) = signal(false);

    // Track which keys belong to this collection for detecting internal vs external drops.
    // Using RwSignal to survive SSR hydration.
    let collection_keys = RwSignal::new(HashSet::<String>::new());

    // Track whether an internal reorder happened during the current drag.
    // This is used to determine whether to call on_remove.
    let internal_reorder_happened = RwSignal::new(false);

    let state = DragAndDropState {
        collection_keys,
        internal_reorder_happened,
        is_disabled: disabled,
        set_is_dragging,
        on_reorder,
        on_insert,
        on_remove,
        drag_options,
        drop_options,
    };

    UseDragAndDropReturn {
        state,
        is_dragging: is_dragging.into(),
    }
}
