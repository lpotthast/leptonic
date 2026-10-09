// Upstream: react-aria/src/tabs/TabsKeyboardDelegate.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    Orientation,
    hooks::collections::{
        Collection, CollectionMemo, Key, KeyboardDelegate, NavigationOptions, SelectionManager,
    },
    utils::i18n::WritingDirection,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
// - Navigation stops after visiting the collection once: if the starting tab was removed and
//   every remaining tab is disabled, upstream's wrap-to-start termination would loop forever.
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
            for _ in 0..c.size() {
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
            None
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use assertr::prelude::*;

    use super::*;
    use crate::{
        hooks::{
            collections::Collection,
            tabs::{UseTabListStateInput, use_tab_list_state},
        },
        testing::with_owner,
    };

    fn delegate(
        keys: RwSignal<Vec<&'static str>>,
        disabled: &[&'static str],
        direction: WritingDirection,
        orientation: Orientation,
    ) -> TabsKeyboardDelegate {
        let collection: CollectionMemo = Memo::new(move |_| {
            Arc::new(Collection::build(|b| {
                for key in keys.get() {
                    b.item(key, key.to_uppercase());
                }
            }))
        });
        let state = use_tab_list_state(UseTabListStateInput {
            collection,
            default_selected_key: None,
            selected_key: None,
            on_selection_change: None,
            disabled_keys: Signal::stored(disabled.iter().map(|key| Key::from(*key)).collect()),
            is_disabled: Signal::stored(false),
        });
        TabsKeyboardDelegate::new(
            collection,
            state.list.list.selection,
            direction,
            orientation,
        )
    }

    /// Arrow keys move to the next/previous enabled tab, wrapping around; Home/End skip
    /// disabled tabs (upstream: TabsKeyboardDelegate).
    #[test]
    fn skips_disabled_tabs_and_wraps() {
        with_owner(|| {
            let keys = RwSignal::new(vec!["a", "b", "c", "d"]);
            let delegate = delegate(
                keys,
                &["a", "c"],
                WritingDirection::Ltr,
                Orientation::Horizontal,
            );
            let options = NavigationOptions::default();
            assert_that!(delegate.key_right_of(&Key::from("b"), options))
                .is_equal_to(Some(Key::from("d")));
            assert_that!(delegate.key_right_of(&Key::from("d"), options))
                .is_equal_to(Some(Key::from("b")));
            assert_that!(delegate.key_left_of(&Key::from("b"), options))
                .is_equal_to(Some(Key::from("d")));
            assert_that!(delegate.first_key(None, false)).is_equal_to(Some(Key::from("b")));
            assert_that!(delegate.last_key(None, false)).is_equal_to(Some(Key::from("d")));
            // Horizontal tabs ignore Up/Down.
            assert_that!(delegate.key_below(&Key::from("b"), options)).is_none();
            assert_that!(delegate.key_above(&Key::from("b"), options)).is_none();
        });
    }

    /// Left/Right follow the reading direction in every orientation; vertical tabs move with
    /// Up/Down (upstream: "getKeyLeftOf/getKeyRightOf follow the locale's text direction").
    #[test]
    fn follows_direction_and_orientation() {
        with_owner(|| {
            let keys = RwSignal::new(vec!["a", "b", "c"]);
            let delegate = delegate(keys, &[], WritingDirection::Rtl, Orientation::Vertical);
            let options = NavigationOptions::default();
            assert_that!(delegate.key_left_of(&Key::from("a"), options))
                .is_equal_to(Some(Key::from("b")));
            assert_that!(delegate.key_right_of(&Key::from("a"), options))
                .is_equal_to(Some(Key::from("c")));
            assert_that!(delegate.key_below(&Key::from("a"), options))
                .is_equal_to(Some(Key::from("b")));
            assert_that!(delegate.key_above(&Key::from("a"), options))
                .is_equal_to(Some(Key::from("c")));
        });
    }

    /// From a tab no longer in the list (the focused tab was removed) with every remaining tab
    /// disabled, the delegate finds none instead of looping forever.
    #[test]
    fn a_removed_start_with_only_disabled_tabs_left_finds_none() {
        with_owner(|| {
            let keys = RwSignal::new(vec!["a", "b", "c"]);
            let delegate = delegate(
                keys,
                &["a", "c"],
                WritingDirection::Ltr,
                Orientation::Horizontal,
            );
            keys.set(vec!["a", "c"]);
            let options = NavigationOptions::default();
            assert_that!(delegate.key_right_of(&Key::from("b"), options)).is_none();
            assert_that!(delegate.key_left_of(&Key::from("b"), options)).is_none();
        });
    }

    /// With every tab disabled, Home/End stay on the first/last tab (as upstream, whose search
    /// ends at the tab it started from).
    #[test]
    fn all_disabled_stays_on_the_ends() {
        with_owner(|| {
            let keys = RwSignal::new(vec!["a", "b"]);
            let delegate = delegate(
                keys,
                &["a", "b"],
                WritingDirection::Ltr,
                Orientation::Horizontal,
            );
            assert_that!(delegate.first_key(None, false)).is_equal_to(Some(Key::from("a")));
            assert_that!(delegate.last_key(None, false)).is_equal_to(Some(Key::from("b")));
        });
    }
}
