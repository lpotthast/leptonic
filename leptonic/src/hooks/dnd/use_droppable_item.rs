use leptos::prelude::*;

use super::use_drag_and_drop::{
    DragAndDropState, DropPosition, DropTarget, InsertEvent, ReorderEvent,
};
use crate::hooks::{use_droppable, DragItem, DropEvent, UseDroppableInput, UseDroppableReturn};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
