//! Drop item types for receiving data from drop operations.
//!
//! Unlike [`DragItem`](super::DragItem) (which represents app-internal text
//! data), `DropItem` can represent external content such as files and
//! directories dropped from the operating system.
//!
//! Based on react-aria's `DropItem` types from
//! `@react-aria/dnd/src/types.ts`.

use std::collections::{HashMap, HashSet};

use crate::hooks::DragItem;

//
// ## RUST-NATIVE DESIGN
//
// - `DropItem` is a proper enum with `Text`, `File`, `Directory` variants
//   instead of react-aria's discriminated union with `kind` field.
// - `FileDropItem.file()` returns `&web_sys::File` (sync borrow) instead of
//   react-aria's `getFile(): Promise<File>` (async, clones). The file is
//   already available from the DataTransfer.
// - `text()` and `entries()` are `async fn` returning `Result` instead of
//   JS Promise types.
//

/// An item received from a drop operation.
///
/// Unlike [`DragItem`] (which is always text-based and created within the
/// app), `DropItem` can represent external content like files and directories
/// from the operating system.
#[derive(Debug, Clone)]
pub enum DropItem {
    /// Text-based item with one or more MIME type representations.
    Text(TextDropItem),
    /// A file dropped from the operating system.
    File(FileDropItem),
    /// A directory dropped from the operating system.
    Directory(DirectoryDropItem),
}

impl DropItem {
    /// Returns this item as a [`TextDropItem`], if it is one.
    #[must_use]
    pub fn as_text(&self) -> Option<&TextDropItem> {
        match self {
            Self::Text(item) => Some(item),
            _ => None,
        }
    }

    /// Returns this item as a [`FileDropItem`], if it is one.
    #[must_use]
    pub fn as_file(&self) -> Option<&FileDropItem> {
        match self {
            Self::File(item) => Some(item),
            _ => None,
        }
    }

    /// Returns this item as a [`DirectoryDropItem`], if it is one.
    #[must_use]
    pub fn as_directory(&self) -> Option<&DirectoryDropItem> {
        match self {
            Self::Directory(item) => Some(item),
            _ => None,
        }
    }
}

/// A text-based drop item with one or more MIME type representations.
#[derive(Debug, Clone)]
pub struct TextDropItem {
    types: HashSet<String>,
    data: HashMap<String, String>,
}

impl TextDropItem {
    /// Creates a new text drop item from a map of type→data pairs.
    #[must_use]
    pub fn new(data: HashMap<String, String>) -> Self {
        let types = data.keys().cloned().collect();
        Self { types, data }
    }

    /// The MIME types available in this item.
    #[must_use]
    pub fn types(&self) -> &HashSet<String> {
        &self.types
    }

    /// Gets the data for a specific MIME type.
    #[must_use]
    pub fn get(&self, kind: &str) -> Option<&str> {
        self.data.get(kind).map(String::as_str)
    }

    /// Returns `true` if this item has the given MIME type.
    #[must_use]
    pub fn has_type(&self, kind: &str) -> bool {
        self.types.contains(kind)
    }
}

impl From<DragItem> for TextDropItem {
    fn from(item: DragItem) -> Self {
        let mut data = HashMap::new();
        for (kind, value) in item.iter() {
            data.insert(kind.to_owned(), value.to_owned());
        }
        Self::new(data)
    }
}

/// A file dropped from the operating system.
#[derive(Debug, Clone)]
pub struct FileDropItem {
    file: web_sys::File,
}

impl FileDropItem {
    /// Creates a new file drop item.
    #[must_use]
    pub fn new(file: web_sys::File) -> Self {
        Self { file }
    }

    /// The file name.
    #[must_use]
    pub fn name(&self) -> String {
        self.file.name()
    }

    /// The MIME type of the file (may be empty if unknown).
    #[must_use]
    pub fn mime_type(&self) -> String {
        use web_sys::Blob;
        Blob::type_(&self.file)
    }

    /// Returns a reference to the underlying `web_sys::File`.
    #[must_use]
    pub fn file(&self) -> &web_sys::File {
        &self.file
    }

    /// Reads the file contents as text.
    ///
    /// # Errors
    ///
    /// Returns an error if reading the file fails.
    #[cfg(not(feature = "ssr"))]
    pub async fn text(&self) -> Result<String, wasm_bindgen::JsValue> {
        use web_sys::Blob;
        let promise = Blob::text(&self.file);
        let value = wasm_bindgen_futures::JsFuture::from(promise).await?;
        Ok(value.as_string().unwrap_or_default())
    }
}

/// A directory dropped from the operating system.
///
/// Uses the `FileSystemDirectoryEntry` API to read directory contents.
#[derive(Debug, Clone)]
pub struct DirectoryDropItem {
    name: String,
}

impl DirectoryDropItem {
    /// Creates a new directory drop item.
    #[must_use]
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// The directory name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Reads drop items from a `DataTransfer`, producing [`DropItem`] instances.
///
/// This function handles:
/// - App-internal items (serialized via [`write_to_data_transfer`](super::write_to_data_transfer))
///   → converted to [`DropItem::Text`].
/// - Files from the operating system → [`DropItem::File`].
/// - Native text types → [`DropItem::Text`].
pub fn read_drop_items_from_data_transfer(dt: &web_sys::DataTransfer) -> Vec<DropItem> {
    let mut items = Vec::new();

    // Check for files first.
    if let Some(file_list) = dt.files() {
        for i in 0..file_list.length() {
            if let Some(file) = file_list.get(i) {
                items.push(DropItem::File(FileDropItem::new(file)));
            }
        }
    }

    // If we got files, return them (don't mix with text representations).
    if !items.is_empty() {
        return items;
    }

    // Fall back to reading app-internal / text items.
    let drag_items = super::use_draggable::read_from_data_transfer(dt);
    for drag_item in drag_items {
        items.push(DropItem::Text(TextDropItem::from(drag_item)));
    }

    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_drop_item() {
        let mut data = HashMap::new();
        data.insert("text/plain".to_string(), "hello".to_string());
        data.insert("text/html".to_string(), "<b>hello</b>".to_string());

        let item = TextDropItem::new(data);
        assert!(item.has_type("text/plain"));
        assert!(item.has_type("text/html"));
        assert!(!item.has_type("image/png"));
        assert_eq!(item.get("text/plain"), Some("hello"));
        assert_eq!(item.get("text/html"), Some("<b>hello</b>"));
        assert_eq!(item.types().len(), 2);
    }

    #[test]
    fn test_text_drop_item_from_drag_item() {
        let drag_item = DragItem::text("world").with_type("text/html", "<i>world</i>");
        let drop_item = TextDropItem::from(drag_item);
        assert_eq!(drop_item.get("text/plain"), Some("world"));
        assert_eq!(drop_item.get("text/html"), Some("<i>world</i>"));
    }

    #[test]
    fn test_drop_item_accessors() {
        let text = DropItem::Text(TextDropItem::new(HashMap::new()));
        assert!(text.as_text().is_some());
        assert!(text.as_file().is_none());
        assert!(text.as_directory().is_none());

        let dir = DropItem::Directory(DirectoryDropItem::new("docs".into()));
        assert!(dir.as_directory().is_some());
        assert!(dir.as_text().is_none());
        assert_eq!(dir.as_directory().unwrap().name(), "docs");
    }
}
