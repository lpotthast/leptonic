// Upstream: react-aria/src/dnd/useDroppableCollection.ts @ 99e6102368
// Upstream: react-aria/test/dnd/useDroppableCollection.test.js @ 99e6102368
use std::{collections::HashSet, rc::Rc, sync::Arc};

use leptos::prelude::*;

use super::{
    drag_manager::{self, DragTarget, DropTargetOptions},
    drop_target_keyboard_navigation::{NavigationDirection, navigate},
    list_drop_target_delegate::DropTargetDelegate,
    types::{
        DragType, DragTypes, DropActivateEvent, DropEvent, DropItem, DropOperation, DropPosition,
        DropTarget, DropTargetKeyDownEvent, DroppableCollectionActivateEvent,
        DroppableCollectionDropEvent, DroppableCollectionInsertDropEvent,
        DroppableCollectionOnItemDropEvent, DroppableCollectionReorderEvent,
        DroppableCollectionRootDropEvent, ItemDropTarget,
    },
    use_auto_scroll::use_auto_scroll,
    use_drop::{DropOperationPointQuery, UseDropInput, UseDropProps, UseDropReturn, use_drop},
    use_droppable_collection_state::{DroppableCollectionState, ItemDropQuery},
    utils::{
        clear_global_dnd_state, dragging_keys, is_internal_drop_operation, set_drop_collection,
        with_dnd_state,
    },
};
use crate::{
    CapturedElement,
    hooks::{
        collections::{Collection, Key, KeyboardDelegate, Node, NodeKind, SelectionMode},
        focus::use_focus_visible::{Modality, set_modality},
    },
    utils::{
        i18n::{WritingDirection, use_direction},
        key::{KeyboardEventKey, KeyboardKey},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The collection element keeps its own id (`collection_id`, react-aria merges a generated one
//   into it). Items and drop indicators get the collection through `DroppableCollectionData`
//   (react-aria: a `WeakMap` keyed by the state).
// - Drop handlers run synchronously (react-aria awaits each of them).
// - `on_key_down` gets a `DropTargetKeyDownEvent` (react-aria: the `KeyboardEvent`).
//
// ## BEHAVIOR DIFFERENCES
// - The default drop handling filters dropped items by `accepted_drag_types` with `DragTypes`,
//   so wildcards (`image/*`, `*/*`) work at drop time as they do while dragging; react-aria
//   compares the items' types exactly there and drops everything a wildcard accepted.
//
// ## OMITTED FEATURES
// - Expanded-parent handling after drops on tree items (`expandedKeys` on the collection): no
//   tree collection takes drops yet (tree grid lists and tree tables come with DnD on the
//   collection atoms).
//
// =============================================================================

/// What droppable items and drop indicators need to know about their collection.
#[derive(Debug, Clone)]
pub struct DroppableCollectionData {
    pub state: DroppableCollectionState,
    /// The collection element's id.
    pub id: String,
    pub element: CapturedElement,
}

/// Input of [`use_droppable_collection`].
#[derive(Clone)]
pub struct UseDroppableCollectionInput {
    pub state: DroppableCollectionState,
    /// The collection element (captured by the collection hook's props).
    pub element: CapturedElement,
    /// The collection element's id.
    pub collection_id: String,
    /// Navigation between drop targets during keyboard drags.
    pub keyboard_delegate: Signal<Arc<dyn KeyboardDelegate>>,
    /// The drop target at a point (native drags).
    pub drop_target_delegate: Arc<dyn DropTargetDelegate>,
    /// Called with key presses during keyboard drags (after the collection handled them).
    pub on_key_down: Option<Callback<DropTargetKeyDownEvent>>,
}

/// Return value of [`use_droppable_collection`].
pub struct UseDroppableCollectionReturn {
    /// Props for the collection element (native drop handling; it isn't described as a drop
    /// target itself).
    pub collection_props: UseDropProps,
    pub data: DroppableCollectionData,
}

/// What a drop changed, to update focus and selection once the collection reflects it.
struct DroppingState {
    collection: Arc<Collection>,
    focused_key: Option<Key>,
    selected_keys: HashSet<Key>,
    target: DropTarget,
    dragging_keys: HashSet<Key>,
    is_internal: bool,
    timeout: Option<TimeoutHandle>,
}

/// The items of `collection` among `nodes` (and their descendants) that `previous` lacks, in
/// order.
fn inserted_items<'a>(
    collection: &'a Collection,
    previous: &Collection,
    nodes: &mut dyn Iterator<Item = &'a Node>,
    inserted: &mut Vec<Key>,
) {
    for node in nodes {
        if node.is_item() && !previous.contains_key(&node.key) {
            inserted.push(node.key.clone());
        }
        // Sections' items and items' child items (not a row's cells). A trait object: a generic
        // iterator type would nest with every level (polymorphic recursion).
        let mut children = collection
            .children(&node.key)
            .filter(|child| child.is_item());
        inserted_items(collection, previous, &mut children, inserted);
    }
}

fn drop_item_types(item: &DropItem) -> DragTypes {
    match item {
        DropItem::Directory(_) => DragTypes::from_types([DragType::Directory]),
        DropItem::File(file) => DragTypes::from_types([DragType::from(file.kind.clone())]),
        DropItem::Text(text) => DragTypes::from_types(text.types().map(DragType::from)),
    }
}

/// A collection that accepts drops: on it, on its items and between them, from native drags and
/// keyboard and screen reader drags.
#[allow(clippy::too_many_lines)]
pub fn use_droppable_collection(
    input: UseDroppableCollectionInput,
) -> UseDroppableCollectionReturn {
    let UseDroppableCollectionInput {
        state,
        element,
        collection_id,
        keyboard_delegate,
        drop_target_delegate,
        on_key_down,
    } = input;
    let next_target: StoredValue<Option<DropTarget>> = StoredValue::new(None);
    let collection_element = move || element.get_untracked().map(|e| (*e).clone());
    let is_internal = move || is_internal_drop_operation(collection_element().as_ref());
    let operation_at = move |target: &DropTarget, types: &DragTypes, allowed: &[DropOperation]| {
        state.drop_operation_at(collection_element().as_ref(), target, types, allowed)
    };

    // The default drop handling: insert, root drop, item drop, move, reorder.
    let default_on_drop = move |e: DroppableCollectionDropEvent| {
        let options = state.options.get_value();
        let dragging_keys = dragging_keys();
        let is_internal = is_internal();
        let DroppableCollectionDropEvent {
            target,
            drop_operation,
            items,
            ..
        } = e;
        let filtered: Vec<DropItem> = items
            .into_iter()
            .filter(|item| {
                let types = drop_item_types(item);
                if !options.accepted_drag_types.accepts(&types) {
                    return false;
                }
                match (&target, options.should_accept_item_drop) {
                    (DropTarget::Item(t), Some(accept)) if t.drop_position == DropPosition::On => {
                        accept.run(ItemDropQuery {
                            target: t.clone(),
                            types,
                        })
                    }
                    _ => true,
                }
            })
            .collect();
        if filtered.is_empty() {
            return;
        }
        match &target {
            DropTarget::Root => {
                if let Some(on_root_drop) = options.on_root_drop {
                    on_root_drop.run(DroppableCollectionRootDropEvent {
                        items: filtered,
                        drop_operation,
                    });
                }
            }
            DropTarget::Item(t) => {
                if t.drop_position == DropPosition::On
                    && let Some(on_item_drop) = options.on_item_drop
                {
                    on_item_drop.run(DroppableCollectionOnItemDropEvent {
                        items: filtered.clone(),
                        drop_operation,
                        is_internal,
                        target: t.clone(),
                    });
                }
                if is_internal && let Some(on_move) = options.on_move {
                    on_move.run(DroppableCollectionReorderEvent {
                        keys: (*dragging_keys).clone(),
                        drop_operation,
                        target: t.clone(),
                    });
                }
                if t.drop_position != DropPosition::On {
                    if !is_internal && let Some(on_insert) = options.on_insert {
                        on_insert.run(DroppableCollectionInsertDropEvent {
                            items: filtered,
                            drop_operation,
                            target: t.clone(),
                        });
                    }
                    if is_internal && let Some(on_reorder) = options.on_reorder {
                        on_reorder.run(DroppableCollectionReorderEvent {
                            keys: (*dragging_keys).clone(),
                            drop_operation,
                            target: t.clone(),
                        });
                    }
                }
            }
        }
    };

    // Focus (and select) what the drop changed, once the collection reflects it.
    let dropping: StoredValue<Option<DroppingState>> = StoredValue::new(None);
    let list = state.list;
    // Runs in a timeout or an Effect: reads and writes the selection untracked.
    let update_focus_after_drop = move || {
        untrack(|| {
            let Some(dropping_state) = dropping.try_update_value(Option::take).flatten() else {
                return;
            };
            let selection = list.selection;
            let collection = list.collection.get_untracked();
            let focused_key = selection.focused_key();
            let prev = &dropping_state.collection;
            if collection.size() > prev.size()
                && selection.selected_keys() == dropping_state.selected_keys
            {
                // Inserted items (also into items, e.g. a tree's): select them, focus the first.
                let mut new_keys = Vec::new();
                inserted_items(&collection, prev, &mut collection.iter(), &mut new_keys);
                selection.set_selected_keys(new_keys.iter().cloned());
                if focused_key == dropping_state.focused_key
                    && let Some(first) = new_keys.first()
                {
                    let node = collection.get(first);
                    let on_item = matches!(&dropping_state.target, DropTarget::Item(t) if t.drop_position == DropPosition::On);
                    let key = match node {
                        Some(n) if n.kind == NodeKind::Cell || on_item => n.parent_key.clone(),
                        _ => Some(first.clone()),
                    };
                    if let Some(key) = key {
                        selection.set_focused_key(Some(key), None);
                    }
                    if selection.selection_mode() == SelectionMode::None {
                        set_modality(Modality::Keyboard);
                    }
                }
            } else if let Some(prev_focused) = &dropping_state.focused_key
                && focused_key.as_ref() == Some(prev_focused)
                && dropping_state.is_internal
                && matches!(&dropping_state.target, DropTarget::Item(t) if t.drop_position != DropPosition::On)
                && collection
                    .get(prev_focused)
                    .and_then(|n| n.parent_key.as_ref())
                    .is_some_and(|parent| dropping_state.dragging_keys.contains(parent))
            {
                let parent = collection
                    .get(prev_focused)
                    .and_then(|n| n.parent_key.clone());
                selection.set_focused_key(parent, None);
                set_modality(Modality::Keyboard);
            } else if focused_key == dropping_state.focused_key
                && let DropTarget::Item(t) = &dropping_state.target
                && t.drop_position == DropPosition::On
                && collection.contains_key(&t.key)
            {
                selection.set_focused_key(Some(t.key.clone()), None);
                set_modality(Modality::Keyboard);
            } else if let Some(focused) = &focused_key
                && !selection.is_selected(focused)
            {
                set_modality(Modality::Keyboard);
            }
            selection.set_focused(true);
        });
    };

    let on_drop = move |e: DropEvent, target: DropTarget| {
        let selection = list.selection;
        dropping.set_value(Some(DroppingState {
            collection: list.collection.get_untracked(),
            focused_key: selection.focused_key(),
            selected_keys: selection.selected_keys(),
            target: target.clone(),
            dragging_keys: (*dragging_keys()).clone(),
            is_internal: is_internal(),
            timeout: None,
        }));
        let event = DroppableCollectionDropEvent {
            x: e.x,
            y: e.y,
            items: e.items,
            drop_operation: e.drop_operation,
            target,
        };
        match state.options.with_value(|o| o.on_drop) {
            Some(on_drop) => on_drop.run(event),
            None => default_on_drop(event),
        }
        let timeout = set_timeout_with_handle(
            update_focus_after_drop,
            std::time::Duration::from_millis(50),
        )
        .ok();
        dropping.update_value(|d| {
            if let Some(d) = d {
                d.timeout = timeout;
            }
        });
    };
    // The collection changed because of the drop: update focus right away.
    Effect::new(move |prev: Option<Arc<Collection>>| {
        let collection = list.collection.get();
        let changed = dropping.with_value(|d| {
            d.as_ref()
                .is_some_and(|d| !Arc::ptr_eq(&d.collection, &collection))
        });
        if prev.is_some() && changed {
            if let Some(timeout) = dropping.with_value(|d| d.as_ref().and_then(|d| d.timeout)) {
                timeout.clear();
            }
            update_focus_after_drop();
        }
        collection
    });
    on_cleanup(move || {
        if let Some(timeout) = dropping
            .try_with_value(|d| d.as_ref().and_then(|d| d.timeout))
            .flatten()
        {
            timeout.clear();
        }
    });

    let direction = use_direction();

    // Native drags.
    let auto_scroll = use_auto_scroll(element);
    let point_delegate = drop_target_delegate.clone();
    let UseDropReturn { mut drop_props, .. } = use_drop(UseDropInput {
        on_drop_enter: Some(Callback::new(move |_| {
            if let Some(target) = next_target.get_value() {
                state.set_target(Some(target));
            }
        })),
        on_drop_move: Some(Callback::new(move |e: super::types::DropMoveEvent| {
            if let Some(target) = next_target.get_value() {
                state.set_target(Some(target));
            }
            auto_scroll.move_to(e.x, e.y);
        })),
        get_drop_operation_for_point: Some(Callback::new(move |q: DropOperationPointQuery| {
            let is_valid = |t: &DropTarget| {
                operation_at(t, &q.types, &q.allowed_operations) != DropOperation::Cancel
            };
            let Some(mut target) = point_delegate.drop_target_from_point(
                q.x,
                q.y,
                direction.get_untracked(),
                &is_valid,
            ) else {
                next_target.set_value(None);
                return DropOperation::Cancel;
            };
            let mut operation = operation_at(&target, &q.types, &q.allowed_operations);
            if operation == DropOperation::Cancel {
                let root = operation_at(&DropTarget::Root, &q.types, &q.allowed_operations);
                if root != DropOperation::Cancel {
                    target = DropTarget::Root;
                    operation = root;
                }
            }
            if operation != DropOperation::Cancel {
                let element = collection_element();
                let is_drop_collection = with_dnd_state(|s| s.drop_collection == element);
                if !is_drop_collection {
                    set_drop_collection(element);
                }
            }
            next_target.set_value((operation != DropOperation::Cancel).then_some(target));
            operation
        })),
        on_drop_exit: Some(Callback::new(move |_| {
            set_drop_collection(None);
            state.set_target(None);
            auto_scroll.stop();
        })),
        on_drop_activate: Some(Callback::new(move |e: DropActivateEvent| {
            if let Some(target @ DropTarget::Item(_)) = state.target.get_untracked()
                && let Some(on_activate) = state.options.with_value(|o| o.on_drop_activate)
            {
                on_activate.run(DroppableCollectionActivateEvent {
                    x: e.x,
                    y: e.y,
                    target,
                });
            }
        })),
        on_drop: Some(Callback::new(move |e: DropEvent| {
            set_drop_collection(collection_element());
            if let Some(target) = state.target.get_untracked() {
                on_drop(e, target);
            }
            if with_dnd_state(|s| s.dragging_collection.is_none()) {
                clear_global_dnd_state();
            }
        })),
        element,
        get_drop_operation: None,
        has_drop_button: false,
        is_disabled: Signal::stored(false),
    });
    // The collection isn't described as a drop target itself (its items and indicators are).
    drop_props.aria_describedby = Signal::stored(None);

    // Keyboard and screen reader drags.
    let registration: StoredValue<Option<u64>> = StoredValue::new(None);
    let unregister = move || {
        if let Some(id) = registration.try_get_value().flatten() {
            drag_manager::unregister_drop_target(id);
            registration.set_value(None);
        }
    };
    Effect::new(move || {
        unregister();
        let Some(el) = element.get() else {
            return;
        };
        let rtl = direction.get() == WritingDirection::Rtl;
        let id = drag_manager::register_drop_target(keyboard_drop_target(
            (*el).clone(),
            &state,
            keyboard_delegate,
            rtl,
            on_drop,
            on_key_down,
        ));
        registration.set_value(Some(id));
    });
    on_cleanup(unregister);

    UseDroppableCollectionReturn {
        collection_props: drop_props,
        data: DroppableCollectionData {
            state,
            id: collection_id,
            element,
        },
    }
}

/// The drag manager registration of a droppable collection: the initial drop target, navigation
/// with the collection's keys, and drops.
#[allow(clippy::too_many_lines)]
fn keyboard_drop_target(
    element: web_sys::Element,
    state: &DroppableCollectionState,
    keyboard_delegate: Signal<Arc<dyn KeyboardDelegate>>,
    rtl: bool,
    on_drop: impl Fn(DropEvent, DropTarget) + Copy + 'static,
    on_key_down: Option<Callback<DropTargetKeyDownEvent>>,
) -> DropTargetOptions {
    let state = *state;
    let collection_element = element.clone();
    let operation_at = move |target: &DropTarget, types: &DragTypes, allowed: &[DropOperation]| {
        state.drop_operation_at(Some(&collection_element), target, types, allowed)
    };
    let operation_for_targets = operation_at.clone();
    let operation_for_enter = operation_at.clone();
    let operation_for_keys = operation_at.clone();
    let get_next_target =
        move |target: Option<&DropTarget>, wrap: bool, direction: NavigationDirection| {
            let delegate = keyboard_delegate.get_untracked();
            state
                .list
                .collection
                .with_untracked(|c| navigate(&*delegate, c, target, direction, rtl, wrap))
        };
    // The next target with a drop operation (two passes over the root at most).
    let next_valid_target = Rc::new(
        move |target: Option<DropTarget>,
              types: &DragTypes,
              allowed: &[DropOperation],
              direction: NavigationDirection,
              wrap: bool|
              -> Option<DropTarget> {
            let mut target = target;
            let mut seen_root = 0;
            loop {
                let next = get_next_target(target.as_ref(), wrap, direction)?;
                let operation = operation_at(&next, types, allowed);
                if next == DropTarget::Root {
                    seen_root += 1;
                }
                let stop = operation != DropOperation::Cancel
                    || state.is_drop_target(Some(&next))
                    || seen_root >= 2;
                target = Some(next);
                if stop {
                    return (operation != DropOperation::Cancel)
                        .then(|| target.take())
                        .flatten();
                }
            }
        },
    );

    let nvt = next_valid_target.clone();
    let get_drop_operation = Rc::new(move |types: &DragTypes, allowed: &[DropOperation]| {
        if let Some(target) = state.target.get_untracked() {
            return operation_for_targets(&target, types, allowed);
        }
        if nvt(None, types, allowed, NavigationDirection::Down, true).is_some() {
            DropOperation::Move
        } else {
            DropOperation::Cancel
        }
    });

    let nvt = next_valid_target.clone();
    let drop_collection_element = element.clone();
    let on_drop_enter = Rc::new(move |_e, drag: &DragTarget| {
        let types = DragTypes::of_items(&drag.items);
        let allowed = &drag.allowed_drop_operations;
        let selection = state.list.selection;
        set_drop_collection(Some(drop_collection_element.clone()));

        // Start at the focused item (or the selection's edge).
        let mut key = selection.focused_key();
        let mut drop_position = DropPosition::After;
        if let Some(k) = &key
            && let Some(node) = state.list.collection.with_untracked(|c| c.get(k).cloned())
            && node.kind == NodeKind::Cell
        {
            key = node.parent_key;
        }
        if let Some(k) = &key
            && selection.is_selected(k)
        {
            let selected = selection.selected_keys();
            if selected.len() > 1 && selection.first_selected_key().as_ref() == Some(k) {
                drop_position = DropPosition::Before;
            } else {
                key = selection.last_selected_key();
            }
        }
        let mut target = key.map(|key| DropTarget::Item(ItemDropTarget { key, drop_position }));
        if let Some(t) = &target
            && operation_for_enter(t, &types, allowed) == DropOperation::Cancel
        {
            target = nvt(
                target.clone(),
                &types,
                allowed,
                NavigationDirection::Down,
                false,
            )
            .or_else(|| {
                nvt(
                    target.clone(),
                    &types,
                    allowed,
                    NavigationDirection::Up,
                    false,
                )
            });
        }
        if target.is_none() {
            target = nvt(None, &types, allowed, NavigationDirection::Down, true);
        }
        state.set_target(target);
    });

    let nvt = next_valid_target;
    let on_key_down = Rc::new(move |e: &web_sys::KeyboardEvent, drag: &DragTarget| {
        let delegate = keyboard_delegate.get_untracked();
        let types = DragTypes::of_items(&drag.items);
        let allowed = &drag.allowed_drop_operations;
        let current = state.target.get_untracked();
        let step = |direction| nvt(current.clone(), &types, allowed, direction, true);
        match e.typed_key() {
            KeyboardKey::ArrowDown => state.set_target(step(NavigationDirection::Down)),
            KeyboardKey::ArrowUp => state.set_target(step(NavigationDirection::Up)),
            KeyboardKey::ArrowLeft => state.set_target(step(NavigationDirection::Left)),
            KeyboardKey::ArrowRight => state.set_target(step(NavigationDirection::Right)),
            KeyboardKey::Home => {
                state.set_target(nvt(None, &types, allowed, NavigationDirection::Down, true));
            }
            KeyboardKey::End => {
                state.set_target(nvt(None, &types, allowed, NavigationDirection::Up, true));
            }
            KeyboardKey::PageDown => {
                let target = match &current {
                    None => nvt(None, &types, allowed, NavigationDirection::Down, true),
                    Some(target) => {
                        let target_key = match target {
                            DropTarget::Item(t) => Some(t.key.clone()),
                            DropTarget::Root => delegate.first_key(None, false),
                        };
                        let last_key = delegate.last_key(None, false);
                        let mut next_key = target_key.and_then(|k| delegate.key_page_below(&k));
                        let mut drop_position = match target {
                            DropTarget::Item(t) => t.drop_position,
                            DropTarget::Root => DropPosition::After,
                        };
                        let at_last = matches!(target, DropTarget::Item(t) if Some(&t.key) == last_key.as_ref());
                        if next_key.is_none() || at_last {
                            next_key = last_key;
                            drop_position = DropPosition::After;
                        }
                        next_key
                            .and_then(|key| {
                                let candidate =
                                    DropTarget::Item(ItemDropTarget { key, drop_position });
                                if operation_for_keys(&candidate, &types, allowed)
                                    == DropOperation::Cancel
                                {
                                    nvt(
                                        Some(candidate.clone()),
                                        &types,
                                        allowed,
                                        NavigationDirection::Down,
                                        false,
                                    )
                                    .or_else(|| {
                                        nvt(
                                            Some(candidate),
                                            &types,
                                            allowed,
                                            NavigationDirection::Up,
                                            false,
                                        )
                                    })
                                } else {
                                    Some(candidate)
                                }
                            })
                            .or(current.clone())
                    }
                };
                state.set_target(target.or(current));
            }
            KeyboardKey::PageUp => {
                let target = match &current {
                    None => nvt(None, &types, allowed, NavigationDirection::Up, true),
                    Some(DropTarget::Root) => current.clone(),
                    Some(DropTarget::Item(t)) => {
                        let candidate = if Some(&t.key) == delegate.first_key(None, false).as_ref()
                        {
                            Some(DropTarget::Root)
                        } else {
                            match delegate.key_page_above(&t.key) {
                                Some(key) => Some(DropTarget::Item(ItemDropTarget {
                                    key,
                                    drop_position: t.drop_position,
                                })),
                                None => delegate.first_key(None, false).map(|key| {
                                    DropTarget::Item(ItemDropTarget {
                                        key,
                                        drop_position: DropPosition::Before,
                                    })
                                }),
                            }
                        };
                        candidate.and_then(|candidate| {
                            if operation_for_keys(&candidate, &types, allowed)
                                == DropOperation::Cancel
                            {
                                nvt(
                                    Some(candidate.clone()),
                                    &types,
                                    allowed,
                                    NavigationDirection::Up,
                                    false,
                                )
                                .or_else(|| {
                                    nvt(
                                        Some(candidate),
                                        &types,
                                        allowed,
                                        NavigationDirection::Down,
                                        false,
                                    )
                                })
                            } else {
                                Some(candidate)
                            }
                        })
                    }
                };
                state.set_target(target.or(current));
            }
            _ => {}
        }
        if let Some(on_key_down) = on_key_down {
            on_key_down.run(DropTargetKeyDownEvent::new(e));
        }
    });

    let drop_element = element.clone();
    DropTargetOptions {
        element: Some(element),
        prevent_focus_on_drop: true,
        get_drop_operation: Some(get_drop_operation),
        on_drop_enter: Some(on_drop_enter),
        on_drop_exit: Some(Rc::new(move |_| {
            set_drop_collection(None);
            state.set_target(None);
        })),
        on_drop_target_enter: Some(Rc::new(move |target| state.set_target(target))),
        on_drop_activate: Some(Rc::new(
            move |e: DropActivateEvent, target: Option<DropTarget>| {
                if let Some(
                    target @ DropTarget::Item(ItemDropTarget {
                        drop_position: DropPosition::On,
                        ..
                    }),
                ) = target
                    && let Some(on_activate) = state.options.with_value(|o| o.on_drop_activate)
                {
                    on_activate.run(DroppableCollectionActivateEvent {
                        x: e.x,
                        y: e.y,
                        target,
                    });
                }
            },
        )),
        on_drop: Some(Rc::new(move |e: DropEvent, target: Option<DropTarget>| {
            set_drop_collection(Some(drop_element.clone()));
            if let Some(current) = state.target.get_untracked() {
                on_drop(e, target.unwrap_or(current));
            }
        })),
        on_key_down: Some(on_key_down),
        activate_button: None,
    }
}
