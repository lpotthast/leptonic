//! Keyboard navigation between drop positions within a collection.
//!
//! Based on react-aria's keyboard navigation logic in
//! `@react-aria/dnd/src/useDroppableCollection.ts`.

use super::types::{DropPosition, DropTarget};
use crate::hooks::selection::keyboard_delegate::KeyboardDelegate;

/// Navigation direction for keyboard-based drop target movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationDirection {
    /// Move to the next item (down or right).
    Next,
    /// Move to the previous item (up or left).
    Previous,
    /// Jump to the first item.
    First,
    /// Jump to the last item.
    Last,
}

/// Navigates to the next drop target position using a keyboard delegate.
///
/// Cycles through positions on each key: `Before → On → After`, then moves
/// to the next/previous key. Falls back to `Root` at collection boundaries.
///
/// # Arguments
///
/// * `keyboard_delegate` - Provides key ordering.
/// * `target` - The current drop target (or `None` if no target is focused).
/// * `direction` - Which direction to navigate.
/// * `is_valid` - Predicate to filter invalid targets.
#[must_use]
pub fn navigate_drop_target(
    keyboard_delegate: &dyn KeyboardDelegate<String>,
    target: Option<&DropTarget>,
    direction: NavigationDirection,
    is_valid: &dyn Fn(&DropTarget) -> bool,
) -> Option<DropTarget> {
    match direction {
        NavigationDirection::First => {
            let first_key = keyboard_delegate.get_first_key(None, true)?;
            find_valid_position(&first_key, true, is_valid)
        }
        NavigationDirection::Last => {
            let last_key = keyboard_delegate.get_last_key(None, true)?;
            find_valid_position_reverse(&last_key, is_valid)
        }
        NavigationDirection::Next => navigate_next(keyboard_delegate, target, is_valid),
        NavigationDirection::Previous => navigate_previous(keyboard_delegate, target, is_valid),
    }
}

/// Navigate forward: cycles Before → On → After on current key, then moves to next key.
fn navigate_next(
    delegate: &dyn KeyboardDelegate<String>,
    target: Option<&DropTarget>,
    is_valid: &dyn Fn(&DropTarget) -> bool,
) -> Option<DropTarget> {
    match target {
        None | Some(DropTarget::Root) => {
            // Start from the first key
            let first_key = delegate.get_first_key(None, true)?;
            find_valid_position(&first_key, true, is_valid)
        }
        Some(DropTarget::Item { key, position }) => {
            // Try the next position on the same key
            let next_positions = match position {
                DropPosition::Before => vec![DropPosition::On, DropPosition::After],
                DropPosition::On => vec![DropPosition::After],
                DropPosition::After => vec![],
            };

            for pos in next_positions {
                let candidate = DropTarget::Item {
                    key: key.clone(),
                    position: pos,
                };
                if is_valid(&candidate) {
                    return Some(candidate);
                }
            }

            // Move to the next key
            let mut current_key = key.clone();
            while let Some(next_key) = delegate.get_key_below(&current_key) {
                if let Some(target) = find_valid_position(&next_key, true, is_valid) {
                    return Some(target);
                }
                current_key = next_key;
            }

            // Reached the end → try root as a wrap-around target
            if is_valid(&DropTarget::Root) {
                return Some(DropTarget::Root);
            }

            None
        }
    }
}

/// Navigate backward: cycles After → On → Before on current key, then moves to previous key.
fn navigate_previous(
    delegate: &dyn KeyboardDelegate<String>,
    target: Option<&DropTarget>,
    is_valid: &dyn Fn(&DropTarget) -> bool,
) -> Option<DropTarget> {
    match target {
        None => {
            // Start from the last key
            let last_key = delegate.get_last_key(None, true)?;
            find_valid_position_reverse(&last_key, is_valid)
        }
        Some(DropTarget::Root) => {
            // Root → go to last key's last position
            let last_key = delegate.get_last_key(None, true)?;
            find_valid_position_reverse(&last_key, is_valid)
        }
        Some(DropTarget::Item { key, position }) => {
            // Try the previous position on the same key
            let prev_positions = match position {
                DropPosition::After => vec![DropPosition::On, DropPosition::Before],
                DropPosition::On => vec![DropPosition::Before],
                DropPosition::Before => vec![],
            };

            for pos in prev_positions {
                let candidate = DropTarget::Item {
                    key: key.clone(),
                    position: pos,
                };
                if is_valid(&candidate) {
                    return Some(candidate);
                }
            }

            // Move to the previous key
            let mut current_key = key.clone();
            while let Some(prev_key) = delegate.get_key_above(&current_key) {
                if let Some(target) = find_valid_position_reverse(&prev_key, is_valid) {
                    return Some(target);
                }
                current_key = prev_key;
            }

            // Reached the start → try root
            if is_valid(&DropTarget::Root) {
                return Some(DropTarget::Root);
            }

            None
        }
    }
}

/// Finds the first valid position on the given key (Before, On, After).
fn find_valid_position(
    key: &str,
    _forward: bool,
    is_valid: &dyn Fn(&DropTarget) -> bool,
) -> Option<DropTarget> {
    for position in [DropPosition::Before, DropPosition::On, DropPosition::After] {
        let candidate = DropTarget::Item {
            key: key.to_owned(),
            position,
        };
        if is_valid(&candidate) {
            return Some(candidate);
        }
    }
    None
}

/// Finds the last valid position on the given key (After, On, Before).
fn find_valid_position_reverse(
    key: &str,
    is_valid: &dyn Fn(&DropTarget) -> bool,
) -> Option<DropTarget> {
    for position in [DropPosition::After, DropPosition::On, DropPosition::Before] {
        let candidate = DropTarget::Item {
            key: key.to_owned(),
            position,
        };
        if is_valid(&candidate) {
            return Some(candidate);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A simple test keyboard delegate for a fixed list.
    struct TestDelegate {
        keys: Vec<String>,
    }

    impl KeyboardDelegate<String> for TestDelegate {
        fn get_key_below(&self, key: &String) -> Option<String> {
            let idx = self.keys.iter().position(|k| k == key)?;
            self.keys.get(idx + 1).cloned()
        }

        fn get_key_above(&self, key: &String) -> Option<String> {
            let idx = self.keys.iter().position(|k| k == key)?;
            if idx == 0 {
                return None;
            }
            self.keys.get(idx - 1).cloned()
        }

        fn get_key_left_of(&self, key: &String) -> Option<String> {
            self.get_key_above(key)
        }

        fn get_key_right_of(&self, key: &String) -> Option<String> {
            self.get_key_below(key)
        }

        fn get_first_key(&self, _from: Option<&String>, _global: bool) -> Option<String> {
            self.keys.first().cloned()
        }

        fn get_last_key(&self, _from: Option<&String>, _global: bool) -> Option<String> {
            self.keys.last().cloned()
        }
    }

    fn accept_before_after(target: &DropTarget) -> bool {
        match target {
            DropTarget::Root => false,
            DropTarget::Item { position, .. } => {
                matches!(position, DropPosition::Before | DropPosition::After)
            }
        }
    }

    #[test]
    fn navigate_next_from_none_goes_to_first_key() {
        let delegate = TestDelegate {
            keys: vec!["a".into(), "b".into(), "c".into()],
        };
        let result = navigate_drop_target(
            &delegate,
            None,
            NavigationDirection::Next,
            &accept_before_after,
        );
        assert_eq!(
            result,
            Some(DropTarget::Item {
                key: "a".into(),
                position: DropPosition::Before,
            })
        );
    }

    #[test]
    fn navigate_next_advances_position_then_key() {
        let delegate = TestDelegate {
            keys: vec!["a".into(), "b".into()],
        };
        let current = DropTarget::Item {
            key: "a".into(),
            position: DropPosition::Before,
        };
        let result = navigate_drop_target(
            &delegate,
            Some(&current),
            NavigationDirection::Next,
            &accept_before_after,
        );
        assert_eq!(
            result,
            Some(DropTarget::Item {
                key: "a".into(),
                position: DropPosition::After,
            })
        );
    }

    #[test]
    fn navigate_previous_from_after_goes_to_before() {
        let delegate = TestDelegate {
            keys: vec!["a".into(), "b".into()],
        };
        let current = DropTarget::Item {
            key: "a".into(),
            position: DropPosition::After,
        };
        let result = navigate_drop_target(
            &delegate,
            Some(&current),
            NavigationDirection::Previous,
            &accept_before_after,
        );
        assert_eq!(
            result,
            Some(DropTarget::Item {
                key: "a".into(),
                position: DropPosition::Before,
            })
        );
    }

    #[test]
    fn navigate_first_goes_to_first_key_before() {
        let delegate = TestDelegate {
            keys: vec!["a".into(), "b".into(), "c".into()],
        };
        let current = DropTarget::Item {
            key: "c".into(),
            position: DropPosition::After,
        };
        let result = navigate_drop_target(
            &delegate,
            Some(&current),
            NavigationDirection::First,
            &accept_before_after,
        );
        assert_eq!(
            result,
            Some(DropTarget::Item {
                key: "a".into(),
                position: DropPosition::Before,
            })
        );
    }
}
