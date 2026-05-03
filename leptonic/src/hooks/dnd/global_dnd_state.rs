//! Global thread-local state for cross-collection drag-and-drop coordination.
//!
//! Tracks which keys are being dragged and which collection element is the
//! source, enabling detection of internal vs external drop operations.
//!
//! Based on react-aria's global drag-and-drop state tracking in
//! `@react-aria/dnd/src/utils.ts`.

use std::collections::HashSet;

#[cfg(feature = "ssr")]
pub fn set_dragging_keys(_keys: &HashSet<String>) {}

#[cfg(feature = "ssr")]
pub fn get_dragging_keys() -> HashSet<String> {
    HashSet::new()
}

#[cfg(feature = "ssr")]
pub fn clear_dragging_keys() {}

#[cfg(feature = "ssr")]
pub fn set_dragging_collection_element(_element: Option<&web_sys::Element>) {}

#[cfg(feature = "ssr")]
pub fn set_drop_collection_element(_element: Option<&web_sys::Element>) {}

#[cfg(feature = "ssr")]
pub fn is_internal_drop_operation(_collection_element: &web_sys::Element) -> bool {
    false
}

#[cfg(not(feature = "ssr"))]
use std::cell::RefCell;

#[cfg(not(feature = "ssr"))]
thread_local! {
    /// The keys currently being dragged.
    static DRAGGING_KEYS: RefCell<HashSet<String>> = RefCell::new(HashSet::new());

    /// The source collection element (the element with the droppable collection hook).
    static DRAGGING_COLLECTION_ELEMENT: RefCell<Option<web_sys::Element>> = const { RefCell::new(None) };

    /// The current target collection element.
    static DROP_COLLECTION_ELEMENT: RefCell<Option<web_sys::Element>> = const { RefCell::new(None) };
}

/// Sets the keys being dragged in the current session.
#[cfg(not(feature = "ssr"))]
#[allow(clippy::implicit_hasher)]
pub fn set_dragging_keys(keys: &HashSet<String>) {
    DRAGGING_KEYS.with(|dk| {
        dk.borrow_mut().clone_from(keys);
    });
}

/// Gets the keys being dragged in the current session.
#[cfg(not(feature = "ssr"))]
pub fn get_dragging_keys() -> HashSet<String> {
    DRAGGING_KEYS.with(|dk| dk.borrow().clone())
}

/// Clears the dragging keys (called at drag end).
#[cfg(not(feature = "ssr"))]
pub fn clear_dragging_keys() {
    DRAGGING_KEYS.with(|dk| dk.borrow_mut().clear());
}

/// Sets the source collection element for the current drag session.
#[cfg(not(feature = "ssr"))]
pub fn set_dragging_collection_element(element: Option<&web_sys::Element>) {
    DRAGGING_COLLECTION_ELEMENT.with(|dce| {
        *dce.borrow_mut() = element.cloned();
    });
}

/// Sets the current drop target collection element.
#[cfg(not(feature = "ssr"))]
pub fn set_drop_collection_element(element: Option<&web_sys::Element>) {
    DROP_COLLECTION_ELEMENT.with(|dce| {
        *dce.borrow_mut() = element.cloned();
    });
}

/// Returns `true` if the given element is the same as the source collection element,
/// indicating an internal (same-collection) drop operation.
#[cfg(not(feature = "ssr"))]
pub fn is_internal_drop_operation(collection_element: &web_sys::Element) -> bool {
    DRAGGING_COLLECTION_ELEMENT.with(|dce| {
        dce.borrow()
            .as_ref()
            .is_some_and(|source| source == collection_element)
    })
}
