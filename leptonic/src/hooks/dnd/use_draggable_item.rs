use leptos::prelude::*;

use super::draggable_collection_state::DraggableCollectionState;
use crate::hooks::{
    DragEndEvent, DragItem, DragMoveEvent, DragStartEvent, UseDraggableInput, UseDraggableReturn,
    use_draggable,
};

//
// ## API DIFFERENCES
//
// - Uses `DraggableCollectionState` with selection awareness instead of
//   react-aria's direct `useDraggableItem` hook.
// - `has_action` parameter mirrors react-aria's `hasAction`: when true,
//   keyboard drag requires Alt+Enter to avoid conflicting with item actions.
//

/// Input for [`use_draggable_collection_item`].
pub struct UseDraggableCollectionItemInput {
    /// A unique identifier for this item within the collection.
    pub key: String,
    /// The draggable collection state (selection-aware).
    pub state: DraggableCollectionState,
    /// Whether this item is disabled.
    pub is_disabled: Signal<bool>,
    /// Whether this item has a primary action (e.g. navigation, selection).
    /// When true, keyboard drag requires Alt+Enter instead of Enter.
    pub has_action: bool,
}

/// Creates draggable props for an item within a selection-aware collection.
///
/// Creates draggable props for an item within a selection-aware collection.
///
/// Supports multi-select drag: when a selected item is dragged, all
/// selected items are included in the drag.
///
/// # Example
///
/// ```ignore
/// #[component]
/// fn CollectionItem(key: String, state: DraggableCollectionState) -> impl IntoView {
///     let drag = use_draggable_collection_item(UseDraggableCollectionItemInput {
///         key,
///         state,
///         is_disabled: Signal::derive(|| false),
///     });
///
///     view! { <div {..drag.drag_props.into_attrs()}>"Drag me"</div> }
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_draggable_collection_item(input: UseDraggableCollectionItemInput) -> UseDraggableReturn {
    let UseDraggableCollectionItemInput {
        key,
        state,
        is_disabled,
        has_action,
    } = input;

    let key_for_items = key.clone();
    let key_for_start = key.clone();
    let state_for_start = state.clone();
    let state_for_move = state.clone();
    let state_for_end = state.clone();
    let state_for_items = state.clone();
    let state_for_ops = state.clone();
    let state_for_preview = state.clone();

    use_draggable(UseDraggableInput {
        is_disabled,
        get_items: Callback::new(move |_| {
            let mut items = state_for_items.get_items(&key_for_items);
            // Add internal key marker for all dragged keys.
            let dragging = state_for_items.dragging_keys().get_untracked();
            if dragging.is_empty() {
                // Before drag starts, include just this key
                items.push(DragItem::custom(
                    "application/x-dnd-key",
                    key_for_items.clone(),
                ));
            } else {
                for k in &dragging {
                    items.push(DragItem::custom("application/x-dnd-key", k.clone()));
                }
            }
            items
        }),
        get_allowed_drop_operations: Callback::new(move |_| {
            state_for_ops.get_allowed_drop_operations()
        }),
        render_drag_preview: Some(Callback::new(move |items: Vec<DragItem>| {
            state_for_preview.get_preview(items)
        })),
        on_drag_start: Some(Callback::new(move |e: DragStartEvent| {
            state_for_start.start_drag(&key_for_start, &e);
        })),
        on_drag_move: Some(Callback::new(move |e: DragMoveEvent| {
            state_for_move.move_drag(e.x, e.y);
        })),
        on_drag_end: Some(Callback::new(move |e: DragEndEvent| {
            state_for_end.end_drag(e.drop_effect, e.x, e.y);
        })),
        has_drag_button: false,
        has_action,
    })
}
