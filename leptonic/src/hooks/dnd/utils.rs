// Upstream: react-aria/src/dnd/utils.ts @ 99e6102368
// Upstream: react-aria/test/dnd/dnd.test.js @ 99e6102368
// Upstream: react-aria/test/dnd/useClipboard.test.js @ 99e6102368
use std::{cell::RefCell, collections::HashSet, rc::Rc};

use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

use super::types::{
    DirectoryDropItem, DragItem, DropItem, DropOperations, FileDropItem, GENERIC_TYPE, TextDropItem,
};
use crate::{
    hooks::{
        collections::Key,
        focus::use_focus_visible::{Modality, get_modality, use_interaction_modality},
    },
    utils::{dom_ext::EventAccessors, shadow_dom::get_event_target},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The global DnD state holds collection elements (react-aria: refs), and lives in a
//   thread-local (the browser is single-threaded).
// - Directory entries are read completely (`get_entries` returns all of them) instead of an
//   async iterator.
//
// =============================================================================

/// Types a native drag carries as one value each (multiple items' data is joined with newlines).
const NATIVE_DRAG_TYPES: [&str; 3] = ["text/plain", "text/uri-list", "text/html"];

/// Holds all items of a drag (with all their types) as JSON, so drops in the browser restore
/// them exactly.
pub(crate) const CUSTOM_DRAG_TYPE: &str = "application/vnd.react-aria.items+json";

/// How the user drags: shown in descriptions and announcements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragModality {
    Keyboard,
    Touch,
    Virtual,
}

fn map_modality(modality: Option<Modality>) -> DragModality {
    let coarse_pointer = || {
        leptos_use::use_window()
            .as_ref()
            .and_then(|w| w.match_media("(pointer: coarse)").ok().flatten())
            .is_some_and(|m| m.matches())
    };
    match modality {
        Some(Modality::Keyboard) => DragModality::Keyboard,
        _ if coarse_pointer() => DragModality::Touch,
        _ => DragModality::Virtual,
    }
}

/// The current drag modality, updated as the interaction modality changes.
pub fn use_drag_modality() -> Signal<DragModality> {
    let modality = use_interaction_modality();
    Signal::derive(move || map_modality(modality.get()))
}

/// The current drag modality.
pub fn get_drag_modality() -> DragModality {
    map_modality(get_modality())
}

/// The element an event happened on, inside shadow roots too (react-aria's `getEventTarget`);
/// `None` when it happened on the window or the document.
pub(crate) fn event_target_element(e: &impl AsRef<web_sys::Event>) -> Option<web_sys::Element> {
    get_event_target(e)
        .unwrap_or_else(|| e.expect_target())
        .dyn_into()
        .ok()
}

/// State shared by all drags and drops of collections.
#[derive(Default)]
pub(crate) struct DndState {
    /// The collection the dragged items come from.
    pub dragging_collection: Option<web_sys::Element>,
    /// Shared: read during every validity check of a drag.
    pub dragging_keys: Rc<HashSet<Key>>,
    /// The collection the drag is over.
    pub drop_collection: Option<web_sys::Element>,
}

thread_local! {
    static DND_STATE: RefCell<DndState> = RefCell::new(DndState::default());
    static DROP_EFFECT: RefCell<Option<&'static str>> = const { RefCell::new(None) };
    static ALLOWED_DROP_OPERATIONS: RefCell<DropOperations> = const { RefCell::new(DropOperations::NONE) };
}

pub(crate) fn with_dnd_state<R>(f: impl FnOnce(&mut DndState) -> R) -> R {
    DND_STATE.with(|s| f(&mut s.borrow_mut()))
}

pub(crate) fn dragging_keys() -> Rc<HashSet<Key>> {
    with_dnd_state(|s| s.dragging_keys.clone())
}

pub(crate) fn set_dragging_keys(keys: HashSet<Key>) {
    with_dnd_state(|s| s.dragging_keys = Rc::new(keys));
}

pub(crate) fn set_dragging_collection(element: Option<web_sys::Element>) {
    with_dnd_state(|s| s.dragging_collection = element);
}

pub(crate) fn set_drop_collection(element: Option<web_sys::Element>) {
    with_dnd_state(|s| s.drop_collection = element);
}

pub(crate) fn clear_global_dnd_state() {
    with_dnd_state(|s| *s = DndState::default());
}

pub(crate) fn snapshot_dnd_state() -> DndState {
    with_dnd_state(|s| DndState {
        dragging_collection: s.dragging_collection.clone(),
        dragging_keys: s.dragging_keys.clone(),
        drop_collection: s.drop_collection.clone(),
    })
}

pub(crate) fn restore_dnd_state(state: DndState) {
    with_dnd_state(|s| *s = state);
}

/// Whether the drag comes from the collection `element` (or, without one, the collection the
/// drag is over).
pub(crate) fn is_internal_drop_operation(element: Option<&web_sys::Element>) -> bool {
    with_dnd_state(|s| {
        s.dragging_collection.is_some()
            && s.dragging_collection.as_ref() == element.or(s.drop_collection.as_ref())
    })
}

/// The drop effect of the last native drop (read by the drag source's `dragend`).
pub(crate) fn global_drop_effect() -> Option<&'static str> {
    DROP_EFFECT.with(|e| *e.borrow())
}

pub(crate) fn set_global_drop_effect(effect: Option<&'static str>) {
    DROP_EFFECT.with(|e| *e.borrow_mut() = effect);
}

/// The operations the current native drag allows (set by its source).
pub(crate) fn global_allowed_drop_operations() -> DropOperations {
    ALLOWED_DROP_OPERATIONS.with(|o| *o.borrow())
}

pub(crate) fn set_global_allowed_drop_operations(operations: DropOperations) {
    ALLOWED_DROP_OPERATIONS.with(|o| *o.borrow_mut() = operations);
}

/// Write drag items to a data transfer: native types as such (joined across items), all items
/// with all their types as JSON when needed to restore them.
pub(crate) fn write_to_data_transfer(data_transfer: &web_sys::DataTransfer, items: &[DragItem]) {
    let mut grouped: Vec<(String, Vec<String>)> = Vec::new();
    let mut needs_custom_data = false;
    let mut custom_data: Vec<serde_json::Map<String, serde_json::Value>> = Vec::new();
    for item in items {
        if item.types().count() > 1 {
            needs_custom_data = true;
        }
        let mut data_by_type = serde_json::Map::new();
        for (kind, data) in item.iter() {
            match grouped.iter_mut().find(|(k, _)| k == kind) {
                Some((_, values)) => {
                    needs_custom_data = true;
                    values.push(data.to_owned());
                }
                None => grouped.push((kind.to_owned(), vec![data.to_owned()])),
            }
            data_by_type.insert(kind.to_owned(), serde_json::Value::String(data.to_owned()));
        }
        custom_data.push(data_by_type);
    }
    let transfer_items = data_transfer.items();
    for (kind, values) in grouped {
        let data = if NATIVE_DRAG_TYPES.contains(&kind.as_str()) {
            values.join("\n")
        } else {
            values.into_iter().next().unwrap_or_default()
        };
        let _ = transfer_items.add_with_str_and_type(&data, &kind);
    }
    if needs_custom_data && let Ok(json) = serde_json::to_string(&custom_data) {
        let _ = transfer_items.add_with_str_and_type(&json, CUSTOM_DRAG_TYPE);
    }
}

/// Read the dropped items of a data transfer.
pub(crate) fn read_from_data_transfer(data_transfer: &web_sys::DataTransfer) -> Vec<DropItem> {
    let mut items = Vec::new();
    let types = data_transfer.types();
    let has_custom_type =
        (0..types.length()).any(|i| types.get(i).as_string().as_deref() == Some(CUSTOM_DRAG_TYPE));
    if has_custom_type
        && let Ok(data) = data_transfer.get_data(CUSTOM_DRAG_TYPE)
        && let Ok(parsed) =
            serde_json::from_str::<Vec<serde_json::Map<String, serde_json::Value>>>(&data)
    {
        for item in parsed {
            items.push(DropItem::Text(TextDropItem::new(
                item.into_iter()
                    .map(|(k, v)| (k, v.as_str().unwrap_or_default().to_owned()))
                    .collect(),
            )));
        }
        return items;
    }

    let mut strings: Vec<(String, String)> = Vec::new();
    let transfer_items = data_transfer.items();
    for i in 0..transfer_items.length() {
        let Some(item) = transfer_items.get(i) else {
            continue;
        };
        if item.kind() == "string" {
            let kind = item.type_();
            let data = data_transfer.get_data(&kind).unwrap_or_default();
            strings.push((
                if kind.is_empty() {
                    GENERIC_TYPE.to_owned()
                } else {
                    kind
                },
                data,
            ));
        } else if item.kind() == "file" {
            match item.webkit_get_as_entry() {
                Ok(Some(entry)) if entry.is_directory() => {
                    items.push(DropItem::Directory(DirectoryDropItem::new(
                        entry.unchecked_into(),
                    )));
                }
                Ok(None) => {}
                _ => {
                    if let Ok(Some(file)) = item.get_as_file() {
                        items.push(DropItem::File(FileDropItem::new(file)));
                    }
                }
            }
        }
    }
    if !strings.is_empty() {
        items.push(DropItem::Text(TextDropItem::new(strings)));
    }
    items
}

/// All entries of a dropped directory.
pub(crate) async fn read_directory(
    directory: &web_sys::FileSystemDirectoryEntry,
) -> Result<Vec<DropItem>, JsValue> {
    let reader = directory.create_reader();
    let mut items = Vec::new();
    loop {
        let batch = js_sys::Promise::new(&mut |resolve, reject| {
            let _ = reader.read_entries_with_callback_and_callback(&resolve, &reject);
        });
        let entries: js_sys::Array = wasm_bindgen_futures::JsFuture::from(batch)
            .await?
            .unchecked_into();
        if entries.length() == 0 {
            return Ok(items);
        }
        for entry in entries.iter() {
            let entry: web_sys::FileSystemEntry = entry.unchecked_into();
            if entry.is_file() {
                let file_entry: web_sys::FileSystemFileEntry = entry.unchecked_into();
                let file = js_sys::Promise::new(&mut |resolve, reject| {
                    file_entry.file_with_callback_and_callback(&resolve, &reject);
                });
                let file: web_sys::File = wasm_bindgen_futures::JsFuture::from(file)
                    .await?
                    .unchecked_into();
                items.push(DropItem::File(FileDropItem::new(file)));
            } else if entry.is_directory() {
                items.push(DropItem::Directory(DirectoryDropItem::new(
                    entry.unchecked_into(),
                )));
            }
        }
    }
}
