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
    /// The next registration id, and the live registrations of each key, the latest last. Every
    /// registration gets an id, so an item only removes its own registration, and a key stays
    /// registered while any item rendering it lives (e.g. two re-rendered items swapping keys).
    elements: StoredValue<(u64, HashMap<Key, Vec<(u64, CapturedElement)>>)>,
    /// Notified when an item registers or unregisters (see [`ItemElements::get_tracked`]).
    changed: Trigger,
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
            changed: Trigger::new(),
        }
    }

    /// Register the element of `key`. It is unregistered when the current reactive owner (the
    /// item) is cleaned up. `element` must live at least as long as that owner (create it in the
    /// item or an ancestor).
    pub fn register(&self, key: Key, element: CapturedElement) {
        let elements = self.elements;
        let changed = self.changed;
        let (id, has_others) = elements
            .try_update_value(|(next_id, map)| {
                let id = *next_id;
                *next_id += 1;
                let registrations = map.entry(key.clone()).or_default();
                registrations.push((id, element));
                (id, registrations.len() > 1)
            })
            .unwrap_or_default();
        #[cfg(not(debug_assertions))]
        let _ = has_others;
        // Once mounted (development only): two rendered items with one key confuse focus and
        // selection (focus can bounce between them). Another item registered with this key must
        // be gone by then (a re-rendered item replaces its old element). Only live
        // registrations are looked at: an item's elements are disposed after its cleanup
        // unregistered them.
        #[cfg(debug_assertions)]
        if has_others {
            let key = key.clone();
            Effect::new(move || {
                let Some(mine) = element.get() else {
                    return;
                };
                let duplicate = elements
                    .try_with_value(|(_, map)| {
                        map.get(&key).is_some_and(|registrations| {
                            registrations.iter().any(|(other, other_element)| {
                                *other != id
                                    && other_element
                                        .get_untracked()
                                        .is_some_and(|e| *e != *mine && e.is_connected())
                            })
                        })
                    })
                    .unwrap_or(false);
                if duplicate && mine.is_connected() {
                    crate::utils::dev_warn!(
                        "Two items of a collection are rendered with the key {key:?}. Each key may \
                         only be rendered once."
                    );
                }
            });
        }
        on_cleanup(move || {
            // The collection may be disposed before its items.
            let _ = elements.try_update_value(|(_, map)| {
                if let Some(registrations) = map.get_mut(&key) {
                    registrations.retain(|(registered, _)| *registered != id);
                    if registrations.is_empty() {
                        map.remove(&key);
                    }
                }
            });
            // (A no-op once the collection is disposed.)
            changed.notify();
        });
        self.changed.notify();
    }

    /// The element of `key`, tracking whether it is rendered: a reactive context runs again when
    /// the item renders (e.g. an item a virtualizer renders once it is focused).
    pub fn get_tracked(&self, key: &Key) -> Option<SendWrapper<web_sys::Element>> {
        self.changed.track();
        self.elements
            .with_value(|(_, map)| Self::latest(map, key))
            .and_then(|element| element.get())
    }

    /// The element of `key`, if it is rendered.
    pub fn get(&self, key: &Key) -> Option<SendWrapper<web_sys::Element>> {
        self.elements
            .with_value(|(_, map)| Self::latest(map, key))
            .and_then(|element| element.get_untracked())
    }

    /// The latest live registration of `key`.
    fn latest(
        map: &HashMap<Key, Vec<(u64, CapturedElement)>>,
        key: &Key,
    ) -> Option<CapturedElement> {
        map.get(key)
            .and_then(|registrations| registrations.last())
            .map(|(_, element)| *element)
    }
}
