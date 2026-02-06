use std::collections::HashSet;

use crate::hooks::{
    use_draggable, use_droppable, DragEndEvent, DragItem, DragMoveEvent, DragStartEvent,
    DropEffect, DropEnterEvent, DropEvent, DropExitEvent, DropMoveEvent, DropOperationEvent,
    UseDraggableInput, UseDraggableReturn, UseDroppableInput, UseDroppableReturn,
};
use leptos::prelude::*;
// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/dnd/src/useDragAndDrop.ts

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
        is_disabled: input.is_disabled,
        set_is_dragging,
        on_reorder: input.on_reorder,
        on_insert: input.on_insert,
        on_remove: input.on_remove,
        drag_options: input.drag_options,
        drop_options: input.drop_options,
    };

    UseDragAndDropReturn {
        state,
        is_dragging: is_dragging.into(),
    }
}

/// Creates draggable props for an item within a drag-and-drop collection.
///
/// This hook should be called at the component level (not inside a callback)
/// to ensure event handlers are properly attached during SSR hydration.
///
/// # Arguments
///
/// * `key` - A unique identifier for this item within the collection.
/// * `state` - The shared state from `use_drag_and_drop`.
///
/// # Returns
///
/// Returns `Some(UseDraggableReturn)` if drag options are configured,
/// or `None` if dragging is not enabled for this collection.
///
/// # Example
///
/// ```ignore
/// #[component]
/// fn DraggableItem(key: String, state: DragAndDropState) -> impl IntoView {
///     let drag = use_draggable_item(key, state);
///
///     if let Some(drag) = drag {
///         view! { <div {..drag.drag_props}>"Drag me"</div> }.into_any()
///     } else {
///         view! { <div>"Not draggable"</div> }.into_any()
///     }
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_draggable_item(key: String, state: DragAndDropState) -> Option<UseDraggableReturn> {
    // Register this key as part of the collection
    state.collection_keys.update(|keys| {
        keys.insert(key.clone());
    });

    let drag_options = state.drag_options.as_ref()?;

    let get_items = drag_options.get_items;
    let on_drag_start = drag_options.on_drag_start;
    let on_drag_move = drag_options.on_drag_move;
    let on_drag_end = drag_options.on_drag_end;
    let allowed_drop_effect = drag_options.allowed_drop_effect;
    let key_for_items = key.clone();
    let key_for_end = key;

    let set_is_dragging = state.set_is_dragging;
    let internal_reorder_happened = state.internal_reorder_happened;
    let on_remove = state.on_remove;
    let is_disabled = state.is_disabled;

    // Wrap callbacks to track dragging state
    let on_start_with_state = Callback::new(move |e: DragStartEvent| {
        set_is_dragging.set(true);
        internal_reorder_happened.set(false);
        if let Some(cb) = on_drag_start {
            cb.run(e);
        }
    });

    let on_end_with_state = Callback::new(move |e: DragEndEvent| {
        set_is_dragging.set(false);
        // If the drop effect was Move and no internal reorder happened,
        // the item was moved to a different collection - call on_remove
        if e.drop_effect == DropEffect::Move
            && !internal_reorder_happened.get_untracked()
            && on_remove.is_some()
        {
            if let Some(on_remove) = on_remove {
                on_remove.run(RemoveEvent {
                    keys: vec![key_for_end.clone()],
                });
            }
        }
        internal_reorder_happened.set(false);
        if let Some(cb) = on_drag_end {
            cb.run(e);
        }
    });

    Some(use_draggable(UseDraggableInput {
        is_disabled,
        get_items: Callback::new(move |_| {
            // Include the key in the items
            let mut items = get_items.run(key_for_items.clone());
            items.push(DragItem::custom(
                "application/x-dnd-key",
                key_for_items.clone(),
            ));
            items
        }),
        allowed_drop_effect,
        on_drag_start: Some(on_start_with_state),
        on_drag_move,
        on_drag_end: Some(on_end_with_state),
        ..Default::default()
    }))
}

/// Creates droppable props for an item within a drag-and-drop collection.
///
/// This hook should be called at the component level (not inside a callback)
/// to ensure event handlers are properly attached during SSR hydration.
///
/// # Arguments
///
/// * `key` - A unique identifier for this item within the collection.
/// * `state` - The shared state from `use_drag_and_drop`.
///
/// # Returns
///
/// Returns `Some(UseDroppableReturn)` if drop options are configured,
/// or `None` if dropping is not enabled for this collection.
///
/// # Example
///
/// ```ignore
/// #[component]
/// fn DroppableItem(key: String, state: DragAndDropState) -> impl IntoView {
///     let drop = use_droppable_item(key, state);
///
///     if let Some(drop) = drop {
///         view! {
///             <div
///                 {..drop.drop_props}
///                 class:drop-target=move || drop.is_drop_target.get()
///             >
///                 "Drop here"
///             </div>
///         }.into_any()
///     } else {
///         view! { <div>"Not droppable"</div> }.into_any()
///     }
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_droppable_item(key: String, state: DragAndDropState) -> Option<UseDroppableReturn> {
    let drop_options = state.drop_options.as_ref()?;

    let mut accepted_types = drop_options.accepted_types.clone();
    // Also accept our internal key type
    accepted_types.push("application/x-dnd-key".to_string());

    let target_key = key;
    let user_on_drop = drop_options.on_drop;
    let collection_keys = state.collection_keys;
    let internal_reorder_happened = state.internal_reorder_happened;
    let on_reorder = state.on_reorder;
    let on_insert = state.on_insert;
    let is_disabled = state.is_disabled;

    Some(use_droppable(UseDroppableInput {
        is_disabled,
        accepted_types,
        get_drop_operation: drop_options.get_drop_operation,
        on_drop_enter: drop_options.on_drop_enter,
        on_drop_move: drop_options.on_drop_move,
        on_drop_exit: drop_options.on_drop_exit,
        on_drop: Some(Callback::new(move |e: DropEvent| {
            // Extract the dragged key from the items
            let dragged_key = e
                .items
                .iter()
                .find(|item| item.kind == "application/x-dnd-key")
                .map(|item| item.data.clone());

            if let Some(dragged_key) = dragged_key {
                // Check if this is an internal reorder or external insert
                let is_internal = collection_keys.with(|keys| keys.contains(&dragged_key));

                let target = DropTarget {
                    key: target_key.clone(),
                    position: DropPosition::After, // Default to after
                };

                if is_internal {
                    // Internal reorder - mark that it happened so on_remove won't be called
                    internal_reorder_happened.set(true);
                    if let Some(on_reorder) = on_reorder {
                        on_reorder.run(ReorderEvent {
                            keys: vec![dragged_key],
                            target,
                        });
                    }
                } else {
                    // External insert
                    if let Some(on_insert) = on_insert {
                        // Filter out the internal key from items
                        let items: Vec<DragItem> = e
                            .items
                            .iter()
                            .filter(|item| item.kind != "application/x-dnd-key")
                            .cloned()
                            .collect();

                        on_insert.run(InsertEvent { items, target });
                    }
                }
            }

            // Call the user's on_drop callback if provided
            if let Some(user_cb) = user_on_drop {
                user_cb.run(e);
            }
        })),
    }))
}

/// Input parameters for the `use_droppable_collection` hook.
#[derive(Clone)]
pub struct UseDroppableCollectionInput {
    /// Whether the collection is disabled.
    pub is_disabled: Signal<bool>,

    /// The acceptable MIME types.
    pub accepted_types: Vec<String>,

    /// Callback to determine the drop operation.
    pub get_drop_operation: Option<Callback<DropOperationEvent, DropEffect>>,

    /// Callback when items are dropped.
    pub on_drop: Option<Callback<CollectionDropEvent>>,

    /// Callback when drag enters.
    pub on_drop_enter: Option<Callback<DropEnterEvent>>,

    /// Callback when drag exits.
    pub on_drop_exit: Option<Callback<DropExitEvent>>,
}

impl Default for UseDroppableCollectionInput {
    fn default() -> Self {
        Self {
            is_disabled: Signal::derive(|| false),
            accepted_types: vec![],
            get_drop_operation: None,
            on_drop: None,
            on_drop_enter: None,
            on_drop_exit: None,
        }
    }
}

/// Event fired when items are dropped on a collection.
#[derive(Debug, Clone)]
pub struct CollectionDropEvent {
    /// The dropped items.
    pub items: Vec<DragItem>,
    /// The drop effect.
    pub drop_effect: DropEffect,
    /// The target in the collection.
    pub target: Option<DropTarget>,
    /// The x coordinate.
    pub x: i32,
    /// The y coordinate.
    pub y: i32,
}

/// The return value of the `use_droppable_collection` hook.
pub struct UseDroppableCollectionReturn {
    /// Props for the collection container.
    pub collection_props: UseDroppableReturn,

    /// Whether an item is being dragged over the collection.
    pub is_drop_target: Signal<bool>,
}

/// Provides drop functionality for a collection of items.
///
/// This hook enables dropping items onto a collection, with support
/// for determining the specific drop target within the collection.
///
/// # Example
///
/// ```ignore
/// let droppable = use_droppable_collection(UseDroppableCollectionInput {
///     accepted_types: vec!["text/plain".to_string()],
///     on_drop: Some(Callback::new(|e: CollectionDropEvent| {
///         if let Some(target) = e.target {
///             tracing::info!("Drop at {:?}: {:?}", target, e.items);
///         }
///     })),
///     ..Default::default()
/// });
///
/// view! {
///     <ul {..droppable.collection_props.drop_props}>
///         // List items...
///     </ul>
/// }
/// ```
pub fn use_droppable_collection(
    input: UseDroppableCollectionInput,
) -> UseDroppableCollectionReturn {
    let on_drop = input.on_drop;

    let collection_props = use_droppable(UseDroppableInput {
        is_disabled: input.is_disabled,
        accepted_types: input.accepted_types,
        get_drop_operation: input.get_drop_operation,
        on_drop_enter: input.on_drop_enter,
        on_drop_exit: input.on_drop_exit,
        on_drop_move: None,
        on_drop: Some(Callback::new(move |e: DropEvent| {
            if let Some(on_drop) = on_drop {
                on_drop.run(CollectionDropEvent {
                    items: e.items,
                    drop_effect: e.drop_effect,
                    target: None, // Would need hit-testing to determine target
                    x: e.x,
                    y: e.y,
                });
            }
        })),
    });

    let is_drop_target = collection_props.is_drop_target;

    UseDroppableCollectionReturn {
        collection_props,
        is_drop_target,
    }
}
