// Upstream: react-aria/src/tabs/TabsKeyboardDelegate.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    hooks::{
        Orientation,
        collections::{
            Collection, CollectionMemo, Key, KeyboardDelegate, NavigationOptions, SelectionManager,
        },
    },
    utils::locale::WritingDirection,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Keyboard navigation for tab lists: arrow keys in the list's orientation move to the
/// previous/next enabled tab, wrapping around.
#[derive(Debug, Clone)]
pub struct TabsKeyboardDelegate {
    collection: CollectionMemo,
    selection: SelectionManager,
    direction: WritingDirection,
    orientation: Orientation,
}

impl TabsKeyboardDelegate {
    pub fn new(
        collection: CollectionMemo,
        selection: SelectionManager,
        direction: WritingDirection,
        orientation: Orientation,
    ) -> Self {
        Self {
            collection,
            selection,
            direction,
            orientation,
        }
    }

    fn is_disabled(&self, key: &Key) -> bool {
        untrack(|| self.selection.is_disabled(key))
    }

    fn with<R>(&self, f: impl FnOnce(&Collection) -> R) -> R {
        self.collection.with_untracked(|c| f(c))
    }

    fn step(&self, start: &Key, forward: bool) -> Option<Key> {
        self.with(|c| {
            let mut key = start.clone();
            loop {
                let next = if forward {
                    c.key_after(&key).or_else(|| c.first_key())
                } else {
                    c.key_before(&key).or_else(|| c.last_key())
                }?
                .clone();
                if !self.is_disabled(&next) || next == *start {
                    return Some(next);
                }
                key = next;
            }
        })
    }

    fn next_key(&self, key: &Key) -> Option<Key> {
        self.step(key, true)
    }

    fn previous_key(&self, key: &Key) -> Option<Key> {
        self.step(key, false)
    }

    fn rtl(&self) -> bool {
        self.direction == WritingDirection::Rtl
    }
}

impl KeyboardDelegate for TabsKeyboardDelegate {
    fn key_left_of(&self, key: &Key, _options: NavigationOptions) -> Option<Key> {
        if self.rtl() {
            self.next_key(key)
        } else {
            self.previous_key(key)
        }
    }

    fn key_right_of(&self, key: &Key, _options: NavigationOptions) -> Option<Key> {
        if self.rtl() {
            self.previous_key(key)
        } else {
            self.next_key(key)
        }
    }

    fn key_above(&self, key: &Key, _options: NavigationOptions) -> Option<Key> {
        match self.orientation {
            Orientation::Horizontal => None,
            Orientation::Vertical => self.previous_key(key),
        }
    }

    fn key_below(&self, key: &Key, _options: NavigationOptions) -> Option<Key> {
        match self.orientation {
            Orientation::Horizontal => None,
            Orientation::Vertical => self.next_key(key),
        }
    }

    fn first_key(&self, _from: Option<&Key>, _global: bool) -> Option<Key> {
        let key = self.with(|c| c.first_key().cloned())?;
        if self.is_disabled(&key) {
            return self.next_key(&key);
        }
        Some(key)
    }

    fn last_key(&self, _from: Option<&Key>, _global: bool) -> Option<Key> {
        let key = self.with(|c| c.last_key().cloned())?;
        if self.is_disabled(&key) {
            return self.previous_key(&key);
        }
        Some(key)
    }
}
