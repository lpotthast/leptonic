// No upstream: the registry of rendered item elements, replacing react-aria's `[data-key]` DOM queries.

use std::collections::{HashMap, hash_map::Entry};

use leptos::{prelude::*, reactive::owner::WeakOwner};
use send_wrapper::SendWrapper;

use super::{FocusItem, FocusStrategy, Key, SelectionManager};
use crate::{
    CapturedElement,
    utils::{
        focus::focus_safely, keyed_triggers::KeyedTriggers, shadow_dom::get_active_element,
        virtual_focus::move_virtual_focus,
    },
};

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
    registry: StoredValue<Registry>,
    /// Notified per key when an item registers or unregisters (see
    /// [`ItemElements::get_tracked`]): only the readers of that key run again, not every reader
    /// on every item's mount and unmount.
    changed: KeyedTriggers<Key>,
}

#[derive(Default)]
struct Registry {
    /// The next registration id. Every registration gets one, so an item only removes its own
    /// registration, and a key stays registered while any item rendering it lives (e.g. two
    /// re-rendered items swapping keys).
    next_id: u64,
    /// The live registrations of each key, the latest last.
    items: HashMap<Key, Registrations>,
    /// The collection's owner (the registry's creator): the focus effect lives as long as it.
    /// Weak: the registry is one of the owner's arena items, and a strong handle would keep the
    /// owner from being dropped.
    owner: Option<WeakOwner>,
    /// Whether the focus effect exists (created for the first item that takes focus).
    follows_focus: bool,
}

struct Registration {
    id: u64,
    element: CapturedElement,
    focus: FocusTarget,
}

/// The live registrations of a key: one, except while a re-rendered item's new element is
/// registered before its old one is gone. The usual single registration is stored inline (no
/// allocation per item).
struct Registrations {
    latest: Registration,
    earlier: Vec<Registration>,
}

impl Registrations {
    fn push(&mut self, registration: Registration) {
        let previous = std::mem::replace(&mut self.latest, registration);
        self.earlier.push(previous);
    }

    #[cfg(debug_assertions)]
    fn iter(&self) -> impl Iterator<Item = &Registration> {
        self.earlier.iter().chain(std::iter::once(&self.latest))
    }

    /// Removes the registration `id`; `false` when it was the last one.
    fn remove(&mut self, id: u64) -> bool {
        if self.latest.id != id {
            self.earlier.retain(|registration| registration.id != id);
            return true;
        }
        match self.earlier.pop() {
            Some(previous) => {
                self.latest = previous;
                true
            }
            None => false,
        }
    }
}

/// How a registered item takes DOM focus when it becomes the focused item
/// (`use_selectable_item`).
#[derive(Debug, Clone)]
pub(crate) struct FocusTarget {
    /// Moves focus elsewhere instead of the element (a grid row's cell, a cell's child).
    pub(crate) focus: Option<FocusItem>,
    /// The collection uses virtual focus: DOM focus stays where it is.
    pub(crate) should_use_virtual_focus: bool,
}

impl Default for ItemElements {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemElements {
    pub fn new() -> Self {
        Self {
            registry: StoredValue::new(Registry {
                owner: Owner::current().map(|owner| owner.downgrade()),
                ..Registry::default()
            }),
            changed: KeyedTriggers::new(),
        }
    }

    /// Register the element of `key` (`use_selectable_item`): it is unregistered when the current
    /// reactive owner (the item) is cleaned up, so `element` must live at least as long as that
    /// owner (create it in the item or an ancestor). The item is focused (as `focus` says)
    /// whenever `key` becomes the focused item while the collection has focus, or the item
    /// renders while focused, or the child focus strategy changes meanwhile (react-aria: an effect
    /// in `useSelectableItem`). One effect for the whole collection follows the focus, instead of
    /// one per item (`performance.md` R6).
    pub(crate) fn register(
        &self,
        key: Key,
        element: CapturedElement,
        focus: FocusTarget,
        selection: SelectionManager,
    ) {
        let registry = self.registry;
        let changed = self.changed;
        let (id, has_others) = registry
            .try_update_value(|registry| {
                let id = registry.next_id;
                registry.next_id += 1;
                let registration = Registration { id, element, focus };
                let has_others = match registry.items.entry(key.clone()) {
                    Entry::Occupied(mut registrations) => {
                        registrations.get_mut().push(registration);
                        true
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(Registrations {
                            latest: registration,
                            earlier: Vec::new(),
                        });
                        false
                    }
                };
                (id, has_others)
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
                let duplicate = registry
                    .try_with_value(|registry| {
                        registry.items.get(&key).is_some_and(|registrations| {
                            registrations.iter().any(|other| {
                                other.id != id
                                    && other
                                        .element
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
        self.changed.notify([&key]);
        on_cleanup(move || {
            // The collection may be disposed before its items.
            let _ = registry.try_update_value(|registry| {
                if let Some(registrations) = registry.items.get_mut(&key)
                    && !registrations.remove(id)
                {
                    registry.items.remove(&key);
                }
            });
            // (A no-op once the collection is disposed.)
            changed.notify([&key]);
        });
        self.follow_focus(selection);
    }

    /// Creates the collection's focus effect, once (see [`register`](Self::register)).
    fn follow_focus(self, selection: SelectionManager) {
        let Some(owner) = self
            .registry
            .try_update_value(|registry| {
                (!std::mem::replace(&mut registry.follows_focus, true))
                    .then(|| registry.owner.clone())
            })
            .flatten()
        else {
            return;
        };
        let registry = self.registry;
        let changed = self.changed;
        let follow = move || {
            // The focused registration and the child focus strategy it was focused with: focus
            // moves only when they change, as each item's effect in react-aria runs only when it
            // becomes the focused one (or the strategy changes while it is).
            Effect::new(
                move |previous: Option<Option<(u64, Option<FocusStrategy>)>>| {
                    let key = selection.focused_key();
                    let has_focus = selection.is_focused();
                    let strategy = selection.child_focus_strategy();
                    if let Some(key) = &key {
                        changed.track(key);
                    }
                    if !has_focus {
                        return None;
                    }
                    let (id, element, target) = registry
                        .try_with_value(|registry| {
                            let registration = &registry.items.get(key.as_ref()?)?.latest;
                            Some((
                                registration.id,
                                registration.element,
                                registration.focus.clone(),
                            ))
                        })
                        .flatten()?;
                    if previous.flatten() == Some((id, strategy)) {
                        return Some((id, strategy));
                    }
                    if target.should_use_virtual_focus {
                        if let Some(el) = element.get_untracked() {
                            move_virtual_focus(Some(&el));
                        }
                    } else if let Some(focus) = &target.focus {
                        focus.focus();
                    } else if let Some(el) = element.get_untracked() {
                        let active = el.owner_document().as_ref().and_then(get_active_element);
                        if active.as_ref() != Some(&*el) {
                            focus_safely(&el);
                        }
                    }
                    Some((id, strategy))
                },
            );
        };
        match owner {
            Some(owner) => {
                // `None`: the collection is being disposed.
                if let Some(owner) = owner.upgrade() {
                    owner.with(follow);
                }
            }
            None => follow(),
        }
    }

    /// The element of `key`, tracking whether it is rendered: a reactive context runs again when
    /// the item renders (e.g. an item a virtualizer renders once it is focused).
    pub fn get_tracked(&self, key: &Key) -> Option<SendWrapper<web_sys::Element>> {
        self.changed.track(key);
        self.registry
            .with_value(|registry| Self::latest(registry, key))
            .and_then(|element| element.get())
    }

    /// The element of `key`, if it is rendered.
    pub fn get(&self, key: &Key) -> Option<SendWrapper<web_sys::Element>> {
        self.registry
            .with_value(|registry| Self::latest(registry, key))
            .and_then(|element| element.get_untracked())
    }

    /// The latest live registration of `key`.
    fn latest(registry: &Registry, key: &Key) -> Option<CapturedElement> {
        registry
            .items
            .get(key)
            .map(|registrations| registrations.latest.element)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use assertr::prelude::*;

    use super::*;
    use crate::{
        hooks::collections::{Collection, SelectionOptions},
        testing::{flush_effects, with_owner},
    };

    /// The registry holds the collection's owner weakly: an owner whose arena holds a strong
    /// handle to itself is never dropped, so its focus effect would outlive the collection (a
    /// collection in a `<For>` row, whose owner is only dropped when the row goes).
    #[test]
    fn the_focus_effect_does_not_keep_the_collection_alive() {
        with_owner(|| {
            let collection_owner = Owner::new();
            let weak = collection_owner.downgrade();
            collection_owner.with(|| {
                let collection = Memo::new(|_| {
                    Arc::new(Collection::build(|b| {
                        let _ = b.item("a", "A");
                    }))
                });
                let selection = SelectionManager::new(collection, SelectionOptions::default());
                let elements = ItemElements::new();
                elements.register(
                    Key::from("a"),
                    CapturedElement::new(),
                    FocusTarget {
                        focus: None,
                        should_use_virtual_focus: false,
                    },
                    selection,
                );
            });
            flush_effects();
            drop(collection_owner);
            assert_that!(weak.upgrade().is_none()).is_true();
        });
    }

    /// A re-rendered item registers its new element before its old one is gone: the latest live
    /// registration counts, whichever goes first.
    #[test]
    fn the_latest_live_registration_of_a_key_counts() {
        with_owner(|| {
            let collection = Memo::new(|_| {
                Arc::new(Collection::build(|b| {
                    let _ = b.item("a", "A");
                }))
            });
            let selection = SelectionManager::new(collection, SelectionOptions::default());
            let elements = ItemElements::new();
            let register = |virtual_focus: bool| {
                let owner = Owner::new();
                owner.with(|| {
                    elements.register(
                        Key::from("a"),
                        CapturedElement::new(),
                        FocusTarget {
                            focus: None,
                            should_use_virtual_focus: virtual_focus,
                        },
                        selection,
                    );
                });
                owner
            };
            let latest = || {
                elements.registry.with_value(|registry| {
                    registry
                        .items
                        .get(&Key::from("a"))
                        .map(|registrations| registrations.latest.focus.should_use_virtual_focus)
                })
            };
            let first = register(false);
            let second = register(true);
            let third = register(false);
            assert_that!(latest()).is_equal_to(Some(false));
            third.cleanup();
            assert_that!(latest()).is_equal_to(Some(true));
            first.cleanup();
            assert_that!(latest()).is_equal_to(Some(true));
            second.cleanup();
            assert_that!(latest()).is_none();
        });
    }
}
