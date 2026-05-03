#![cfg_attr(feature = "ssr", allow(dead_code, unused_imports))]

use leptos::prelude::*;

#[cfg(not(feature = "ssr"))]
use super::drag_manager::KeyDownForwardEvent;
#[cfg(not(feature = "ssr"))]
use super::drop_target_keyboard_navigation::{NavigationDirection, navigate_drop_target};
use super::{
    drop_target_delegate::DropTargetDelegate, droppable_collection_state::DroppableCollectionState,
    global_dnd_state, types::DropTarget,
};
#[cfg(not(feature = "ssr"))]
use crate::hooks::AllowedDropOperations;
#[cfg(not(feature = "ssr"))]
use crate::hooks::DragTypes;
use crate::hooks::{
    DropEffect, DropEvent, DropExitEvent, DropOperationForPointEvent, UseAutoScrollInput,
    UseDroppableInput, UseDroppableReturn, selection::keyboard_delegate::KeyboardDelegate,
    use_auto_scroll, use_droppable,
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/dnd/src/useDroppableCollection.ts

//
// ## LEPTOS-SPECIFIC ADAPTATIONS
//
// - Uses `Box<dyn DropTargetDelegate>` and `Box<dyn KeyboardDelegate<String>>`
//   stored in `StoredValue` instead of React refs.
// - Keyboard navigation is integrated via `DragManager.on_key_down` forwarding
//   instead of react-aria's `useDroppableCollectionRef` approach.
//
// ## ELEMENT ACCESS
// - Uses the caller-provided `collection_ref: Signal<Option<Element>>`
//   for DragManager re-registration instead of `document.getElementById`.
//   Same SSR+hydration rationale as `use_droppable`.
//
// ## OMITTED
// - `aria-describedby` on collection element (removed; description is on
//   individual drop targets managed by `use_droppable`).
//

/// Input parameters for the `use_droppable_collection` hook.
pub struct UseDroppableCollectionInput {
    /// The droppable collection state (created via `use_droppable_collection_state`).
    pub state: DroppableCollectionState,

    /// The keyboard delegate for navigating drop positions.
    pub keyboard_delegate: Box<dyn KeyboardDelegate<String>>,

    /// The drop target delegate for pointer hit-testing.
    pub drop_target_delegate: Box<dyn DropTargetDelegate>,

    /// Whether the collection is disabled.
    pub is_disabled: Signal<bool>,

    /// The acceptable MIME types (empty = accept all).
    pub accepted_types: Vec<String>,

    /// The scrollable container element for auto-scrolling during drag.
    /// When provided, the collection auto-scrolls when the pointer is near
    /// the container's edges during a drag operation.
    pub collection_ref: Signal<Option<web_sys::Element>>,
}

/// The return value of the `use_droppable_collection` hook.
pub struct UseDroppableCollectionReturn {
    /// The underlying droppable return (props for the collection container).
    pub collection_props: UseDroppableReturn,

    /// Whether a drag is currently over this collection.
    pub is_drop_target: Signal<bool>,
}

/// Provides drop functionality for a collection of items with hit-testing,
/// keyboard navigation, and semantic dispatch.
///
/// This hook:
/// - Registers the collection as a drop target with pointer-based hit-testing
///   via a [`DropTargetDelegate`].
/// - Supports keyboard navigation between drop positions within the collection
///   via a [`KeyboardDelegate`].
/// - Dispatches drops to semantic callbacks (reorder, insert, root drop, etc.)
///   via [`DroppableCollectionState`].
///
/// # Example
///
/// ```ignore
/// let collection_state = use_droppable_collection_state(DroppableCollectionStateInput {
///     collection_keys: keys_signal,
///     on_reorder: Some(Callback::new(|e: CollectionReorderEvent| {
///         // Handle reorder
///     })),
///     ..Default::default()
/// });
///
/// let collection = use_droppable_collection(UseDroppableCollectionInput {
///     state: collection_state.clone(),
///     keyboard_delegate: Box::new(ListKeyboardDelegate::new(
///         keys_signal,
///         Signal::derive(|| HashSet::new()),
///         Orientation::Vertical,
///         Signal::derive(|| WritingDirection::Ltr),
///     )),
///     drop_target_delegate: Box::new(/* ... */),
///     is_disabled: Signal::derive(|| false),
///     accepted_types: vec!["text/plain".to_string()],
/// });
///
/// view! {
///     <ul {..collection.collection_props.drop_props.into_attrs()}>
///         // Items with data-key attributes...
///     </ul>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_droppable_collection(
    input: UseDroppableCollectionInput,
) -> UseDroppableCollectionReturn {
    cfg_if::cfg_if! {
        if #[cfg(feature = "ssr")] {
            let UseDroppableCollectionInput {
                state: _,
                keyboard_delegate: _,
                drop_target_delegate: _,
                is_disabled,
                accepted_types: _,
                collection_ref: _,
            } = input;

            let collection_props = use_droppable(UseDroppableInput {
                is_disabled,
                ..Default::default()
            });
            UseDroppableCollectionReturn {
                is_drop_target: collection_props.is_drop_target,
                collection_props,
            }
        } else {
            let UseDroppableCollectionInput {
                state,
                keyboard_delegate,
                drop_target_delegate,
                is_disabled: disabled,
                accepted_types,
                collection_ref,
            } = input;

            // Store delegates in StoredValue for use in closures.
            let kb_delegate: StoredValue<Box<dyn KeyboardDelegate<String>>, LocalStorage> =
                StoredValue::new_local(keyboard_delegate);
            let dt_delegate: StoredValue<Box<dyn DropTargetDelegate>, LocalStorage> =
                StoredValue::new_local(drop_target_delegate);

    // Auto-scroll: scrolls the container when the pointer is near its edges.
    let auto_scroll = use_auto_scroll(UseAutoScrollInput {
        container: collection_ref,
    });
    let auto_scroll_stop_for_drop = auto_scroll.stop;
    let auto_scroll_stop_for_exit = auto_scroll.stop;

    let state_for_point = state.clone();
    let state_for_drop = state.clone();
    #[cfg(not(feature = "ssr"))]
    let state_for_key = state.clone();
    let state_for_cleanup = state.clone();

    // Callback for position-aware drop operation — routes through hit-testing
    // delegate and collection state validation.
    let auto_scroll_move = auto_scroll.move_to;
    let get_drop_operation_for_point = Callback::new(move |e: DropOperationForPointEvent| {
        // Drive auto-scroll with client coordinates from the drag event.
        auto_scroll_move.run((e.x, e.y));

        let dragging_keys = global_dnd_state::get_dragging_keys();
        let is_internal = !dragging_keys.is_empty();

        let target = dt_delegate.with_value(|delegate| {
            delegate.get_drop_target_from_point(e.x, e.y, &|candidate| {
                state_for_point.get_drop_operation(
                    candidate,
                    &e.types,
                    e.allowed_operations,
                    is_internal,
                    &dragging_keys,
                ) != DropEffect::None
            })
        });

        if let Some(ref target) = target {
            let effect = state_for_point.get_drop_operation(
                target,
                &e.types,
                e.allowed_operations,
                is_internal,
                &dragging_keys,
            );

            // Update the tracked target in state.
            state_for_point.set_target(Some(target));

            effect
        } else {
            state_for_point.set_target(None);
            DropEffect::None
        }
    });

    // Keyboard handler for collection-level navigation.
    #[cfg(not(feature = "ssr"))]
    let on_key_down = Callback::new(move |e: KeyDownForwardEvent| -> bool {
        let direction = match e.key.as_str() {
            "ArrowDown" | "ArrowRight" => Some(NavigationDirection::Next),
            "ArrowUp" | "ArrowLeft" => Some(NavigationDirection::Previous),
            "Home" => Some(NavigationDirection::First),
            "End" => Some(NavigationDirection::Last),
            _ => None,
        };

        let Some(direction) = direction else {
            return false;
        };

        let current_target = state_for_key.target().get_untracked();
        let dragging_keys = global_dnd_state::get_dragging_keys();

        let new_target = kb_delegate.with_value(|delegate| {
            navigate_drop_target(
                delegate.as_ref(),
                current_target.as_ref(),
                direction,
                &|candidate| {
                    // Types are not available during keyboard navigation;
                    // use empty known set so acceptance is based on callbacks only.
                    let empty_types = DragTypes::Known(std::collections::HashSet::new());
                    state_for_key.get_drop_operation(
                        candidate,
                        &empty_types,
                        AllowedDropOperations::ALL,
                        !dragging_keys.is_empty(),
                        &dragging_keys,
                    ) != DropEffect::None
                },
            )
        });

        if let Some(ref new_target) = new_target {
            state_for_key.set_target(Some(new_target));
            true
        } else {
            false
        }
    });

    // Exit handler: stop auto-scroll and clear drop collection element.
    let on_drop_exit_handler = Callback::new(move |_e: DropExitEvent| {
        auto_scroll_stop_for_exit.run(());
        global_dnd_state::set_drop_collection_element(None);
    });

    // Drop handler that routes through state's semantic dispatch.
    let on_drop_handler = Callback::new(move |e: DropEvent| {
        auto_scroll_stop_for_drop.run(());

        // Set the drop collection element so draggable collection state can
        // detect internal drops.
        let drop_element = collection_ref.get_untracked();
        global_dnd_state::set_drop_collection_element(drop_element.as_ref());

        let target = state_for_drop
            .target()
            .get_untracked()
            .unwrap_or(DropTarget::Root);
        let dragging_keys = global_dnd_state::get_dragging_keys();
        let is_internal = !dragging_keys.is_empty();

        state_for_drop.dispatch_drop(e.items, e.drop_effect, &target, is_internal, &dragging_keys);

        // Clear target and drop collection element after drop.
        state_for_drop.set_target(None);
        global_dnd_state::set_drop_collection_element(None);
    });

    // Create the underlying droppable.
    let collection_props = use_droppable(UseDroppableInput {
        is_disabled: disabled,
        accepted_types,
        get_drop_operation: None,
        get_drop_operation_for_point: Some(get_drop_operation_for_point),
        on_drop_enter: None,
        on_drop_move: None,
        on_drop_exit: Some(on_drop_exit_handler),
        on_drop: Some(on_drop_handler),
        on_drop_activate: None,
    });

    // Register with DragManager with our keyboard handler.
    #[cfg(not(feature = "ssr"))]
    {
        let dm_id = collection_props.droppable_id.clone();

        Effect::new(move || {
            if disabled.get() {
                return;
            }
            let Some(element) = collection_ref.get() else {
                return;
            };
            // Ensure DOM id matches the droppable's generated id for consistency.
            element.set_id(&dm_id);

            // Re-register with key forwarding support.
            // Note: This overwrites the registration done by use_droppable,
            // which is intentional — we want the collection-level keyboard handler.
            super::drag_manager::register_drop_target(super::drag_manager::RegisteredDropTarget {
                id: dm_id.clone(),
                element,
                accepted_types: vec![], // Collection accepts all types through delegate
                get_drop_operation: None, // Handled by get_drop_operation_for_point
                on_drop_enter: None,
                on_drop_exit: None,
                on_drop: None, // Handled by the use_droppable on_drop
                on_drop_activate: None,
                on_key_down: Some(on_key_down),
                prevent_focus_on_drop: true,
                activate_button: None,
            });

            let cleanup_id = dm_id.clone();
            on_cleanup(move || {
                super::drag_manager::unregister_drop_target(&cleanup_id);
            });
        });
    }

    // Focus management after drop: watch focus_after_drop_key and focus the
    // corresponding element when it becomes available.
    #[cfg(not(feature = "ssr"))]
    {
        let state_for_focus = state.clone();
        Effect::new(move || {
            if let Some(key) = state_for_focus.focus_after_drop_key.get() {
                // Find the element by data-key attribute and focus it.
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    let selector = format!("[data-key=\"{key}\"]");
                    if let Ok(Some(el)) = document.query_selector(&selector) {
                        use wasm_bindgen::JsCast;
                        if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
                            let _ = html_el.focus();
                        }
                    }
                }
                // Clear the signal after focusing.
                state_for_focus.focus_after_drop_key.set(None);
            }
        });
    }

    // Clear target on cleanup.
    on_cleanup(move || {
        state_for_cleanup.set_target(None);
    });

            let is_drop_target = collection_props.is_drop_target;

            UseDroppableCollectionReturn {
                collection_props,
                is_drop_target,
            }
        }
    }
}
