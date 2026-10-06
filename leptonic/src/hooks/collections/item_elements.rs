use std::collections::HashMap;

use leptos::prelude::*;
use send_wrapper::SendWrapper;

use super::Key;
use crate::utils::CapturedElement;

/// Maps the keys of a collection to their rendered elements.
///
/// Item hooks register the element they are spread onto. Collection hooks use the registry to
/// focus items, scroll them into view and measure them. It replaces DOM queries for
/// `[data-key="..."]`, which break with nested collections and keys that aren't valid in CSS
/// selectors.
///
/// The registry is empty during server-side rendering; it is only used in effects and event
/// handlers.
#[derive(Debug, Clone, Copy)]
pub struct ItemElements {
    /// Every registration gets an id, so an item only removes its own registration.
    elements: StoredValue<(u64, HashMap<Key, (u64, CapturedElement)>)>,
}

impl Default for ItemElements {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemElements {
    pub fn new() -> Self {
        Self {
            elements: StoredValue::new((0, HashMap::new())),
        }
    }

    /// Register the element of `key`. It is unregistered when the current reactive owner (the
    /// item) is cleaned up.
    pub fn register(&self, key: Key, element: CapturedElement) {
        let elements = self.elements;
        let id = elements
            .try_update_value(|(next_id, map)| {
                let id = *next_id;
                *next_id += 1;
                map.insert(key.clone(), (id, element));
                id
            })
            .unwrap_or_default();
        on_cleanup(move || {
            // The collection may be disposed before its items.
            let _ = elements.try_update_value(|(_, map)| {
                // Only remove our own registration: a re-rendered item may have registered the
                // same key again in the meantime.
                if map
                    .get(&key)
                    .is_some_and(|(registered, _)| *registered == id)
                {
                    map.remove(&key);
                }
            });
        });
    }

    /// The element of `key`, if it is rendered.
    pub fn get(&self, key: &Key) -> Option<SendWrapper<web_sys::Element>> {
        self.elements
            .with_value(|(_, map)| map.get(key).map(|(_, element)| *element))
            .and_then(|element| element.get_untracked())
    }
}
