use leptos::prelude::*;

use crate::hooks::{
    use_draggable, DragEndEvent, DragItem, DragStartEvent, DropEffect, UseDraggableInput,
    UseDraggableReturn,
};

use super::use_drag_and_drop::{DragAndDropState, RemoveEvent};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
