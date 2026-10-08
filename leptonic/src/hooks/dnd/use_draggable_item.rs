// Upstream: react-aria/src/dnd/useDraggableItem.ts @ 99e6102368
use leptos::prelude::*;

use super::{
    messages,
    types::{DragEndEvent, DragMoveEvent, DragStartEvent, DropOperation},
    use_drag::{UseDragInput, UseDragProps, UseDragReturn, use_drag},
    use_draggable_collection_state::DraggableCollectionState,
    utils::{
        DragModality, clear_global_dnd_state, is_internal_drop_operation, set_dragging_keys,
        use_drag_modality,
    },
};
use crate::{
    hooks::{SelectionMode, UseButtonInput, collections::Key},
    utils::{
        EventHandler,
        intl_strings::{DndStrings, use_localized_strings},
        use_description::use_description,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The drag button comes back as a `UseButtonInput` for `use_button`; its label (which changes
//   with the selection) is the separate `drag_button_label` signal.
// - Whether the item has a drag button or an action is fixed when the hook runs (the collection's
//   selection mode is followed).
//
// =============================================================================

/// Input of [`use_draggable_item`].
#[derive(Debug, Clone)]
pub struct UseDraggableItemInput {
    pub state: DraggableCollectionState,
    /// The item's key.
    pub key: Key,
    /// Keyboard and screen reader drags start from a drag button (`drag_button`).
    pub has_drag_button: bool,
    /// The item has an action (Enter), so keyboard drags start with Alt + Enter.
    pub has_action: bool,
}

/// Return value of [`use_draggable_item`].
pub struct UseDraggableItemReturn {
    pub drag_props: UseDragProps,
    /// For `use_button`, with `has_drag_button`.
    pub drag_button: UseButtonInput,
    /// The drag button's `aria-label` ("Drag Inbox", "Drag 3 selected items").
    pub drag_button_label: Signal<String>,
    pub is_dragging: Signal<bool>,
}

/// A draggable item of a collection: dragging a selected item drags all selected items.
pub fn use_draggable_item(input: UseDraggableItemInput) -> UseDraggableItemReturn {
    let UseDraggableItemInput {
        state,
        key,
        has_drag_button,
        has_action,
    } = input;
    let selection = state.list.selection;
    let item_key = StoredValue::new(key.clone());
    let is_disabled = Signal::derive(move || {
        state.is_disabled.get() || item_key.with_value(|k| selection.is_disabled(k))
    });

    let UseDragReturn {
        mut drag_props,
        drag_button,
        is_dragging,
    } = use_drag(UseDragInput {
        items: Signal::derive(move || item_key.with_value(|k| state.items(k))),
        allowed_drop_operations: state.allowed_drop_operations,
        preview: state.preview,
        on_drag_start: Some(Callback::new(move |e: DragStartEvent| {
            item_key.with_value(|k| state.start_drag(k, e));
            set_dragging_keys(state.dragging_keys.get_untracked());
        })),
        on_drag_move: Some(Callback::new(move |e: DragMoveEvent| state.move_drag(e))),
        on_drag_end: Some(Callback::new(move |e: DragEndEvent| {
            let is_internal =
                e.drop_operation != DropOperation::Cancel && is_internal_drop_operation(None);
            state.end_drag(e, is_internal);
            clear_global_dnd_state();
        })),
        has_drag_button,
        is_disabled,
    });

    let keys_for_drag = Signal::derive(move || {
        selection.selected_keys();
        item_key.with_value(|k| state.keys_for_drag(k).len())
    });
    let is_selected = Signal::derive(move || {
        keys_for_drag.get() > 1 && item_key.with_value(|k| selection.is_selected(k))
    });
    let modality = use_drag_modality();
    let strings = use_localized_strings::<DndStrings>();
    // The item itself starts drags (no drag button) in a selectable collection: describe how; it
    // has no click to start them (touch: long press; NVDA/JAWS are in forms mode in collections).
    let describes = move || !has_drag_button && selection.selection_mode() != SelectionMode::None;
    let item_description = use_description(Signal::derive(move || {
        describes().then(|| {
            let modality = modality.get();
            let alt = has_action && modality == DragModality::Keyboard;
            let count = is_selected.get().then(|| keys_for_drag.get());
            messages::drag_item_description(&strings.read(), modality, count, alt)
        })
    }));
    let drag_description = drag_props.aria_describedby;
    drag_props.aria_describedby = Signal::derive(move || {
        // With an action, a long press selects the item on touch devices: no drag description.
        if is_disabled.get() || (has_action && modality.get() == DragModality::Touch) {
            None
        } else if describes() {
            item_description.get()
        } else {
            drag_description.get()
        }
    });
    let click = drag_props.on_click;
    drag_props.on_click = EventHandler::new(move |e: web_sys::MouseEvent| {
        if !untrack(describes) {
            click.call(e);
        }
    });
    if !has_drag_button && has_action {
        // Enter performs the action: keyboard drags start with Alt + Enter.
        let keydown = drag_props.on_keydown_capture;
        let keyup = drag_props.on_keyup_capture;
        drag_props.on_keydown_capture = EventHandler::new(move |e: web_sys::KeyboardEvent| {
            if e.alt_key() {
                keydown.call(e);
            }
        });
        drag_props.on_keyup_capture = EventHandler::new(move |e: web_sys::KeyboardEvent| {
            if e.alt_key() {
                keyup.call(e);
            }
        });
    }

    let collection = state.list.collection;
    let label = Signal::derive(move || {
        if is_selected.get() {
            strings.read().drag_selected_items(keys_for_drag.get())
        } else {
            let text = item_key.with_value(|k| {
                collection.with(|c| {
                    c.get(k)
                        .map(|n| n.text_value.to_string())
                        .unwrap_or_default()
                })
            });
            strings.read().drag_item(&text)
        }
    });

    UseDraggableItemReturn {
        drag_props,
        drag_button: UseButtonInput {
            is_disabled,
            ..drag_button
        },
        drag_button_label: label,
        is_dragging,
    }
}
