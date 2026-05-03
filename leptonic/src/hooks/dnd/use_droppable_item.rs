use leptos::{attr, attr::Attr, prelude::*};

use super::{
    drag_manager, droppable_collection_state::DroppableCollectionState, types::DropTarget,
};
#[cfg(not(feature = "ssr"))]
use crate::hooks::AllowedDropOperations;
use crate::hooks::{DragTypes, DropEffect, IntoAttrs};

//
// ## API DIFFERENCES
//
// - `use_collection_droppable_item` provides per-item DragManager registration,
//   ARIA hiding, and auto-focus for items within `DroppableCollectionState`-based
//   collections. React-aria achieves this through `useDroppableItem` combined
//   with `useVirtualDrop`.
//

/// Input for [`use_collection_droppable_item`].
pub struct UseCollectionDroppableItemInput {
    /// The drop target this item represents.
    pub target: DropTarget,
    /// The droppable collection state.
    pub state: DroppableCollectionState,
    /// Whether this item is disabled.
    pub is_disabled: Signal<bool>,
}

/// Return value of [`use_collection_droppable_item`].
pub struct UseCollectionDroppableItemReturn {
    /// Props for the droppable item element.
    pub drop_item_props: UseCollectionDroppableItemProps,
    /// Whether this item is the current drop target.
    pub is_drop_target: Signal<bool>,
}

/// Props for a collection droppable item.
#[derive(Debug)]
pub struct UseCollectionDroppableItemProps {
    /// The `aria-hidden` attribute — hides invalid items during keyboard drag.
    pub aria_hidden: Signal<Option<&'static str>>,
}

impl IntoAttrs for UseCollectionDroppableItemProps {
    type Attrs = UseCollectionDroppableItemAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::AriaHidden, self.aria_hidden),)
    }
}

/// Attributes for a collection droppable item.
pub type UseCollectionDroppableItemAttrs = (Attr<attr::AriaHidden, Signal<Option<&'static str>>>,);

/// Provides per-item drop behavior within a `DroppableCollectionState`-based
/// collection.
///
/// This hook:
/// - Registers the item with the `DragManager` so it participates in ARIA
///   hiding during keyboard drag sessions.
/// - Reactively sets `aria-hidden` on items that are not valid drop targets.
/// - Auto-focuses the item when it becomes the active drop target during a
///   keyboard drag session.
///
/// # Example
///
/// ```ignore
/// let item = use_collection_droppable_item(UseCollectionDroppableItemInput {
///     target: DropTarget::Item { key: "item-1".into(), position: DropPosition::On },
///     state: collection_state.clone(),
///     is_disabled: Signal::derive(|| false),
/// });
///
/// view! {
///     <div
///         {..item.drop_item_props.into_attrs()}
///         class:drop-target=move || item.is_drop_target.get()
///     >
///         "Item content"
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_collection_droppable_item(
    input: UseCollectionDroppableItemInput,
) -> UseCollectionDroppableItemReturn {
    let UseCollectionDroppableItemInput {
        target,
        state,
        is_disabled,
    } = input;

    let target_for_active = target.clone();
    let state_for_active = state.clone();

    // Whether this item is the current drop target.
    let is_drop_target =
        Signal::derive(move || state_for_active.is_drop_target(&target_for_active));

    // Subscribe to keyboard drag session state.
    let session_active = drag_manager::use_drag_session_active();

    // Compute whether this item is a valid drop target in the current session.
    let target_for_valid = target.clone();
    let state_for_valid = state.clone();
    let is_valid_drop_target = Signal::derive(move || {
        if !session_active.get() || is_disabled.get() {
            return false;
        }
        // Get session drag info to validate against.
        let items = drag_manager::get_session_drag_items();
        let allowed = drag_manager::get_session_allowed_operations();
        let (Some(items), Some(allowed)) = (items, allowed) else {
            return false;
        };
        let types = DragTypes::Known(
            items
                .iter()
                .flat_map(|item| item.types().map(String::from).collect::<Vec<_>>())
                .collect(),
        );
        let dragging_keys = super::global_dnd_state::get_dragging_keys();
        let is_internal = !dragging_keys.is_empty();
        state_for_valid.get_drop_operation(
            &target_for_valid,
            &types,
            allowed,
            is_internal,
            &dragging_keys,
        ) != DropEffect::None
    });

    // aria-hidden: hide during keyboard drag when this item is not a valid target.
    let aria_hidden = Signal::derive(move || {
        if session_active.get() && !is_valid_drop_target.get() {
            Some("true")
        } else {
            None
        }
    });

    // Register with DragManager and set up auto-focus.
    #[cfg(not(feature = "ssr"))]
    {
        let target_for_reg = target.clone();
        let state_for_reg = state.clone();

        // Use an effect to register/unregister with DragManager.
        // The element must be found by data-key attribute since we don't have a NodeRef.
        Effect::new(move || {
            if is_disabled.get() {
                return;
            }

            let target_key = match &target_for_reg {
                DropTarget::Root => return,
                DropTarget::Item { key, .. } => key.clone(),
            };

            let Some(document) = web_sys::window().and_then(|w| w.document()) else {
                return;
            };
            let selector = format!("[data-key=\"{target_key}\"]");
            let Ok(Some(element)) = document.query_selector(&selector) else {
                return;
            };

            let state_for_cb = state_for_reg.clone();
            let target_for_cb = target_for_reg.clone();
            let get_drop_operation = Callback::new(
                move |(types, allowed): (DragTypes, AllowedDropOperations)| {
                    let dragging_keys = super::global_dnd_state::get_dragging_keys();
                    let is_internal = !dragging_keys.is_empty();
                    state_for_cb.get_drop_operation(
                        &target_for_cb,
                        &types,
                        allowed,
                        is_internal,
                        &dragging_keys,
                    )
                },
            );

            drag_manager::register_drop_item(drag_manager::RegisteredDropItem {
                element: element.clone(),
                target: target_for_reg.clone(),
                get_drop_operation: Some(get_drop_operation),
                activate_button: None,
            });

            let cleanup_element = element.clone();
            on_cleanup(move || {
                drag_manager::unregister_drop_item(&cleanup_element);
            });
        });

        // Auto-focus the item when it becomes the active drop target during a
        // keyboard drag session.
        Effect::new(move || {
            if !session_active.get() || !is_drop_target.get() {
                return;
            }

            let target_key = match &target {
                DropTarget::Root => return,
                DropTarget::Item { key, .. } => key.clone(),
            };

            let Some(document) = web_sys::window().and_then(|w| w.document()) else {
                return;
            };
            let selector = format!("[data-key=\"{target_key}\"]");
            if let Ok(Some(el)) = document.query_selector(&selector) {
                use wasm_bindgen::JsCast;
                if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
                    let _ = html_el.focus();
                }
            }
        });
    }

    UseCollectionDroppableItemReturn {
        drop_item_props: UseCollectionDroppableItemProps { aria_hidden },
        is_drop_target,
    }
}
