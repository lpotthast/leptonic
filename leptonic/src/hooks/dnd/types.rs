// Upstream: @react-types/shared/src/dnd.d.ts @ 99e6102368
// Upstream: react-aria/src/dnd/constants.ts @ 99e6102368
use std::{collections::HashSet, sync::Arc};

use send_wrapper::SendWrapper;

use crate::{
    hooks::collections::Key,
    utils::{
        key::{KeyboardEventKey, KeyboardKey},
        point::Point,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `DragItem` is an ordered list of (type, data) pairs instead of a `{[type]: string}` object.
// - Drag types are `DragType::Mime(..)` or `DragType::Directory` (react-aria: strings and a
//   `DIRECTORY_DRAG_TYPE` symbol).
// - `TextDropItem` holds its data: `get_text` is synchronous (the data is available when the drop
//   happens). File and directory items read asynchronously, as in react-aria.
// - `DropTarget` is an enum (`Root` / `Item`), `DropOperation` an enum.
// - `DragPreview.offset` is a `Point` (react-aria: `{x, y}`).
// - The key presses a droppable collection passes on during keyboard drags are a
//   `DropTargetKeyDownEvent` (react-aria: the `KeyboardEvent`).
//
// =============================================================================

/// What a drop does with the dragged data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DropOperation {
    Copy,
    Link,
    Move,
    /// The drop is not possible (here).
    Cancel,
}

impl DropOperation {
    /// The `dataTransfer.dropEffect` value.
    pub(crate) fn as_drop_effect(self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Link => "link",
            Self::Move => "move",
            Self::Cancel => "none",
        }
    }

    /// From a `dataTransfer.dropEffect` value.
    pub(crate) fn from_drop_effect(effect: &str) -> Self {
        match effect {
            "copy" => Self::Copy,
            "link" => Self::Link,
            "move" => Self::Move,
            _ => Self::Cancel,
        }
    }

    pub(crate) fn bit(self) -> u8 {
        match self {
            Self::Move => DropOperations::MOVE,
            Self::Copy => DropOperations::COPY,
            Self::Link => DropOperations::LINK,
            Self::Cancel => 0,
        }
    }
}

/// A set of drop operations (react-aria's `DROP_OPERATION` bit field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct DropOperations(pub(crate) u8);

impl DropOperations {
    pub(crate) const MOVE: u8 = 1;
    pub(crate) const COPY: u8 = 1 << 1;
    pub(crate) const LINK: u8 = 1 << 2;
    pub(crate) const ALL: Self = Self(Self::MOVE | Self::COPY | Self::LINK);
    pub(crate) const NONE: Self = Self(0);

    pub(crate) fn from_operations(operations: &[DropOperation]) -> Self {
        Self(operations.iter().fold(0, |bits, op| bits | op.bit()))
    }

    /// From a `dataTransfer.effectAllowed` value.
    pub(crate) fn from_effect_allowed(effect: &str) -> Self {
        Self(match effect {
            "copy" => Self::COPY,
            "link" => Self::LINK,
            "move" => Self::MOVE,
            "copyMove" => Self::COPY | Self::MOVE,
            "copyLink" => Self::COPY | Self::LINK,
            "linkMove" => Self::LINK | Self::MOVE,
            "all" | "uninitialized" => Self::ALL.0,
            _ => 0,
        })
    }

    /// The `dataTransfer.effectAllowed` value.
    pub(crate) fn as_effect_allowed(self) -> &'static str {
        match self.0 {
            Self::COPY => "copy",
            Self::LINK => "link",
            Self::MOVE => "move",
            b if b == Self::COPY | Self::MOVE => "copyMove",
            b if b == Self::COPY | Self::LINK => "copyLink",
            b if b == Self::LINK | Self::MOVE => "linkMove",
            b if b == Self::ALL.0 => "all",
            _ => "none",
        }
    }

    pub(crate) fn contains(self, operation: DropOperation) -> bool {
        self.0 & operation.bit() != 0
    }

    /// The operations in react-aria's order: move, copy, link.
    pub(crate) fn to_vec(self) -> Vec<DropOperation> {
        [
            DropOperation::Move,
            DropOperation::Copy,
            DropOperation::Link,
        ]
        .into_iter()
        .filter(|op| self.contains(*op))
        .collect()
    }

    /// `operation` if it is allowed, otherwise `Cancel`.
    pub(crate) fn restrict(self, operation: DropOperation) -> DropOperation {
        if self.contains(operation) {
            operation
        } else {
            DropOperation::Cancel
        }
    }
}

/// The data of a dragged item, in one or more representations: (MIME type, data) pairs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DragItem {
    entries: Vec<(String, String)>,
}

impl DragItem {
    /// An item without data. Add representations with [`DragItem::with`].
    pub fn new() -> Self {
        Self::default()
    }

    /// An item with a `text/plain` representation.
    pub fn text(text: impl Into<String>) -> Self {
        Self::new().with("text/plain", text)
    }

    /// Add (or replace) the representation of type `kind`.
    #[must_use]
    pub fn with(mut self, kind: impl Into<String>, data: impl Into<String>) -> Self {
        let kind = kind.into();
        let data = data.into();
        match self.entries.iter_mut().find(|(k, _)| *k == kind) {
            Some(entry) => entry.1 = data,
            None => self.entries.push((kind, data)),
        }
        self
    }

    /// The types of the representations.
    pub fn types(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|(k, _)| k.as_str())
    }

    /// The (type, data) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries.iter().map(|(k, d)| (k.as_str(), d.as_str()))
    }

    /// The data of type `kind`.
    pub fn get(&self, kind: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|(k, _)| k == kind)
            .map(|(_, d)| d.as_str())
    }
}

/// The type of dragged data a drop target accepts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DragType {
    /// A MIME type. `image/*` matches all image types, `*/*` everything.
    Mime(Arc<str>),
    /// A dropped directory.
    Directory,
}

impl From<&str> for DragType {
    fn from(value: &str) -> Self {
        Self::Mime(Arc::from(value))
    }
}

impl From<String> for DragType {
    fn from(value: String) -> Self {
        Self::Mime(Arc::from(value))
    }
}

/// The type of data without a type (react-aria's `GENERIC_TYPE`).
pub(crate) const GENERIC_TYPE: &str = "application/octet-stream";

/// The types of the data being dragged.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DragTypes {
    types: HashSet<String>,
    /// Files whose types the browser doesn't reveal before the drop.
    includes_unknown_types: bool,
}

impl DragTypes {
    /// The types of drag items (keyboard and screen reader drags).
    pub fn of_items(items: &[DragItem]) -> Self {
        Self {
            types: items
                .iter()
                .flat_map(|item| item.types().map(ToOwned::to_owned))
                .collect(),
            includes_unknown_types: false,
        }
    }

    /// Exactly `types`.
    pub(crate) fn from_types(types: impl IntoIterator<Item = DragType>) -> Self {
        let mut result = Self::default();
        for kind in types {
            match kind {
                DragType::Mime(mime) => {
                    result.types.insert(mime.to_string());
                }
                DragType::Directory => {
                    result.types.insert(GENERIC_TYPE.to_owned());
                }
            }
        }
        result
    }

    /// The types of a native drag (react-aria's `DragTypes` class).
    pub(crate) fn from_data_transfer(data_transfer: &web_sys::DataTransfer) -> Self {
        let mut types = HashSet::new();
        let mut has_files = false;
        let items = data_transfer.items();
        for i in 0..items.length() {
            let Some(item) = items.get(i) else {
                continue;
            };
            let kind = item.type_();
            if kind == crate::hooks::dnd::utils::CUSTOM_DRAG_TYPE {
                continue;
            }
            if item.kind() == "file" {
                has_files = true;
            }
            types.insert(if kind.is_empty() {
                GENERIC_TYPE.to_owned()
            } else {
                kind
            });
        }
        let dt_types = data_transfer.types();
        let includes_files =
            (0..dt_types.length()).any(|i| dt_types.get(i).as_string().as_deref() == Some("Files"));
        Self {
            types,
            includes_unknown_types: !has_files && includes_files,
        }
    }

    /// Whether the dragged data has data of type `kind`.
    pub fn has(&self, kind: &DragType) -> bool {
        if self.includes_unknown_types {
            return true;
        }
        match kind {
            DragType::Directory => self.types.contains(GENERIC_TYPE),
            DragType::Mime(mime) if &**mime == "*/*" => true,
            DragType::Mime(mime) => match mime.strip_suffix("/*") {
                Some(prefix) => self.types.iter().any(|t| t.starts_with(prefix)),
                None => self.types.contains(&**mime),
            },
        }
    }

    /// Whether the dragged data has data of any of `kinds`.
    pub fn has_any(&self, kinds: &[DragType]) -> bool {
        kinds.iter().any(|k| self.has(k))
    }
}

/// Dropped data.
#[derive(Debug, Clone)]
pub enum DropItem {
    Text(TextDropItem),
    File(FileDropItem),
    Directory(DirectoryDropItem),
}

/// Dropped text data, in one or more representations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextDropItem {
    entries: Vec<(String, String)>,
}

impl TextDropItem {
    pub(crate) fn new(entries: Vec<(String, String)>) -> Self {
        Self { entries }
    }

    /// The types of the representations.
    pub fn types(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|(k, _)| k.as_str())
    }

    /// Whether there is data of type `kind`.
    pub fn has_type(&self, kind: &str) -> bool {
        self.types().any(|t| t == kind)
    }

    /// The data of type `kind`.
    pub fn get_text(&self, kind: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|(k, _)| k == kind)
            .map(|(_, d)| d.as_str())
    }
}

/// A dropped file.
#[derive(Debug, Clone)]
pub struct FileDropItem {
    /// The MIME type.
    pub kind: String,
    pub name: String,
    file: SendWrapper<web_sys::File>,
}

impl FileDropItem {
    pub(crate) fn new(file: web_sys::File) -> Self {
        let kind = file.type_();
        Self {
            kind: if kind.is_empty() {
                GENERIC_TYPE.to_owned()
            } else {
                kind
            },
            name: file.name(),
            file: SendWrapper::new(file),
        }
    }

    /// The file.
    pub fn file(&self) -> web_sys::File {
        (*self.file).clone()
    }

    /// The file's contents as text.
    pub async fn get_text(&self) -> Result<String, wasm_bindgen::JsValue> {
        let text = wasm_bindgen_futures::JsFuture::from(self.file.text()).await?;
        Ok(text.as_string().unwrap_or_default())
    }
}

/// A dropped directory.
#[derive(Debug, Clone)]
pub struct DirectoryDropItem {
    pub name: String,
    entry: SendWrapper<web_sys::FileSystemDirectoryEntry>,
}

impl DirectoryDropItem {
    pub(crate) fn new(entry: web_sys::FileSystemDirectoryEntry) -> Self {
        Self {
            name: entry.name(),
            entry: SendWrapper::new(entry),
        }
    }

    /// The files and directories in the directory.
    pub async fn get_entries(&self) -> Result<Vec<DropItem>, wasm_bindgen::JsValue> {
        crate::hooks::dnd::utils::read_directory(&self.entry).await
    }
}

/// Where in relation to an item a drop happens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DropPosition {
    Before,
    On,
    After,
}

/// A drop target in a collection: the collection itself, or a position at an item.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DropTarget {
    Root,
    Item(ItemDropTarget),
}

impl DropTarget {
    pub fn item(key: impl Into<Key>, drop_position: DropPosition) -> Self {
        Self::Item(ItemDropTarget {
            key: key.into(),
            drop_position,
        })
    }
}

/// A drop position at a collection item.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemDropTarget {
    pub key: Key,
    pub drop_position: DropPosition,
}

/// A drag starts. Coordinates are relative to the viewport.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DragStartEvent {
    pub x: f64,
    pub y: f64,
}

/// A drag moves.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DragMoveEvent {
    pub x: f64,
    pub y: f64,
}

/// A drag ends: dropped (`drop_operation` is not `Cancel`) or canceled.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DragEndEvent {
    pub x: f64,
    pub y: f64,
    pub drop_operation: DropOperation,
}

/// A drag enters a drop target. Coordinates are relative to the drop target.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DropEnterEvent {
    pub x: f64,
    pub y: f64,
}

/// A drag moves over a drop target.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DropMoveEvent {
    pub x: f64,
    pub y: f64,
}

/// A drag has hovered a drop target for a while (e.g. to open it).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DropActivateEvent {
    pub x: f64,
    pub y: f64,
}

/// A drag leaves a drop target.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DropExitEvent {
    pub x: f64,
    pub y: f64,
}

/// Data is dropped.
#[derive(Debug, Clone)]
pub struct DropEvent {
    pub x: f64,
    pub y: f64,
    pub items: Vec<DropItem>,
    pub drop_operation: DropOperation,
}

/// A drag enters a collection drop target.
#[derive(Debug, Clone, PartialEq)]
pub struct DroppableCollectionEnterEvent {
    pub x: f64,
    pub y: f64,
    pub target: DropTarget,
}

/// A drag has hovered a collection drop target for a while.
#[derive(Debug, Clone, PartialEq)]
pub struct DroppableCollectionActivateEvent {
    pub x: f64,
    pub y: f64,
    pub target: DropTarget,
}

/// A drag leaves a collection drop target.
#[derive(Debug, Clone, PartialEq)]
pub struct DroppableCollectionExitEvent {
    pub x: f64,
    pub y: f64,
    pub target: DropTarget,
}

/// Data is dropped on a collection drop target.
#[derive(Debug, Clone)]
pub struct DroppableCollectionDropEvent {
    pub x: f64,
    pub y: f64,
    pub items: Vec<DropItem>,
    pub drop_operation: DropOperation,
    pub target: DropTarget,
}

/// External data is dropped between items.
#[derive(Debug, Clone)]
pub struct DroppableCollectionInsertDropEvent {
    pub items: Vec<DropItem>,
    pub drop_operation: DropOperation,
    pub target: ItemDropTarget,
}

/// Data is dropped on the collection itself.
#[derive(Debug, Clone)]
pub struct DroppableCollectionRootDropEvent {
    pub items: Vec<DropItem>,
    pub drop_operation: DropOperation,
}

/// Data is dropped on an item.
#[derive(Debug, Clone)]
pub struct DroppableCollectionOnItemDropEvent {
    pub items: Vec<DropItem>,
    pub drop_operation: DropOperation,
    pub is_internal: bool,
    pub target: ItemDropTarget,
}

/// Items of the collection are dropped within it (reordered or moved).
#[derive(Debug, Clone)]
pub struct DroppableCollectionReorderEvent {
    pub keys: HashSet<Key>,
    pub drop_operation: DropOperation,
    pub target: ItemDropTarget,
}

/// A drag of collection items starts.
#[derive(Debug, Clone)]
pub struct DraggableCollectionStartEvent {
    pub x: f64,
    pub y: f64,
    pub keys: HashSet<Key>,
}

/// A drag of collection items moves.
#[derive(Debug, Clone)]
pub struct DraggableCollectionMoveEvent {
    pub x: f64,
    pub y: f64,
    pub keys: HashSet<Key>,
}

/// A drag of collection items ends.
#[derive(Debug, Clone)]
pub struct DraggableCollectionEndEvent {
    pub x: f64,
    pub y: f64,
    pub drop_operation: DropOperation,
    pub keys: HashSet<Key>,
    /// The items were dropped in the collection they were dragged from.
    pub is_internal: bool,
}

/// The data types a drop target accepts.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum AcceptedDragTypes {
    #[default]
    All,
    Types(Vec<DragType>),
}

impl AcceptedDragTypes {
    /// Whether data of `types` is accepted.
    pub fn accepts(&self, types: &DragTypes) -> bool {
        match self {
            Self::All => true,
            Self::Types(accepted) => types.has_any(accepted),
        }
    }
}

/// An element and offset to show while dragging (react-aria's drag preview).
#[derive(Debug, Clone)]
pub struct DragPreview {
    pub element: SendWrapper<web_sys::Element>,
    /// The position of the pointer in the preview. Defaults to where the pointer is in the
    /// dragged element.
    pub offset: Option<Point>,
}

/// A key pressed during a keyboard drag over a droppable collection, after the collection moved
/// its drop target (react-aria's `onKeyDown`). The drag manager handles every key press of a
/// keyboard drag itself: the event's default is prevented and its propagation stopped already.
#[derive(Debug, Clone)]
pub struct DropTargetKeyDownEvent {
    /// The pressed key.
    pub key: KeyboardKey,
    event: SendWrapper<web_sys::KeyboardEvent>,
}

impl DropTargetKeyDownEvent {
    pub(crate) fn new(event: &web_sys::KeyboardEvent) -> Self {
        Self {
            key: event.typed_key(),
            event: SendWrapper::new(event.clone()),
        }
    }

    /// The underlying event (modifier keys, ...).
    pub fn event(&self) -> &web_sys::KeyboardEvent {
        &self.event
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn drag_types_match_wildcards_and_exact_types() {
        let types = DragTypes::of_items(&[
            DragItem::text("hello"),
            DragItem::new().with("image/png", "..."),
        ]);
        assert_that!(types.has(&DragType::from("text/plain"))).is_true();
        assert_that!(types.has(&DragType::from("image/*"))).is_true();
        assert_that!(types.has(&DragType::from("*/*"))).is_true();
        assert_that!(types.has(&DragType::from("text/html"))).is_false();
        assert_that!(types.has(&DragType::Directory)).is_false();
    }

    #[test]
    fn drop_operations_keep_react_arias_order() {
        let ops = DropOperations::from_operations(&[DropOperation::Link, DropOperation::Copy]);
        assert_that!(ops.to_vec()).is_equal_to(vec![DropOperation::Copy, DropOperation::Link]);
        assert_that!(ops.as_effect_allowed()).is_equal_to("copyLink");
        assert_that!(ops.restrict(DropOperation::Move)).is_equal_to(DropOperation::Cancel);
        assert_that!(DropOperations::from_effect_allowed("all")).is_equal_to(DropOperations::ALL);
    }
}
