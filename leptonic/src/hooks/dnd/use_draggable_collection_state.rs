// Upstream: react-stately/src/dnd/useDraggableCollectionState.ts @ 99e6102368
use std::collections::HashSet;

use leptos::prelude::*;

use super::types::{
    DragEndEvent, DragItem, DragMoveEvent, DragPreview, DragStartEvent,
    DraggableCollectionEndEvent, DraggableCollectionMoveEvent, DraggableCollectionStartEvent,
    DropOperation,
};
use crate::hooks::collections::{Key, ListState};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `get_items` gets the dragged keys only (react-aria also passes the items' values; our
//   collections hold keys and text, look values up by key).
// - Hook-owned state: `dragged_key` and `dragging_keys` are signals.
//
// =============================================================================

/// Input of [`use_draggable_collection_state`].
#[derive(Clone)]
pub struct UseDraggableCollectionStateInput {
    /// The collection and its selection.
    pub list: ListState,
    /// The data of the dragged items.
    pub get_items: Callback<HashSet<Key>, Vec<DragItem>>,
    pub preview: Option<Callback<Vec<DragItem>, Option<DragPreview>>>,
    /// The operations drags allow, in order of preference. Defaults to move, copy, link.
    pub get_allowed_drop_operations: Option<Callback<(), Vec<DropOperation>>>,
    pub on_drag_start: Option<Callback<DraggableCollectionStartEvent>>,
    pub on_drag_move: Option<Callback<DraggableCollectionMoveEvent>>,
    pub on_drag_end: Option<Callback<DraggableCollectionEndEvent>>,
    pub is_disabled: Signal<bool>,
}

/// The state of a collection whose items can be dragged.
#[derive(Clone, Copy)]
pub struct DraggableCollectionState {
    pub list: ListState,
    /// The item the drag started from.
    pub dragged_key: Signal<Option<Key>>,
    /// All dragged items.
    pub dragging_keys: Signal<HashSet<Key>>,
    pub is_disabled: Signal<bool>,
    pub(crate) preview: Option<Callback<Vec<DragItem>, Option<DragPreview>>>,
    pub(crate) get_allowed_drop_operations: Option<Callback<(), Vec<DropOperation>>>,
    get_items: Callback<HashSet<Key>, Vec<DragItem>>,
    set_dragged_key: WriteSignal<Option<Key>>,
    set_dragging_keys: WriteSignal<HashSet<Key>>,
    on_drag_start: Option<Callback<DraggableCollectionStartEvent>>,
    on_drag_move: Option<Callback<DraggableCollectionMoveEvent>>,
    on_drag_end: Option<Callback<DraggableCollectionEndEvent>>,
}

impl std::fmt::Debug for DraggableCollectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DraggableCollectionState")
            .field("dragged_key", &self.dragged_key.get_untracked())
            .finish_non_exhaustive()
    }
}

impl DraggableCollectionState {
    /// Whether `key` is being dragged.
    pub fn is_dragging(&self, key: &Key) -> bool {
        self.dragging_keys.with(|keys| keys.contains(key))
    }

    /// The items a drag starting at `key` drags: all selected items if `key` is selected (but
    /// not those whose parent is selected too), else only `key`.
    pub fn keys_for_drag(&self, key: &Key) -> HashSet<Key> {
        let selection = self.list.selection;
        untrack(|| {
            if !selection.is_selected(key) {
                return std::iter::once(key.clone()).collect();
            }
            let selected = selection.selected_keys();
            self.list.collection.with(|c| {
                selected
                    .iter()
                    .filter(|key| {
                        let Some(mut node) = c.get(key) else {
                            return false;
                        };
                        while let Some(parent) = node.parent_key.as_ref() {
                            if selected.contains(parent) {
                                return false;
                            }
                            match c.get(parent) {
                                Some(parent) => node = parent,
                                None => break,
                            }
                        }
                        true
                    })
                    .cloned()
                    .collect()
            })
        })
    }

    /// The data of a drag starting at `key`.
    pub fn items(&self, key: &Key) -> Vec<DragItem> {
        self.get_items.run(self.keys_for_drag(key))
    }

    pub(crate) fn start_drag(&self, key: &Key, event: DragStartEvent) {
        let keys = self.keys_for_drag(key);
        self.set_dragging_keys.set(keys.clone());
        self.set_dragged_key.set(Some(key.clone()));
        self.list.selection.set_focused(false);
        if let Some(on_start) = self.on_drag_start {
            on_start.run(DraggableCollectionStartEvent {
                x: event.x,
                y: event.y,
                keys,
            });
        }
    }

    pub(crate) fn move_drag(&self, event: DragMoveEvent) {
        if let Some(on_move) = self.on_drag_move {
            on_move.run(DraggableCollectionMoveEvent {
                x: event.x,
                y: event.y,
                keys: self.dragging_keys.get_untracked(),
            });
        }
    }

    pub(crate) fn end_drag(&self, event: DragEndEvent, is_internal: bool) {
        if let Some(on_end) = self.on_drag_end {
            on_end.run(DraggableCollectionEndEvent {
                x: event.x,
                y: event.y,
                drop_operation: event.drop_operation,
                keys: self.dragging_keys.get_untracked(),
                is_internal,
            });
        }
        self.set_dragging_keys.set(HashSet::new());
        self.set_dragged_key.set(None);
    }
}

/// Creates the state of a collection whose items can be dragged.
pub fn use_draggable_collection_state(
    input: UseDraggableCollectionStateInput,
) -> DraggableCollectionState {
    let UseDraggableCollectionStateInput {
        list,
        get_items,
        preview,
        get_allowed_drop_operations,
        on_drag_start,
        on_drag_move,
        on_drag_end,
        is_disabled,
    } = input;
    let (dragged_key, set_dragged_key) = signal(None);
    let (dragging_keys, set_dragging_keys) = signal(HashSet::new());
    DraggableCollectionState {
        list,
        dragged_key: dragged_key.into(),
        dragging_keys: dragging_keys.into(),
        is_disabled,
        preview,
        get_allowed_drop_operations,
        get_items,
        set_dragged_key,
        set_dragging_keys,
        on_drag_start,
        on_drag_move,
        on_drag_end,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use assertr::prelude::*;

    use super::*;
    use crate::hooks::{
        SelectionMode,
        collections::{Collection, Selection, SelectionOptions, UseListStateInput, use_list_state},
    };

    #[test]
    fn dragging_a_selected_item_drags_the_selection_without_nested_items() {
        Owner::new().with(|| {
            let collection = Memo::new(|_| {
                Arc::new(Collection::build(|b| {
                    b.item("a", "A").children(|c| {
                        c.item("a1", "A1");
                    });
                    b.item("b", "B");
                    b.item("c", "C");
                }))
            });
            let list = use_list_state(UseListStateInput {
                collection,
                selection: SelectionOptions {
                    selection_mode: Signal::stored(SelectionMode::Multiple),
                    default_selection: Selection::keys([
                        Key::from("a"),
                        Key::from("a1"),
                        Key::from("b"),
                    ]),
                    ..SelectionOptions::default()
                },
            });
            let state = use_draggable_collection_state(UseDraggableCollectionStateInput {
                list,
                get_items: Callback::new(|keys: HashSet<Key>| {
                    let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
                    keys.sort();
                    keys.into_iter().map(DragItem::text).collect()
                }),
                preview: None,
                get_allowed_drop_operations: None,
                on_drag_start: None,
                on_drag_move: None,
                on_drag_end: None,
                is_disabled: Signal::stored(false),
            });
            let mut keys: Vec<String> = state
                .keys_for_drag(&Key::from("b"))
                .iter()
                .map(ToString::to_string)
                .collect();
            keys.sort();
            assert_that!(keys).is_equal_to(vec!["a".to_owned(), "b".to_owned()]);
            assert_that!(state.keys_for_drag(&Key::from("c")).len()).is_equal_to(1);
            assert_that!(state.items(&Key::from("c")).len()).is_equal_to(1);
        });
    }
}
