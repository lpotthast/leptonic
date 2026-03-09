use leptos::prelude::*;

use super::use_drag_and_drop::DropTarget;
use crate::hooks::{
    use_droppable, DragItem, DropEffect, DropEnterEvent, DropEvent, DropExitEvent,
    DropOperationEvent, UseDroppableInput, UseDroppableReturn,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
    pub x: f64,
    /// The y coordinate.
    pub y: f64,
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
///     <ul {..droppable.collection_props.drop_props.into_attrs()}>
///         // List items...
///     </ul>
/// }
/// ```
pub fn use_droppable_collection(
    input: UseDroppableCollectionInput,
) -> UseDroppableCollectionReturn {
    let UseDroppableCollectionInput {
        is_disabled: disabled,
        accepted_types,
        get_drop_operation,
        on_drop,
        on_drop_enter,
        on_drop_exit,
    } = input;

    let collection_props = use_droppable(UseDroppableInput {
        is_disabled: disabled,
        accepted_types,
        get_drop_operation,
        on_drop_enter,
        on_drop_exit,
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
