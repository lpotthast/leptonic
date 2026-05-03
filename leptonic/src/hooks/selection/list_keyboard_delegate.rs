use std::collections::HashSet;

use leptos::prelude::*;

use super::{
    SelectionKey, keyboard_delegate::KeyboardDelegate, use_selection_state::DisabledBehavior,
};
use crate::{hooks::form::use_checkbox_group::Orientation, utils::locale::WritingDirection};

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/selection/src/ListKeyboardDelegate.ts

// REACT-ARIA DEVIATIONS
//
// ## API DIFFERENCES
//
// - Uses generic `K` key type and `Signal<Vec<K>>` instead of Collection<Node>.
// - No LayoutDelegate — page navigation is deferred (returns None by default).
//   The collection hook will add page navigation support via DOM measurement when
//   a collection_ref is available.
// - Left/right navigation methods return None for vertical stack layout (matching
//   react-aria which sets them to undefined for vertical stacks).
// - No typeahead search (get_key_for_search returns None). Typeahead is handled
//   separately by use_type_select which iterates keys with a label callback.

/// A keyboard navigation delegate for linear list collections.
///
/// Implements `KeyboardDelegate<K>` for simple ordered lists where items
/// are navigated sequentially. Automatically skips disabled keys.
///
/// For vertical orientation:
/// - Up/Down navigate between items
/// - Left/Right return None (no horizontal navigation)
///
/// For horizontal orientation:
/// - Left/Right navigate between items (RTL-aware)
/// - Up/Down return None (no vertical navigation)
#[derive(Clone)]
pub struct ListKeyboardDelegate<K>
where
    K: SelectionKey,
{
    /// All keys in order.
    all_keys: Signal<Vec<K>>,
    /// Keys that should be skipped during navigation.
    disabled_keys: Signal<HashSet<K>>,
    /// Layout orientation.
    orientation: Orientation,
    /// Writing direction for RTL support.
    direction: Signal<WritingDirection>,
    /// How disabled items behave — when `Selection`, disabled items are navigable.
    disabled_behavior: DisabledBehavior,
}

// Manual Copy impl to avoid derive adding K: Copy bound.
// All fields (Signal, Orientation, Signal) are Copy regardless of K.
impl<K: SelectionKey> Copy for ListKeyboardDelegate<K> {}

impl<K> ListKeyboardDelegate<K>
where
    K: SelectionKey,
{
    pub fn new(
        all_keys: Signal<Vec<K>>,
        disabled_keys: Signal<HashSet<K>>,
        orientation: Orientation,
        direction: Signal<WritingDirection>,
    ) -> Self {
        Self {
            all_keys,
            disabled_keys,
            orientation,
            direction,
            disabled_behavior: DisabledBehavior::default(),
        }
    }

    /// Create a delegate with a specific disabled behavior.
    #[must_use]
    pub fn with_disabled_behavior(mut self, disabled_behavior: DisabledBehavior) -> Self {
        self.disabled_behavior = disabled_behavior;
        self
    }

    /// Find the next key after the given key, skipping disabled keys when
    /// `disabled_behavior == All`.
    fn get_next_key(&self, key: &K) -> Option<K> {
        let keys = self.all_keys.get_untracked();
        let disabled = self.disabled_keys.get_untracked();
        let skip_disabled = self.disabled_behavior == DisabledBehavior::All;

        let current_idx = keys.iter().position(|k| k == key)?;

        for candidate in &keys[current_idx + 1..] {
            if skip_disabled && disabled.contains(candidate) {
                continue;
            }
            return Some(candidate.clone());
        }
        None
    }

    /// Find the previous key before the given key, skipping disabled keys when
    /// `disabled_behavior == All`.
    fn get_previous_key(&self, key: &K) -> Option<K> {
        let keys = self.all_keys.get_untracked();
        let disabled = self.disabled_keys.get_untracked();
        let skip_disabled = self.disabled_behavior == DisabledBehavior::All;

        let current_idx = keys.iter().position(|k| k == key)?;

        for candidate in keys[..current_idx].iter().rev() {
            if skip_disabled && disabled.contains(candidate) {
                continue;
            }
            return Some(candidate.clone());
        }
        None
    }
}

impl<K> KeyboardDelegate<K> for ListKeyboardDelegate<K>
where
    K: SelectionKey,
{
    fn get_key_below(&self, key: &K) -> Option<K> {
        if self.orientation == Orientation::Vertical {
            self.get_next_key(key)
        } else {
            None
        }
    }

    fn get_key_above(&self, key: &K) -> Option<K> {
        if self.orientation == Orientation::Vertical {
            self.get_previous_key(key)
        } else {
            None
        }
    }

    fn get_key_left_of(&self, key: &K) -> Option<K> {
        if self.orientation == Orientation::Horizontal {
            let is_rtl = self.direction.get_untracked() == WritingDirection::Rtl;
            if is_rtl {
                self.get_next_key(key)
            } else {
                self.get_previous_key(key)
            }
        } else {
            None
        }
    }

    fn get_key_right_of(&self, key: &K) -> Option<K> {
        if self.orientation == Orientation::Horizontal {
            let is_rtl = self.direction.get_untracked() == WritingDirection::Rtl;
            if is_rtl {
                self.get_previous_key(key)
            } else {
                self.get_next_key(key)
            }
        } else {
            None
        }
    }

    fn get_first_key(&self, _from_key: Option<&K>, _global: bool) -> Option<K> {
        let keys = self.all_keys.get_untracked();
        let disabled = self.disabled_keys.get_untracked();
        let skip_disabled = self.disabled_behavior == DisabledBehavior::All;

        keys.iter()
            .find(|k| !skip_disabled || !disabled.contains(*k))
            .cloned()
    }

    fn get_last_key(&self, _from_key: Option<&K>, _global: bool) -> Option<K> {
        let keys = self.all_keys.get_untracked();
        let disabled = self.disabled_keys.get_untracked();
        let skip_disabled = self.disabled_behavior == DisabledBehavior::All;

        keys.iter()
            .rev()
            .find(|k| !skip_disabled || !disabled.contains(*k))
            .cloned()
    }

    // Page navigation and search are left as default (None).
    // Page navigation requires DOM layout measurement which is handled
    // at the collection hook level. Type-ahead search is handled by
    // use_type_select which has access to label callbacks.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_delegate(
        keys: Vec<&'static str>,
        disabled: HashSet<&'static str>,
    ) -> ListKeyboardDelegate<&'static str> {
        ListKeyboardDelegate::new(
            Signal::stored(keys),
            Signal::stored(disabled),
            Orientation::Vertical,
            Signal::stored(WritingDirection::Ltr),
        )
    }

    #[test]
    fn test_get_key_below_basic() {
        let _owner = Owner::new();
        _owner.with(|| {
            let delegate = make_delegate(vec!["a", "b", "c"], HashSet::new());
            assert_eq!(delegate.get_key_below(&"a"), Some("b"));
            assert_eq!(delegate.get_key_below(&"b"), Some("c"));
            assert_eq!(delegate.get_key_below(&"c"), None);
        });
    }

    #[test]
    fn test_get_key_above_basic() {
        let _owner = Owner::new();
        _owner.with(|| {
            let delegate = make_delegate(vec!["a", "b", "c"], HashSet::new());
            assert_eq!(delegate.get_key_above(&"c"), Some("b"));
            assert_eq!(delegate.get_key_above(&"b"), Some("a"));
            assert_eq!(delegate.get_key_above(&"a"), None);
        });
    }

    #[test]
    fn test_skips_disabled_keys() {
        let _owner = Owner::new();
        _owner.with(|| {
            let delegate =
                make_delegate(vec!["a", "b", "c", "d"], ["b", "c"].into_iter().collect());
            assert_eq!(delegate.get_key_below(&"a"), Some("d"));
            assert_eq!(delegate.get_key_above(&"d"), Some("a"));
        });
    }

    #[test]
    fn test_first_last_skip_disabled() {
        let _owner = Owner::new();
        _owner.with(|| {
            let delegate = make_delegate(vec!["a", "b", "c"], ["a", "c"].into_iter().collect());
            assert_eq!(delegate.get_first_key(None, false), Some("b"));
            assert_eq!(delegate.get_last_key(None, false), Some("b"));
        });
    }

    #[test]
    fn test_horizontal_left_right_ltr() {
        let _owner = Owner::new();
        _owner.with(|| {
            let delegate = ListKeyboardDelegate::new(
                Signal::stored(vec!["a", "b", "c"]),
                Signal::stored(HashSet::new()),
                Orientation::Horizontal,
                Signal::stored(WritingDirection::Ltr),
            );
            assert_eq!(delegate.get_key_right_of(&"a"), Some("b"));
            assert_eq!(delegate.get_key_left_of(&"b"), Some("a"));
            // Vertical nav should return None for horizontal list
            assert_eq!(delegate.get_key_below(&"a"), None);
            assert_eq!(delegate.get_key_above(&"c"), None);
        });
    }

    #[test]
    fn test_horizontal_left_right_rtl() {
        let _owner = Owner::new();
        _owner.with(|| {
            let delegate = ListKeyboardDelegate::new(
                Signal::stored(vec!["a", "b", "c"]),
                Signal::stored(HashSet::new()),
                Orientation::Horizontal,
                Signal::stored(WritingDirection::Rtl),
            );
            // RTL: right goes to previous, left goes to next
            assert_eq!(delegate.get_key_right_of(&"b"), Some("a"));
            assert_eq!(delegate.get_key_left_of(&"b"), Some("c"));
        });
    }
}
